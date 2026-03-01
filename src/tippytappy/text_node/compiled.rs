use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::{ContentLabeler, Mention, Text, TextNodeView};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CompiledTextNode {
    Text(Text),
    MemberMention(i32),
    LegislationMention(i32),
    PostMention(Uuid),
}

impl CompiledTextNode {
    pub fn into_view(self, carriage: &impl ContentLabeler) -> TextNodeView {
        match self {
            CompiledTextNode::Text(text) => TextNodeView::Text(text),
            CompiledTextNode::LegislationMention(id) => {
                let label = carriage.get_legislation_nameid(id);

                TextNodeView::LegislationMention {
                    attrs: Mention {
                        id,
                        label: label.unwrap_or(format!("Legislation #{id}")),
                    },
                }
            }
            CompiledTextNode::MemberMention(member_id) => {
                let label = carriage.get_member_handle(member_id);

                TextNodeView::MemberMention {
                    attrs: Mention {
                        id: member_id,
                        label: label.unwrap_or("@{UNKNOWN}".to_string()),
                    },
                }
            }
            CompiledTextNode::PostMention(id) => {
                let label = carriage.get_content_label(id);
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
