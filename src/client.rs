use peanut::client::Client;
use peanut::error::SdkError;
use peanut::handler::{Handler, ResponseBody};
use peanut::query::{Query, QueryError};
use reqwest::header::{AUTHORIZATION, HeaderValue};
use url::Url;

#[derive(Clone)]
pub struct PeacherClient {
    pub base: Url,
    /// API key for bearer token authentication (required)
    pub api_key: String,
    pub client: reqwest::Client,
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
        let auth_value = format!("Bearer {}", self.api_key);
        request.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).expect("Invalid auth header value"),
        );

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

impl PeacherClient {
    /// Create a new PeacherClient with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        let client = reqwest::Client::new();
        Self {
            base: Url::parse("https://peacher.app").unwrap(),
            api_key: api_key.into(),
            client,
        }
    }

    /// Set a custom base URL
    pub fn with_base_url(mut self, url: Url) -> Self {
        self.base = url;
        self
    }
}
