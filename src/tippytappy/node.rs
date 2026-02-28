use crate::tippytappy::*;
use markdown::mdast::Node as MdNode;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node<S: State> {
    Image {
        attrs: ImageAttributes,
    },
    Heading {
        attrs: HeadingAttributes,
        content: Vec<TextNode<S>>,
    },
    OrderedList(OrderedList<S>),
    BulletList(BulletListNode<S>),
    Paragraph {
        #[serde(default)]
        content: Vec<TextNode<S>>,
    },
    Blockquote {
        content: Vec<Node<S>>,
    },
    Details {
        attrs: DetailAttributes,
    },
    HorizontalRule,
}

impl Node<View> {
    pub fn compile(self, carriage: &mut CompileCarriage) -> Node<Compiled> {
        match self {
            Node::Image { attrs } => Node::Image {
                attrs: attrs.compile(carriage),
            },
            Node::OrderedList(oln) => Node::OrderedList(oln.compile(carriage)),
            Node::BulletList(bln) => Node::BulletList(bln.compile(carriage)),
            Node::Heading { attrs, content } => {
                let new_content = content.into_iter().map(|node| node.compile(carriage));
                Node::Heading {
                    attrs,
                    content: new_content.collect(),
                }
            }
            Node::Paragraph { content } => {
                let new_content = content.into_iter().map(|node| node.compile(carriage));

                Node::Paragraph {
                    content: new_content.collect(),
                }
            }
            Node::Blockquote { content } => {
                let new_content = content.into_iter().map(|node| node.compile(carriage));

                Node::Blockquote {
                    content: new_content.collect(),
                }
            }
            Node::Details { attrs } => Node::Details { attrs },
            Node::HorizontalRule => Node::HorizontalRule,
        }
    }
    pub fn from_mdast(node: MdNode) -> Result<Self, ParseError> {
        match node {
            MdNode::Root(_) => Err(ParseError::other("Found root in invalid position")),

            MdNode::Paragraph(p) => {
                let content = p
                    .children
                    .into_iter()
                    .flat_map(|child| collect_text_nodes(child, &[]))
                    .collect();
                Ok(Node::Paragraph { content })
            }

            MdNode::Heading(h) => {
                let content = h
                    .children
                    .into_iter()
                    .flat_map(|child| collect_text_nodes(child, &[]))
                    .collect();
                Ok(Node::Heading {
                    attrs: HeadingAttributes {
                        level: h.depth as u32,
                    },
                    content,
                })
            }

            MdNode::Blockquote(bq) => {
                let content = bq
                    .children
                    .into_iter()
                    .map(Node::from_mdast)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Node::Blockquote { content })
            }

            MdNode::List(list) => {
                let items = list
                    .children
                    .into_iter()
                    .map(|item| match item {
                        MdNode::ListItem(li) => {
                            let content = li
                                .children
                                .into_iter()
                                .map(Node::from_mdast)
                                .collect::<Result<Vec<_>, _>>()?;
                            Ok(ListChild::new(content))
                        }
                        _ => Err(ParseError::other("Expected list item inside list")),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                if list.ordered {
                    Ok(Node::OrderedList(OrderedList::new(
                        list.start.unwrap_or(1),
                        items,
                    )))
                } else {
                    Ok(Node::BulletList(BulletListNode::new(items)))
                }
            }

            MdNode::ThematicBreak(_) => Ok(Node::HorizontalRule),

            MdNode::Code(code) => {
                let content = vec![TextNode::text_node(Text {
                    text: code.value,
                    marks: vec![Mark::Code],
                })];
                Ok(Node::Paragraph { content })
            }

            MdNode::Image(img) => Ok(Node::Image {
                attrs: ImageAttributes {
                    alt: if img.alt.is_empty() {
                        None
                    } else {
                        Some(img.alt)
                    },
                    src: Url::parse(&img.url).ok(),
                    title: img.title,
                    height: None,
                    width: None,
                },
            }),

            MdNode::Html(html) => Ok(Node::Paragraph {
                content: vec![TextNode::text_node(Text {
                    text: html.value,
                    marks: vec![],
                })],
            }),

            MdNode::Table(table) => {
                let content = table
                    .children
                    .into_iter()
                    .flat_map(|row| match row {
                        MdNode::TableRow(tr) => tr
                            .children
                            .into_iter()
                            .flat_map(|cell| match cell {
                                MdNode::TableCell(tc) => tc
                                    .children
                                    .into_iter()
                                    .flat_map(|child| collect_text_nodes(child, &[]))
                                    .collect::<Vec<_>>(),
                                other => collect_text_nodes(other, &[]),
                            })
                            .collect::<Vec<_>>(),
                        other => collect_text_nodes(other, &[]),
                    })
                    .collect();
                Ok(Node::Paragraph { content })
            }

            MdNode::Definition(_) | MdNode::FootnoteDefinition(_) => {
                Ok(Node::Paragraph { content: vec![] })
            }

            // Inline nodes appearing at block level — wrap in a paragraph
            node @ (MdNode::Text(_)
            | MdNode::Strong(_)
            | MdNode::Emphasis(_)
            | MdNode::Link(_)
            | MdNode::InlineCode(_)
            | MdNode::Delete(_)
            | MdNode::Break(_)
            | MdNode::FootnoteReference(_)) => {
                let content = collect_text_nodes(node, &[]);
                Ok(Node::Paragraph { content })
            }

            // Structural nodes that shouldn't appear at block level
            MdNode::ListItem(_) | MdNode::TableRow(_) | MdNode::TableCell(_) => Err(
                ParseError::other("Unexpected structural node at block level"),
            ),

            // MDX, frontmatter, math, and other unsupported nodes
            _ => Err(ParseError::other("Unsupported mdast node type")),
        }
    }
}

impl<S: State> Node<S> {
    //pub fn compile(self, carriage: &mut CompileCarriage)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DetailNode<S: State> {
    DetailsSummary { content: Vec<TextNode<S>> },
    DetailsContent { content: Vec<Node<S>> },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DetailAttributes {
    open: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HeadingAttributes {
    level: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "content")]
pub enum ContentNode<S: State> {
    Doc(Vec<Node<S>>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", content = "attrs")]
pub enum OtherNode {
    Image(ImageAttributes),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ImageAttributes {
    alt: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
    src: Option<Url>,
    title: Option<String>,
}
impl ImageAttributes {
    pub fn compile(self, carriage: &mut CompileCarriage) -> Self {
        if let Some(alt) = &self.alt {
            carriage.push_str(alt);
        }
        if let Some(title) = &self.title {
            carriage.push_str(title);
        }
        self
    }
}

/// Recursively flattens mdast inline nodes into a list of [`TextNode`]s,
/// accumulating marks (code, link) as we descend.
fn collect_text_nodes(node: MdNode, marks: &[Mark]) -> Vec<TextNode<View>> {
    match node {
        MdNode::Text(t) => vec![TextNode::text_node(Text {
            text: t.value,
            marks: marks.to_vec(),
        })],

        MdNode::InlineCode(ic) => {
            let mut new_marks = marks.to_vec();
            new_marks.push(Mark::Code);
            vec![TextNode::text_node(Text {
                text: ic.value,
                marks: new_marks,
            })]
        }

        MdNode::Link(link) => {
            let mut new_marks = marks.to_vec();
            new_marks.push(Mark::Link {
                attrs: LinkAttributes {
                    href: link.url,
                    target: None,
                    rel: None,
                    class: None,
                    title: link.title,
                },
            });
            link.children
                .into_iter()
                .flat_map(|child| collect_text_nodes(child, &new_marks))
                .collect()
        }

        MdNode::Strong(s) => s
            .children
            .into_iter()
            .flat_map(|child| collect_text_nodes(child, marks))
            .collect(),

        MdNode::Emphasis(e) => e
            .children
            .into_iter()
            .flat_map(|child| collect_text_nodes(child, marks))
            .collect(),

        MdNode::Delete(d) => d
            .children
            .into_iter()
            .flat_map(|child| collect_text_nodes(child, marks))
            .collect(),

        MdNode::Break(_) => vec![TextNode::text_node(Text {
            text: "\n".to_string(),
            marks: marks.to_vec(),
        })],

        MdNode::Image(img) => vec![TextNode::text_node(Text {
            text: img.alt,
            marks: marks.to_vec(),
        })],

        MdNode::FootnoteReference(fr) => vec![TextNode::text_node(Text {
            text: format!("[{}]", fr.identifier),
            marks: marks.to_vec(),
        })],

        _ => vec![],
    }
}

#[cfg(test)]
fn parse_nodes(md: &str) -> Vec<Node<View>> {
    let doc = Document::parse_markdown(md).unwrap();
    let json = serde_json::to_value(&doc).unwrap();
    // Extract the content vec by round-tripping through the Document
    // We can't access doc.content directly (private), but we can
    // serialize → deserialize into a helper.
    #[derive(Deserialize)]
    struct DocHelper {
        content: Vec<Node<View>>,
    }
    let helper: DocHelper = serde_json::from_value(json).unwrap();
    helper.content
}

#[cfg(test)]
fn plain(s: &str) -> TextNode<View> {
    TextNode::text_node(Text {
        text: s.to_string(),
        marks: vec![],
    })
}

#[cfg(test)]
fn coded(s: &str) -> TextNode<View> {
    TextNode::text_node(Text {
        text: s.to_string(),
        marks: vec![Mark::Code],
    })
}

#[cfg(test)]
fn linked(s: &str, href: &str) -> TextNode<View> {
    TextNode::text_node(Text {
        text: s.to_string(),
        marks: vec![Mark::Link {
            attrs: LinkAttributes {
                href: href.to_string(),
                target: None,
                rel: None,
                class: None,
                title: None,
            },
        }],
    })
}

#[cfg(test)]
fn linked_with_title(s: &str, href: &str, title: &str) -> TextNode<View> {
    TextNode::text_node(Text {
        text: s.to_string(),
        marks: vec![Mark::Link {
            attrs: LinkAttributes {
                href: href.to_string(),
                target: None,
                rel: None,
                class: None,
                title: Some(title.to_string()),
            },
        }],
    })
}

#[cfg(test)]
fn coded_and_linked(s: &str, href: &str) -> TextNode<View> {
    TextNode::text_node(Text {
        text: s.to_string(),
        marks: vec![
            Mark::Link {
                attrs: LinkAttributes {
                    href: href.to_string(),
                    target: None,
                    rel: None,
                    class: None,
                    title: None,
                },
            },
            Mark::Code,
        ],
    })
}

// -----------------------------------------------------------------------
// Basic block elements
// -----------------------------------------------------------------------

#[test]
fn simple_paragraph() {
    let nodes = parse_nodes("Hello world");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![plain("Hello world")]
        }]
    );
}

#[test]
fn multiple_paragraphs() {
    let nodes = parse_nodes("First\n\nSecond");
    assert_eq!(
        nodes,
        vec![
            Node::Paragraph {
                content: vec![plain("First")]
            },
            Node::Paragraph {
                content: vec![plain("Second")]
            },
        ]
    );
}

#[test]
fn heading_levels() {
    for level in 1u32..=6 {
        let md = format!("{} Heading {level}", "#".repeat(level as usize));
        let nodes = parse_nodes(&md);
        assert_eq!(
            nodes,
            vec![Node::Heading {
                attrs: HeadingAttributes { level },
                content: vec![plain(&format!("Heading {level}"))],
            }]
        );
    }
}

#[test]
fn horizontal_rule() {
    let nodes = parse_nodes("above\n\n---\n\nbelow");
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[1], Node::HorizontalRule);
}

#[test]
fn blockquote_simple() {
    let nodes = parse_nodes("> quoted text");
    assert_eq!(
        nodes,
        vec![Node::Blockquote {
            content: vec![Node::Paragraph {
                content: vec![plain("quoted text")]
            }]
        }]
    );
}

#[test]
fn blockquote_nested() {
    let nodes = parse_nodes("> outer\n>\n>> inner");
    match &nodes[0] {
        Node::Blockquote { content } => {
            let has_nested = content.iter().any(|n| matches!(n, Node::Blockquote { .. }));
            assert!(has_nested, "Expected nested blockquote");
        }
        other => panic!("Expected blockquote, got {other:?}"),
    }
}

#[test]
fn fenced_code_block() {
    let nodes = parse_nodes("```rust\nfn main() {}\n```");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![coded("fn main() {}")]
        }]
    );
}

