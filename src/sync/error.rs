use std::fmt;

use thiserror::Error;

use crate::prelude::{ExternalId, SdkError};

pub type SyncResult<T> = Result<T, SyncError>;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("{0}")]
    Sdk(#[from] SdkError),

    #[error("External not found for resource: {0}")]
    NotFound(ExternalId),

    #[error("Missing External ID: {0}")]
    NoExternalId(String),

    #[error("Something internally failed: {0}")]
    InternalIssue(String),
}

impl PartialEq for SyncError {
    fn eq(&self, other: &Self) -> bool {
        use SyncError::*;

        match (self, other) {
            (Sdk(_), Sdk(_)) => true,
            (NotFound(i1), NotFound(i2)) => i1 == i2,
            (NoExternalId(i1), NoExternalId(i2)) => i1 == i2,
            (InternalIssue(i1), InternalIssue(i2)) => i1 == i2,
            _ => false,
        }
    }
}

impl SyncError {
    pub fn no_external_id(msg: impl fmt::Debug) -> Self {
        Self::NoExternalId(format!("{:?}", msg))
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalIssue(msg.into())
    }
}
