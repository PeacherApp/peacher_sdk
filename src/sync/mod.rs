mod client;

pub use client::*;

// Note: legislation_sync module kept for reference but not currently used. Needs to be removed soon.
// mod legislation_sync;
// use legislation_sync::*;

mod builder;
pub use builder::*;

use ahash::{HashMap, HashSet};
use reqwest::StatusCode;
use thiserror::Error;

use crate::prelude::*;
use chrono::NaiveDate;
use tracing::{error, info};

/// Result of syncing jurisdiction and its chambers
#[derive(Debug, Clone)]
pub struct JurisdictionAndChambersSyncResult {
    pub jurisdiction_id: i32,
    pub jurisdiction_name: String,
    pub jurisdiction_created: bool,
    pub chambers_created: Vec<ListChamberResponse>,
    pub chambers_updated: Vec<ListChamberResponse>,
}

/// Result of syncing sessions
#[derive(Debug, Clone)]
pub struct SessionsSyncResult {
    pub created: Vec<GetSessionResponse>,
    pub updated: Vec<GetSessionResponse>,
}

/// Result of syncing members
#[derive(Debug, Clone)]
pub struct MembersSyncResult {
    pub created: Vec<MemberView>,
    pub updated: Vec<MemberView>,
}

/// Result of syncing legislation
#[derive(Debug, Clone)]
pub struct LegislationSyncResult {
    pub created: Vec<LegislationView>,
    pub updated: Vec<LegislationView>,
    /// True if stopped due to hitting known items (optimization)
    pub stopped_early: bool,
}

/// Result of syncing votes
#[derive(Debug)]
pub struct VotesSyncResult {
    pub created: Vec<i32>, // Vote IDs
    pub updated: Vec<i32>,
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("{0}")]
    Sdk(#[from] SdkError),

    #[error("Missing reference: {0} with external_id {1}")]
    MissingReference(&'static str, ExternalId),

    #[error(
        "The jurisdiction for the external client was not found. Create it first or enable dangerously_create."
    )]
    JurisdictionNotFound,

    #[error("Chamber not found: {0}")]
    ChamberNotFound(ExternalId),

    #[error("Session not found: {0}")]
    SessionNotFound(ExternalId),

    #[error("Member not found: {0}")]
    MemberNotFound(ExternalId),

    #[error("Votes do not exist for {0}")]
    NoVotes(ExternalId),
}

// ============================================================================
// Legacy Types (for backward compatibility during transition)
// ============================================================================

pub struct SyncSessionView {
    pub id: i32,
    pub name: String,
    pub current: bool,
    pub external: ExternalOwner,
    pub starts_at: Option<NaiveDate>,
    pub ends_at: Option<NaiveDate>,
    pub chambers: Vec<SmallChamberView>,
}

pub struct SyncChamberSessionView {
    pub session_id: i32,
    pub chamber: ChamberSessionView,
}

/// Coordinates synchronization between an external data source and the Peacher API.
///
/// # Design Principles
///
/// 1. **API is purely CRUD** - No knowledge of ExternalClient or sync concerns
/// 2. **ApiSync is just an API consumer** - Like a web frontend or API user
/// 3. **Minimize ExternalClient calls** - Peacher API can be called freely, external sources are expensive
/// 4. **Query first, then create/update** - Check existence via list endpoints, then decide
/// 5. **Fail if references missing** - Sync order matters (jurisdiction → chambers → sessions → members → legislation → votes)
pub struct ApiSync<'p, E, P = PeacherClient> {
    external: E,
    peacher: &'p P,
    /// Cached jurisdiction ID (resolved during build)
    jurisdiction_id: Option<i32>,
    ext_config: ExternalClientConfig,
}

impl<'p, E: ExternalClient, P> ApiSync<'p, E, P> {
    pub fn new_from_clients(external: E, peacher: &'p P) -> Self {
        let ext_config = external.config();
        Self {
            external,
            peacher,
            jurisdiction_id: None,
            ext_config,
        }
    }

    pub fn peacher(&self) -> &'p P {
        self.peacher
    }

    pub fn external(&self) -> &E {
        &self.external
    }

    pub fn jurisdiction_id(&self) -> Option<i32> {
        self.jurisdiction_id
    }
    /// Adjust the running config for the sync client.
    pub fn config_mut(&mut self) -> &mut ExternalClientConfig {
        &mut self.ext_config
    }
    pub fn config(&self) -> &ExternalClientConfig {
        &self.ext_config
    }
}