#[test]
fn code_block_multiline() {
    let nodes = parse_nodes("```\nline one\nline two\nline three\n```");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![coded("line one\nline two\nline three")]
        }]
    );
}

// -----------------------------------------------------------------------
// Lists
// -----------------------------------------------------------------------

#[test]
fn unordered_list() {
    let nodes = parse_nodes("- alpha\n- beta\n- gamma");
    assert_eq!(
        nodes,
        vec![Node::BulletList(BulletListNode::new(vec![
            ListChild::new(vec![Node::Paragraph {
                content: vec![plain("alpha")]
            }]),
            ListChild::new(vec![Node::Paragraph {
                content: vec![plain("beta")]
            }]),
            ListChild::new(vec![Node::Paragraph {
                content: vec![plain("gamma")]
            }]),
        ]))]
    );
}

#[test]
fn ordered_list() {
    let nodes = parse_nodes("1. first\n2. second\n3. third");
    assert_eq!(
        nodes,
        vec![Node::OrderedList(OrderedList::new(
            1,
            vec![
                ListChild::new(vec![Node::Paragraph {
                    content: vec![plain("first")]
                }]),
                ListChild::new(vec![Node::Paragraph {
                    content: vec![plain("second")]
                }]),
                ListChild::new(vec![Node::Paragraph {
                    content: vec![plain("third")]
                }]),
            ]
        ))]
    );
}

