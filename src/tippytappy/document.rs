use crate::tippytappy::*;
use markdown::{ParseOptions, mdast::Node as MdNode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct View;

impl State for View {
    type TextNode = TextNodeView;
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename = "doc")]
pub struct Document {
    content: Vec<Node<View>>,
}

impl Document {
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