impl<'p, E: ExternalClient> ApiSync<'p, E> {
    pub fn new(external: E, peacher: &'p PeacherClient) -> Self {
        Self::new_from_clients(external, peacher)
    }
}

impl<'p, E, P> ApiSync<'p, E, P>
where
    E: ExternalClient,
    P: Client,
{
    pub fn builder(external_client: E, peacher_client: &'p P) -> ApiSyncBuilder<'p, E, P> {
        ApiSyncBuilder::new(external_client, peacher_client)
    }

    // ========================================================================
    // ID Resolution Helpers
    // ========================================================================

    /// Resolve external chamber ID to internal ID
    pub async fn resolve_chamber(&self, ext_id: &ExternalId) -> Result<i32, SyncError> {
        let chambers = ListChambers::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;

        chambers
            .data
            .first()
            .map(|c| c.id)
            .ok_or_else(|| SyncError::ChamberNotFound(ext_id.clone()))
    }

    /// Resolve external session ID to internal ID
    pub async fn resolve_session(&self, ext_id: &ExternalId) -> Result<i32, SyncError> {
        let sessions = ListSessions(SessionParams::default().with_external_id(ext_id.val_str()))
            .request(self.peacher)
            .await?;

        sessions
            .data
            .first()
            .map(|s| s.id)
            .ok_or_else(|| SyncError::SessionNotFound(ext_id.clone()))
    }

    /// Resolve external member ID to internal ID
    pub async fn resolve_member(&self, ext_id: &ExternalId) -> Result<i32, SyncError> {
        let members = ListMembers::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;

        members
            .data
            .first()
            .map(|m| m.id)
            .ok_or_else(|| SyncError::MemberNotFound(ext_id.clone()))
    }

    /// Resolve external jurisdiction ID to internal ID
    pub async fn resolve_jurisdiction(&self, ext_id: &ExternalId) -> Result<i32, SyncError> {
        let jurisdictions = ListJurisdictions::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;

        jurisdictions
            .data
            .first()
            .map(|j| j.id)
            .ok_or(SyncError::JurisdictionNotFound)
    }

    /// helper to resolve the current jurisdiction based on the external client
    pub async fn resolve_internal_jurisdiction(&self) -> Result<i32, SyncError> {
        let jurisdiction = self.external.get_jurisdiction();
        let jurisdictions = ListJurisdictions::default()
            .with_external_id(jurisdiction.external_id.val_str())
            .request(self.peacher)
            .await?;

        jurisdictions
            .data
            .first()
            .map(|j| j.id)
            .ok_or(SyncError::JurisdictionNotFound)
    }

    // ========================================================================
    // Fine-grained Sync Methods
    // ========================================================================

    /// Sync jurisdiction AND chambers together.
    ///
    /// ExternalClient::get_jurisdiction() returns the jurisdiction with its chambers,
    /// so one client = one jurisdiction + its chambers. They sync together.
    pub async fn sync_jurisdiction_and_chambers(
        &mut self,
    ) -> Result<JurisdictionAndChambersSyncResult, SyncError> {
        let ext_jurisdiction = self.external.get_jurisdiction();

        info!(
            "Syncing jurisdiction '{}' (ext_id: {})",
            ext_jurisdiction.name,
            ext_jurisdiction.external_id.val_str()
        );

        // Check if jurisdiction exists
        let existing_jurisdictions = ListJurisdictions::default()
            .with_external_id(ext_jurisdiction.external_id.val_str())
            .request(self.peacher)
            .await?;

        let (jurisdiction_id, jurisdiction_name, jurisdiction_created) =
            if let Some(existing) = existing_jurisdictions.data.first() {
                info!(
                    "Jurisdiction '{}' already exists (id: {})",
                    existing.name, existing.id
                );
                (existing.id, existing.name.clone(), false)
            } else {
                // Create jurisdiction
                let mut ext_metadata = ExternalMetadata::new(ext_jurisdiction.external_id.clone());
                if let Some(ref url) = ext_jurisdiction.url {
                    ext_metadata.set_url(url.clone());
                }

                let created = CreateJurisdiction::new(&ext_jurisdiction.name)
                    .external_metadata(ext_metadata)
                    .request(self.peacher)
                    .await?;

                info!(
                    "Created jurisdiction '{}' (id: {})",
                    created.name, created.id
                );
                (created.id, created.name, true)
            };

        self.jurisdiction_id = Some(jurisdiction_id);

        // Sync chambers
        let existing_chambers = ListChambers::default()
            .with_jurisdiction(jurisdiction_id)
            .request(self.peacher)
            .await?;

        let existing_ext_ids: HashSet<String> = existing_chambers
            .data
            .iter()
            .filter_map(|c| {
                c.external
                    .as_ref()
                    .map(|e| e.external_id.val_str().to_string())
            })
            .collect();

        let mut chambers_created = Vec::new();
        let mut chambers_updated = Vec::new();

        for ext_chamber in &ext_jurisdiction.chambers {
            let ext_id_str = ext_chamber.external_id.val_str().to_string();

            if existing_ext_ids.contains(&ext_id_str) {
                // Chamber exists - we could call PATCH here, but for now just note it
                if let Some(existing) = existing_chambers.data.iter().find(|c| {
                    c.external
                        .as_ref()
                        .is_some_and(|e| e.external_id.val_str() == ext_id_str)
                }) {
                    chambers_updated.push(existing.clone());
                }
            } else {
                // Create chamber
                let mut chamber_req = CreateChamber::new(jurisdiction_id, &ext_chamber.name);
                let mut ext_metadata = ExternalMetadata::new(ext_chamber.external_id.clone());
                if let Some(ref url) = ext_chamber.url {
                    ext_metadata.set_url(url.clone());
                }
                chamber_req = chamber_req.external_metadata(ext_metadata);

                let created = chamber_req.request(self.peacher).await?;
                info!(
                    "Created chamber '{}' (id: {}, ext_id: {})",
                    created.name, created.id, ext_id_str
                );
                chambers_created.push(created);
            }
        }

        Ok(JurisdictionAndChambersSyncResult {
            jurisdiction_id,
            jurisdiction_name,
            jurisdiction_created,
            chambers_created,
            chambers_updated,
        })
    }

    /// Sync sessions for a jurisdiction.
    pub async fn sync_sessions(
        &self,
        jurisdiction_id: i32,
    ) -> Result<SessionsSyncResult, SyncError> {
        info!("Syncing sessions for jurisdiction {}", jurisdiction_id);

        // Get existing sessions from Peacher
        let existing_sessions =
            ListSessions(SessionParams::default().with_jurisdiction(jurisdiction_id))
                .request(self.peacher)
                .await?;

        let existing_ext_ids: HashMap<String, GetSessionResponse> = existing_sessions
            .data
            .iter()
            .filter_map(|s| {
                let ext_id = s
                    .external
                    .as_ref()
                    .map(|e| e.external_id.val_str().to_string())?;

                Some((ext_id, s.clone()))
            })
            .collect();

        // Get all chambers for this jurisdiction (needed to link sessions to chambers)
        let chambers = ListChambers::default()
            .with_jurisdiction(jurisdiction_id)
            .request(self.peacher)
            .await?;

        // Get sessions from external source
        let external_sessions = self.external.list_sessions().await?;

        let mut created = Vec::new();
        let mut updated = Vec::new();

        for ext_session in external_sessions {
            let ext_id_str = ext_session.external_id.val_str().to_string();

            match existing_ext_ids.get(&ext_id_str) {
                Some(id) => {
                    println!("here");
                    let response = UpdateSession::new(
                        id.id,
                        UpdateSessionRequest {
                            name: Some(ext_session.name),
                            starts_at: ext_session.starts_at,
                            ends_at: ext_session.ends_at,
                        },
                    )
                    .request(self.peacher())
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

                    let response = CreateSession::new(jurisdiction_id, session_req)
                        .request(self.peacher)
                        .await?;

                    info!(
                        "Created session '{}' (id: {}, ext_id: {})",
                        response.name, response.id, ext_id_str
                    );

                    // Link session to all chambers in the jurisdiction
                    // This is required because legislation and member_sessions have foreign key
                    // constraints on (chamber_id, session_id) referencing chamber_sessions
                    for chamber in &chambers.data {
                        match LinkChamberToSession::new(
                            response.id,
                            LinkSessionToChamberRequest::new(chamber.id),
                        )
                        .request(self.peacher)
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

    /// Sync members for a specific session and chamber.
    pub async fn sync_members(
        &self,
        session_id: i32,
        session_ext_id: &ExternalId,
        chamber_id: i32,
        chamber_ext_id: &ExternalId,
    ) -> Result<MembersSyncResult, SyncError> {
        info!(
            "Syncing members for session {} chamber {}",
            session_id, chamber_id
        );

        // Get existing members - we need to check by external_id
        // Note: The current API doesn't have a session/chamber filter on ListMembers
        // We'll get all members and filter, or use the chamber session endpoint
        let chamber_session = ListSessionMembers::new(chamber_id, session_id)
            .request(self.peacher)
            .await?;

        let existing_ext_ids: HashMap<String, i32> = chamber_session
            .members
            .iter()
            .filter_map(|m| {
                m.external
                    .as_ref()
                    .map(|e| (e.external_id.val_str().to_string(), m.member.id))
            })
            .collect();

        // Get members from external source
        let external_members = self
            .external
            .list_members(session_ext_id, chamber_ext_id)
            .await?;

        let mut created = Vec::new();
        let mut updated = Vec::new();

        for ext_member in external_members {
            let ext_id_str = ext_member.member.external_id.val_str().to_string();

            if let Some(&internal_id) = existing_ext_ids.get(&ext_id_str) {
                // Member exists in this session - call PATCH to update
                let update_req = ext_member.member.to_update_member_request();
                let member = UpdateMember::new(internal_id, update_req)
                    .request(self.peacher)
                    .await?;

                updated.push(member);
            } else {
                // Member not in this session - check if they exist globally
                let global_members = ListMembers::default()
                    .with_external_id(&ext_id_str)
                    .request(self.peacher)
                    .await?;

                let (member_id, is_new) = if let Some(existing) = global_members.data.first() {
                    // Member exists globally but not in this session - link them and update
                    info!(
                        "Member '{}' already exists (id: {}), linking to session {} and updating",
                        existing.display_name, existing.id, session_id
                    );

                    // Update member info (including party) for this session
                    let update_req = ext_member.member.to_update_member_request();
                    let _ = UpdateMember::new(existing.id, update_req)
                        .request(self.peacher)
                        .await?;

                    (existing.id, false)
                } else {
                    // Create new member
                    let create_req = ext_member.member.to_create_member_request();
                    let member = CreateMember::new(create_req).request(self.peacher).await?;

                    info!(
                        "Created member '{}' (id: {}, ext_id: {})",
                        member.display_name, member.id, ext_id_str
                    );
                    (member.id, true)
                };

                // Link to chamber/session
                let mut link_req = LinkMemberToChamber::new(chamber_id, session_id, member_id);

                if !self.ext_config.get_member_has_details {
                    link_req = link_req
                        .appointed_at(ext_member.appointed_at)
                        .expunged_at(ext_member.vacated_at);
                }
                if let Some(district_id) = ext_member.district_number {
                    link_req.set_district(Some(district_id));
                }

                link_req.request(self.peacher).await?;

                if is_new {
                    // Fetch the created member to return
                    let members = ListMembers::default()
                        .with_external_id(&ext_id_str)
                        .request(self.peacher)
                        .await?;
                    if let Some(member) = members.data.into_iter().next() {
                        created.push(member);
                    }
                }
            }
        }

        info!(
            "Members sync complete: {} created, {} updated",
            created.len(),
            updated.len()
        );

        Ok(MembersSyncResult { created, updated })
    }

    /// Sync a single member by external ID when discovered during operations like vote sync.
    ///
    /// This is called when we encounter a member reference that doesn't exist in the database.
    /// It fetches the member from the external source and creates them in Peacher.
    async fn sync_single_member(&self, member_ext_id: &ExternalId) -> Result<i32, SyncError> {
        info!("Auto-syncing missing member: {}", member_ext_id.val_str());

        // Check if member exists globally (might be in a different chamber/session)
        let existing = ListMembers::default()
            .with_external_id(member_ext_id.val_str())
            .request(self.peacher)
            .await?;

        if let Some(existing_member) = existing.data.first() {
            info!(
                "Member '{}' (id: {}) already exists globally",
                existing_member.display_name, existing_member.id
            );
            return Ok(existing_member.id);
        }

        // Member doesn't exist - fetch from external source
        let ext_member = self.external.get_member(member_ext_id).await?;

        // Create member
        let create_req = ext_member.to_create_member_request();
        let member = CreateMember::new(create_req).request(self.peacher).await?;

        info!(
            "Auto-synced member '{}' (id: {}, ext_id: {})",
            member.display_name,
            member.id,
            member_ext_id.val_str()
        );

        Ok(member.id)
    }

    /// Sync legislation for a session.
    pub async fn sync_legislation(
        &self,
        session_id: i32,
        session_ext_id: &ExternalId,
    ) -> Result<LegislationSyncResult, SyncError> {
        let config = self.external.config();

        info!(
            "Syncing legislation for session {} (ext_id: {})",
            session_id,
            session_ext_id.val_str()
        );

        // Get all existing legislation external_ids for this session
        let mut existing_ext_ids: HashMap<String, LegislationView> = HashMap::default();
        let mut page = 1u64;
        loop {
            let params = LegislationParams {
                session_id: Some(session_id),
                page: Some(page),
                page_size: Some(100),
                ..Default::default()
            };

            let result = params.request(self.peacher).await?;
            let is_empty = result.data.is_empty();

            for leg in result.data {
                if let Some(ref ext) = leg.external {
                    existing_ext_ids.insert(
                        ext.external_id.val_str().to_string(),
                        leg.into_legislation_view(),
                    );
                }
            }

            if result.page >= result.num_pages || is_empty {
                break;
            }
            page += 1;
        }

        info!(
            "Found {} existing legislation items",
            existing_ext_ids.len()
        );

        let mut created = Vec::new();
        let mut updated = Vec::new();
        let mut ext_page = 1u64;
        let page_size = 50u64;
        let mut consecutive_known = 0;
        let mut stopped_early = false;

        loop {
            let batch = self
                .external
                .fetch_legislation(session_ext_id, ext_page, page_size)
                .await?;

            if batch.data.is_empty() {
                break;
            }

            for ext_leg in batch.data {
                let ext_id_str = ext_leg.external_id.val_str().to_string();

                match existing_ext_ids.get(&ext_id_str) {
                    Some(leg) => {
                        consecutive_known += 1;
                        // TODO: we actually should update the legislation if possible.
                        // if no changes were made, then we will increase consecutive_known by 1.
                        updated.push(leg.clone());
                        // If ordering is Latest, we can stop early when hitting known items
                        if config.legislation_order == ExtOrder::Latest && consecutive_known > 10 {
                            info!(
                                "Hit {} consecutive known items, stopping early",
                                consecutive_known
                            );
                            stopped_early = true;
                            break;
                        }
                    }
                    None => {
                        consecutive_known = 0;

                        // Get chamber ID for this legislation
                        let chamber_id = if let Some(ref chamber_ext_id) = ext_leg.chamber_id {
                            self.resolve_chamber(chamber_ext_id).await?
                        } else {
                            // Default to first chamber or fail
                            return Err(SyncError::MissingReference(
                                "chamber",
                                ext_leg.external_id.clone(),
                            ));
                        };

                        let req = ext_leg.into_create_legislation_request();

                        // Create legislation
                        let leg = CreateLegislation::new(chamber_id, session_id, req)
                            .request(self.peacher)
                            .await?;

                        info!(
                            "Created legislation '{}' (id: {}, ext_id: {})",
                            leg.name_id, leg.id, ext_id_str
                        );
                        created.push(leg);
                    }
                }
            }

            if stopped_early || batch.page >= batch.num_pages {
                break;
            }
            ext_page += 1;
        }

        info!(
            "Legislation sync complete: {} created, {} updated, stopped_early: {}",
            created.len(),
            updated.len(),
            stopped_early
        );

        Ok(LegislationSyncResult {
            created,
            updated,
            stopped_early,
        })
    }

    /// Sync votes for a piece of legislation.
    pub async fn sync_votes(
        &self,
        legislation_id: i32,
        legislation_ext_id: &ExternalId,
    ) -> Result<VotesSyncResult, SyncError> {
        info!(
            "Syncing votes for legislation {} (ext_id: {})",
            legislation_id,
            legislation_ext_id.val_str()
        );

        // Get existing votes from Peacher
        // Note: We'd need external vote IDs to check existence
        // For now, we create all votes (API handles duplicates via external_id)
        let _existing_votes = GetLegislationVotes(legislation_id)
            .request(self.peacher)
            .await?;

        // Fetch votes from external source
        let external_legislation = self.external.get_legislation(legislation_ext_id).await?;
        let Some(external_votes) = external_legislation.votes else {
            return Err(SyncError::NoVotes(legislation_ext_id.clone()));
        };

        let mut created = Vec::new();
        let mut updated = Vec::new();

        for ext_vote in external_votes {
            // Build member votes - need to resolve external member IDs to internal IDs
            let mut member_votes = Vec::new();
            for ext_member_vote in &ext_vote.votes {
                let member_id = match self.resolve_member(&ext_member_vote.member_id).await {
                    Ok(id) => id,
                    Err(SyncError::MemberNotFound(ref ext_id)) => {
                        info!(
                            "Member {} not found in database, attempting to sync from external source",
                            ext_id.val_str()
                        );

                        match self.ext_config.behavior_when_member_doesnt_exist {
                            MemberAction::Skip => {
                                continue;
                            }
                            MemberAction::Create => {
                                // Try to sync the missing member
                                match self.sync_single_member(&ext_member_vote.member_id).await {
                                    Ok(synced_id) => {
                                        info!(
                                            "Successfully auto-synced member {} (id: {})",
                                            ext_member_vote.member_id.val_str(),
                                            synced_id
                                        );
                                        synced_id
                                    }
                                    Err(sync_err) => {
                                        // Production case: member can't be fetched from external API
                                        // Fail the entire vote sync per user's requirement
                                        error!(
                                            "Failed to auto-sync member {}: {:?}",
                                            ext_member_vote.member_id.val_str(),
                                            sync_err
                                        );
                                        return Err(SyncError::MemberNotFound(
                                            ext_member_vote.member_id.clone(),
                                        ));
                                    }
                                }
                            }
                            MemberAction::Fail => {
                                return Err(SyncError::MemberNotFound(
                                    ext_member_vote.member_id.clone(),
                                ));
                            }
                        }
                    }
                    Err(other_err) => {
                        // Other resolution errors (network, SDK errors, etc.)
                        return Err(other_err);
                    }
                };

                member_votes.push(MemberVoteInput::new(member_id, ext_member_vote.vote));
            }

            // Create vote

            let mut ext_metadata = ExternalMetadata::new(ext_vote.external_id.clone());

            if let Some(url) = ext_vote.url {
                ext_metadata.set_url(url.clone());
            }

            let vote_name = ext_vote.vote_name.clone();
            let ext_vote_id = ext_vote.external_id.val_str().to_owned();
            let vote_req = CreateVoteRequest {
                name: ext_vote.vote_name.clone(),
                occurred_at: ext_vote.date_occurred,
                member_votes: member_votes.clone(),
                external_metadata: Some(ext_metadata),
                vote_type: ext_vote.vote_type,
            };

            match CreateVote::new(legislation_id, vote_req)
                .request(self.peacher)
                .await
            {
                Ok(vote_id) => {
                    info!(
                        "Created vote '{}' (id: {}, ext_id: {})",
                        vote_name, vote_id, ext_vote_id
                    );
                    created.push(vote_id);
                }
                Err(e) => {
                    info!(
                        "Failed to create vote '{}': {} (may already exist)",
                        vote_name, e
                    );
                    if self.ext_config.behavior_when_legislation_vote_exists
                        == LegVoteAction::Update
                        && let SdkError::Status(StatusCode::CONFLICT, val) = e
                        && let Ok(err) = serde_json::from_str::<ErrorResponse>(&val)
                        && let Ok(id) = err.description.parse()
                    {
                        let req = UpdateVoteRequest {
                            name: Some(ext_vote.vote_name),
                            occurred_at: ext_vote.date_occurred,
                            member_votes: Some(member_votes),
                        };

                        UpdateVote::new(legislation_id, id, req)
                            .request(self.peacher)
                            .await?;

                        info!("here in update");
                        updated.push(id);
                        //we can update
                    }
                }
            }
        }

        info!(
            "Votes sync complete: {} created, {} updated",
            created.len(),
            updated.len()
        );

        Ok(VotesSyncResult { created, updated })
    }
}
