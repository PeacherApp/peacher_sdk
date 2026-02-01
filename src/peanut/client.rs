use std::fmt;

use http::StatusCode;
use reqwest::Request;
use serde::de::DeserializeOwned;
use url::Url;

use crate::request::BodyError;

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

// pub trait SdkConfig: Send + Sync {
//     fn base_url(&self) -> Url;

//     /// This function can be used to configure the reqwest client.
//     ///
//     /// For example, if you need to authenticate with the config before
//     /// running to get back a session key, you should do this here.
//     ///
//     /// If this fails, the sdk will not be otherwise queried.
//     fn before_sdk_run(&self, client: Client) -> impl Future<Output = SdkResult<Client>> {
//         async move { Ok(client) }
//     }

//     fn before_request(&self, builder: RequestBuilder) -> RequestBuilder {
//         builder
//     }

//     #[expect(unused_variables)]
//     fn after_response(&self, status: StatusCode) -> impl Future<Output = SdkResult<()>> {
//         crate::DoNothing
//     }

//     fn fail_on_bad_status(&self) -> bool {
//         true
//     }
// }

// /// This is a base client that can be used to perform SDK requests.
// ///
// /// This is distinct from [`UtilityClient`]. `UtilityClient` is used for generic client requests.
// /// This is typically to some external api.
// ///
// pub trait SdkClient: Send + Sync {
//     type Sdk;

//     // /// If this client actually calls out to some external resource. Useful if you need
//     // /// to grab an api token
//     // const FAKES_REQUESTS: bool;

//     /// Handle a request from a route that is tied to your config.
//     ///
//     /// Notice the trait bound. in order to complete a url, the route handler needs your specific config.
//     fn request<H>(&self, handler: &H) -> impl Future<Output = SdkResult<H::ResponseBody>>
//     where
//         H: Handler;
// }
