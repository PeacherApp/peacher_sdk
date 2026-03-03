use ahash::HashMap;
use uuid::Uuid;

/// Defines relationships to ids and labels for a [`DocumentView`](crate::tippytappy::DocumentView).
///
///
/// [`DocumentView::compile`](crate::tippytappy::DocumentView::compile) produces this result.
///
/// it can be used to turn content within a [`CompiledDocument`](crate::tippytappy::CompiledView) to
/// turn into a [`DocumentView`](crate::tippytappy::DocumentView).
///
pub struct ContentRelationships {
    pub(super) legislation_nameids: HashMap<i32, String>,
    pub(super) member_handles: HashMap<i32, String>,
    pub(super) sibling_labels: HashMap<Uuid, String>,
}

impl ContentRelationships {
    pub fn from_iters(
        legislation: impl IntoIterator<Item = (i32, String)>,
        members: impl IntoIterator<Item = (i32, String)>,
        siblings: impl IntoIterator<Item = (Uuid, String)>,
    ) -> Self {
        Self {
            legislation_nameids: legislation.into_iter().collect(),
            member_handles: members.into_iter().collect(),
            sibling_labels: siblings.into_iter().collect(),
        }
    }

    pub fn legislation_ids(&self) -> impl Iterator<Item = i32> {
        self.legislation_nameids.keys().copied()
    }
    pub fn member_ids(&self) -> impl Iterator<Item = i32> {
        self.member_handles.keys().copied()
    }
    pub fn sibling_ids(&self) -> impl Iterator<Item = Uuid> {
        self.sibling_labels.keys().copied()
    }

    /// returns a nameid associated with legislation
    pub fn get_legislation_nameid(&self, id: i32) -> Option<String> {
        self.legislation_nameids.get(&id).cloned()
    }
    /// returns a label associated with a member.
    pub fn get_member_handle(&self, id: i32) -> Option<String> {
        self.member_handles.get(&id).cloned()
    }
    /// returns a label associated with a member.
    pub fn get_content_label(&self, id: Uuid) -> Option<String> {
        self.sibling_labels.get(&id).cloned()
    }
}
