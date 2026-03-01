use serde::{Deserialize, Serialize};

use crate::tippytappy::{CompileCarriage, Compiled, ContentLabeler, Node, State, View};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct OrderedList<S: State> {
    pub attrs: ListAttributes,
    pub content: Vec<ListChild<S>>,
}

impl OrderedList<View> {
    pub fn compile(self, carriage: &mut CompileCarriage) -> OrderedList<Compiled> {
        let new_children = self
            .content
            .into_iter()
            .map(|child| child.compile(carriage))
            .collect();
        OrderedList {
            attrs: self.attrs,
            content: new_children,
        }
    }
}
impl OrderedList<Compiled> {
    pub fn into_view(self, carriage: &impl ContentLabeler) -> OrderedList<View> {
        let new_children = self
            .content
            .into_iter()
            .map(|child| child.into_view(carriage))
            .collect();
        OrderedList {
            attrs: self.attrs,
            content: new_children,
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
impl ListChild<View> {
    pub fn compile(self, carriage: &mut CompileCarriage) -> ListChild<Compiled> {
        match self {
            ListChild::ListItem { content } => {
                let new_content = content.into_iter().map(|node| node.compile(carriage));
                ListChild::ListItem {
                    content: new_content.collect(),
                }
            }
        }
    }
}
impl ListChild<Compiled> {
    pub fn into_view(self, carriage: &impl ContentLabeler) -> ListChild<View> {
        match self {
            ListChild::ListItem { content } => {
                let new_content = content.into_iter().map(|node| node.into_view(carriage));
                ListChild::ListItem {
                    content: new_content.collect(),
                }
            }
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

impl BulletListNode<View> {
    pub fn compile(self, carriage: &mut CompileCarriage) -> BulletListNode<Compiled> {
        let new_content = self.content.into_iter().map(|node| node.compile(carriage));

        BulletListNode {
            content: new_content.collect(),
        }
    }
}

impl BulletListNode<Compiled> {
    pub fn into_view(self, carriage: &impl ContentLabeler) -> BulletListNode<View> {
        let new_content = self
            .content
            .into_iter()
            .map(|node| node.into_view(carriage));

        BulletListNode {
            content: new_content.collect(),
        }
    }
}

impl<S: State> BulletListNode<S> {
    pub(crate) fn new(content: Vec<ListChild<S>>) -> Self {
        Self { content }
    }
}
