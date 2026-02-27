use serde::{Deserialize, Serialize};

use crate::peanut::multipart::{MultipartForm, Part};
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UploadResponse {
    pub url: String,
}

/// Upload an image file via multipart form data.
pub struct UploadImage {
    file_data: Vec<u8>,
    file_name: String,
}

impl UploadImage {
    pub fn new(file_name: impl Into<String>, file_data: Vec<u8>) -> Self {
        Self {
            file_name: file_name.into(),
            file_data,
        }
    }
}

impl Handler for UploadImage {
    type ResponseBody = UploadResponse;

    fn method(&self) -> Method {
        Method::Post
    }

    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/uploads".into()
    }

    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        let form = MultipartForm::new().add_part(
            "file",
            Part::bytes(self.file_data.clone()).file_name(&self.file_name),
        );
        builder.multipart(form)
    }
}
