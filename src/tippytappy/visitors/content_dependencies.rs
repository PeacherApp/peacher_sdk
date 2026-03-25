use ahash::HashSet;
use uuid::Uuid;

use crate::tippytappy::*;

/// Defines relationships to ids and labels for a [`DocumentView`](crate::tippytappy::DocumentView).
///
///
/// [`DocumentView::compile`](crate::tippytappy::DocumentView::compile) produces this result.
///
/// it can be used to turn content within a [`CompiledDocument`](crate::tippytappy::CompiledView) to
/// turn into a [`DocumentView`](crate::tippytappy::DocumentView).
///
#[derive(Default)]
pub struct ContentDependencies {
    legislation_ids: HashSet<i32>,
    member_ids: HashSet<i32>,
    content_ids: HashSet<Uuid>,
}

impl ContentDependencies {
    pub fn num_legislation_ids(&self) -> usize {
        self.legislation_ids.len()
    }
    /// Guaranteed to be a unique iteration of legislation IDs
    pub fn legislation_ids(&self) -> impl Iterator<Item = i32> {
        self.legislation_ids.iter().copied()
    }
    pub fn num_member_ids(&self) -> usize {
        self.member_ids.len()
    }
    /// Guaranteed to be a unique iteration of member IDs
    pub fn member_ids(&self) -> impl Iterator<Item = i32> {
        self.member_ids.iter().copied()
    }
    pub fn num_content_ids(&self) -> usize {
        self.content_ids.len()
    }
    /// Guaranteed to be a unique iteration of content IDs
    pub fn content_ids(&self) -> impl Iterator<Item = Uuid> {
        self.content_ids.iter().copied()
    }
}

impl NodeVisitor<Compiled> for ContentDependencies {
    type OutputState = Compiled;
    fn visit_text_node(&mut self, node: CompiledTextNode) -> CompiledTextNode {
        match &node {
            CompiledTextNode::LegislationMention(id) => {
                self.legislation_ids.insert(*id);
            }
            CompiledTextNode::MemberMention(id) => {
                self.member_ids.insert(*id);
            }
            CompiledTextNode::PostMention(id) => {
                self.content_ids.insert(*id);
            }
            CompiledTextNode::Text(_) => {}
        }
        node
    }
}
impl NodeVisitor<View> for ContentDependencies {
    type OutputState = View;
    fn visit_text_node(&mut self, node: TextNodeView) -> TextNodeView {
        match &node {
            TextNodeView::LegislationMention { attrs } => {
                self.legislation_ids.insert(attrs.id);
            }
            TextNodeView::MemberMention { attrs } => {
                // carriage.push_str(&attrs.label);
                self.member_ids.insert(attrs.id);
            }
            TextNodeView::PostMention { attrs } => {
                self.content_ids.insert(attrs.id);
            }
            TextNodeView::Text(_) => {}
        }
        node
    }
}
