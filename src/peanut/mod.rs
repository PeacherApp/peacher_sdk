pub mod client;
// pub mod clients;
pub mod error;
pub mod handler;
pub mod method;
pub mod multipart;
pub mod query;
pub mod request;

#[cfg(feature = "axum_test")]
mod axum_test;

pub mod prelude {
    pub use crate::peanut::client::*;
    pub use crate::peanut::error::*;
    pub use crate::peanut::handler::*;
    pub use crate::peanut::method::*;
    pub use crate::peanut::request::*;
}

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::peanut::error::SdkResult;

pub(crate) struct DoNothing;
impl Future for DoNothing {
    type Output = SdkResult<()>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<SdkResult<()>> {
        Poll::Ready(Ok(()))
    }
}
