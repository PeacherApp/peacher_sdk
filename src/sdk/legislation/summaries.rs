use std::borrow::Cow;

use crate::{paginated, prelude::*, tippytappy::DocumentView};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum SummaryKind {
    User,
    PrimarySource,
    Ai,
}

/// The type of summary for this content
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum CreateSummaryKind {
    PrimarySource,
    User,
}
impl CreateSummaryKind {
    pub fn to_summary_kind(&self) -> SummaryKind {
        match self {
            CreateSummaryKind::PrimarySource => SummaryKind::PrimarySource,
            CreateSummaryKind::User => SummaryKind::User,
        }
    }
}

/// Request to create a new summary.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateSummaryRequest {
    pub kind: CreateSummaryKind,
    pub content: SetContentRequest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SummaryView {
    pub legislation_id: i32,
    pub contents: ContentView,
    pub kind: SummaryKind,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Visibility {
    NotVisible,
    Public,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct ModeratorContentParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub legislation_id: Option<i32>,
    pub review_state: CommaSeparated<ReviewState>,
}

paginated!(ModeratorContentParams);

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct SummaryParams {
    pub order_by: SummaryOrder,
    pub order: Ordering,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(SummaryParams);

#[derive(
    Default, Clone, Copy, EnumString, Display, Debug, PartialEq, Eq, Serialize, Deserialize,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SummaryOrder {
    #[default]
    Weight,
}

/// Handler to create a summary on legislation
pub struct CreateSummary {
    legislation_id: i32,
    body: SetContentRequest,
}

impl CreateSummary {
    pub fn markdown(legislation_id: i32, markdown: impl Into<String>) -> Self {
        Self {
            legislation_id,
            body: SetContentRequest::Markdown(markdown.into()),
        }
    }

    pub fn document(legislation_id: i32, doc: DocumentView) -> Self {
        Self {
            legislation_id,
            body: SetContentRequest::Document(doc),
        }
    }
}

impl Handler for CreateSummary {
    type ResponseBody = SummaryView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/summaries", self.legislation_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler to list summaries needing moderator approval
pub struct ListContentNeedingReview {
    pub params: ModeratorContentParams,
}

impl Default for ListContentNeedingReview {
    fn default() -> Self {
        Self {
            params: ModeratorContentParams {
                page: None,
                page_size: None,
                legislation_id: None,
                review_state: CommaSeparated::default(),
            },
        }
    }
}

impl ListContentNeedingReview {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GetHandler for ListContentNeedingReview {
    type ResponseBody = Paginated<SummaryView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/content/review".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Handler to list summaries for a piece of legislation
pub struct ListSummaries {
    legislation_id: i32,
    params: SummaryParams,
}

impl ListSummaries {
    pub fn new(legislation_id: i32) -> Self {
        Self {
            legislation_id,
            params: SummaryParams::default(),
        }
    }

    pub fn with_params(legislation_id: i32, params: SummaryParams) -> Self {
        Self {
            legislation_id,
            params,
        }
    }
}

impl GetHandler for ListSummaries {
    type ResponseBody = Paginated<SummaryView>;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/summaries", self.legislation_id).into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}
