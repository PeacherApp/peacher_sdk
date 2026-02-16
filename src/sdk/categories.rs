use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: String,
    pub primary_color: String,
    pub secondary_color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCategoryRequest {
    pub description: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

/// List all categories
pub struct ListCategories;

impl GetHandler for ListCategories {
    type ResponseBody = Vec<Category>;

    fn path(&self) -> Cow<'_, str> {
        "/api/categories".into()
    }
}

/// Create a category
pub struct CreateCategory {
    body: CreateCategoryRequest,
}

impl CreateCategory {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        primary_color: impl Into<String>,
        secondary_color: impl Into<String>,
    ) -> Self {
        Self {
            body: CreateCategoryRequest {
                name: name.into(),
                description: description.into(),
                primary_color: primary_color.into(),
                secondary_color: secondary_color.into(),
            },
        }
    }
}

impl Handler for CreateCategory {
    type ResponseBody = Category;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/categories".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Update a category
pub struct UpdateCategory {
    name: String,
    body: UpdateCategoryRequest,
}

impl UpdateCategory {
    pub fn new(name: impl Into<String>, body: UpdateCategoryRequest) -> Self {
        Self {
            name: name.into(),
            body,
        }
    }
}

impl Handler for UpdateCategory {
    type ResponseBody = Category;

    fn method(&self) -> Method {
        Method::Patch
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/categories/{}", self.name).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

/// Delete a category
pub struct DeleteCategory(pub String);

impl DeleteCategory {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl Handler for DeleteCategory {
    type ResponseBody = NoResponse;

    fn method(&self) -> Method {
        Method::Delete
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/categories/{}", self.0).into()
    }
}

// --- Account Categories ---

/// Request body for setting account categories
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetAccountCategoriesRequest {
    pub categories: Vec<String>,
}

/// Get categories for the current account
pub struct GetAccountCategories;

impl GetHandler for GetAccountCategories {
    type ResponseBody = Vec<Category>;

    fn path(&self) -> Cow<'_, str> {
        "/api/account/categories".into()
    }
}

/// Set categories for the current account (replaces all)
pub struct SetAccountCategories {
    body: SetAccountCategoriesRequest,
}

impl SetAccountCategories {
    pub fn new(categories: Vec<String>) -> Self {
        Self {
            body: SetAccountCategoriesRequest { categories },
        }
    }
}

impl Handler for SetAccountCategories {
    type ResponseBody = Vec<Category>;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        "/api/account/categories".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

// --- Legislation Categories ---

/// Request body for setting legislation categories
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetLegislationCategoriesRequest {
    pub categories: Vec<String>,
}

/// Get categories for a piece of legislation
pub struct GetLegislationCategories(pub i32);

impl GetHandler for GetLegislationCategories {
    type ResponseBody = Vec<Category>;

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/categories", self.0).into()
    }
}

/// Set categories for a piece of legislation (replaces all)
pub struct SetLegislationCategories {
    legislation_id: i32,
    body: SetLegislationCategoriesRequest,
}

impl SetLegislationCategories {
    pub fn new(legislation_id: i32, categories: Vec<String>) -> Self {
        Self {
            legislation_id,
            body: SetLegislationCategoriesRequest { categories },
        }
    }
}

impl Handler for SetLegislationCategories {
    type ResponseBody = Vec<Category>;

    fn method(&self) -> Method {
        Method::Put
    }

    fn path(&self) -> Cow<'_, str> {
        format!("/api/legislation/{}/categories", self.legislation_id).into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(&self.body)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Category {
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub created_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z"))]
    pub updated_at: DateTime<FixedOffset>,
    pub name: String,
    pub description: String,
    pub primary_color: String,
    pub secondary_color: String,
}
