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
    pub email: NotificationPreference<EmailNotificationPreferences>,
    pub in_app: NotificationPreference<InAppNotificationPreferences>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreferenceMeta {
    pub id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreference<P> {
    /// if this is None, then the preference has never been recorded.
    pub meta: Option<NotificationPreferenceMeta>,
    pub preference: P,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum NotificationPreferenceKind {
    InApp(InAppNotificationPreferences),
    Email(EmailNotificationPreferences),
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
pub struct InAppNotificationPreferences {
    pub enabled: bool,
}
impl Default for InAppNotificationPreferences {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EmailNotificationPreferences {
    pub enabled: bool,
}

#[expect(clippy::derivable_impls)]
impl Default for EmailNotificationPreferences {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// A notification ready for delivery to a member.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Notification {
    BulkReviews(NotifyBulkReview),
    ReportReviewed(NotifyReportReviewed),
    NewReports(Vec<ReportCreated>),
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
