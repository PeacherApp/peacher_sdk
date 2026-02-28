use std::collections::HashSet;

use crate::tippytappy::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default)]
pub struct CompileCarriage {
    searchable_text: String,
    legislation_ids: HashSet<i32>,
    member_ids: HashSet<i32>,
    sibling_content_ids: HashSet<Uuid>,
}
impl CompileCarriage {
    pub fn push_str(&mut self, value: &str) {
        self.searchable_text.push_str(value);
    }
    pub fn mentions_legislation(&mut self, legislation_id: i32) {
        self.legislation_ids.insert(legislation_id);
    }
    pub fn mentions_member(&mut self, member_id: i32) {
        self.member_ids.insert(member_id);
    }
    pub fn mentions_content(&mut self, post_id: Uuid) {
        self.sibling_content_ids.insert(post_id);
    }
    pub fn finish(self, document: CompiledDocument) -> CompilationResult {
        CompilationResult {
            searchable_text: self.searchable_text,
            relationships: ContentRelationships {
                legislation_ids: self.legislation_ids,
                member_ids: self.member_ids,
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

pub struct ContentRelationships {
    pub legislation_ids: HashSet<i32>,
    pub member_ids: HashSet<i32>,
    pub sibling_content_ids: HashSet<Uuid>,
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
}
