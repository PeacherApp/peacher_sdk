use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{paginated, prelude::*};

/// List posts with optional filters
#[derive(Default)]
pub struct ListPosts(pub PostParams);

impl GetHandler for ListPosts {
    // Use Value for flexibility - actual type is Paginated<PostSummary<MemberView>>
    type ResponseBody = Paginated<serde_json::Value>;

    fn path(&self) -> Cow<'_, str> {
        "/api/posts".into()
    }

    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct PostParams {
    pub search: Option<String>,
    pub status: Option<String>,
    pub visibility: Option<String>,
    pub sort: Option<String>,
    pub legislation_id: Option<i32>,
    pub member_id: Option<i32>,
    pub district_id: Option<i32>,
    pub jurisdiction_id: Option<i32>,
    #[serde(skip)]
    pub author: Option<i32>,
    #[serde(skip)]
    pub viewer: Option<i32>,

    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(PostParams);

impl PostParams {
    pub fn set_author(mut self, author: i32) -> Self {
        self.author = Some(author);
        self
    }

    pub fn set_viewer(mut self, id: Option<i32>) -> Self {
        self.viewer = id;
        self
    }
}

/// Get a single post by ID
pub struct GetPost(pub i32);

impl GetHandler for GetPost {
    // Use Value for flexibility - actual type is ViewPostResponse
    type ResponseBody = serde_json::Value;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}", self.0).into()
    }
}

/// Create a new post (requires authentication)
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub publish: bool,
    pub cover_image: Option<String>,
    #[serde(default)]
    pub legislation_ids: Vec<i32>,
    #[serde(default)]
    pub member_ids: Vec<i32>,
    #[serde(default)]
    pub district_ids: Vec<(i32, i32)>,
    #[serde(default)]
    pub jurisdiction_ids: Vec<i32>,
}

impl CreatePost {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            publish: false,
            ..Default::default()
        }
    }

    pub fn publish(mut self) -> Self {
        self.publish = true;
        self
    }
}

impl Handler for CreatePost {
    // Use Value for flexibility - actual type is PostSummary<i32>
    type ResponseBody = serde_json::Value;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/posts".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(self)
    }
}

/// Update a post (requires authentication)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdatePost {
    #[serde(skip)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub publish: bool,
    #[serde(default)]
    pub legislation_ids: Vec<i32>,
    #[serde(default)]
    pub member_ids: Vec<i32>,
    #[serde(default)]
    pub district_ids: Vec<(i32, i32)>,
    #[serde(default)]
    pub jurisdiction_ids: Vec<i32>,
}

impl UpdatePost {
    pub fn new(id: i32, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
            excerpt: None,
            publish: false,
            legislation_ids: vec![],
            member_ids: vec![],
            district_ids: vec![],
            jurisdiction_ids: vec![],
        }
    }

    pub fn publish(mut self) -> Self {
        self.publish = true;
        self
    }
}

impl Handler for UpdatePost {
    // Use Value for flexibility - actual type is PostContentResponse
    type ResponseBody = serde_json::Value;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(self)
    }
}

/// Delete a post (requires authentication)
pub struct DeletePost(pub i32);

impl Handler for DeletePost {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}", self.0).into()
    }
}

/// Get post content for editing (requires authentication, must be author)
pub struct GetPostContent(pub i32);

impl GetHandler for GetPostContent {
    // Use Value for flexibility - actual type is PostContentResponse
    type ResponseBody = serde_json::Value;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}/content", self.0).into()
    }
}

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
