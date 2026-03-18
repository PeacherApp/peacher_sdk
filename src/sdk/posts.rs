use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{paginated, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RemovedPost {
    pub title: String,
    pub district_id: i32,
    pub pinned: bool,
    pub content: RemovedContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostUnderReview {
    pub title: String,
    pub district_id: i32,
    pub pinned: bool,
    pub content: ContentUnderReview,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PostDetails {
    pub link: Option<PostLink>,
    pub title: String,
    pub district_id: i32,
    pub num_comments: u32,
    pub pinned: bool,
    pub content: ContentDetails,
    pub editable_until: Option<DateTime<FixedOffset>>,
}
impl PostDetails {
    pub fn id(&self) -> Uuid {
        self.content.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
#[expect(clippy::large_enum_variant)]
pub enum PostView {
    Content(PostDetails),
    UnderReview(PostUnderReview),
    Removed(RemovedPost),
}
impl PostView {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Content(c) => c.content.id,
            Self::UnderReview(c) => c.content.id,
            Self::Removed(c) => c.content.id,
        }
    }
    pub fn district_id(&self) -> i32 {
        match self {
            Self::Content(c) => c.district_id,
            Self::Removed(c) => c.district_id,
            Self::UnderReview(c) => c.district_id,
        }
    }
    pub fn title(&self) -> &str {
        match self {
            Self::Content(c) => &c.title,
            Self::Removed(c) => &c.title,
            Self::UnderReview(c) => &c.title,
        }
    }
    pub fn pinned(&self) -> bool {
        match self {
            Self::Content(c) => c.pinned,
            Self::Removed(r) => r.pinned,
            Self::UnderReview(u) => u.pinned,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PostLink {
    Article(Url),
    Media(AttachmentResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum PostSort {
    #[default]
    Hot,
    New,
    Top,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct PostParams {
    /// Comma-separated list of district IDs, e.g. "1,2,3"
    pub district_ids: CommaSeparated<i32>,
    pub author_id: Option<i32>,
    pub search: Option<String>,
    pub pinned: Option<bool>,
    pub sort: PostSort,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(PostParams);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePostRequest {
    pub media: Option<NewPostMedia>,
    pub title: String,
    pub district_id: i32,
    pub body: SetContentRequest,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum NewPostMedia {
    Attachment { attachment_id: Uuid },
    Article { href: Url },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<SetContentRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DeletePostRequest {
    pub remove_title: bool,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PinPostRequest {
    pub pinned: bool,
}

/// List posts with optional filters
#[derive(Default)]
pub struct ListPosts {
    pub params: PostParams,
}

impl ListPosts {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GetHandler for ListPosts {
    type ResponseBody = Paginated<PostView>;

    fn path(&self) -> Cow<'_, str> {
        "/api/posts".into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Get a single post by content_item_id
pub struct GetPost(pub Uuid);

impl GetHandler for GetPost {
    type ResponseBody = PostView;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}", self.0).into()
    }
}

/// Create a new post
pub struct CreatePost {
    body: CreatePostRequest,
}

impl CreatePost {
    pub fn new(body: CreatePostRequest) -> Self {
        Self { body }
    }
}

impl Handler for CreatePost {
    type ResponseBody = PostView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/posts".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Update a post (within editable window or admin)
pub struct UpdatePost {
    id: Uuid,
    body: UpdatePostRequest,
}

impl UpdatePost {
    pub fn new(id: Uuid, body: UpdatePostRequest) -> Self {
        Self { id, body }
    }
}

impl Handler for UpdatePost {
    type ResponseBody = PostView;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Delete a post
pub struct DeletePost(pub Uuid);

impl Handler for DeletePost {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}", self.0).into()
    }
}

/// Pin or unpin a post (community moderator or admin)
pub struct PinPost {
    id: Uuid,
    body: PinPostRequest,
}

impl PinPost {
    pub fn new(id: Uuid, pinned: bool) -> Self {
        Self {
            id,
            body: PinPostRequest { pinned },
        }
    }
}

impl Handler for PinPost {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}/pin", self.id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// List posts for a specific community
pub struct ListCommunityPosts {
    community_id: i32,
    pub params: PostParams,
}

impl ListCommunityPosts {
    pub fn new(community_id: i32) -> Self {
        Self {
            community_id,
            params: PostParams::default(),
        }
    }
}

impl GetHandler for ListCommunityPosts {
    type ResponseBody = Paginated<PostView>;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/communities/{}/posts", self.community_id).into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}
