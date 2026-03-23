use crate::tippytappy::{node_kind::iter_node_children_text, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Compiled;
impl State for Compiled {
    type TextNode = CompiledTextNode;
}

pub struct CompilationResult {
    pub relationships: ContentRelationships,
    pub searchable_text: String,
    pub document: CompiledDocument,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename = "doc")]
pub struct CompiledDocument {
    content: Vec<Node<Compiled>>,
}

impl CompiledDocument {
    pub fn from_nodes(nodes: impl IntoIterator<Item = Node<Compiled>>) -> Self {
        Self {
            content: nodes.into_iter().collect(),
        }
    }

    pub fn parse_json(value: serde_json::Value) -> Result<Self, ParseError> {
        let value = serde_json::from_value(value).map_err(|e| {
            tracing::error!("Invalid value passed for compiled document. Error: {e}");
            ParseError::Json(e)
        })?;

        Ok(value)
    }

    pub fn visit<V>(mut self, relationships: &mut V) -> Self
    where
        V: NodeVisitor<Compiled, OutputState = Compiled>,
    {
        self.content = self
            .content
            .into_iter()
            .map(|node| node.process(relationships))
            .collect();

        self
    }

    pub fn visit_and_decompile<V>(self, relationships: &mut V) -> DocumentView
    where
        V: NodeVisitor<Compiled, OutputState = View>,
    {
        DocumentView::from_nodes(
            self.content
                .into_iter()
                .map(|node| node.process(relationships)),
        )
    }
    pub fn into_value(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
impl NodeKind for CompiledDocument {
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
    where
        F: FnMut(&'slf str) -> bool,
    {
        iter_node_children_text(self.content.iter(), func)
    }
}
