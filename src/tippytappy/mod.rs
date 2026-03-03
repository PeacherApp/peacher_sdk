mod error;
pub use error::*;

mod node;
pub use node::*;

mod list;
pub use list::*;

mod text_node;
pub use text_node::*;

mod document_view;
pub use document_view::*;

mod compiled_document;
pub use compiled_document::*;

mod relationships;
pub use relationships::*;

use serde::{Serialize, de::DeserializeOwned};

pub trait State: Default {
    #[cfg(feature = "utoipa")]
    type TextNode: DeserializeOwned
        + Serialize
        + std::fmt::Debug
        + PartialEq
        + Eq
        + Clone
        + utoipa::ToSchema;
    #[cfg(not(feature = "utoipa"))]
    type TextNode: DeserializeOwned + Serialize + std::fmt::Debug + PartialEq + Eq + Clone;
}
