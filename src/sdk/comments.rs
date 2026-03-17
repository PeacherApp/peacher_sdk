use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{paginated, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CommentView {
    pub content: ContentView,
    pub post_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub depth: i32,
    pub reply_count: i64,
}
impl CommentView {
    pub fn id(&self) -> Uuid {
        self.content.id()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct CommentParams {
    pub parent_comment_id: Option<Uuid>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(CommentParams);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCommentRequest {
    pub body: SetContentRequest,
    pub parent_comment_id: Option<Uuid>,
}

/// List comments for a post (top-level by default, or replies to a specific comment)
pub struct ListPostComments {
    post_id: Uuid,
    pub params: CommentParams,
}

impl ListPostComments {
    pub fn new(post_id: Uuid) -> Self {
        Self {
            post_id,
            params: CommentParams::default(),
        }
    }
}

impl GetHandler for ListPostComments {
    type ResponseBody = Paginated<CommentView>;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}/comments", self.post_id).into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}

/// Get a single comment by content_item_id
pub struct GetComment(pub Uuid);

impl GetHandler for GetComment {
    type ResponseBody = CommentView;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/comments/{}", self.0).into()
    }
}

/// Create a comment on a post
pub struct CreateComment {
    post_id: Uuid,
    body: CreateCommentRequest,
}

impl CreateComment {
    pub fn new(post_id: Uuid, body: CreateCommentRequest) -> Self {
        Self { post_id, body }
    }
}

impl Handler for CreateComment {
    type ResponseBody = CommentView;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/posts/{}/comments", self.post_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// List replies to a comment
pub struct ListCommentReplies {
    comment_id: Uuid,
    pub params: CommentParams,
}

impl ListCommentReplies {
    pub fn new(comment_id: Uuid) -> Self {
        Self {
            comment_id,
            params: CommentParams::default(),
        }
    }
}

impl GetHandler for ListCommentReplies {
    type ResponseBody = Paginated<CommentView>;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/comments/{}/replies", self.comment_id).into()
    }

    fn params(&self) -> impl SdkParams {
        self.params.clone()
    }
}
