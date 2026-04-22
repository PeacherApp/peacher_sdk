use std::{borrow::Cow, error::Error};

use serde::Serialize;
use thiserror::Error;

use crate::peanut::multipart::MultipartForm;

#[derive(Error, Debug)]
pub enum BodyError {
    #[error("Body Serialization: {0}")]
    Serialize(Box<dyn Error + Send + Sync>),
    #[error("Body Deserialization: {0}")]
    Deserialize(Box<dyn Error + Send + Sync>),
}

pub struct BodyBuilder {
    #[allow(clippy::type_complexity)]
    pub(super) inner: Option<Result<(Cow<'static, str>, Vec<u8>), BodyError>>,
}
impl BodyBuilder {
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        match serde_json::to_vec(json) {
            Ok(body) => {
                self.inner = Some(Ok(("application/json".into(), body)));
            }
            Err(err) => self.inner = Some(Err(BodyError::Serialize(Box::new(err)))),
        }
        self
    }
    pub fn form<T: Serialize + ?Sized>(mut self, form: &T) -> Self {
        match serde_urlencoded::to_string(form) {
            Ok(body) => {
                self.inner = Some(Ok((
                    "application/x-www-form-urlencoded".into(),
                    body.into(),
                )));
            }
            Err(err) => self.inner = Some(Err(BodyError::Serialize(Box::new(err)))),
        }

        self
    }

    /// Form body using bracket-notation nesting (`foo[bar]=baz`).
    ///
    /// Use this for Stripe's REST API, which expects nested parameters in
    /// `application/x-www-form-urlencoded` with bracket syntax rather than flat keys.
    pub fn qs<T: Serialize>(mut self, form: &T) -> Self {
        match serde_qs::to_string(form) {
            Ok(body) => {
                self.inner = Some(Ok((
                    "application/x-www-form-urlencoded".into(),
                    body.into(),
                )));
            }
            Err(err) => self.inner = Some(Err(BodyError::Serialize(Box::new(err)))),
        }
        self
    }

    /// Set a multipart body with a custom content type (e.g., "multipart/form-data; boundary=...")
    pub fn multipart(mut self, form: MultipartForm) -> Self {
        // TODO: we can go back and stream body potentially. In the event of reqwest,
        // that operation becomes a bit redundant.
        let header = form.content_type();
        let bytes = form.into_form_bytes();

        self.inner = Some(Ok((header.into(), bytes)));

        self
    }
}
