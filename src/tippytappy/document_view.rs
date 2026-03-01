use crate::tippytappy::*;
use markdown::{ParseOptions, mdast::Node as MdNode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct View;

impl State for View {
    type TextNode = TextNodeView;
}

/// Associated particular node labels
/// related content within a [`CompiledDocument`] to
/// turn into a [`DocumentView`]
pub trait ContentLabeler {
    /// returns a nameid associated with legislation
    fn get_legislation_nameid(&self, id: i32) -> Option<String>;
    /// returns a label associated with a member.
    fn get_member_handle(&self, id: i32) -> Option<String>;
    /// get the label for a piece of content
    fn get_content_label(&self, id: Uuid) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename = "doc")]
pub struct DocumentView {
    content: Vec<Node<View>>,
}

impl DocumentView {
    pub fn from_nodes(nodes: impl IntoIterator<Item = Node<View>>) -> Self {
        Self {
            content: nodes.into_iter().collect(),
        }
    }
    pub fn parse_json(value: serde_json::Value) -> Result<Self, ParseError> {
        let value = serde_json::from_value(value).map_err(|e| {
            tracing::error!("Invalid value passed for document. Error: {e}");
            ParseError::Json(e)
        })?;

        Ok(value)
    }
    pub fn parse_compiled_json(value: serde_json::Value) -> Result<Self, ParseError> {
        let value = serde_json::from_value(value).map_err(|e| {
            tracing::error!("Invalid value passed for document. Error: {e}");
            ParseError::Json(e)
        })?;

        Ok(value)
    }

    pub fn parse_markdown(markdown: &str) -> Result<Self, ParseError> {
        let parse_options = ParseOptions::gfm();

        let markdown = markdown::to_mdast(markdown, &parse_options)?;

        let MdNode::Root(root) = markdown else {
            return Err(ParseError::other("root element is not a root node!"));
        };

        let content = root
            .children
            .into_iter()
            .map(Node::from_mdast)
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { content })
    }

    pub fn compile(self) -> CompilationResult {
        let mut carriage = CompileCarriage::default();

        let compiled_nodes = self
            .content
            .into_iter()
            .map(|node| node.compile(&mut carriage));
        let document = CompiledDocument::from_nodes(compiled_nodes);
        carriage.finish(document)
    }
}