#[test]
fn ordered_list_custom_start() {
    let nodes = parse_nodes("5. fifth\n6. sixth");
    match &nodes[0] {
        Node::OrderedList(ol) => assert_eq!(ol.attrs.start, 5),
        other => panic!("Expected ordered list, got {other:?}"),
    }
}

#[test]
fn list_with_inline_formatting() {
    let nodes = parse_nodes("- normal\n- **bold**\n- `code`");
    assert_eq!(
        nodes,
        vec![Node::BulletList(BulletListNode::new(vec![
            ListChild::new(vec![Node::Paragraph {
                content: vec![plain("normal")]
            }]),
            ListChild::new(vec![Node::Paragraph {
                content: vec![plain("bold")]
            }]),
            ListChild::new(vec![Node::Paragraph {
                content: vec![coded("code")]
            }]),
        ]))]
    );
}

#[test]
fn list_with_nested_paragraphs() {
    let md = "- paragraph one\n\n  paragraph two in same item";
    let nodes = parse_nodes(md);
    match &nodes[0] {
        Node::BulletList(bl) => {
            let ListChild::ListItem { content } = &bl.content[0];
            assert!(
                content.len() >= 2,
                "Expected multiple paragraphs in list item, got {}",
                content.len()
            );
        }
        other => panic!("Expected bullet list, got {other:?}"),
    }
}

