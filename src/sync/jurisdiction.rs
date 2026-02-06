use std::sync::Arc;

use ahash::HashMap;
use tracing::info;

use crate::prelude::*;

/// Result of syncing jurisdiction and its chambers
#[derive(Debug, Clone)]
pub struct JurisdictionAndChambersSyncResult {
    pub jurisdiction_id: i32,
    pub jurisdiction_name: String,
    pub jurisdiction_created: bool,
    pub chambers_created: Vec<ListChamberResponse>,
    pub chambers_updated: Vec<ListChamberResponse>,
}

pub struct JurisdictionSync<'caller, 'client, E, P> {
    external: &'caller E,
    mapper: &'caller mut ClientMapper<'client, P>,
}

impl<'caller, 'client, E: ExternalClient, P: Client> JurisdictionSync<'caller, 'client, E, P> {
    pub fn new(external: &'caller E, mapper: &'caller mut ClientMapper<'client, P>) -> Self {
        Self { external, mapper }
    }

    pub async fn get(&mut self) -> SyncResult<Arc<GetJurisdictionResponse>> {
        let client_provided_jurisdiction = self.external.get_jurisdiction();
        let val = self
            .mapper
            .jurisdiction(&client_provided_jurisdiction.external_id)
            .await?;
        Ok(val)
    }

    /// Syncs the jurisdiction and chambers (creating if doesn't exist)
    pub async fn sync(&mut self) -> SyncResult<JurisdictionAndChambersSyncResult> {
        let client_provided_jurisdiction = self.external.get_jurisdiction();

        info!(
            "Syncing jurisdiction '{}' (ext_id: {})",
            client_provided_jurisdiction.name,
            client_provided_jurisdiction.external_id.val_str()
        );

        let (jurisdiction, jurisdiction_created) = match self
            .mapper
            .jurisdiction(&client_provided_jurisdiction.external_id)
            .await
        {
            Ok(jurisdiction) => (jurisdiction, false),
            Err(SyncError::NotFound(external_id)) => {
                // Create jurisdiction
                let mut ext_metadata = ExternalMetadata::new(external_id);
                if let Some(ref url) = client_provided_jurisdiction.url {
                    ext_metadata.set_url(url.clone());
                }

                let created = CreateJurisdiction::new(&client_provided_jurisdiction.name)
                    .external_metadata(ext_metadata)
                    .request(self.mapper.client())
                    .await?;
                let created = self.mapper.store_jurisdiction(created);

                info!(
                    "Created jurisdiction '{}' (id: {})",
                    created.name, created.id
                );

                (created, true)
            }
            Err(e) => return Err(e),
        };

        // Sync chambers
        let known_chambers = ListChambers::default()
            .with_jurisdiction(jurisdiction.id)
            .request(self.mapper.client())
            .await?
            .data
            .into_iter()
            .filter_map(|known_chamber| {
                let external_id = known_chamber.external.as_ref()?.external_id.clone();
                Some((external_id, known_chamber))
            })
            .collect::<HashMap<_, _>>();

        let mut chambers_created = Vec::new();
        let mut chambers_updated = Vec::new();

        for client_provided_chamber in &client_provided_jurisdiction.chambers {
            match known_chambers.get(&client_provided_chamber.external_id) {
                Some(response) => {
                    //TODO: will need to update chambers here
                    chambers_updated.push(response.clone())
                }
                None => {
                    let mut chamber_req =
                        CreateChamber::new(jurisdiction.id, &client_provided_chamber.name);
                    let mut ext_metadata =
                        ExternalMetadata::new(client_provided_chamber.external_id.clone());
                    if let Some(ref url) = client_provided_chamber.url {
                        ext_metadata.set_url(url.clone());
                    }
                    chamber_req = chamber_req.external_metadata(ext_metadata);

                    let created = chamber_req.request(self.mapper.client()).await?;
                    info!(
                        "Created chamber '{}' (id: {}, ext_id: {})",
                        created.name, created.id, client_provided_chamber.external_id
                    );
                    chambers_created.push(created);
                }
            }
        }

        Ok(JurisdictionAndChambersSyncResult {
            jurisdiction_id: jurisdiction.id,
            jurisdiction_name: jurisdiction.name.clone(),
            jurisdiction_created,
            chambers_created,
            chambers_updated,
        })
    }
}
