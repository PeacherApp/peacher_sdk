mod client;
pub use client::*;

mod error;
pub use error::*;

mod jurisdiction;
pub use jurisdiction::*;

mod sessions;
pub use sessions::*;

mod external_map;
pub use external_map::*;

// Note: legislation_sync module kept for reference but not currently used. Needs to be removed soon.
// mod legislation_sync;
// use legislation_sync::*;

use ahash::HashMap;
use reqwest::StatusCode;

use crate::prelude::*;
use chrono::NaiveDate;
use tracing::{error, info};

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

#[derive(Debug)]
pub struct LegislationDetailSyncResult {
    pub legislation: LegislationView,
    pub votes: Option<VotesSyncResult>,
}

/// Result of syncing votes
#[derive(Debug)]
pub struct VotesSyncResult {
    pub created: Vec<i32>, // Vote IDs
    pub updated: Vec<i32>,
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
}

impl<'p, E: ExternalClient, P: Client> ApiSync<'p, E, P> {
    pub fn new(external: E, peacher: &'p P) -> Self {
        Self { external, peacher }
    }

    pub fn peacher(&self) -> &'p P {
        self.peacher
    }

    pub fn mapper(&self) -> ExternalIdQuery<'p, P> {
        ExternalIdQuery::new(self.peacher())
    }

    pub fn external(&self) -> &E {
        &self.external
    }

    /// Sync jurisdiction AND chambers together.
    ///
    /// ExternalClient::get_jurisdiction() returns the jurisdiction with its chambers,
    /// so one client = one jurisdiction + its chambers. They sync together.
    pub fn jurisdiction(&self) -> JurisdictionSync<'_, E, P> {
        JurisdictionSync::new(&self.external, self.peacher)
    }

    /// Sync sessions for a jurisdiction.
    pub fn sessions(&self) -> AllSessionsSync<'_, E, P> {
        AllSessionsSync::new(&self.external, self.peacher)
    }

    pub async fn sync_all(&self) -> SyncResult<()> {
        self.jurisdiction().sync().await?;
        self.sessions().sync_sessions().await?;
        todo!()
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

                link_req = link_req
                    .appointed_at(ext_member.appointed_at)
                    .expunged_at(ext_member.vacated_at);
                link_req.set_district(ext_member.district_number);
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

    /// This will call `get_legislation` for all known legislation
    pub async fn sync_known_legislation_details(
        &self,
        session_id: i32,
        start_at_page: u64,
    ) -> Result<(), SyncError> {
        info!("syncing with legislation details");
        let mut page = start_at_page;
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
                let Some(ext) = leg.external else {
                    continue;
                };

                self.sync_legislation_details(leg.id, &ext.external_id)
                    .await?;
            }

            if result.page >= result.num_pages || is_empty {
                break;
            }
            page += 1;
        }
        Ok(())
    }

    /// Sync legislation for a session.
    pub async fn sync_legislation(
        &self,
        session_id: i32,
        session_ext_id: &ExternalId,
        start_at_page: u64,
    ) -> Result<LegislationSyncResult, SyncError> {
        info!(
            "Syncing legislation for session {} (ext_id: {}) page {start_at_page}",
            session_id,
            session_ext_id.val_str()
        );

        // Get all existing legislation external_ids for this session
        let mut existing_ext_ids: HashMap<String, LegislationView> = HashMap::default();
        let mut page = start_at_page;
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
            let batch = attempt_request(3, |val| {
                *val = format!("External Fetch Legislation Request for {session_ext_id}, (page: {ext_page}, page_size: {page_size})");
                self.external
                    .fetch_legislation(session_ext_id, ext_page, page_size)
            })
            .await?;
            if batch.data.is_empty() {
                break;
            }

            for ext_leg in batch.data {
                let ext_id_str = ext_leg.external_id.val_str().to_string();

                match existing_ext_ids.get(&ext_id_str) {
                    Some(leg) => {
                        info!(
                            "Found existing '{}' (id: {}, ext_id: {})",
                            leg.name_id, leg.id, ext_id_str
                        );
                        consecutive_known += 1;
                        // TODO: we actually should update the legislation if possible.
                        // if no changes were made, then we will increase consecutive_known by 1.
                        updated.push(leg.clone());
                        // If ordering is Latest, we can stop early when hitting known items
                        if consecutive_known > 10 {
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

                        let leg = attempt_request(3, |name| {
                            *name = format!("Creating legislation for {chamber_id}, {req:?}");
                            CreateLegislation::new(chamber_id, session_id, req.clone())
                                .request(self.peacher)
                        })
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
    pub async fn sync_legislation_details(
        &self,
        legislation_id: i32,
        legislation_ext_id: &ExternalId,
    ) -> Result<LegislationDetailSyncResult, SyncError> {
        info!(
            "Syncing votes for legislation {} (ext_id: {})",
            legislation_id,
            legislation_ext_id.val_str()
        );

        // Fetch votes from external source
        let external_legislation = attempt_request(3, |name| {
            *name = format!("`get_legislation` for {legislation_ext_id}");
            self.external.get_legislation(legislation_ext_id)
        })
        .await?;

        let votes_sync_result = match external_legislation.votes.clone() {
            Some(external_votes) => {
                Some(sync_leg_votes(self, legislation_id, external_votes).await?)
            }
            None => None,
        };

        info!("Updating legislation details...");
        let result = UpdateLegislation::new(
            legislation_id,
            external_legislation.into_update_legislation_request(),
        )
        .request(self.peacher)
        .await?;
        Ok(LegislationDetailSyncResult {
            legislation: result,
            votes: votes_sync_result,
        })
    }
}

async fn attempt_request<F, Fut, T, E>(retries: u32, mut request: F) -> Result<T, E>
where
    F: FnMut(&mut String) -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut retry_count = 0;
    loop {
        let mut value = String::new();

        match request(&mut value).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                retry_count += 1;

                tracing::error!("Error performing request({value}): {e:?}");

                if retry_count >= retries {
                    return Err(e);
                }

                //todo
            }
        };
    }
}

async fn sync_leg_votes<'p, E, P>(
    sync: &ApiSync<'p, E, P>,
    legislation_id: i32,

    external_votes: impl IntoIterator<Item = ExternalLegislationVote>,
) -> Result<VotesSyncResult, SyncError>
where
    E: ExternalClient,
    P: Client,
{
    let mut created = Vec::new();
    let mut updated = Vec::new();

    for ext_vote in external_votes {
        // Build member votes - need to resolve external member IDs to internal IDs
        let mut member_votes = Vec::new();
        for ext_member_vote in &ext_vote.votes {
            let member_id = match sync.resolve_member(&ext_member_vote.member_id).await {
                Ok(id) => id,
                Err(SyncError::MemberNotFound(ref ext_id)) => {
                    info!(
                        "Member {} not found in database, attempting to sync from external source",
                        ext_id.val_str()
                    );

                    match sync.ext_config.behavior_when_member_doesnt_exist {
                        MemberAction::Skip => {
                            continue;
                        }
                        MemberAction::Create => {
                            // Try to sync the missing member
                            match sync.sync_single_member(&ext_member_vote.member_id).await {
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
            .request(sync.peacher())
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
                    "Failed to create vote '{}': {} (may already exist).",
                    vote_name, e
                );
                if sync.ext_config.behavior_when_legislation_vote_exists == LegVoteAction::Update
                    && let SdkError::Status(StatusCode::CONFLICT, val) = e
                    && let Ok(err) = serde_json::from_str::<ErrorResponse>(&val)
                    && let Ok(id) = err.description.parse()
                {
                    info!("Updating vote");
                    let req = UpdateVoteRequest {
                        name: Some(ext_vote.vote_name),
                        occurred_at: ext_vote.date_occurred,
                        member_votes: Some(member_votes),
                    };

                    UpdateVote::new(legislation_id, id, req)
                        .request(sync.peacher())
                        .await?;

                    updated.push(id);
                    //we can update
                }
            }
        }
    }

    Ok(VotesSyncResult { created, updated })
}
