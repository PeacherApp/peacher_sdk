use crate::prelude::*;

/// Trait for external data sources (e.g., state legislative APIs).
///
/// External clients are pure fetchers - they return all available data
/// without filtering. ApiSync handles the diffing against Peacher's database.
///
/// You should employ a strategy of a two-step process. You should typically start
/// with fetching your data from an external api source and saving that data.
///
/// for whatever type that implements this trait, it should have all the legislative
/// data necessary to perform these requests. As it's unpredictable how could clients
/// may return data, this should be called after your sync.
pub trait ExternalClient: Send + Sync {
    /// Get the jurisdiction this client represents (non-async, known at construction time)
    fn get_jurisdiction(&self) -> ExternalJurisdiction;

    /// List all sessions - no filtering, return everything
    fn list_sessions(&self) -> impl Future<Output = SyncResult<Vec<ExternalSession>>>;

    /// List all chamber members - no filtering
    fn list_members(
        &self,
        session_id: &ExternalId,
        chamber_id: &ExternalId,
    ) -> impl Future<Output = SyncResult<Vec<ExtChamberSessionMember>>>;

    /// Fetch legislation page - pagination controlled by ApiSync.
    ///
    /// You must return in the order of most recently updated legislation to least.
    /// The caller assumes pages start at 0.
    fn fetch_legislation(
        &self,
        session_id: &ExternalId,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = SyncResult<Paginated<ExternalLegislation>>>;
}
