mod companions;
pub use companions::*;

mod jurisdiction;
pub use jurisdiction::*;

mod legislation;
pub use legislation::*;

mod legislation_vote;
pub use legislation_vote::*;

mod member;
pub use member::*;

mod session;
pub use session::*;

use std::{
    convert::Infallible,
    fmt::{self, Debug},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::commaparam;

/// An id that's in our database, so not from an external resource.
#[derive(Clone, Copy)]
pub struct DatabaseId(pub i32);

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalId(String);
impl FromStr for ExternalId {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

commaparam!(ExternalId, "ga-114,us-372,il-10");

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

