use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::{node_kind::NodeKind, *};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CompiledTextNode {
    Text(Text),
    MemberMention(i32),
    LegislationMention(i32),
    PostMention(Uuid),
}
impl NodeKind for CompiledTextNode {
    /// Iterates only through the text variant.
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
    where
        F: FnMut(&'slf str) -> bool,
    {
        if let CompiledTextNode::Text(Text { text, .. }) = self {
            func(text)
        } else {
            true
        }
    }
}

