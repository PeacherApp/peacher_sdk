use http::{
    HeaderValue, StatusCode,
    header::{CONTENT_TYPE, InvalidHeaderValue},
};
use reqwest::Request;
use thiserror::Error;

use crate::prelude::*;

pub trait Query<T, C> {
    type Error;
    fn query(&self, client: &C) -> impl Future<Output = Result<T, QueryError<Self::Error>>>;
}
impl<T, C, E> Query<T, C> for E
where
    C: Client,
    E: Handler<ResponseBody = T>,
    T: ResponseBody,
{
    type Error = C::Error;
    async fn query(&self, client: &C) -> Result<T, QueryError<C::Error>> {
        let endpoint = self.path();
        let mut url = client.endpoint(&endpoint).map_err(QueryError::Client)?;
        if let Some(params) = self.params().into_params() {
            url.set_query(Some(&params));
        }
        let mut request = Request::new(self.method().to_reqwest_method(), url);

        self.headers(request.headers_mut());

        let body = self.request_body(BodyBuilder { inner: None });

        match body.inner {
            Some(Ok((content_type, body_bytes))) => {
                if !request.headers().contains_key(CONTENT_TYPE) {
                    request.headers_mut().insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str(&content_type).map_err(QueryError::Headers)?,
                    );
                }

                *request.body_mut() = Some(body_bytes.into());
            }
            Some(Err(e)) => return Err(QueryError::Body(e)),
            None => {}
        }

        // let new_req: http::Request<reqwest::Body> = request
        //     .try_into()
        //     .map_err(|e| QueryError::Body(BodyError::Serialize(Box::new(e))))?;

        let response = client.execute(request).await.map_err(QueryError::Client)?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let text = match response.text().await {
                Ok(text) => text,
                Err(e) => e.to_string(),
            };

            return Err(QueryError::Status(status, text));
        }

        let body = T::extract_from_response(response)
            .await
            .map_err(QueryError::ResponseBody)?;

        Ok(body)
    }
}

#[derive(Error, Debug)]
pub enum QueryError<C> {
    #[error("Invalid Header {0}")]
    Headers(InvalidHeaderValue),
    #[error("(Request) {0}")]
    Body(BodyError),
    #[error("(Response) {0}")]
    ResponseBody(BodyError),
    #[error("(Status) {0}: {1}")]
    Status(StatusCode, String),
    #[error("(Client) {0}")]
    Client(C),
}
