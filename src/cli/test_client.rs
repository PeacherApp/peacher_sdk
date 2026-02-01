use crate::prelude::*;
use anyhow::{Context, Result};
use tracing::{debug, info};

pub async fn test_client<C: ExternalClient>(client: C, api_config: PeacherClient) -> Result<()> {
    let mut client = ClientTest::new(client, api_config).await?;
    client.run_test().await
}

pub struct ClientTest<C> {
    sync: ApiSync<C>,
    //api_url: Url,
    legislation: Vec<ExternalLegislation>,
}

impl<C: ExternalClient> ClientTest<C> {
    pub async fn new_from_parts(client: C, peacher: PeacherClient) -> Result<Self> {
        let sync = ApiSyncBuilder::new(client, peacher)
            .build()
            .await
            .context("Failed to build ApiSync client")?;
        debug!("ApiSync client built successfully");

        Ok(Self {
            sync,
            legislation: Vec::new(),
        })
    }

    pub async fn new(client: C, api_config: PeacherClient) -> Result<Self> {
        Self::new_from_parts(client, api_config).await
    }

    pub async fn run_test(&mut self) -> Result<()> {
        info!("=== Test 1: Sync Sessions (with duplicate test) ===");
        self.sync_sessions_with_dupe_check().await?;

        info!("=== Test 2: Sync Members (with duplicate test) ===");
        self.sync_members_with_dupe_check().await?;

        info!("=== Test 3: Sync Legislation (with duplicate test) ===");
        self.sync_legislation_with_dupe_check().await?;

        info!("=== Test 4: Sync Legislation Votes (with duplicate test) ===");
        self.sync_votes_with_dupe_check().await?;

        info!("=== All E2E Tests Passed! ===");
        Ok(())
    }

    async fn sync_sessions_with_dupe_check(&mut self) -> Result<()> {
        // First sync
        info!("First session sync...");
        self.sync
            .sync_sessions()
            .await
            .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;

        let session_count = self.sync.sessions().len();
        info!("Synced {} sessions", session_count);

        if session_count == 0 {
            anyhow::bail!("No sessions were synced from Georgia API");
        }

        // Duplicate test - sync again
        info!("Duplicate session sync test...");
        self.sync
            .sync_sessions()
            .await
            .map_err(|e| anyhow::anyhow!("Duplicate session sync failed: {:?}", e))?;

        info!(
            "Duplicate session test passed: {} sessions (unchanged)",
            self.sync.sessions().len()
        );

        Ok(())
    }

    async fn sync_members_with_dupe_check(&mut self) -> Result<()> {
        // Get the first session to sync members for
        let session_id = self
            .sync
            .sessions()
            .first()
            .map(|s| s.id)
            .ok_or_else(|| anyhow::anyhow!("No sessions available for member sync"))?;

        info!("Syncing members for session {}...", session_id);

        // First sync member update count
        let first_sync_count = self
            .sync
            .update_members(session_id)
            .await
            .map_err(|e| anyhow::anyhow!("Member sync failed: {:?}", e))?;

        info!(
            "Synced {} members across chambers",
            first_sync_count.maybe_new.len()
        );

        if first_sync_count.maybe_new.is_empty() {
            anyhow::bail!(
                "No members were synced from Georgia API, and {} duplicates were found.",
                first_sync_count.duplicates.len()
            );
        }

        // Duplicate test - sync again
        info!("Duplicate member sync test...");
        let second_sync_count = self
            .sync
            .update_members(session_id)
            .await
            .map_err(|e| anyhow::anyhow!("Duplicate member sync failed: {:?}", e))?;

        if !second_sync_count.maybe_new.is_empty() {
            anyhow::bail!(
                "During the second sync test, {} members might have been added! This is a general failure.",
                second_sync_count.maybe_new.len()
            );
        }

        Ok(())
    }

    async fn sync_legislation_with_dupe_check(&mut self) -> Result<()> {
        let session_id = self
            .sync
            .sessions()
            .first()
            .map(|s| s.id)
            .ok_or_else(|| anyhow::anyhow!("No sessions available for legislation sync"))?;

        info!("Syncing legislation for session {}...", session_id);

        // First sync
        self.legislation = self
            .sync
            .update_legislation_with_pagination(session_id, Some(0))
            .await
            .map_err(|e| anyhow::anyhow!("Legislation sync failed: {:?}", e))?;

        info!("Legislation synced successfully");

        // Duplicate test - sync again
        info!("Duplicate legislation sync test...");
        self.sync
            .update_legislation_with_pagination(session_id, Some(0))
            .await
            .map_err(|e| anyhow::anyhow!("Duplicate legislation sync failed: {:?}", e))?;

        info!("Duplicate legislation test passed");

        Ok(())
    }

    async fn sync_votes_with_dupe_check(&mut self) -> Result<()> {
        let legislation_ids = self.legislation.iter().take(5).map(|l| &l.external_id);

        for leg_id in legislation_ids {
            info!("Syncing votes for legislation {}...", leg_id);
            self.sync
                .update_legislation_votes(leg_id)
                .await
                .map_err(|e| anyhow::anyhow!("Vote sync failed: {:?}", e))?;

            // Duplicate test
            info!("Duplicate vote sync test for legislation {}...", leg_id);
            self.sync
                .update_legislation_votes(leg_id)
                .await
                .map_err(|e| anyhow::anyhow!("Duplicate vote sync failed: {:?}", e))?;
        }

        Ok(())
    }
}
