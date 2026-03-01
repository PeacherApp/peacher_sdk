mod rate;
pub use rate::*;
use strum::{Display, EnumString, VariantArray};

use std::borrow::Cow;

use crate::{
    commaparam,
    prelude::*,
    tippytappy::{self, CompiledDocument, DocumentView},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumString, Display, Hash, VariantArray,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ReviewState {
    Public,
    UnderReview,
    Reviewed,
}
commaparam!(ReviewState);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    /// Indicates that the content has been posted
    /// as public, but has not been placed under review
    /// or approved.
    Public,
    /// Indicates that some content is currently under review.
    ///
    /// In this state, only the author of the content
    /// and moderation will observe this value in [`ContentDetails`].
    UnderReview,
    /// Indicates that some content was explictly approved
    /// by the moderation team
    Approved,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ContentDetails {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub searchable_text: String,
    pub document: tippytappy::DocumentView,
    pub author: Option<MemberWithPartyView>,
    pub status: ContentStatus,
    /// This is the sum of sentiments where
    /// +1 is a positive sentiment, and -1 is a negative sentiment.
    pub rating: i32,
    pub viewer_sentiment: Option<Sentiment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemoveContentRequest {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemovedContent {
    pub id: Uuid,
    pub reason_removed: String,
    pub removed_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Post,
    Comment,
    Summary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ContentTypeId {
    /// Post ID
    Post(i32),
    /// Comment ID
    Comment(Uuid),
    /// Legislation ID
    Summary(Option<i32>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AdminContentView {
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub searchable_text: String,
    pub document: DocumentView,
    pub kind: ContentTypeId,
    pub author: Option<i32>,
    pub reason_removed: Option<String>,
    pub removed_by: Option<i32>,
    pub removed_at: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
#[expect(clippy::large_enum_variant)]
pub enum ContentView {
    /// Content that has been removed
    Removed(RemovedContent),
    /// Viewable content based on the viewer
    Content(ContentDetails),
    /// Some content that is currently under review.
    ///
    /// Viewers that see this variant are not part of the moderation team.
    UnderReview(Uuid),
}
impl ContentView {
    pub fn id(&self) -> Uuid {
        match self {
            ContentView::Removed(removed) => removed.id,
            ContentView::Content(content) => content.id,
            ContentView::UnderReview(id) => *id,
        }
    }
}

/// Request to create or update content.
///
/// This may be a new summary, an article, a comment, or otherwise.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
pub enum SetContentRequest {
    Document(DocumentView),
    Markdown(String),
}

/// Handler to update content (author or admin only)
pub struct UpdateContent {
    content_id: Uuid,
    body: SetContentRequest,
}

impl UpdateContent {
    pub fn markdown(content_id: Uuid, markdown: impl Into<String>) -> Self {
        Self {
            content_id,
            body: SetContentRequest::Markdown(markdown.into()),
        }
    }

    pub fn document(content_id: Uuid, doc: DocumentView) -> Self {
        Self {
            content_id,
            body: SetContentRequest::Document(doc),
        }
    }
}

impl Handler for UpdateContent {
    type ResponseBody = ContentView;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/content/{}", self.content_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Handler to remove content (author, moderator, or admin)
pub struct RemoveContent {
    content_id: Uuid,
    body: RemoveContentRequest,
}

impl RemoveContent {
    pub fn new(content_id: Uuid, reason: impl Into<String>) -> Self {
        Self {
            content_id,
            body: RemoveContentRequest {
                reason: reason.into(),
            },
        }
    }
}

impl Handler for RemoveContent {
    type ResponseBody = ContentView;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/content/{}", self.content_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Request to review (approve/reject) a summary as a moderator.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content")]
pub enum ReviewContentRequest {
    Approve,
    Remove(RemoveContentRequest),
}
/// Handler to review (approve/reject) a summary
pub struct ReviewContent {
    summary_id: uuid::Uuid,
    body: ReviewContentRequest,
}

impl ReviewContent {
    pub fn approve(summary_id: uuid::Uuid) -> Self {
        Self {
            summary_id,
            body: ReviewContentRequest::Approve,
        }
    }

    pub fn reject(summary_id: uuid::Uuid, reason: impl Into<String>) -> Self {
        Self {
            summary_id,
            body: ReviewContentRequest::Remove(RemoveContentRequest {
                reason: reason.into(),
            }),
        }
    }
}

impl Handler for ReviewContent {
    type ResponseBody = ContentView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/content/{}/review", self.summary_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}
