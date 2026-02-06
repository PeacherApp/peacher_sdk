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
    pub unchanged: Vec<i32>,
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
