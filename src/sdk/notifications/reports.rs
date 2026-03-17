use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::sdk::{MemberView, ReviewStatus};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReportReviewed {
    pub reviewed_at: DateTime<FixedOffset>,
    pub reviewer: MemberView,
    pub message: String,
    pub status: ReviewStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReportCreated {
    pub created_at: DateTime<FixedOffset>,
    pub reporter: MemberView,
    pub report_reason: String,
}
