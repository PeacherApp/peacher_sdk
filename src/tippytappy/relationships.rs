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
    pub legislation_nameids: HashMap<i32, String>,
    pub member_handles: HashMap<i32, String>,
    pub sibling_content_ids: HashMap<Uuid, String>,
}

impl ContentRelationships {
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
        self.sibling_content_ids.get(&id).cloned()
    }
}
