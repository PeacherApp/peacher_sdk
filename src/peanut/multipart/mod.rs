//! MIT License
//!
//! Copyright (c) 2023 Joseph Lenton
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

mod part;
pub use part::*;

mod body;
mod boundary;
mod error;
mod form;

use std::fmt::Display;
use std::io::Cursor;

use crate::multipart::form::Form;

#[derive(Debug, Default)]
pub struct MultipartForm {
    inner: Form<'static>,
}

impl MultipartForm {
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a text part, and adds it to be sent.
    pub fn add_text<N, T>(mut self, name: N, text: T) -> Self
    where
        N: Display,
        T: ToString,
    {
        self.inner.add_text(name, text.to_string());
        self
    }

    /// Adds a new section to this multipart form to be sent.
    pub fn add_part<N>(mut self, name: N, part: Part) -> Self
    where
        N: Display,
    {
        let reader = Cursor::new(part.bytes);
        self.inner.add_reader_2(
            name,
            reader,
            part.file_name,
            Some(part.mime_type),
            part.headers,
        );

        self
    }

    /// Returns the content type this form will use when it is sent.
    pub fn content_type(&self) -> String {
        self.inner.content_type()
    }

    pub fn into_form_bytes(self) -> Vec<u8> {
        self.inner.into_bytes()
    }
}
