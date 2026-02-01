use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, Hash)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}
impl Method {
    pub fn to_reqwest_method(&self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Patch => reqwest::Method::PATCH,
            Method::Put => reqwest::Method::PUT,
            Method::Post => reqwest::Method::POST,
            Method::Delete => reqwest::Method::DELETE,
        }
    }
}
