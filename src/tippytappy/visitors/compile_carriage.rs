use ahash::HashMap;
use uuid::Uuid;

use crate::tippytappy::*;

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
            relationships: ContentRelationships::new(
                self.legislation_ids,
                self.member_ids,
                self.sibling_content_ids,
            ),
            document,
        }
    }
}

impl NodeVisitor<View> for CompileCarriage {
    type OutputState = Compiled;
    fn visit_text_node(&mut self, node: TextNodeView) -> CompiledTextNode {
        match node {
            TextNodeView::LegislationMention { attrs } => {
                self.mentions_legislation(attrs.id, attrs.label.clone());
                CompiledTextNode::LegislationMention(attrs.id)
            }
            TextNodeView::MemberMention { attrs } => {
                // carriage.push_str(&attrs.label);
                self.mentions_member(attrs.id, attrs.label.clone());
                CompiledTextNode::MemberMention(attrs.id)
            }
            TextNodeView::PostMention { attrs } => {
                self.mentions_content(attrs.id, attrs.label.clone());
                CompiledTextNode::PostMention(attrs.id)
            }
            TextNodeView::Text(text) => {
                self.push_str(&text.text);
                CompiledTextNode::Text(text)
            }
        }
    }
}
