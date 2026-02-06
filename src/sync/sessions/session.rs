use std::sync::Arc;

use crate::prelude::*;

pub struct SessionSync<'caller, 'client, E, P> {
    session: ExternalId,
    external: &'caller E,
    mapper: &'caller mut ClientMapper<'client, P>,
}

impl<'caller, 'client, E: ExternalClient, P: Client> SessionSync<'caller, 'client, E, P> {
    pub fn new(
        session: ExternalId,
        external: &'caller E,
        mapper: &'caller mut ClientMapper<'client, P>,
    ) -> Self {
        Self {
            session,
            external,
            mapper,
        }
    }
    pub async fn delete(self) -> SyncResult<()> {
        let session = self.mapper.session(&self.session).await?;
        DeleteSession(session.id)
            .request(self.mapper.client())
            .await?;
        Ok(())
    }

    pub async fn get(&mut self) -> SyncResult<Arc<GetSessionResponse>> {
        let session = self.mapper.session(&self.session).await?;
        Ok(session)
    }

    pub fn legislation<'slf>(&'slf mut self) -> LegislationSync<'slf, 'client, E, P> {
        LegislationSync::new(self.session.clone(), self.external, self.mapper)
    }

    /// Get a sync instance for a specific chamber of the session.
    pub fn members<'slf, 'chamber>(
        &'slf mut self,
        chamber: &'chamber ExternalId,
    ) -> MembersSync<'slf, 'chamber, 'client, E, P> {
        MembersSync::new(&self.session, chamber, self.external, self.mapper)
    }
    pub async fn sync_all_members(&mut self) -> SyncResult<Vec<MembersSyncResult>> {
        let session = self.mapper.session(&self.session).await?;

        let mut responses = Vec::with_capacity(session.chambers.len());

        for chamber in &session.chambers {
            let Some(external) = &chamber.external else {
                continue;
            };

            match self.members(&external.external_id).sync().await {
                Ok(result) => responses.push(result),
                Err(SyncError::NotFound(_)) => {}
                Err(e) => return Err(e),
            };
        }

        Ok(responses)
    }
}
