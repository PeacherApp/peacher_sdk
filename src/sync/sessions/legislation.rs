use ahash::HashMap;
use http::StatusCode;
use tracing::info;

use crate::prelude::*;

pub struct LegislationSync<'caller, 'client, E, P> {
    session: ExternalId,
    external: &'caller E,
    mapper: &'caller mut ClientMapper<'client, P>,
}

impl<'caller, 'client, E: ExternalClient, P: Client> LegislationSync<'caller, 'client, E, P> {
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

    pub async fn sync(
        &mut self,
        max_consecutive_unknown: Option<u32>,
    ) -> Result<LegislationSyncResult, SyncError> {
        let session = self.mapper.session(&self.session).await?;

        info!(
            "Syncing legislation for session {} (ext_id: {})",
            session.id, self.session
        );

        // Get all existing legislation external_ids for this session
        let mut known_legislation: HashMap<ExternalId, LegislationView> = HashMap::default();
        let mut page = 0;
        loop {
            let params = LegislationParams {
                session_id: Some(session.id),
                page: Some(page),
                page_size: Some(100),
                ..Default::default()
            };

            let result = params.request(self.mapper.client()).await?;
            let is_empty = result.data.is_empty();

            for leg in result.data {
                if let Some(ref ext) = leg.external {
                    known_legislation.insert(ext.external_id.clone(), leg.into_legislation_view());
                }
            }

            if result.page + 1 >= result.num_pages || is_empty {
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
        let mut ext_page = 0u64;
        let page_size = 50u64;
        let mut consecutive_known = 0;
        let mut stopped_early = false;

        loop {
            println!(
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
                info!(
                    "Beginning sync for {}({})",
                    ext_leg.name_id, ext_leg.external_id
                );

                let outcome =
                    sync_legislation(self.mapper, session.id, &known_legislation, ext_leg).await?;
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
                        if outcome.votes.created.is_empty() && outcome.votes.updated.is_empty() {
                            consecutive_known += 1;
                        } else {
                            consecutive_known += 0;
                            updated.push(val)
                        }
                    }
                };

                if max_consecutive_unknown.is_some_and(|max| consecutive_known > max) {
                    info!(
                        "Hit {} consecutive known items, stopping early",
                        consecutive_known
                    );
                    stopped_early = true;
                    break;
                }
            }

            if stopped_early || batch.page + 1 >= batch.num_pages {
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
    mapper: &mut ClientMapper<'_, P>,
    session_id: i32,
    known_legislation: &HashMap<ExternalId, LegislationView>,
    ext_leg: ExternalLegislation,
) -> SyncResult<LegislationUpdateOutcome> {
    let votes = ext_leg.votes.clone();
    let sponsors = ext_leg.sponsors.clone();
    let legislation_outcome = match known_legislation.get(&ext_leg.external_id) {
        Some(leg) => {
            info!(
                "Found existing '{}' (id: {}, ext_id: {})",
                leg.name_id, leg.id, ext_leg.external_id
            );

            if ext_leg.needs_update(leg) {
                let update =
                    UpdateLegislation::new(leg.id, ext_leg.into_update_legislation_request())
                        .request(mapper.client())
                        .await?;
                LegislationViewOutcome::Updated(update)
            } else {
                LegislationViewOutcome::NotChanged(leg.clone())
            }
        }
        None => {
            let chamber = mapper.chamber(&ext_leg.chamber_id).await?;

            let ext_id = ext_leg.external_id.clone();
            let req = ext_leg.into_create_legislation_request();

            let leg = CreateLegislation::new(chamber.id, session_id, req.clone())
                .request(mapper.client())
                .await?;

            info!(
                "Created legislation '{}' (id: {}, ext_id: {})",
                leg.name_id, leg.id, ext_id
            );
            LegislationViewOutcome::Created(leg)
        }
    };

    let val = sync_legislation_votes(mapper, legislation_outcome.view(), votes).await?;
    sync_legislation_sponsors(mapper, legislation_outcome.view(), sponsors).await?;
    Ok(LegislationUpdateOutcome {
        view: legislation_outcome,
        votes: val,
    })
}

async fn sync_legislation_votes<P: Client>(
    mapper: &mut ClientMapper<'_, P>,
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
        //
        let ext_metadata = ExternalMetadata {
            external_id: ext_vote.external_id.clone(),
            url: ext_vote.url.clone(),
            externally_updated_at: None,
        };
        let chamber = mapper.chamber(&ext_vote.chamber_id).await.unwrap();

        let vote_name = ext_vote.vote_name.clone();
        let ext_vote_id = ext_vote.external_id.val_str().to_owned();
        let vote_req = CreateVoteRequest {
            name: ext_vote.vote_name.clone(),
            chamber: chamber.id,
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

async fn sync_legislation_sponsors<P: Client>(
    mapper: &mut ClientMapper<'_, P>,
    legislation: &LegislationView,
    external_sponsors: Vec<ExternalSponsor>,
) -> SyncResult<()> {
    let mut sponsor_inputs = Vec::new();

    for ext_sponsor in &external_sponsors {
        let member = mapper.member(&ext_sponsor.external_member_id).await?;
        sponsor_inputs.push(SponsorInput {
            member_id: member.id,
            sponsor_type: ext_sponsor.sponsor_type,
            sponsored_at: ext_sponsor.sponsored_at,
        });
    }

    let req = PutSponsorsRequest {
        sponsors: sponsor_inputs,
    };

    PutSponsors::new(legislation.id, req)
        .request(mapper.client())
        .await?;

    info!(
        "Synced {} sponsors for legislation '{}'",
        external_sponsors.len(),
        legislation.name_id
    );

    Ok(())
}
