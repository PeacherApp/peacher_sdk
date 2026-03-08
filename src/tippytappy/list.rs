use serde::{Deserialize, Serialize};

use crate::tippytappy::{
    node_kind::{ProcessNode, iter_node_children_text},
    *,
};

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

impl ProcessNode<CompileCarriage> for OrderedList<View> {
    type Output = OrderedList<Compiled>;
    fn process(self, visitor: &mut CompileCarriage) -> Self::Output {
        OrderedList {
            attrs: self.attrs,
            content: self
                .content
                .into_iter()
                .map(|c| c.process(visitor))
                .collect(),
        }
    }
}
impl ProcessNode<ContentRelationships> for OrderedList<Compiled> {
    type Output = OrderedList<View>;
    fn process(self, visitor: &mut ContentRelationships) -> Self::Output {
        OrderedList {
            attrs: self.attrs,
            content: self
                .content
                .into_iter()
                .map(|c| c.process(visitor))
                .collect(),
        }
    }
}

impl<S: State> OrderedList<S> {
    pub(crate) fn new(start: u32, content: Vec<ListChild<S>>) -> Self {
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

impl ProcessNode<CompileCarriage> for ListChild<View> {
    type Output = ListChild<Compiled>;
    fn process(self, visitor: &mut CompileCarriage) -> Self::Output {
        match self {
            ListChild::ListItem { content } => ListChild::ListItem {
                content: content.into_iter().map(|c| c.process(visitor)).collect(),
            },
        }
    }
}
impl ProcessNode<ContentRelationships> for ListChild<Compiled> {
    type Output = ListChild<View>;
    fn process(self, visitor: &mut ContentRelationships) -> Self::Output {
        match self {
            ListChild::ListItem { content } => ListChild::ListItem {
                content: content.into_iter().map(|c| c.process(visitor)).collect(),
            },
        }
    }
}

impl<S: State> ListChild<S> {
    pub(crate) fn new(content: Vec<Node<S>>) -> Self {
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

impl ProcessNode<CompileCarriage> for BulletListNode<View> {
    type Output = BulletListNode<Compiled>;
    fn process(self, visitor: &mut CompileCarriage) -> Self::Output {
        BulletListNode {
            content: self
                .content
                .into_iter()
                .map(|c| c.process(visitor))
                .collect(),
        }
    }
}
impl ProcessNode<ContentRelationships> for BulletListNode<Compiled> {
    type Output = BulletListNode<View>;
    fn process(self, visitor: &mut ContentRelationships) -> Self::Output {
        BulletListNode {
            content: self
                .content
                .into_iter()
                .map(|c| c.process(visitor))
                .collect(),
        }
    }
}

impl<S: State> BulletListNode<S> {
    pub(crate) fn new(content: Vec<ListChild<S>>) -> Self {
        Self { content }
    }
}
