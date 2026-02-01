// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(unused)]

use std::{
    iter::Peekable,
    pin::Pin,
    task::{Context, Poll},
    vec::IntoIter,
};

use bytes::{BufMut, BytesMut};
use futures_core::Stream;
use futures_util::{
    AsyncRead,
    io::{AllowStdIo, Cursor},
};
use http::{HeaderName, header};

use crate::peanut::multipart::{
    error::Error,
    form::{Inner, Part},
};

static CONTENT_DISPOSITION: HeaderName = header::CONTENT_DISPOSITION;
static CONTENT_TYPE: HeaderName = header::CONTENT_TYPE;

/// Async streamable Multipart body.
pub struct Body<'a> {
    /// The amount of data to write with each chunk.
    pub(super) buf: BytesMut,

    /// The active reader.
    pub(super) current: Option<Box<dyn 'a + AsyncRead + Send + Unpin>>,

    /// The parts as an iterator. When the iterator stops
    /// yielding, the body is fully written.
    pub(super) parts: Peekable<IntoIter<Part<'a>>>,

    /// The multipart boundary.
    pub(super) boundary: String,
}

impl<'a> Body<'a> {
    /// Writes a CLRF.
    fn write_crlf(&mut self) {
        self.buf.put_slice(b"\r\n");
    }

    /// Implements section 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    fn write_boundary(&mut self) {
        self.buf.put_slice(b"--");
        self.buf.put_slice(self.boundary.as_bytes());
    }

    /// Writes the last form boundary.
    ///
    /// [See](https://tools.ietf.org/html/rfc2046#section-5.1).
    fn write_final_boundary(&mut self) {
        self.write_boundary();
        self.buf.put_slice(b"--");
    }

    /// Writes the Content-Disposition, and Content-Type headers.
    fn write_headers(&mut self, part: &Part) {
        self.write_crlf();
        self.buf.put_slice(CONTENT_TYPE.as_ref());
        self.buf.put_slice(b": ");
        self.buf.put_slice(part.content_type.as_bytes());
        self.write_crlf();
        self.buf.put_slice(CONTENT_DISPOSITION.as_ref());
        self.buf.put_slice(b": ");
        self.buf.put_slice(part.content_disposition.as_bytes());

        for (header_name, header_value) in &part.headers {
            self.write_crlf();
            self.buf.put_slice(header_name.as_str().as_bytes());
            self.buf.put_slice(b": ");
            self.buf.put_slice(header_value.as_bytes());
        }

        self.write_crlf();
        self.write_crlf();
    }
}

impl<'a> Stream for Body<'a> {
    type Item = Result<BytesMut, Error>;

    /// Iterate over each form part, and write it out.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let body = self.get_mut();

