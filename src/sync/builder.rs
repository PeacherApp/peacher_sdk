use tracing::info;

use crate::prelude::*;

/// Builder for ApiSync that handles initial jurisdiction resolution.
pub struct ApiSyncBuilder<'a, E> {
    external: E,
    peacher: &'a PeacherClient,
    dangerously_create: bool,
}

impl<'a, E> ApiSyncBuilder<'a, E>
where
    E: ExternalClient,
{
    pub fn new(external_client: E, peacher_client: &'a PeacherClient) -> Self {
        Self {
            external: external_client,
            peacher: peacher_client,
            dangerously_create: false,
        }
    }

    /// If the jurisdiction doesn't exist, create it.
    ///
    /// WARNING: This will create a new jurisdiction if one doesn't exist.
    /// Make sure you understand the implications before enabling this.
    pub fn dangerously_create_jurisdiction(mut self) -> Self {
        self.dangerously_create = true;
        self
    }

    /// Build the ApiSync client.
    ///
    /// If the jurisdiction doesn't exist and `dangerously_create` is not set,
    /// this will return an error.
    pub async fn build(self) -> Result<ApiSync<'a, E>, SyncError> {
        let ext_jurisdiction = self.external.get_jurisdiction();

        info!(
            "Building ApiSync for jurisdiction '{}' (ext_id: {})",
            ext_jurisdiction.name,
            ext_jurisdiction.external_id.val_str()
        );

        // Check if jurisdiction exists by external_id
        let existing_jurisdictions = ListJurisdictions::default()
            .with_external_id(ext_jurisdiction.external_id.val_str())
            .request(self.peacher)
            .await?;

        let jurisdiction_id = if let Some(existing) = existing_jurisdictions.data.first() {
            info!(
                "Found existing jurisdiction '{}' (id: {})",
                existing.name, existing.id
            );
            existing.id
        } else if self.dangerously_create {
            // Create new jurisdiction with chambers
            info!("Creating new jurisdiction '{}'", ext_jurisdiction.name);

            let mut ext_metadata = ExternalMetadata::new(ext_jurisdiction.external_id.clone());
            if let Some(ref url) = ext_jurisdiction.url {
                ext_metadata.set_url(url.clone());
            }

            let created = CreateJurisdiction::new(&ext_jurisdiction.name)
                .external_metadata(ext_metadata)
                .request(self.peacher)
                .await?;

            // Create chambers for this jurisdiction
            for chamber in &ext_jurisdiction.chambers {
                let mut chamber_req = CreateChamber::new(created.id, &chamber.name);
                let mut chamber_meta = ExternalMetadata::new(chamber.external_id.clone());
                if let Some(ref url) = chamber.url {
                    chamber_meta.set_url(url.clone());
                }
                chamber_req = chamber_req.external_metadata(chamber_meta);

                let created_chamber = chamber_req.request(self.peacher).await?;
                info!(
                    "Created chamber '{}' (id: {})",
                    created_chamber.name, created_chamber.id
                );
            }

            info!(
                "Created jurisdiction '{}' (id: {})",
                created.name, created.id
            );
            created.id
        } else {
            return Err(SyncError::JurisdictionNotFound);
        };

        Ok(ApiSync {
            ext_config: self.external.config(),
            external: self.external,
            peacher: self.peacher,
            jurisdiction_id: Some(jurisdiction_id),
        })
    }
}
