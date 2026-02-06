use ahash::HashMap;
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
        let session = self.mapper().session(&self.session).await?;

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
                match known_legislation.get(&ext_leg.external_id) {
                    Some(leg) => {
                        info!(
                            "Found existing '{}' (id: {}, ext_id: {})",
                            leg.name_id, leg.id, ext_leg.external_id
                        );

                        if ext_leg.needs_update(leg) {
                            let update = UpdateLegislation::new(
                                leg.id,
                                ext_leg.into_update_legislation_request(),
                            )
                            .request(self.peacher)
                            .await?;
                            consecutive_known = 0;
                            updated.push(update)
                        } else {
                            consecutive_known += 1;
                            // TODO: we actually should update the legislation if possible.
                            // if no changes were made, then we will increase consecutive_known by 1.
                            updated.push(leg.clone());
                            // If ordering is Latest, we can stop early when hitting known items
                        }

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

                        let chamber = self.mapper().chamber(&ext_leg.chamber_id).await?;

                        let ext_id = ext_leg.external_id.clone();
                        let req = ext_leg.into_create_legislation_request();

                        let leg = CreateLegislation::new(chamber.id, session.id, req.clone())
                            .request(self.peacher)
                            .await?;

                        info!(
                            "Created legislation '{}' (id: {}, ext_id: {})",
                            leg.name_id, leg.id, ext_id
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

        // Ok(LegislationSyncResult {
        //     created,
        //     updated,
        //     stopped_early,
        // })
        todo!()
    }
}
