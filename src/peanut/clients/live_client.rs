use reqwest::{Client, Request, Response};

use crate::prelude::*;

/// A live client that makes HTTP requests.
/// Clone is supported when the config type is Clone.
/// Note: reqwest::Client is internally Arc'd so cloning is cheap.
pub struct LiveClient<C> {
    client: Client,
    config: C,
}

impl<C: Clone> Clone for LiveClient<C> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
        }
    }
}

impl<C: SdkConfig> LiveClient<C> {
    pub async fn default() -> SdkResult<Self>
    where
        C: Default,
    {
        Self::new(C::default()).await
    }
    /// Creates a new config. Panics if TLS is not findable.
    pub async fn new(config: C) -> SdkResult<Self> {
        let client = Client::builder().cookie_store(true).build().unwrap();
        Self::new_with_client(client, config).await
    }
    pub async fn new_with_client(client: reqwest::Client, config: C) -> SdkResult<Self> {
        let client = config.before_sdk_run(client).await?;

        Ok(Self { client, config })
    }

    pub fn config(&self) -> &C {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Note: this will call `SdkConfig::before_request`.
    pub(crate) fn inner_request<H>(&self, handler: &H) -> Request
    where
        H: Handler,
    {
        let mut path = handler.url(&self.config);

        // the path may have params if the implementor just puts them on the path.
        // However, if they return some here, then you are free to overwrite the query
        if let Some(params) = handler.params().into_params() {
            path.set_query(Some(&params));
        }

        let request = self
            .client
            .request(handler.method().to_reqwest_method(), path);

        let RequestHeaders { inner: mut request } =
            handler.headers(RequestHeaders { inner: request });

        request = self.config.before_request(request);

        request = handler.request_body().set_request_body(request);

        request.build().unwrap()
    }

    /// Note: this will call `SdkConfig::after_request`. Wrappers should ultimately call this.
    pub(crate) async fn inner_response<H>(
        &self,
        handler: &H,
        response: Response,
    ) -> SdkResult<H::ResponseBody>
    where
        H: Handler,
    {
        handler.after_response().await?;

        self.config.after_response(response.status()).await?;
        if self.config.fail_on_bad_status() && response.status().is_server_error()
            || response.status().is_client_error()
        {
            let status = response.status();
            match response.text().await {
                Ok(text) => {
                    return Err(SdkError::Status2(status, text));
                }
                Err(e) => {
                    return Err(SdkError::Status2(
                        status,
                        format!("(Could not extract text for status) {e:?}"),
                    ));
                }
            }
        }

        H::ResponseBody::extract_from_response(response).await
    }
}

impl<C: SdkConfig> SdkClient for LiveClient<C> {
    type Sdk = C;
    async fn request<H>(&self, handler: &H) -> SdkResult<H::ResponseBody>
    where
        H: Handler,
    {
        let request = self.inner_request(handler);
        let response = self.client.execute(request).await.unwrap();
        self.inner_response(handler, response).await
    }
}
