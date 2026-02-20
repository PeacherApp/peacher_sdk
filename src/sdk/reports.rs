use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::sdk::{AdminContentView, MemberView};
use crate::{paginated, prelude::*};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "id", rename_all = "snake_case")]
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
    pub filter: ListReportParams,
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
pub struct ListReportParams {
    pub id: Option<i32>,
    /// Note that as a non-moderator, this parameter
    ///
    /// is automatically overwritten to be your member id.
    pub reporter: Option<i32>,
    pub report_type: Option<ReportType>,
    pub review_status: Option<ReviewStatus>,
    pub created_after: Option<DateTime<FixedOffset>>,
    pub created_before: Option<DateTime<FixedOffset>>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(ListReportParams, 1000);

#[test]
fn ensure_list_reports_page_size_limit() {
    let params = ListReportParams {
        page_size: Some(5001),
        ..Default::default()
    };

    assert_eq!(params.page_size(), 1000);
}

/// Handler to create a report
pub struct CreateReport {
    body: CreateReportRequest,
}

impl CreateReport {
    pub fn content(content_id: Uuid, details: impl Into<String>) -> Self {
        Self {
            body: CreateReportRequest {
                kind: ReportedKind::Content(content_id),
                details: details.into(),
            },
        }
    }

    pub fn member(member_id: i32, details: impl Into<String>) -> Self {
        Self {
            body: CreateReportRequest {
                kind: ReportedKind::Member(member_id),
                details: details.into(),
            },
        }
    }
}

impl Handler for CreateReport {
    type ResponseBody = ReportView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/reports".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler to list reports (moderator+)
#[derive(Default)]
pub struct ListReports {
    pub params: ListReportParams,
}

impl ListReports {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GetHandler for ListReports {
    type ResponseBody = Paginated<ReportView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/reports".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Handler to review a single report (moderator+)
pub struct ReviewReport {
    report_id: i32,
    body: ReviewReportRequest,
}

impl ReviewReport {
    pub fn new(
        report_id: i32,
        review_status: ReviewStatus,
        review_result: impl Into<String>,
    ) -> Self {
        Self {
            report_id,
            body: ReviewReportRequest {
                review_status,
                review_result: review_result.into(),
            },
        }
    }
}

impl Handler for ReviewReport {
    type ResponseBody = ReportView;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/reports/{}", self.report_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler to bulk review reports (moderator+)
pub struct BulkReviewReports {
    body: BulkReviewReportsRequest,
}

impl BulkReviewReports {
    pub fn new(
        filter: ListReportParams,
        review_status: ReviewStatus,
        review_result: impl Into<String>,
    ) -> Self {
        Self {
            body: BulkReviewReportsRequest {
                filter,
                review_status,
                review_result: review_result.into(),
            },
        }
    }
}

impl Handler for BulkReviewReports {
    type ResponseBody = BulkReviewResponse;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/reports".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}
