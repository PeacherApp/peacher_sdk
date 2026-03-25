use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::{Text, node_kind::NodeKind};

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
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
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
