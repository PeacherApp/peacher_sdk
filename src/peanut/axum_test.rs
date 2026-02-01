use axum_test::{TestResponse, TestServer};
use serde::de::DeserializeOwned;

use crate::peanut::prelude::*;

impl Client for TestServer {
    type Error = anyhow::Error;
    type Response = TestResponse;
    fn endpoint(&self, path: &str) -> Result<url::Url, Self::Error> {
        self.server_url(path)
    }
    async fn execute(&self, request: reqwest::Request) -> Result<TestResponse, Self::Error> {
        let mut t = self.method(request.method().clone(), request.url().path());
        if let Some(query) = request.url().query() {
            t = t.add_raw_query_param(query);
        }

        // Get content-type from request headers if present
        let content_type = request
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Add non-content-type headers
        for (key, v) in request.headers() {
            if key != http::header::CONTENT_TYPE {
                t = t.add_header(key, v);
            }
        }

        // Set body with explicit content-type to prevent bytes() from setting default
        if let Some(body) = request.body()
            && let Some(bytes) = body.as_bytes()
        {
            if let Some(ct) = content_type {
                t = t.content_type(&ct).bytes(bytes.to_vec().into());
            } else {
                t = t.bytes(bytes.to_vec().into());
            }
        }

        Ok(t.await)
    }
}

impl PeanutResponse for TestResponse {
    fn status(&self) -> http::StatusCode {
        TestResponse::status_code(self)
    }
    async fn json<T: DeserializeOwned>(self) -> Result<T, BodyError> {
        Ok(TestResponse::json(&self))
    }

    async fn text(self) -> Result<String, BodyError> {
        Ok(TestResponse::text(&self))
    }
}
