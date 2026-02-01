use crate::peanut::prelude::*;
use serde::{Deserialize, Serialize};

pub struct GetHealthCheck;

impl GetHandler for GetHealthCheck {
    type ResponseBody = HealthResponse;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/_health".into()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct HealthResponse {
    #[cfg_attr(feature = "utoipa", schema(example = "ok"))]
    pub status: String,
    #[cfg_attr(feature = "utoipa", schema(example = "0.1.0"))]
    pub version: String,
    #[cfg_attr(feature = "utoipa", schema(example = "development"))]
    pub environment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PingResponse {
    #[cfg_attr(feature = "utoipa", schema(example = "pong"))]
    pub message: String,
}

pub struct GetPing;

impl GetHandler for GetPing {
    type ResponseBody = PingResponse;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/_ping".into()
    }
}
