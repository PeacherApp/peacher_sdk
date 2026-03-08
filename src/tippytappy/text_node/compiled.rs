use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::{node_kind::ProcessNode, *};

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
    fn iter_text<'slf, F>(&'slf self, mut func: F) -> bool
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

impl ProcessNode<ContentRelationships> for CompiledTextNode {
    type Output = TextNodeView;
    fn process(self, relationships: &mut ContentRelationships) -> Self::Output {
        match self {
            CompiledTextNode::Text(text) => TextNodeView::Text(text),
            CompiledTextNode::LegislationMention(id) => {
                let label = relationships.get_legislation_nameid(id);

                TextNodeView::LegislationMention {
                    attrs: Mention {
                        id,
                        label: label.unwrap_or(format!("Legislation #{id}")),
                    },
                }
            }
            CompiledTextNode::MemberMention(member_id) => {
                let label = relationships.get_member_handle(member_id);

                TextNodeView::MemberMention {
                    attrs: Mention {
                        id: member_id,
                        label: label.unwrap_or("@{UNKNOWN}".to_string()),
                    },
                }
            }
            CompiledTextNode::PostMention(id) => {
                let label = relationships.get_content_label(id);
                TextNodeView::PostMention {
                    attrs: Mention {
                        id,
                        label: label.unwrap_or("UNKNOWN".to_string()),
                    },
                }
            }
        }
    }
}