// -----------------------------------------------------------------------
// Inline formatting
// -----------------------------------------------------------------------

#[test]
fn inline_code() {
    let nodes = parse_nodes("Use `println!` here");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![plain("Use "), coded("println!"), plain(" here")]
        }]
    );
}

#[test]
fn link_basic() {
    let nodes = parse_nodes("[click me](https://example.com)");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![linked("click me", "https://example.com")]
        }]
    );
}

#[test]
fn link_with_title() {
    let nodes = parse_nodes(r#"[click](https://example.com "My Title")"#);
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![linked_with_title(
                "click",
                "https://example.com",
                "My Title"
            )]
        }]
    );
}

#[test]
fn bold_text_no_mark() {
    let nodes = parse_nodes("before **bold** after");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![plain("before "), plain("bold"), plain(" after")]
        }]
    );
}

#[test]
fn italic_text_no_mark() {
    let nodes = parse_nodes("before *italic* after");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![plain("before "), plain("italic"), plain(" after")]
        }]
    );
}

#[test]
fn strikethrough_no_mark() {
    let nodes = parse_nodes("before ~~deleted~~ after");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![plain("before "), plain("deleted"), plain(" after")]
        }]
    );
}

#[test]
fn mixed_inline() {
    let nodes = parse_nodes("plain `code` and [link](https://x.com) end");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![
                plain("plain "),
                coded("code"),
                plain(" and "),
                linked("link", "https://x.com"),
                plain(" end"),
            ]
        }]
    );
}

