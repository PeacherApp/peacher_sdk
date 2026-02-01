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
        reqwest::Response::json(self)
            .await
            .map_err(|e| BodyError::Deserialize(Box::new(e)))
    }

    async fn text(self) -> Result<String, BodyError> {
        self.text()
            .await
            .map_err(|e| BodyError::Deserialize(Box::new(e)))
    }
}
