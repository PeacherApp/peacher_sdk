mod content_relationships;
pub use content_relationships::*;

mod compile_carriage;
pub use compile_carriage::*;

mod content_dependencies;
pub use content_dependencies::*;

use crate::tippytappy::State;

pub trait NodeVisitor<S: State> {
    type OutputState: State;
    fn visit_text_node(&mut self, node: S::TextNode) -> <Self::OutputState as State>::TextNode;
}
