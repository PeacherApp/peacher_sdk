use ahash::HashMap;
use http::StatusCode;
use tracing::info;

use crate::prelude::*;

pub struct LegislationSync<'s, E, P> {
    session: ExternalId,
    external: &'s E,
    peacher: &'s P,
}

impl<'s, E: ExternalClient, P: Client> LegislationSync<'s, E, P> {
    pub fn new(session: ExternalId, external: &'s E, peacher: &'s P) -> Self {
        Self {
            session,
            external,
            peacher,
        }
    }
    fn mapper(&self) -> ExternalIdQuery<'s, P> {
        ExternalIdQuery::new(self.peacher)
    }

    pub async fn sync(&self) -> Result<LegislationSyncResult, SyncError> {
        let mapper = self.mapper();
        let session = mapper.session(&self.session).await?;

        info!(
            "Syncing legislation for session {} (ext_id: {})",
            session.id, self.session
        );

        // Get all existing legislation external_ids for this session
        let mut known_legislation: HashMap<ExternalId, LegislationView> = HashMap::default();
        let mut page = 1;
        loop {
            let params = LegislationParams {
                session_id: Some(session.id),
                page: Some(page),
                page_size: Some(100),
                ..Default::default()
            };

            let result = params.request(self.peacher).await?;
            let is_empty = result.data.is_empty();

            for leg in result.data {
                if let Some(ref ext) = leg.external {
                    known_legislation.insert(ext.external_id.clone(), leg.into_legislation_view());
                }
            }

            if result.page >= result.num_pages || is_empty {
                break;
            }
            page += 1;
        }

        info!(
            "Found {} existing legislation items",
            known_legislation.len()
        );

        let mut created = Vec::new();
        let mut updated = Vec::new();
        let mut ext_page = 1u64;
        let page_size = 50u64;
        let mut consecutive_known = 0;
        let mut stopped_early = false;

        loop {
            format!(
                "External Fetch Legislation Request for {}, (page: {ext_page}, page_size: {page_size})",
                self.session
            );
            let batch = self
                .external
                .fetch_legislation(&self.session, ext_page, page_size)
                .await?;

            if batch.data.is_empty() {
                break;
            }

            for ext_leg in batch.data {
                let outcome = sync_legislation(
                    self.peacher,
                    &mapper,
                    session.id,
                    &known_legislation,
                    ext_leg,
                )
                .await?;
                match outcome.view {
                    LegislationViewOutcome::Created(val) => {
                        consecutive_known = 0;
                        created.push(val);
                    }
                    LegislationViewOutcome::Updated(val) => {
                        consecutive_known = 0;
                        updated.push(val);
                    }
                    LegislationViewOutcome::NotChanged(val) => {
                        if outcome.votes.created.len() == 0 && outcome.votes.updated.len() == 0 {
                            consecutive_known += 1;
                        } else {
                            consecutive_known += 0;
                            updated.push(val)
                        }
                    }
                };

                if consecutive_known > 10 {
                    info!(
                        "Hit {} consecutive known items, stopping early",
                        consecutive_known
                    );
                    stopped_early = true;
                    break;
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

        // Ok(LegislationSyncResult {
        //     created,
        //     updated,
        //     stopped_early,
        // })
        todo!()
    }
}

pub enum LegislationViewOutcome {
    Updated(LegislationView),
    NotChanged(LegislationView),
    Created(LegislationView),
}
impl LegislationViewOutcome {
    pub fn view(&self) -> &LegislationView {
        match self {
            Self::Updated(l) | Self::NotChanged(l) | Self::Created(l) => l,
        }
    }
}

pub struct LegislationUpdateOutcome {
    view: LegislationViewOutcome,
    votes: VotesSyncResult,
}

async fn sync_legislation<P: Client>(
    peacher: &P,
    mapper: &ExternalIdQuery<'_, P>,
    session_id: i32,
    known_legislation: &HashMap<ExternalId, LegislationView>,
    ext_leg: ExternalLegislation,
) -> SyncResult<LegislationUpdateOutcome> {
    let votes = ext_leg.votes.clone();
    let legislation_outcome = match known_legislation.get(&ext_leg.external_id) {
        Some(leg) => {
            info!(
                "Found existing '{}' (id: {}, ext_id: {})",
                leg.name_id, leg.id, ext_leg.external_id
            );

            if ext_leg.needs_update(leg) {
                let update =
                    UpdateLegislation::new(leg.id, ext_leg.into_update_legislation_request())
                        .request(peacher)
                        .await?;
                LegislationViewOutcome::Updated(update)
                // consecutive_known = 0;
                // updated.push(update)
            } else {
                LegislationViewOutcome::NotChanged(leg.clone())
                // consecutive_known += 1;
                // // TODO: we actually should update the legislation if possible.
                // // if no changes were made, then we will increase consecutive_known by 1.
                // updated.push(leg.clone());
                // // If ordering is Latest, we can stop early when hitting known items
            }
        }
        None => {
            let chamber = mapper.chamber(&ext_leg.chamber_id).await?;

            let ext_id = ext_leg.external_id.clone();
            let req = ext_leg.into_create_legislation_request();

            let leg = CreateLegislation::new(chamber.id, session_id, req.clone())
                .request(peacher)
                .await?;

            info!(
                "Created legislation '{}' (id: {}, ext_id: {})",
                leg.name_id, leg.id, ext_id
            );
            LegislationViewOutcome::Created(leg)
        }
    };

    let val = sync_legislation_votes(mapper, legislation_outcome.view(), votes).await?;
    Ok(LegislationUpdateOutcome {
        view: legislation_outcome,
        votes: val,
    })
}

async fn sync_legislation_votes<P: Client>(
    mapper: &ExternalIdQuery<'_, P>,
    legislation: &LegislationView,
    external_votes: impl IntoIterator<Item = ExternalLegislationVote>,
) -> SyncResult<VotesSyncResult> {
    let mut created = Vec::new();
    let mut updated = Vec::new();
    let mut unchanged = Vec::new();

    for ext_vote in external_votes {
        // Build member votes - need to resolve external member IDs to internal IDs
        let mut member_votes = Vec::new();
        for ext_member_vote in &ext_vote.votes {
            let member = mapper.member(&ext_member_vote.member_id).await?;
            member_votes.push(MemberVoteInput::new(member.id, ext_member_vote.vote));
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

        match CreateVote::new(legislation.id, vote_req)
            .request(mapper.client())
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

                if let SdkError::Status(StatusCode::CONFLICT, val) = e
                    && let Ok(err) = serde_json::from_str::<ErrorResponse>(&val)
                    && let Ok(id) = err.description.parse()
                {
                    info!("Updating vote");

                    let known_vote = GetLegislationVoteDetails::new(legislation.id, id)
                        .request(mapper.client())
                        .await?;

                    if known_vote.vote_name == ext_vote.vote_name
                        && known_vote.member_votes.len() == member_votes.len()
                        && known_vote.occurred_at == ext_vote.date_occurred
                    {
                        unchanged.push(id);
                    } else {
                        let req = UpdateVoteRequest {
                            name: Some(ext_vote.vote_name),
                            occurred_at: ext_vote.date_occurred,
                            member_votes: Some(member_votes),
                        };

                        UpdateVote::new(legislation.id, id, req)
                            .request(mapper.client())
                            .await?;

                        updated.push(id);
                    }
                }
            }
        }
    }

    Ok(VotesSyncResult {
        created,
        updated,
        unchanged,
    })
}
