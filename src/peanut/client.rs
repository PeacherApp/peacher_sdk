use std::fmt;

use http::StatusCode;
use reqwest::Request;
use serde::de::DeserializeOwned;
use url::Url;

use crate::peanut::request::BodyError;

pub trait Client {
    type Error: fmt::Debug + Send + Sync + 'static;
    type Response: PeanutResponse;
    fn endpoint(&self, path: &str) -> Result<Url, Self::Error>;

    fn execute(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>>;
}

pub trait PeanutResponse {
    fn status(&self) -> StatusCode;
    fn json<T: DeserializeOwned>(self) -> impl Future<Output = Result<T, BodyError>>;
    fn text(self) -> impl Future<Output = Result<String, BodyError>>;
}

impl PeanutResponse for reqwest::Response {
    fn status(&self) -> StatusCode {
        reqwest::Response::status(self)
    }

    async fn json<T: DeserializeOwned>(self) -> Result<T, BodyError> {
        let bytes = self
            .bytes()
            .await
            .map_err(|e| BodyError::Deserialize(Box::new(e)))?;

        serde_json::from_slice::<T>(&bytes).map_err(|e| {
            BodyError::Deserialize(Box::new(JsonDecodeError {
                source: e,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            }))
        })
    }

    async fn text(self) -> Result<String, BodyError> {
        self.text()
            .await
            .map_err(|e| BodyError::Deserialize(Box::new(e)))
    }
}

/// Wraps a `serde_json` decode failure with a preview of the raw body so
/// the resulting log line identifies both the offending field (via serde's
/// path/line/column) and the actual bytes Stripe (or any other upstream)
/// returned.
#[derive(Debug)]
struct JsonDecodeError {
    source: serde_json::Error,
    body: String,
}

impl fmt::Display for JsonDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MAX_PREVIEW: usize = 2048;
        let preview = if self.body.len() > MAX_PREVIEW {
            format!(
                "{}… (truncated, {} bytes total)",
                &self.body[..MAX_PREVIEW],
                self.body.len()
            )
        } else {
            self.body.clone()
        };
        write!(f, "{} — raw body: {}", self.source, preview)
    }
}

impl std::error::Error for JsonDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
