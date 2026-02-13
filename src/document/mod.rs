use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct RootNode {
    #[serde(rename = "type")]
    node_type: String,
    content: Vec<Node>,
}

impl RootNode {
    pub fn to_search_text(&self) -> String {
        let mut result = String::new();

        for content in self.content.iter() {
            content.append_string_content(&mut result);
        }

        result
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node {
    Image {
        attrs: ImageAttributes,
    },
    Heading {
        attrs: HeadingAttributes,
        content: Vec<TextNode>,
    },
    Paragraph {
        #[serde(default)]
        content: Vec<TextNode>,
    },
    Blockquote {
        content: Vec<Node>,
    },
    Details {
        attrs: DetailAttributes,
    },
    HorizontalRule,
}

impl Node {
    pub fn append_string_content(&self, string: &mut String) {
        match self {
            Node::Image { attrs } => {
                if let Some(alt) = &attrs.alt {
                    string.push_str(alt);
                }
                if let Some(title) = &attrs.title {
                    string.push_str(title);
                }
            }
            Node::Heading { content, .. } | Node::Paragraph { content } => {
                for text_node in content {
                    let TextNode::Text { text, .. } = text_node;
                    string.push_str(text);
                }
            }
            Node::Blockquote { content } => {
                for node in content {
                    node.append_string_content(string);
                }
            }
            Node::Details { .. } | Node::HorizontalRule => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DetailNode {
    DetailsSummary { content: Vec<TextNode> },
    DetailsContent { content: Vec<Node> },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DetailAttributes {
    open: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TextNode {
    Text {
        text: String,
        #[serde(default)]
        marks: Vec<Mark>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Mark {
    Highlight,
    Code,
    Link { attrs: LinkAttributes },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct LinkAttributes {
    href: String,
    target: Option<String>,
    rel: Option<String>,
    class: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct HeadingAttributes {
    level: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "content")]
pub enum ContentNode {
    Doc(Vec<Node>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "attrs")]
pub enum OtherNode {
    Image(ImageAttributes),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ImageAttributes {
    alt: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
    src: Option<Url>,
    title: Option<String>,
}
