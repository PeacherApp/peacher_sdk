use crate::tippytappy::*;
use markdown::{ParseOptions, mdast::Node as MdNode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct View;

impl State for View {
    type TextNode = TextNodeView;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename = "doc")]
pub struct DocumentView {
    content: Vec<Node<View>>,
}

/// Need to manually implement this since
///
/// utoipa does not generate the correct type for structs that are internally tagged.
#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for DocumentView {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        use utoipa::openapi::schema::Type;
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "type",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(Type::String)
                    .enum_values::<_, &str>(Some(["doc"])),
            )
            .required("type")
            .property(
                "content",
                utoipa::openapi::schema::ArrayBuilder::new()
                    .items(utoipa::openapi::Ref::from_schema_name("Node_View")),
            )
            .required("content")
            .into()
    }
}

#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for DocumentView {
    fn name() -> std::borrow::Cow<'static, str> {
        "DocumentView".into()
    }

    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        <Node<View> as utoipa::ToSchema>::schemas(schemas);
    }
}

impl DocumentView {
    pub fn from_nodes(nodes: impl IntoIterator<Item = Node<View>>) -> Self {
        Self {
            content: nodes.into_iter().collect(),
        }
    }
    pub fn parse_json(value: serde_json::Value) -> Result<Self, ParseError> {
        let value = serde_json::from_value(value).map_err(|e| {
            tracing::error!("Invalid value passed for document view. Error: {e}");
            ParseError::Json(e)
        })?;

        Ok(value)
    }

    pub fn parse_markdown(markdown: &str) -> Result<Self, ParseError> {
        let parse_options = ParseOptions::gfm();

        let markdown = markdown::to_mdast(markdown, &parse_options)?;

        let MdNode::Root(root) = markdown else {
            return Err(ParseError::other("root element is not a root node!"));
        };

        let content = root
            .children
            .into_iter()
            .map(Node::from_mdast)
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { content })
    }

    pub fn compile(self) -> CompilationResult {
        let mut carriage = CompileCarriage::default();

        let compiled_nodes = self
            .content
            .into_iter()
            .map(|node| node.compile(&mut carriage));
        let document = CompiledDocument::from_nodes(compiled_nodes);
        carriage.finish(document)
    }
}
