use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::paginated;
use crate::sdk::{AdminContentView, MemberView};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReportedKind {
    Content(Uuid),
    Member(i32),
}
impl ReportedKind {
    pub fn report_type(&self) -> ReportType {
        match self {
            ReportedKind::Content(_) => ReportType::Content,
            ReportedKind::Member(_) => ReportType::Member,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateReportRequest {
    pub kind: ReportedKind,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString, Display)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ReportType {
    Content,
    Member,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString, Display, Default, Hash,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ReviewStatus {
    #[default]
    Pending,
    Resolved,
    Dismissed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateReportResponse {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum ReportDetails {
    Content(AdminContentView),
    Member(MemberView),
}
impl ReportDetails {
    pub fn report_type(&self) -> ReportType {
        match self {
            ReportDetails::Content(_) => ReportType::Content,
            ReportDetails::Member(_) => ReportType::Member,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReportView {
    pub id: i32,
    pub reporter: Option<i32>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub details: ReportDetails,
    pub reviewer: Option<i32>,
    pub review_status: ReviewStatus,
    pub review_result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReviewReportRequest {
    pub review_status: ReviewStatus,
    pub review_result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BulkReviewReportsRequest {
    pub filter: ReportParams,
    pub review_status: ReviewStatus,
    pub review_result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BulkReviewResponse {
    pub updated: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ReportParams {
    pub id: Option<i32>,
    pub reporter: Option<i32>,
    pub report_type: Option<ReportType>,
    pub review_status: Option<ReviewStatus>,
    pub created_after: Option<DateTime<FixedOffset>>,
    pub created_before: Option<DateTime<FixedOffset>>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(ReportParams);
