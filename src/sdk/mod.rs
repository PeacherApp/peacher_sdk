// use std::borrow::Cow;

// use reqwest::RequestBuilder;

// use crate::prelude::*;

// pub trait Endpoint {
//     fn method(&self) -> Method;
//     fn path(&self) -> Cow<'_, str>;
//     fn params(&self) -> impl Params {
//         NoValue
//     }
//     fn request_body(&self) -> &impl RequestBody;
//     fn headers(&self, headers: RequestHeaders) -> RequestHeaders {
//         headers
//     }
// }

// pub trait RequestBody {
//     fn set_request_body(&self, request: RequestBuilder) -> RequestBuilder;
// }

// struct NoValue;

// pub trait Params {
//     fn into_params(self) -> Option<String>;
// }
// impl Params for NoValue {
//     fn into_params(self) -> Option<String> {
//         None
//     }
// }
// impl RequestBody for NoValue {
//     fn set_request_body(&self, request: RequestBuilder) -> RequestBuilder {
//         request
//     }
// }
