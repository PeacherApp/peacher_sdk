use std::io::Write;

use crate::prelude::*;
use base64::{prelude::BASE64_STANDARD, write::EncoderWriter};
use reqwest::header::{AUTHORIZATION, HeaderValue};
use url::Url;

#[derive(Clone)]
pub struct StripeClient {
    /// The URL where stripe is located
    pub base: Url,
    /// API key for bearer token authentication (required)
    pub secret: String,
    pub client: reqwest::Client,
}

impl StripeClient {
    /// Create a new PeacherClient with the given API key.
    ///
    /// If you do not have an API key, use [`PeacherClient::default`]
    pub fn new(secret: impl Into<String>) -> Self {
        let client = reqwest::Client::new();
        Self {
            base: Url::parse("https://api.stripe.com").unwrap(),
            secret: secret.into(),
            client,
        }
    }
}

impl Client for StripeClient {
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

        let mut buf = b"Basic ".to_vec();
        {
            let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
            let _ = write!(encoder, "{}:", &self.secret);
        }
        let mut header = HeaderValue::from_maybe_shared(bytes::Bytes::from(buf))
            .expect("base64 is always valid HeaderValue");
        header.set_sensitive(true);
        request.headers_mut().insert(AUTHORIZATION, header);
        self.client.execute(request).await
    }
}
