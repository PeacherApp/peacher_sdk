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
    pub use crate::client::*;
    pub use crate::error::*;
    pub use crate::handler::*;
    pub use crate::method::*;
    pub use crate::request::*;
}

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::error::SdkResult;

pub(crate) struct DoNothing;
impl Future for DoNothing {
    type Output = SdkResult<()>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<SdkResult<()>> {
        Poll::Ready(Ok(()))
    }
}
