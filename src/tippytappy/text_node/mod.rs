mod view;
pub use view::*;

mod compiled;
pub use compiled::*;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// #[serde(rename_all = "camelCase")]
// pub struct TextNode<S: State> {
//     #[serde(flatten)]
//     inner: S::TextNode,
// }

// impl TextNode<View> {
//     pub fn compile(self, carriage: &mut CompileCarriage) -> TextNode<Compiled> {
//         let new_inner = self.inner.compile(carriage);
//         TextNode { inner: new_inner }
//     }

//     #[allow(clippy::self_named_constructors)]
//     pub fn text_node(text: Text) -> Self {
//         Self {
//             inner: TextNodeView::Text(text),
//         }
//     }
//     pub fn text(&self) -> &str {
//         self.inner.text()
//     }
//     pub fn inner(&self) -> &TextNodeView {
//         &self.inner
//     }
// }

// impl TextNode<Compiled> {
//     pub fn into_view(self, relationships: &ContentRelationships) -> TextNode<View> {
//         let new_inner = self.inner.into_view(relationships);

//         TextNode { inner: new_inner }
//     }
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    #[serde(default)]
    pub marks: Vec<Mark>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Mark {
    Highlight,
    Code,
    Underline,
    Italic,
    Bold,
    Link { attrs: LinkAttributes },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LinkAttributes {
    pub href: String,
    pub target: Option<String>,
    pub rel: Option<String>,
    pub class: Option<String>,
    pub title: Option<String>,
}