// -----------------------------------------------------------------------
// Tricky inline nesting
// -----------------------------------------------------------------------

#[test]
fn inline_code_in_heading() {
    let nodes = parse_nodes("## The `Config` struct");
    assert_eq!(
        nodes,
        vec![Node::Heading {
            attrs: HeadingAttributes { level: 2 },
            content: vec![plain("The "), coded("Config"), plain(" struct")],
        }]
    );
}

#[test]
fn link_in_heading() {
    let nodes = parse_nodes("# Visit [Example](https://example.com) now");
    assert_eq!(
        nodes,
        vec![Node::Heading {
            attrs: HeadingAttributes { level: 1 },
            content: vec![
                plain("Visit "),
                linked("Example", "https://example.com"),
                plain(" now"),
            ],
        }]
    );
}

#[test]
fn code_inside_link() {
    // [`some_fn()`](https://docs.rs) — code nested inside link
    let nodes = parse_nodes("[`some_fn()`](https://docs.rs)");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![coded_and_linked("some_fn()", "https://docs.rs")]
        }]
    );
}

#[test]
fn link_inside_bold() {
    let nodes = parse_nodes("**[bold link](https://example.com)**");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![linked("bold link", "https://example.com")]
        }]
    );
}

#[test]
fn bold_and_italic_nested() {
    let nodes = parse_nodes("***both***");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![plain("both")]
        }]
    );
}

#[test]
fn inline_code_preserves_special_chars() {
    let nodes = parse_nodes("Use `<div class=\"foo\">` in HTML");
    assert_eq!(
        nodes,
        vec![Node::Paragraph {
            content: vec![
                plain("Use "),
                coded("<div class=\"foo\">"),
                plain(" in HTML"),
            ]
        }]
    );
}

#[test]
fn heading_only_code() {
    let nodes = parse_nodes("### `only code`");
    assert_eq!(
        nodes,
        vec![Node::Heading {
            attrs: HeadingAttributes { level: 3 },
            content: vec![coded("only code")],
        }]
    );
}

#[test]
fn heading_with_multiple_links() {
    let nodes = parse_nodes("## [Foo](https://foo.com) and [Bar](https://bar.com)");
    assert_eq!(
        nodes,
        vec![Node::Heading {
            attrs: HeadingAttributes { level: 2 },
            content: vec![
                linked("Foo", "https://foo.com"),
                plain(" and "),
                linked("Bar", "https://bar.com"),
            ],
        }]
    );
}

// -----------------------------------------------------------------------
// Images
// -----------------------------------------------------------------------

#[test]
fn inline_image_becomes_alt_text() {
    // In markdown, ![alt](url) inside a paragraph is inline.
    // collect_text_nodes converts it to its alt text.
    let nodes = parse_nodes("![alt text](https://example.com/img.png)");
    match &nodes[0] {
        Node::Paragraph { content } => {
            let texts: Vec<&str> = content.iter().map(|t| t.text()).collect();
            assert!(texts.contains(&"alt text"));
        }
        other => panic!("Expected paragraph, got {other:?}"),
    }
}

// -----------------------------------------------------------------------
// HTML blocks
// -----------------------------------------------------------------------

#[test]
fn html_block() {
    let nodes = parse_nodes("<div>\n  <p>hello</p>\n</div>");
    match &nodes[0] {
        Node::Paragraph { content } => {
            assert!(content[0].text().contains("<div>"));
        }
        other => panic!("Expected paragraph, got {other:?}"),
    }
}

// -----------------------------------------------------------------------
// GFM tables
// -----------------------------------------------------------------------

#[test]
fn gfm_table() {
    let nodes = parse_nodes("| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |");
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        Node::Paragraph { content } => {
            let texts: Vec<&str> = content.iter().map(|t| t.text()).collect();
            for expected in &["A", "B", "1", "2", "3", "4"] {
                assert!(texts.contains(expected), "Missing table cell: {expected}");
            }
        }
        other => panic!("Expected paragraph, got {other:?}"),
    }
}

