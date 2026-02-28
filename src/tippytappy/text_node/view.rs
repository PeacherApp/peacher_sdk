use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::{CompileCarriage, CompiledTextNode, Text};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TextNodeView {
    Text(Text),
    MemberMention { attrs: Mention<i32> },
    LegislationMention { attrs: Mention<i32> },
    PostMention { attrs: Mention<Uuid> },
}

impl TextNodeView {
    pub fn text(&self) -> &str {
        match self {
            TextNodeView::Text(Text { text, .. }) => text,
            TextNodeView::MemberMention { attrs } => &attrs.label,
            TextNodeView::LegislationMention { attrs } => &attrs.label,
            TextNodeView::PostMention { attrs } => &attrs.label,
        }
    }
    pub fn compile(self, carriage: &mut CompileCarriage) -> CompiledTextNode {
        match self {
            TextNodeView::LegislationMention { attrs } => {
                // carriage.push_str(&attrs.label);
                carriage.mentions_legislation(attrs.id);
                CompiledTextNode::LegislationMention(attrs.id)
            }
            TextNodeView::MemberMention { attrs } => {
                // carriage.push_str(&attrs.label);
                carriage.mentions_member(attrs.id);
                CompiledTextNode::MemberMention(attrs.id)
            }
            TextNodeView::PostMention { attrs } => {
                carriage.mentions_content(attrs.id);
                CompiledTextNode::PostMention(attrs.id)
            }
            TextNodeView::Text(text) => {
                carriage.push_str(&text.text);
                CompiledTextNode::Text(text)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mention<Id> {
    // should be i32
    pub id: Id,
    pub label: String,
}
