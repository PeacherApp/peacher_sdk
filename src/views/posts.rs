use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

/// Summary view of a post (used in listings)
/// Note: The author field uses MemberView when fetched via API
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostSummaryView {
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub id: i32,
    pub title: String,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub cover_image: Option<Url>,
    pub views: i32,
    pub author: MemberView,
    pub article_url: Option<Url>,
    pub excerpt: String,
    pub legislation_ids: Vec<i32>,
    pub member_ids: Vec<i32>,
    pub district_ids: Vec<(i32, i32)>,
    pub jurisdiction_ids: Vec<i32>,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostSummary<M> {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    pub id: i32,
    pub title: String,
    #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>, format = DateTime))]
    pub published_at: Option<DateTime<FixedOffset>>,
    pub cover_image: Option<Url>,
    pub views: i32,
    pub author: M,
    pub article_url: Option<Url>,
    pub excerpt: String,
    pub legislation_ids: Vec<i32>,
    pub member_ids: Vec<i32>,
    pub district_ids: Vec<(i32, i32)>,
    pub jurisdiction_ids: Vec<i32>,
}
impl<M> PostSummary<M> {
    pub fn update_author<N>(self, func: impl FnOnce(M) -> N) -> PostSummary<N> {
        PostSummary {
            created_at: self.created_at,
            updated_at: self.updated_at,
            id: self.id,
            title: self.title,
            published_at: self.published_at,
            cover_image: self.cover_image,
            views: self.views,
            author: func(self.author),
            excerpt: self.excerpt,
            article_url: self.article_url,
            legislation_ids: self.legislation_ids,
            member_ids: self.member_ids,
            district_ids: self.district_ids,
            jurisdiction_ids: self.jurisdiction_ids,
        }
    }
}
/*
Might be dead code

#[derive(Serialize, ToSchema)]
pub struct PostResponse {
    #[schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z")]
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub title: String,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub published_at: Option<DateTimeWithTimeZone>,
    pub cover_image: Option<String>,
    pub author: i32,
    pub views: i32,
}
 */

/// Full post view (used when viewing a single post)
/// Note: Uses serde_json::Value for complex nested types that vary
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViewPostResponse {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    pub id: i32,
    pub title: String,
    pub content: String,
    #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>, format = DateTime))]
    pub published_at: Option<DateTime<FixedOffset>>,
    pub cover_image: Option<Url>,
    pub author: MemberView,
    pub excerpt: String,
    pub views: i32,
    pub likes: u64,
    pub comments: u64,
    pub user_sentiment: Option<UserSentiment>,
    pub tags: Vec<TagResponse>,
    pub attachments: Vec<AttachmentResponse>,
    pub legislation_ids: Vec<i32>,
    pub member_ids: Vec<i32>,
    pub district_ids: Vec<(i32, i32)>,
    pub jurisdiction_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCoverImageResponse {
    pub cover_image: String,
}

/// Response for editing a post - includes the full content.
///
/// Does not utilize typical excerpt/content summarizing methodology.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostContentResponse {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    pub id: i32,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub published_at: Option<DateTime<FixedOffset>>,
    pub cover_image: Option<String>,
    pub author: i32,
    pub legislation_ids: Vec<i32>,
    pub member_ids: Vec<i32>,
    pub district_ids: Vec<(i32, i32)>,
    pub jurisdiction_ids: Vec<i32>,
}