// -----------------------------------------------------------------------
// GFM autolinks
// -----------------------------------------------------------------------

#[test]
fn autolink_gfm() {
    let nodes = parse_nodes("Visit https://example.com today");
    match &nodes[0] {
        Node::Paragraph { content } => {
            let has_link = content.iter().any(|t| match t.inner() {
                TextNodeView::Text(text) => {
                    text.marks.iter().any(|m| matches!(m, Mark::Link { .. }))
                }
                _ => false,
            });
            assert!(has_link, "Expected autolinked URL to have link mark");
        }
        other => panic!("Expected paragraph, got {other:?}"),
    }
}

// -----------------------------------------------------------------------
// to_search_text integration
// -----------------------------------------------------------------------

#[test]
fn search_text_extracts_all_content() {
    let md = "# Title\n\nParagraph with `code` and [link](https://x.com).\n\n- one\n- two";
    let doc = Document::parse_markdown(md).unwrap();
    let text = doc.compile().searchable_text;
    for expected in &["Title", "Paragraph with ", "code", "link", "one", "two"] {
        assert!(
            text.contains(expected),
            "Missing in search text: {expected}"
        );
    }
}

#[test]
fn search_text_from_blockquote() {
    let doc = Document::parse_markdown("> important quote").unwrap();
    assert!(doc.compile().searchable_text.contains("important quote"));
}

#[test]
fn mixed_document_structure() {
    let md = "\
# Welcome

Here is a paragraph.

> A wise quote

---

1. First
2. Second

- bullet a
- bullet b

```
code block
```

Final paragraph with [a link](https://example.com).";

    let nodes = parse_nodes(md);
    assert_eq!(
        nodes,
        vec![
            // # Welcome
            Node::Heading {
                attrs: HeadingAttributes { level: 1 },
                content: vec![plain("Welcome")],
            },
            // Here is a paragraph.
            Node::Paragraph {
                content: vec![plain("Here is a paragraph.")],
            },
            // > A wise quote
            Node::Blockquote {
                content: vec![Node::Paragraph {
                    content: vec![plain("A wise quote")],
                }],
            },
            // ---
            Node::HorizontalRule,
            // 1. First  2. Second
            Node::OrderedList(OrderedList::new(
                1,
                vec![
                    ListChild::new(vec![Node::Paragraph {
                        content: vec![plain("First")],
                    }]),
                    ListChild::new(vec![Node::Paragraph {
                        content: vec![plain("Second")],
                    }]),
                ],
            )),
            // - bullet a  - bullet b
            Node::BulletList(BulletListNode::new(vec![
                ListChild::new(vec![Node::Paragraph {
                    content: vec![plain("bullet a")],
                }]),
                ListChild::new(vec![Node::Paragraph {
                    content: vec![plain("bullet b")],
                }]),
            ])),
            // ```\ncode block\n```
            Node::Paragraph {
                content: vec![coded("code block")],
            },
            // Final paragraph with [a link](https://example.com).
            Node::Paragraph {
                content: vec![
                    plain("Final paragraph with "),
                    linked("a link", "https://example.com"),
                    plain("."),
                ],
            },
        ]
    );
}

#[test]
fn blockquote_with_formatting() {
    let nodes = parse_nodes("> **bold** and `code` in a quote");
    assert_eq!(
        nodes,
        vec![Node::Blockquote {
            content: vec![Node::Paragraph {
                content: vec![
                    plain("bold"),
                    plain(" and "),
                    coded("code"),
                    plain(" in a quote"),
                ],
            }],
        }]
    );
}

#[test]
fn empty_paragraphs_not_duplicated() {
    let nodes = parse_nodes("above\n\n\n\nbelow");
    assert_eq!(nodes.len(), 2);
}

// -----------------------------------------------------------------------
// JSON round-trip
// -----------------------------------------------------------------------

#[test]
fn json_round_trip() {
    let md = "# Hello\n\nWorld with `code`.";
    let doc = Document::parse_markdown(md).unwrap();
    let json = serde_json::to_value(&doc).unwrap();
    let doc2: Document = serde_json::from_value(json.clone()).unwrap();
    let json2 = serde_json::to_value(&doc2).unwrap();
    assert_eq!(json, json2);
}