        match body.current {
            None => {
                if let Some(part) = body.parts.next() {
                    body.write_boundary();
                    body.write_headers(&part);

                    let read: Box<dyn AsyncRead + Send + Unpin> = match part.inner {
                        Inner::Read(read) => Box::new(AllowStdIo::new(read)),
                        Inner::Text(s) => Box::new(Cursor::new(s)),
                    };

                    body.current = Some(read);

                    cx.waker().wake_by_ref();

                    Poll::Ready(Some(Ok(body.buf.split())))
                } else {
                    // No current part, and no parts left means there is nothing
                    // left to write.
                    //
                    Poll::Ready(None)
                }
            }
            Some(ref mut read) => {
                // Reserve some space to read the next part
                body.buf.reserve(256);
                let len_before = body.buf.len();

                // Init the remaining capacity to 0, and get a mut slice to it
                body.buf.resize(body.buf.capacity(), 0);
                let slice = &mut body.buf.as_mut()[len_before..];

                match Pin::new(read).poll_read(cx, slice) {
                    Poll::Pending => {
                        body.buf.truncate(len_before);
                        Poll::Pending
                    }
                    // Read some data.
                    Poll::Ready(Ok(bytes_read)) => {
                        body.buf.truncate(len_before + bytes_read);

                        if bytes_read == 0 {
                            // EOF: No data left to read. Get ready to move onto write the next part.
                            body.current = None;
                            body.write_crlf();
                            if body.parts.peek().is_none() {
                                // If there is no next part, write the final boundary
                                body.write_final_boundary();
                                body.write_crlf();
                            }
                        }

                        Poll::Ready(Some(Ok(body.buf.split())))
                    }
                    // Error reading from underlying stream.
                    Poll::Ready(Err(e)) => {
                        body.buf.truncate(len_before);
                        Poll::Ready(Some(Err(Error::ContentRead(e))))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Body;
    use crate::peanut::multipart::{error::Error, form::Form};
    use bytes::BytesMut;
    use futures_util::TryStreamExt;
    use std::io::Cursor;

    async fn form_output(form: Form<'_>) -> String {
        let result: Result<BytesMut, Error> = Body::from(form).try_concat().await;

        assert!(result.is_ok());

        let bytes = result.unwrap();
        let data = std::str::from_utf8(bytes.as_ref()).unwrap();

        data.into()
    }

    #[tokio::test]
    async fn add_text_returns_expected_result() {
        let mut form = Form::default();

        form.add_text("test", "Hello World!");

        let data = form_output(form).await;

        assert!(data.contains("Hello World!"));
    }

    #[tokio::test]
    async fn add_reader_returns_expected_result() {
        let bytes = Cursor::new("Hello World!");
        let mut form = Form::default();

        form.add_reader_2("input", bytes, None, None, Default::default());

        let data = form_output(form).await;

        assert!(data.contains("Hello World!"));
    }

    struct FixedBoundary;
    impl crate::peanut::multipart::boundary::BoundaryGenerator for FixedBoundary {
        fn generate_boundary() -> String {
            "boundary".to_owned()
        }
    }

    #[tokio::test]
    async fn test_form_body_stream() {
        let mut form = Form::new::<FixedBoundary>();
        // Text fields
        form.add_text("name1", "value1");
        form.add_text("name2", "value2");

        // Reader field
        form.add_reader_2(
            "input",
            Cursor::new("Hello World!"),
            None,
            None,
            Default::default(),
        );

        let result: BytesMut = Body::from(form).try_concat().await.unwrap();

        assert_eq!(
            result.as_ref(),
            [
                b"--boundary\r\n".as_ref(),
                b"content-type: text/plain\r\n".as_ref(),
                b"content-disposition: form-data; name=\"name1\"\r\n".as_ref(),
                b"\r\n".as_ref(),
                b"value1\r\n".as_ref(),
                b"--boundary\r\n".as_ref(),
                b"content-type: text/plain\r\n".as_ref(),
                b"content-disposition: form-data; name=\"name2\"\r\n".as_ref(),
                b"\r\n".as_ref(),
                b"value2\r\n".as_ref(),
                b"--boundary\r\n".as_ref(),
                b"content-type: application/octet-stream\r\n".as_ref(),
                b"content-disposition: form-data; name=\"input\"\r\n".as_ref(),
                b"\r\n".as_ref(),
                b"Hello World!\r\n".as_ref(),
                b"--boundary--\r\n".as_ref(),
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u8>>()
        );
    }

    // #[tokio::test]
    // async fn test_content_type_header_format() {
    //     use http::Request;

    //     let mut form = Form::new::<FixedBoundary>();
    //     // Text fields
    //     form.add_text("name1", "value1");
    //     form.add_text("name2", "value2");

    //     let builder = Request::builder();
    //     let body = form.set_body::<Body>(builder).unwrap();

    //     assert_eq!(
    //         body.headers().get("Content-Type").unwrap().as_bytes(),
    //         b"multipart/form-data; boundary=boundary",
    //     )
    // }
}
