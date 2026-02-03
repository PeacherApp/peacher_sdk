use std::str::FromStr;

use crate::{
    peanut::query::{Query as _, QueryError},
    prelude::*,
};
use reqwest::header::{AUTHORIZATION, HeaderValue};
use url::Url;

impl Default for PeacherClient {
    fn default() -> Self {
        Self {
            base: Url::from_str("https://api.peacher.app/").unwrap(),
            api_key: None,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Clone)]
pub struct PeacherClient {
    /// The URL where peacher is located
    pub base: Url,
    /// API key for bearer token authentication (required)
    pub api_key: Option<String>,
    pub client: reqwest::Client,
}

impl PeacherClient {
    /// Create a new PeacherClient with the given API key.
    ///
    /// If you do not have an API key, use [`PeacherClient::default`]
    pub fn new(api_key: impl Into<String>) -> Self {
        let client = reqwest::Client::new();
        Self {
            base: Url::parse("https://api.peacher.app").unwrap(),
            api_key: Some(api_key.into()),
            client,
        }
    }

    /// Set a custom base URL
    pub fn with_base_url(mut self, url: Url) -> Self {
        self.base = url;
        self
    }
}

impl Client for PeacherClient {
    type Error = reqwest::Error;
    type Response = reqwest::Response;

    fn endpoint(&self, path: &str) -> Result<Url, Self::Error> {
        Ok(self.base.join(path).unwrap())
    }

    async fn execute(
        &self,
        mut request: reqwest::Request,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // Add Authorization: Bearer {api_key} header
        if let Some(api_key) = &self.api_key {
            let auth_value = format!("Bearer {}", api_key);
            request.headers_mut().insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value).expect("Invalid auth header value"),
            );
        }
        self.client.execute(request).await
    }
}

/// Extension trait to add `.request()` method to handlers
pub trait HandlerExt<T>: Handler<ResponseBody = T>
where
    T: ResponseBody,
{
    /// Execute the request against the given client
    fn request<P: Client>(self, client: &P) -> impl Future<Output = Result<T, SdkError>>;
}

impl<H, T> HandlerExt<T> for H
where
    H: Handler<ResponseBody = T>,
    T: ResponseBody,
{
    async fn request<P: Client>(self, client: &P) -> Result<T, SdkError> {
        self.query(client).await.map_err(|e| match e {
            QueryError::Status(status, text) => SdkError::Status(status, text),
            QueryError::Body(body_err) => SdkError::message(format!("Body error: {}", body_err)),
            QueryError::ResponseBody(body_err) => {
                SdkError::message(format!("Response body error: {}", body_err))
            }
            QueryError::Headers(e) => SdkError::message(format!("Header error: {}", e)),
            QueryError::Client(c) => SdkError::message(format!("Client Errored: {c:?}")),
        })
    }
}
