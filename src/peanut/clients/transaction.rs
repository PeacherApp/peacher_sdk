use std::collections::HashMap;

use http::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum TransactionError {
    Status,
    InvalidBody,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    pub method: Method,
    pub url: Url,
    pub headers: TransactionHeaders,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    pub status: u16,
    pub headers: TransactionHeaders,
    pub body: Result<serde_json::Value, TransactionError>,
}

impl TransactionResponse {
    pub fn new_err(status: u16, headers: TransactionHeaders, error: TransactionError) -> Self {
        Self {
            status,
            headers,
            body: Err(error),
        }
    }

    pub fn new(status: u16, headers: TransactionHeaders, body: serde_json::Value) -> Self {
        Self {
            status,
            headers,
            body: Ok(body),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    request: TransactionRequest,
    /// None if there was an error sending the request
    response: Option<TransactionResponse>,
}
impl Transaction {
    pub fn request(&self) -> &TransactionRequest {
        &self.request
    }
    pub fn response(&self) -> Option<&TransactionResponse> {
        self.response.as_ref()
    }
    pub fn new(request: TransactionRequest, response: TransactionResponse) -> Self {
        Self {
            request,
            response: Some(response),
        }
    }
    pub fn req_err(request: TransactionRequest) -> Self {
        Self {
            request,
            response: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TransactionHeaders(HashMap<String, String>);
impl TransactionHeaders {
    pub fn from_headermap(map: &HeaderMap) -> Self {
        let mut this = Self::default();
        for (key, value) in map {
            this.insert(key, value);
        }
        this
    }
    pub fn insert(&mut self, key: &HeaderName, value: &HeaderValue) {
        self.0
            .insert(key.to_string(), value.to_str().unwrap().to_string());
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SavedResponse {
    Text(String),
    Json(serde_json::Value),
    Nothing,
}
