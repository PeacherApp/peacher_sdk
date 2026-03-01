use crate::tippytappy::*;
use ahash::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default)]
pub struct CompileCarriage {
    searchable_text: String,
    legislation_ids: HashMap<i32, String>,
    member_ids: HashMap<i32, String>,
    sibling_content_ids: HashMap<Uuid, String>,
}
impl CompileCarriage {
    pub fn push_str(&mut self, value: &str) {
        self.searchable_text.push_str(value);
    }
    pub fn mentions_legislation(&mut self, legislation_id: i32, nameid: String) {
        self.legislation_ids.insert(legislation_id, nameid);
    }
    pub fn mentions_member(&mut self, member_id: i32, handle: String) {
        self.member_ids.insert(member_id, handle);
    }
    pub fn mentions_content(&mut self, post_id: Uuid, label: String) {
        self.sibling_content_ids.insert(post_id, label);
    }
    pub fn finish(self, document: CompiledDocument) -> CompilationResult {
        CompilationResult {
            searchable_text: self.searchable_text,
            relationships: ContentRelationships {
                legislation_nameids: self.legislation_ids,
                member_handles: self.member_ids,
                sibling_content_ids: self.sibling_content_ids,
            },
            document,
        }
    }
}

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

    pub fn to_view(self, relationships: &ContentRelationships) -> DocumentView {
        DocumentView::from_nodes(
            self.content
                .into_iter()
                .map(|node| node.into_view(relationships)),
        )
    }
}
