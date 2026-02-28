mod error;
pub use error::*;

mod node;
pub use node::*;

mod list;
pub use list::*;

mod text_node;
pub use text_node::*;

mod document;
pub use document::*;

mod compiled;
pub use compiled::*;

use serde::{Serialize, de::DeserializeOwned};

pub trait State: Default {
    type TextNode: DeserializeOwned + Serialize + std::fmt::Debug + PartialEq + Eq + Clone;
}
