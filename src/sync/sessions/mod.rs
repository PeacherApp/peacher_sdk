mod members;
pub use members::*;

mod session;
pub use session::*;

mod legislation;
pub use legislation::*;

use ahash::HashMap;
use tracing::info;

use crate::prelude::*;

/// Result of syncing sessions
#[derive(Debug, Clone)]
pub struct SessionsSyncResult {
    pub created: Vec<GetSessionResponse>,
    pub updated: Vec<GetSessionResponse>,
}

pub struct AllSessionsSync<'c, 's, E, P> {
    external: &'s E,
    mapper: &'c mut ClientMapper<'s, P>,
}

impl<'c, 's, E: ExternalClient, P: Client> AllSessionsSync<'c, 's, E, P> {
    pub fn new(external: &'s E, mapper: &'c mut ClientMapper<'s, P>) -> Self {
        Self { external, mapper }
    }

    pub async fn session<'m>(
        &'m mut self,
        id: &ExternalId,
    ) -> SyncResult<SessionSync<'m, 's, E, P>> {
        Ok(SessionSync::new(id.clone(), &self.external, self.mapper))
    }

    pub async fn with_session_id<'m>(
        &'m mut self,
        id: i32,
    ) -> SyncResult<SessionSync<'m, 's, E, P>> {
        let session = GetSession(id).request(self.mapper.client()).await?;

        let Some(external_owner) = session.external else {
            return Err(SyncError::no_external_id(session));
        };

        Ok(SessionSync::new(
            external_owner.external_id,
            &self.external,
            &mut self.mapper,
        ))
    }

    /// Sync the available sessions.
    ///
    /// ### Does
    /// - Create or update sessions
    /// - Link the chambers to these sessions
    ///
    /// ### Does not
    /// - Sync members
    /// - Sync legislation
    pub async fn sync_sessions(&mut self) -> SyncResult<SessionsSyncResult> {
        let jurisdiction_id = self.external.get_jurisdiction();
        let jurisdiction = self
            .mapper
            .jurisdiction(&jurisdiction_id.external_id)
            .await?;

        info!("Syncing sessions for jurisdiction {}", jurisdiction.id);

        let existing_sessions =
            ListSessions(SessionParams::default().with_jurisdiction(jurisdiction.id))
                .request(self.mapper.client())
                .await?
                .data
                .into_iter()
                .filter_map(|session| {
                    let external_id = session.external.as_ref()?.external_id.clone();
                    Some((external_id, session))
                })
                .collect::<HashMap<_, _>>();

        // Get all chambers for this jurisdiction (needed to link sessions to chambers)
        let chambers = ListChambers::default()
            .with_jurisdiction(jurisdiction.id)
            .request(self.mapper.client())
            .await?;

        // Get sessions from external source
        let external_sessions = self.external.list_sessions().await?;

        let mut created = Vec::new();
        let mut updated = Vec::new();

        for ext_session in external_sessions {
            match existing_sessions.get(&ext_session.external_id) {
                Some(id) => {
                    let response = UpdateSession::new(
                        id.id,
                        UpdateSessionRequest {
                            name: Some(ext_session.name),
                            starts_at: ext_session.starts_at,
                            ends_at: ext_session.ends_at,
                        },
                    )
                    .request(self.mapper.client())
                    .await?;
                    updated.push(response);
                }
                None => {
                    // Create session
                    let mut session_req = CreateSessionRequest::new(&ext_session.name);
                    if let Some(starts_at) = ext_session.starts_at {
                        session_req = session_req.starts_at(starts_at);
                    }
                    if let Some(ends_at) = ext_session.ends_at {
                        session_req = session_req.ends_at(ends_at);
                    }

                    let mut ext_metadata = ExternalMetadata::new(ext_session.external_id.clone());
                    if let Some(ref url) = ext_session.url {
                        ext_metadata.set_url(url.clone());
                    }
                    session_req = session_req.external_metadata(ext_metadata);

                    let response = CreateSession::new(jurisdiction.id, session_req)
                        .request(self.mapper.client())
                        .await?;

                    info!(
                        "Created session '{}' (id: {}, ext_id: {})",
                        response.name, response.id, ext_session.external_id
                    );

                    // Link session to all chambers in the jurisdiction
                    // This is required because legislation and member_sessions have foreign key
                    // constraints on (chamber_id, session_id) referencing chamber_sessions
                    for chamber in &chambers.data {
                        match LinkChamberToSession::new(
                            response.id,
                            LinkSessionToChamberRequest::new(chamber.id),
                        )
                        .request(self.mapper.client())
                        .await
                        {
                            Ok(_) => {
                                info!(
                                    "Linked session {} to chamber {} ('{}')",
                                    response.id, chamber.id, chamber.name
                                );
                            }
                            Err(SdkError::Status(status, _)) if status == 409 => {
                                // Already linked, this is fine
                                info!(
                                    "Session {} already linked to chamber {} ('{}')",
                                    response.id, chamber.id, chamber.name
                                );
                            }
                            Err(e) => {
                                // Fail on actual errors - if linking fails, downstream operations will also fail
                                return Err(SyncError::Sdk(e));
                            }
                        }
                    }

                    created.push(response);
                }
            }
        }

        info!(
            "Sessions sync complete: {} created, {} existing",
            created.len(),
            updated.len()
        );

        Ok(SessionsSyncResult { created, updated })
    }
}
