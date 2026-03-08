use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::{
    CompileCarriage, CompiledTextNode, Text,
    node_kind::{NodeKind, ProcessNode},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TextNodeView {
    Text(Text),
    MemberMention { attrs: Mention<i32> },
    LegislationMention { attrs: Mention<i32> },
    PostMention { attrs: Mention<Uuid> },
}

impl NodeKind for TextNodeView {
    /// Will iterate through names and labels
    fn iter_text<'slf, F>(&'slf self, mut func: F) -> bool
    where
        F: FnMut(&'slf str) -> bool,
    {
        match self {
            TextNodeView::Text(Text { text, .. }) => func(text),
            TextNodeView::MemberMention { attrs } => func(&attrs.label),
            TextNodeView::LegislationMention { attrs } => func(&attrs.label),
            TextNodeView::PostMention { attrs } => func(&attrs.label),
        }
    }
}

impl ProcessNode<CompileCarriage> for TextNodeView {
    type Output = CompiledTextNode;

    fn process(self, carriage: &mut CompileCarriage) -> CompiledTextNode {
        match self {
            TextNodeView::LegislationMention { attrs } => {
                // carriage.push_str(&attrs.label);
                carriage.mentions_legislation(attrs.id, attrs.label);
                CompiledTextNode::LegislationMention(attrs.id)
            }
            TextNodeView::MemberMention { attrs } => {
                // carriage.push_str(&attrs.label);
                carriage.mentions_member(attrs.id, attrs.label);
                CompiledTextNode::MemberMention(attrs.id)
            }
            TextNodeView::PostMention { attrs } => {
                carriage.mentions_content(attrs.id, attrs.label);
                CompiledTextNode::PostMention(attrs.id)
            }
            TextNodeView::Text(text) => {
                carriage.push_str(&text.text);
                CompiledTextNode::Text(text)
            }
        }
    }
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Mention<Id> {
    // should be i32
    pub id: Id,
    pub label: String,
}
