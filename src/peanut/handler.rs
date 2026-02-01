use crate::prelude::*;
use http::HeaderMap;
use serde::{Serialize, de::DeserializeOwned};
use std::borrow::Cow;

pub trait ResponseBody: Sized {
    fn extract_from_response<R: PeanutResponse>(
        response: R,
    ) -> impl Future<Output = Result<Self, BodyError>>;
}

pub struct NoResponse;
impl ResponseBody for NoResponse {
    fn extract_from_response<R: PeanutResponse>(
        _: R,
    ) -> impl Future<Output = Result<Self, BodyError>> {
        NoResponse
    }
}

impl Future for NoResponse {
    type Output = Result<Self, BodyError>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(Ok(NoResponse))
    }
}

impl<T> ResponseBody for T
where
    T: DeserializeOwned,
{
    fn extract_from_response<R: PeanutResponse>(
        response: R,
    ) -> impl Future<Output = Result<Self, BodyError>> {
        response.json()
    }
}

pub trait Handler: Send {
    type ResponseBody: ResponseBody;

    fn method(&self) -> Method;
    fn path(&self) -> Cow<'_, str>;

    /// you can typically use serde_url_encoded here
    fn params(&self) -> impl SdkParams {
        NoParams
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder
    }

    #[expect(unused_variables)]
    fn headers(&self, headers: &mut HeaderMap) {}

    fn after_response(&self) -> impl Future<Output = SdkResult<()>> {
        crate::peanut::DoNothing
    }
}

#[derive(Clone, Copy)]
pub struct NoParams;
impl SdkParams for NoParams {
    fn into_params(self) -> Option<String> {
        None
    }
}

/// Should return a url encoded query string.
///
/// If you need to just write some raw parameters, use [`Params`].
pub trait SdkParams: Send {
    fn into_params(self) -> Option<String>;
}
#[derive(Clone)]
pub struct Params(pub String);
impl Params {
    pub fn new(val: impl ToString) -> Self {
        Self(val.to_string())
    }
}

impl SdkParams for Params {
    fn into_params(self) -> Option<String> {
        Some(self.0)
    }
}

impl<S: Serialize + Send> SdkParams for S {
    fn into_params(self) -> Option<String> {
        match serde_qs::to_string(&self) {
            Ok(result) => Some(result),
            Err(e) => {
                tracing::error!("Could not serialize params for request: {e:?}");
                None
            }
        }
    }
}

pub trait GetHandler: Send {
    type ResponseBody: ResponseBody;
    fn path(&self) -> Cow<'_, str>;
    /// you can typically use serde_url_encoded here
    fn params(&self) -> impl SdkParams {
        NoParams
    }
    #[expect(unused_variables)]
    fn headers(&self, headers: &mut HeaderMap) {}

    fn after_response(&self) -> impl Future<Output = SdkResult<()>> {
        crate::peanut::DoNothing
    }
}

impl<T> Handler for T
where
    T: GetHandler,
{
    type ResponseBody = <Self as GetHandler>::ResponseBody;

    fn headers(&self, headers: &mut HeaderMap) {
        GetHandler::headers(self, headers)
    }

    fn params(&self) -> impl SdkParams {
        GetHandler::params(self)
    }

    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> Cow<'_, str> {
        GetHandler::path(self)
    }
    fn after_response(&self) -> impl Future<Output = SdkResult<()>> {
        GetHandler::after_response(self)
    }
}

// pub struct Route<Sdk, Req, Res, Params> {
//     method: Method,
//     path: String,
//     params: Params,
//     body: Req,
//     _p: PhantomData<(Sdk, Res)>,
// }

// impl<T> Route<T, NoRequestBody, serde_json::Value, &'static str> {
//     pub fn new(method: Method, val: impl ToString) -> Self {
//         Self {
//             method,
//             path: val.to_string(),
//             body: NoRequestBody,
//             params: "",
//             _p: PhantomData,
//         }
//     }
//     pub fn get(val: impl ToString) -> Self {
//         Self::new(Method::Get, val)
//     }
//     pub fn post(val: impl ToString) -> Self {
//         Self::new(Method::Post, val)
//     }
// }
// impl<Sdk, Req, Res, Params> Route<Sdk, Req, Res, Params> {
//     //pub fn params(&self, )
//     pub fn body<Res2>(self, body: Res2) -> Route<Sdk, Res2, Res, Params> {
//         Route {
//             method: self.method,
//             path: self.path,
//             params: self.params,
//             body,
//             _p: PhantomData,
//         }
//     }
//     pub fn respond_with<N>(self) -> Route<Sdk, Req, N, Params> {
//         Route {
//             method: self.method,
//             path: self.path,
//             params: self.params,
//             body: self.body,
//             _p: PhantomData,
//         }
//     }
//     pub fn params<NewParams>(self, params: NewParams) -> Route<Sdk, Req, Res, NewParams> {
//         Route {
//             method: self.method,
//             path: self.path,
//             params,
//             body: self.body,
//             _p: PhantomData,
//         }
//     }
// }

// impl<Sdk, Req, Res, Params> Handler for Route<Sdk, Req, Res, Params>
// where
//     Sdk: SdkConfig,
//     Req: SdkRequestBody,
//     Res: ResponseType,
//     Params: SdkParams + Clone,
// {
//     type ResponseBody = Res;
//     fn method(&self) -> Method {
//         self.method
//     }
//     fn path(&self) -> Cow<'_, str> {
//         Cow::Borrowed(&self.path)
//     }
//     fn params(&self) -> impl SdkParams {
//         self.params.clone()
//     }
//     fn request_body(&self) -> &Self::RequestBody {
//         &self.body
//     }
// }
