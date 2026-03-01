use crate::tippytappy::*;
use ahash::HashMap;
use markdown::{ParseOptions, mdast::Node as MdNode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct View;

impl State for View {
    type TextNode = TextNodeView;
}

// /// Required to convert a compiled document into a view
// pub struct ViewCarriage {
//     pub legislation: HashMap<i32, String>,
//     pub members: HashMap<i32, String>,
//     pub siblings: HashMap<Uuid, String>,
// }
// impl ViewCarriage {
//     pub fn new_from_iterators(
//         legislation: impl IntoIterator<Item = (i32, String)>,
//         members: impl IntoIterator<Item = (i32, String)>,
//         siblings: impl IntoIterator<Item = (Uuid, String)>,
//     ) -> Self {
//         Self {
//             legislation: legislation.into_iter().collect(),
//             members: members.into_iter().collect(),
//             siblings: siblings.into_iter().collect(),
//         }
//     }
// }

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
