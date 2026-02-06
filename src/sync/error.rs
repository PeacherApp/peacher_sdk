use thiserror::Error;

use crate::prelude::{ExternalId, SdkError};

pub type SyncResult<T> = Result<T, SyncError>;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("{0}")]
    Sdk(#[from] SdkError),

    #[error("External not found for resource: {0}")]
    NotFound(ExternalId),

    #[error("Something internally failed: {0}")]
    InternalIssue(String),
}
impl SyncError {
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalIssue(msg.into())
    }
}
