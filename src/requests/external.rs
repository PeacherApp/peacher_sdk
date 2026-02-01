use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// An id that's in our database, so not from an external resource.
#[derive(Clone, Copy)]
pub struct DatabaseId(pub i32);

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalId(String);

impl fmt::Display for ExternalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

macro_rules! impl_from {
    ($ty:ty) => {
        impl From<$ty> for ExternalId {
            fn from(value: $ty) -> Self {
                ExternalId::new(value)
            }
        }
    };
}

impl_from!(i32);
impl_from!(u32);
impl_from!(i64);
impl_from!(u64);
impl_from!(i128);
impl_from!(isize);
impl_from!(usize);
impl_from!(String);
impl_from!(&str);

impl ExternalId {
    pub fn new(id: impl ToString) -> Self {
        Self(id.to_string())
    }
    pub fn val<T>(&self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        // this should panic if invalid. This is fatal
        self.0.parse().unwrap()
    }

    pub fn val_u32(&self) -> u32 {
        self.val()
    }
    pub fn val_i32(&self) -> i32 {
        self.val()
    }
    pub fn val_str(&self) -> &str {
        &self.0
    }
}

/// Metadata for external resources that tracks the external ID and URL
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalMetadata {
    pub external_id: ExternalId,
    pub url: Option<String>,
}

impl ExternalMetadata {
    pub fn new(external_id: impl Into<ExternalId>) -> Self {
        Self {
            external_id: external_id.into(),
            url: None,
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = Some(url.into());
    }
}
