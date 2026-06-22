mod reports;
pub use reports::*;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::{
    paginated,
    sdk::{BulkReviewReportsRequest, MemberView, ReviewStatus},
};

#[cfg(feature = "awards")]
use crate::sdk::AwardTier;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct NotificationParams {
    #[serde(skip)]
    pub member_id: Option<i32>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
paginated!(NotificationParams);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationView {
    pub created_at: DateTime<FixedOffset>,
    pub id: Uuid,
    pub details: Notification,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreferencesResponse {
    pub email: NotificationConfig<EmailPreferences>,
    pub in_app: NotificationConfig<InAppPreferences>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreferenceMeta {
    pub id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationConfig<P> {
    /// if this is None, then the preference has never been recorded.
    pub meta: Option<NotificationPreferenceMeta>,
    pub member_id: i32,
    pub preference: P,
}
impl<P: Default> NotificationConfig<P> {
    pub fn new(member_id: i32) -> Self {
        Self {
            meta: None,
            member_id,
            preference: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum NotificationPreferenceKind {
    InApp(InAppPreferences),
    Email(EmailPreferences),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateNotificationPreferenceResponse {
    pub updated_at: DateTime<FixedOffset>,
    pub preference: NotificationPreferenceKind,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PreferenceType {
    InApp,
    Email,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InAppPreferences {
    pub enabled: bool,
}
impl Default for InAppPreferences {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EmailPreferences {
    pub enabled: bool,
}

#[expect(clippy::derivable_impls)]
impl Default for EmailPreferences {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// A notification ready for delivery to a member.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
#[expect(clippy::large_enum_variant)]
pub enum Notification {
    BulkReviews(NotifyBulkReview),
    ReportReviewed(NotifyReportReviewed),
    NewReports(Vec<ReportCreated>),
    #[cfg(feature = "awards")]
    AwardReceived(NotifyAwardReceived),
}

#[cfg(feature = "awards")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotifyAwardReceived {
    pub award_id: i32,
    pub tier: AwardTier,
    pub content_item_id: Uuid,
    /// `None` when the giver chose to be anonymous.
    pub giver: Option<MemberView>,
    pub awarded_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotifyBulkReview {
    pub reviewer: MemberView,
    pub reviews: Vec<InnerBulkReview>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InnerBulkReview {
    pub ids: Vec<i32>,
    pub reviewed_at: DateTime<FixedOffset>,
    pub review: BulkReviewReportsRequest,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotifyReportReviewed {
    pub report_id: i32,
    pub reviewed_at: DateTime<FixedOffset>,
    pub message: String,
    pub status: ReviewStatus,
}
