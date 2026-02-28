use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tippytappy::Text;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CompiledTextNode {
    Text(Text),
    MemberMention(i32),
    LegislationMention(i32),
    PostMention(Uuid),
}
