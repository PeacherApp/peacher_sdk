use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExtOrder {
    Latest,
    Earliest,
}

/// Configuration that tells ApiSync how an external client behaves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalClientConfig {
    /// How fetch_legislation orders results by default
    pub legislation_order: ExtOrder,
    /// Whether get_legislation() returns more fields than fetch_legislation()
    pub get_legislation_has_details: bool,
    /// Whether get_member() returns more fields than list_representatives()
    pub get_member_has_details: bool,
    /// If a member exists that doesn't appear in a legislative vote, should
    /// how should the syncer behave?
    pub behavior_when_member_doesnt_exist: MemberAction,
    /// Set the behavior of what should happen if, when syncing votes,
    /// legislation is updated.
    ///
    /// WARNING: This is dangerous if your external IDs are not unique!!!
    /// Be sure that you are setting unique external ids on your votes!
    pub behavior_when_legislation_vote_exists: LegVoteAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegVoteAction {
    Fail,
    Update,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberAction {
    Fail,
    Skip,
    Create,
}

impl Default for ExternalClientConfig {
    fn default() -> Self {
        Self {
            legislation_order: ExtOrder::Latest,
            get_legislation_has_details: false,
            get_member_has_details: false,
            behavior_when_member_doesnt_exist: MemberAction::Fail,
            behavior_when_legislation_vote_exists: LegVoteAction::Fail,
        }
    }
}

/// Trait for external data sources (e.g., state legislative APIs).
///
/// External clients are pure fetchers - they return all available data
/// without filtering. ApiSync handles the diffing against Peacher's database.
pub trait ExternalClient: Send + Sync {
    /// Returns configuration about how this client behaves.
    ///
    /// Override this to customize behavior for specific external sources.
    fn config(&self) -> ExternalClientConfig {
        ExternalClientConfig::default()
    }

    /// Get the jurisdiction this client represents (non-async, known at construction time)
    fn get_jurisdiction(&self) -> ExternalJurisdiction;

    /// List all sessions - no filtering, return everything
    fn list_sessions(&self) -> impl Future<Output = SdkResult<Vec<ExternalSession>>>;

    /// Get a specific session by external ID
    fn get_session(
        &self,
        session_id: &ExternalId,
    ) -> impl Future<Output = SdkResult<ExternalSession>>;

    /// Get a specific member by external ID
    fn get_member(&self, member_id: &ExternalId)
    -> impl Future<Output = SdkResult<ExternalMember>>;

    /// List all chamber members - no filtering
    fn list_members(
        &self,
        session_id: &ExternalId,
        chamber_id: &ExternalId,
    ) -> impl Future<Output = SdkResult<Vec<ExtChamberSessionMember>>>;

    /// Get a specific piece of legislation
    fn get_legislation(
        &self,
        legislation_id: &ExternalId,
    ) -> impl Future<Output = SdkResult<ExternalLegislation>>;

    /// Fetch legislation page - pagination controlled by ApiSync.
    ///
    /// The caller assumes pages start at 1.
    fn fetch_legislation(
        &self,
        session_id: &ExternalId,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = SdkResult<Paginated<ExternalLegislation>>>;
}
