use serde::{Deserialize, Serialize};

use crate::tippytappy::{node_kind::iter_node_children_text, *};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct OrderedList<S: State> {
    pub attrs: ListAttributes,
    pub content: Vec<ListChild<S>>,
}
impl<S: State> NodeKind for OrderedList<S> {
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
    where
        F: FnMut(&'slf str) -> bool,
    {
        iter_node_children_text(self.content.iter(), func)
    }
}

impl<S: State> OrderedList<S> {
    pub fn process<V: NodeVisitor<S>>(self, visitor: &mut V) -> OrderedList<V::OutputState> {
        OrderedList {
            attrs: self.attrs,
            content: self
                .content
                .into_iter()
                .map(|c| c.process(visitor))
                .collect(),
        }
    }

    pub fn new(start: u32, content: Vec<ListChild<S>>) -> Self {
        Self {
            attrs: ListAttributes {
                start,
                attr_type: None,
            },
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ListChild<S: State> {
    ListItem { content: Vec<Node<S>> },
}

impl<S: State> NodeKind for ListChild<S> {
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
    where
        F: FnMut(&'slf str) -> bool,
    {
        match self {
            ListChild::ListItem { content } => iter_node_children_text(content.iter(), func),
        }
    }
}
impl<S: State> ListChild<S> {
    pub fn process<V: NodeVisitor<S>>(self, visitor: &mut V) -> ListChild<V::OutputState> {
        match self {
            ListChild::ListItem { content } => ListChild::ListItem {
                content: content.into_iter().map(|c| c.process(visitor)).collect(),
            },
        }
    }

    pub fn new(content: Vec<Node<S>>) -> Self {
        Self::ListItem { content }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ListAttributes {
    pub start: u32,
    #[serde(rename = "type")]
    pub attr_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct BulletListNode<S: State> {
    pub content: Vec<ListChild<S>>,
}

impl<S: State> NodeKind for BulletListNode<S> {
    fn iter_text<'slf, F>(&'slf self, func: &mut F) -> bool
    where
        F: FnMut(&'slf str) -> bool,
    {
        iter_node_children_text(self.content.iter(), func)
    }
}
impl<S: State> BulletListNode<S> {
    pub fn process<V: NodeVisitor<S>>(self, visitor: &mut V) -> BulletListNode<V::OutputState> {
        BulletListNode {
            content: self
                .content
                .into_iter()
                .map(|c| c.process(visitor))
                .collect(),
        }
    }

    pub fn new(content: Vec<ListChild<S>>) -> Self {
        Self { content }
    }
}
