// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod part;
pub(super) use part::*;
use std::{fmt::Display, io::Read};

use bytes::BytesMut;
use http::{HeaderName, HeaderValue};
use mime::Mime;

use crate::multipart::{
    body::Body,
    boundary::{BoundaryGenerator, RandomAsciiGenerator},
};

/// Implements the multipart/form-data media type as described by
/// RFC 7578.
///
/// [See](https://tools.ietf.org/html/rfc7578#section-1).
#[derive(Debug)]
pub struct Form<'a> {
    parts: Vec<Part<'a>>,

    /// The auto-generated boundary as described by 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    boundary: String,
}

impl<'a> Default for Form<'a> {
    /// Creates a new form with the default boundary generator.
    #[inline]
    fn default() -> Form<'a> {
        Form::new::<RandomAsciiGenerator>()
    }
}

impl<'a> Form<'a> {
    /// Creates a new form with the specified boundary generator function.
    pub fn new<G>() -> Form<'a>
    where
        G: BoundaryGenerator,
    {
        Form {
            parts: vec![],
            boundary: G::generate_boundary(),
        }
    }

    /// Adds a text part to the Form.
    pub fn add_text<N, T>(&mut self, name: N, text: T)
    where
        N: Display,
        T: Into<String>,
    {
        self.parts.push(Part::new::<_, String>(
            Inner::Text(text.into()),
            name,
            None,
            None,
            Default::default(),
        ))
    }

    pub fn add_reader_2<F, R>(
        &mut self,
        name: F,
        read: R,
        filename: Option<String>,
        mime: Option<Mime>,
        headers: Vec<(HeaderName, HeaderValue)>,
    ) where
        F: Display,
        R: 'a + Read + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::Read(read),
            name,
            mime,
            filename,
            headers,
        ));
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        let mut result = Vec::new();

        for part in &mut self.parts {
            // Start boundary
            result.extend_from_slice(b"--");
            result.extend_from_slice(self.boundary.as_bytes());
            result.extend_from_slice(b"\r\n");

            // Content-Disposition header
            result.extend_from_slice(b"Content-Disposition: ");
            result.extend_from_slice(part.content_disposition.as_bytes());
            result.extend_from_slice(b"\r\n");

            // Content-Type header
            result.extend_from_slice(b"Content-Type: ");
            result.extend_from_slice(part.content_type.as_bytes());
            result.extend_from_slice(b"\r\n");

            // Custom headers
            for (name, value) in &part.headers {
                result.extend_from_slice(name.as_str().as_bytes());
                result.extend_from_slice(b": ");
                result.extend_from_slice(value.as_bytes());
                result.extend_from_slice(b"\r\n");
            }

            // Empty line before body
            result.extend_from_slice(b"\r\n");

            // Body content
            match &mut part.inner {
                Inner::Text(text) => {
                    result.extend_from_slice(text.as_bytes());
                }
                Inner::Read(reader) => {
                    let mut buf = Vec::new();
                    reader
                        .read_to_end(&mut buf)
                        .expect("failed to read part content");
                    result.extend_from_slice(&buf);
                }
            }

            result.extend_from_slice(b"\r\n");
        }

        // Closing boundary
        result.extend_from_slice(b"--");
        result.extend_from_slice(self.boundary.as_bytes());
        result.extend_from_slice(b"--\r\n");

        result
    }

    // /// Updates a request instance with the multipart Content-Type header
    // /// and the payload data.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use hyper::{Method, Request};
    // /// use rust_multipart_rfc7578_2::client::multipart;
    // ///
    // /// let mut req_builder = Request::post("http://localhost:80/upload");
    // /// let mut form = multipart::Form::default();
    // ///
    // /// form.add_text("text", "Hello World!");
    // /// let req = form.set_body::<multipart::Body>(req_builder).unwrap();
    // /// ```
    // pub fn set_body<B>(self, req: Builder) -> Result<Request<B>, http::Error>
    // where
    //     B: From<Body<'a>>,
    // {
    //     self.set_body_convert::<B, B>(req)
    // }

    // /// Updates a request instance with the multipart Content-Type header
    // /// and the payload data.
    // ///
    // /// Allows converting body into an intermediate type.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use http_body_util::BodyDataStream;
    // /// use hyper::{Method, Request};
    // /// use rust_multipart_rfc7578_2::client::multipart;
    // ///
    // /// let mut req_builder = Request::post("http://localhost:80/upload");
    // /// let mut form = multipart::Form::default();
    // ///
    // /// form.add_text("text", "Hello World!");
    // /// let req = form
    // ///     .set_body_convert::<multipart::Body, multipart::Body>(req_builder)
    // ///     .unwrap();
    // /// ```
    // // Dev note: I am not sure this function is useful anymore, I could not fix the test
    // // with something besides an identity transform.
    // pub fn set_body_convert<B, I>(self, req: Builder) -> Result<Request<B>, http::Error>
    // where
    //     I: From<Body<'a>> + Into<B>,
    // {
    //     req.header(&CONTENT_TYPE, self.content_type().as_str())
    //         .body(I::from(Body::from(self)).into())
    // }

    pub fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", &self.boundary)
    }
}

impl<'a> From<Form<'a>> for Body<'a> {
    /// Turns a `Form` into a multipart `Body`.
    fn from(form: Form<'a>) -> Self {
        Body {
            buf: BytesMut::with_capacity(2048),
            current: None,
            parts: form.parts.into_iter().peekable(),
            boundary: form.boundary,
        }
    }
}
