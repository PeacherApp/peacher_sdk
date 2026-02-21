mod client;
use std::sync::Arc;

pub use client::*;

mod error;
pub use error::*;

mod jurisdiction;
pub use jurisdiction::*;

mod sessions;
pub use sessions::*;

mod external_map;
pub use external_map::*;

use crate::prelude::*;
use chrono::NaiveDate;

/// Result of syncing members
#[derive(Debug, Clone)]
pub struct MembersSyncResult {
    pub created: Vec<Arc<MemberWithPartyView>>,
    pub updated: Vec<Arc<MemberWithPartyView>>,
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
    pub external_id: Option<ExternalId>,
    pub external_url: Option<Url>,
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
    mapper: ClientMapper<'p, P>,
}

impl<'p, E: ExternalClient, P: Client> ApiSync<'p, E, P> {
    pub fn new(external: E, peacher: &'p P) -> Self {
        let mapper = ClientMapper::new(peacher);
        Self { external, mapper }
    }

    pub fn peacher(&self) -> &'p P {
        self.mapper.client()
    }

    pub fn mapper(&mut self) -> &mut ClientMapper<'p, P> {
        &mut self.mapper
    }

    pub fn external(&self) -> &E {
        &self.external
    }

    /// Sync jurisdiction AND chambers together.
    ///
    /// ExternalClient::get_jurisdiction() returns the jurisdiction with its chambers,
    /// so one client = one jurisdiction + its chambers. They sync together.
    pub fn jurisdiction<'slf>(&'slf mut self) -> JurisdictionSync<'slf, 'p, E, P> {
        JurisdictionSync::new(&self.external, &mut self.mapper)
    }

    /// Sync sessions for a jurisdiction.
    pub fn sessions<'slf>(&'slf mut self) -> AllSessionsSync<'slf, 'p, E, P> {
        AllSessionsSync::new(&self.external, &mut self.mapper)
    }

    pub async fn sync_all(&self) -> SyncResult<()> {
        todo!()
        // self.jurisdiction().sync().await?;
        // self.sessions().sync_sessions().await?;
        //self.sessions().
    }
}
