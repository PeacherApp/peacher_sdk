mod view;
pub use view::*;

mod compiled;
pub use compiled::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    #[serde(default)]
    pub marks: Vec<Mark>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Mark {
    Highlight,
    Code,
    Underline,
    Italic,
    Bold,
    Link { attrs: LinkAttributes },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LinkAttributes {
    pub href: String,
    pub target: Option<String>,
    pub rel: Option<String>,
    pub class: Option<String>,
    pub title: Option<String>,
}
