use markdown::mdast;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub enum Node {
    Root(Root),
    Blockquote(Blockquote),
    FootnoteDefinition(FootnoteDefinition),
    MdxJsxFlowElement(MdxJsxFlowElement),
    List(List),
    MdxjsEsm(MdxjsEsm),
    Toml(Toml),
    Yaml(Yaml),
    Break(Break),
    InlineCode(InlineCode),
    InlineMath(InlineMath),
    Delete(Delete),
    Emphasis(Emphasis),
    MdxTextExpression(MdxTextExpression),
    FootnoteReference(FootnoteReference),
    Html(Html),
    Image(Image),
    ImageReference(ImageReference),
    MdxJsxTextElement(MdxJsxTextElement),
    Link(Link),
    LinkReference(LinkReference),
    Strong(Strong),
    Text(Text),
    Code(Code),
    Math(Math),
    MdxFlowExpression(MdxFlowExpression),
    Heading(Heading),
    Table(Table),
    ThematicBreak(ThematicBreak),
    TableRow(TableRow),
    TableCell(TableCell),
    ListItem(ListItem),
    Definition(Definition),
    Paragraph(Paragraph),
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Root {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::Node> for Node {
    fn from(node: mdast::Node) -> Self {
        match node {
            mdast::Node::Root(root) => Node::Root(root.into()),
            mdast::Node::Blockquote(blockquote) => Node::Blockquote(blockquote.into()),
            mdast::Node::FootnoteDefinition(footnote_definition) => {
                Node::FootnoteDefinition(footnote_definition.into())
            }
            mdast::Node::MdxJsxFlowElement(mdx_jsx_flow_element) => {
                Node::MdxJsxFlowElement(mdx_jsx_flow_element.into())
            }
            mdast::Node::List(list) => Node::List(list.into()),
            mdast::Node::MdxjsEsm(mdxjs_esm) => Node::MdxjsEsm(mdxjs_esm.into()),
            mdast::Node::Toml(toml) => Node::Toml(toml.into()),
            mdast::Node::Yaml(yaml) => Node::Yaml(yaml.into()),
            mdast::Node::Break(break_node) => Node::Break(break_node.into()),
            mdast::Node::InlineCode(inline_code) => Node::InlineCode(inline_code.into()),
            mdast::Node::InlineMath(inline_math) => Node::InlineMath(inline_math.into()),
            mdast::Node::Delete(delete) => Node::Delete(delete.into()),
            mdast::Node::Emphasis(emphasis) => Node::Emphasis(emphasis.into()),
            mdast::Node::MdxTextExpression(mdx_text_expression) => {
                Node::MdxTextExpression(mdx_text_expression.into())
            }
            mdast::Node::FootnoteReference(footnote_reference) => {
                Node::FootnoteReference(footnote_reference.into())
            }
            mdast::Node::Html(html) => Node::Html(html.into()),
            mdast::Node::Image(image) => Node::Image(image.into()),
            mdast::Node::ImageReference(image_reference) => {
                Node::ImageReference(image_reference.into())
            }
            mdast::Node::MdxJsxTextElement(mdx_jsx_text_element) => {
                Node::MdxJsxTextElement(mdx_jsx_text_element.into())
            }
            mdast::Node::Link(link) => Node::Link(link.into()),
            mdast::Node::LinkReference(link_reference) => {
                Node::LinkReference(link_reference.into())
            }
            mdast::Node::Strong(strong) => Node::Strong(strong.into()),
            mdast::Node::Text(text) => Node::Text(text.into()),
            mdast::Node::Code(code) => Node::Code(code.into()),
            mdast::Node::Math(math) => Node::Math(math.into()),
            mdast::Node::MdxFlowExpression(mdx_flow_expression) => {
                Node::MdxFlowExpression(mdx_flow_expression.into())
            }
            mdast::Node::Heading(heading) => Node::Heading(heading.into()),
            mdast::Node::Table(table) => Node::Table(table.into()),
            mdast::Node::ThematicBreak(thematic_break) => {
                Node::ThematicBreak(thematic_break.into())
            }
            mdast::Node::TableRow(table_row) => Node::TableRow(table_row.into()),
            mdast::Node::TableCell(table_cell) => Node::TableCell(table_cell.into()),
            mdast::Node::ListItem(list_item) => Node::ListItem(list_item.into()),
            mdast::Node::Definition(definition) => Node::Definition(definition.into()),
            mdast::Node::Paragraph(paragraph) => Node::Paragraph(paragraph.into()),
        }
    }
}

impl From<mdast::Root> for Root {
    fn from(root: mdast::Root) -> Self {
        Root {
            children: root.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Blockquote {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::Blockquote> for Blockquote {
    fn from(blockquote: mdast::Blockquote) -> Self {
        Blockquote {
            children: blockquote.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct FootnoteDefinition {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl From<mdast::FootnoteDefinition> for FootnoteDefinition {
    fn from(footnote_definition: mdast::FootnoteDefinition) -> Self {
        FootnoteDefinition {
            children: footnote_definition
                .children
                .into_iter()
                .map(Node::from)
                .collect(),
            identifier: footnote_definition.identifier,
            label: footnote_definition.label,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxJsxFlowElement {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub attributes: Vec<AttributeContent>,
}

impl From<mdast::MdxJsxFlowElement> for MdxJsxFlowElement {
    fn from(mdx_jsx_flow_element: mdast::MdxJsxFlowElement) -> Self {
        MdxJsxFlowElement {
            children: mdx_jsx_flow_element
                .children
                .into_iter()
                .map(Node::from)
                .collect(),
            name: mdx_jsx_flow_element.name,
            attributes: mdx_jsx_flow_element
                .attributes
                .into_iter()
                .map(AttributeContent::from)
                .collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub enum AttributeContent {
    Expression(MdxJsxExpressionAttribute),
    Property(MdxJsxAttribute),
}

impl From<mdast::AttributeContent> for AttributeContent {
    fn from(attribute_content: mdast::AttributeContent) -> Self {
        match attribute_content {
            mdast::AttributeContent::Expression(expression) => {
                AttributeContent::Expression(expression.into())
            }
            mdast::AttributeContent::Property(property) => {
                AttributeContent::Property(property.into())
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxJsxExpressionAttribute {
    /// Value.
    pub value: String,
}

impl From<mdast::MdxJsxExpressionAttribute> for MdxJsxExpressionAttribute {
    fn from(mdx_jsx_expression_attribute: mdast::MdxJsxExpressionAttribute) -> Self {
        MdxJsxExpressionAttribute {
            value: mdx_jsx_expression_attribute.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxJsxAttribute {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

impl From<mdast::MdxJsxAttribute> for MdxJsxAttribute {
    fn from(mdx_jsx_attribute: mdast::MdxJsxAttribute) -> Self {
        MdxJsxAttribute {
            name: mdx_jsx_attribute.name,
            value: mdx_jsx_attribute.value.map(AttributeValue::from),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub enum AttributeValue {
    Expression(AttributeValueExpression),
    Literal(String),
}

impl From<mdast::AttributeValue> for AttributeValue {
    fn from(attribute_value: mdast::AttributeValue) -> Self {
        match attribute_value {
            mdast::AttributeValue::Expression(expression) => {
                AttributeValue::Expression(expression.into())
            }
            mdast::AttributeValue::Literal(literal) => AttributeValue::Literal(literal),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct AttributeValueExpression {
    pub value: String,
}

impl From<mdast::AttributeValueExpression> for AttributeValueExpression {
    fn from(attribute_value_expression: mdast::AttributeValueExpression) -> Self {
        AttributeValueExpression {
            value: attribute_value_expression.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct List {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    pub ordered: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    pub spread: bool,
}

impl From<mdast::List> for List {
    fn from(list: mdast::List) -> Self {
        List {
            children: list.children.into_iter().map(Node::from).collect(),
            ordered: list.ordered,
            start: list.start,
            spread: list.spread,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxjsEsm {
    pub value: String,
}

impl From<mdast::MdxjsEsm> for MdxjsEsm {
    fn from(mdxjs_esm: mdast::MdxjsEsm) -> Self {
        MdxjsEsm {
            value: mdxjs_esm.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Toml {
    pub value: String,
}

impl From<mdast::Toml> for Toml {
    fn from(toml: mdast::Toml) -> Self {
        Toml { value: toml.value }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Yaml {
    pub value: String,
}

impl From<mdast::Yaml> for Yaml {
    fn from(yaml: mdast::Yaml) -> Self {
        Yaml { value: yaml.value }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Break {}

impl From<mdast::Break> for Break {
    fn from(_: mdast::Break) -> Self {
        Break {}
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct InlineCode {
    pub value: String,
}

impl From<mdast::InlineCode> for InlineCode {
    fn from(inline_code: mdast::InlineCode) -> Self {
        InlineCode {
            value: inline_code.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct InlineMath {
    pub value: String,
}

impl From<mdast::InlineMath> for InlineMath {
    fn from(inline_math: mdast::InlineMath) -> Self {
        InlineMath {
            value: inline_math.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Delete {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::Delete> for Delete {
    fn from(delete: mdast::Delete) -> Self {
        Delete {
            children: delete.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Emphasis {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::Emphasis> for Emphasis {
    fn from(emphasis: mdast::Emphasis) -> Self {
        Emphasis {
            children: emphasis.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxTextExpression {
    pub value: String,
}

impl From<mdast::MdxTextExpression> for MdxTextExpression {
    fn from(mdx_text_expression: mdast::MdxTextExpression) -> Self {
        MdxTextExpression {
            value: mdx_text_expression.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct FootnoteReference {
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl From<mdast::FootnoteReference> for FootnoteReference {
    fn from(footnote_reference: mdast::FootnoteReference) -> Self {
        FootnoteReference {
            identifier: footnote_reference.identifier,
            label: footnote_reference.label,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Html {
    pub value: String,
}

impl From<mdast::Html> for Html {
    fn from(html: mdast::Html) -> Self {
        Html { value: html.value }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Image {
    pub alt: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl From<mdast::Image> for Image {
    fn from(image: mdast::Image) -> Self {
        Image {
            alt: image.alt,
            url: image.url,
            title: image.title,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct ImageReference {
    pub alt: String,
    #[serde(rename = "referenceType")]
    pub reference_kind: ReferenceKind,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl From<mdast::ImageReference> for ImageReference {
    fn from(image_reference: mdast::ImageReference) -> Self {
        ImageReference {
            alt: image_reference.alt,
            reference_kind: image_reference.reference_kind.into(),
            identifier: image_reference.identifier,
            label: image_reference.label,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxJsxTextElement {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[schema(no_recursion)]
    pub attributes: Vec<AttributeContent>,
}

impl From<mdast::MdxJsxTextElement> for MdxJsxTextElement {
    fn from(mdx_jsx_text_element: mdast::MdxJsxTextElement) -> Self {
        MdxJsxTextElement {
            children: mdx_jsx_text_element
                .children
                .into_iter()
                .map(Node::from)
                .collect(),
            name: mdx_jsx_text_element.name,
            attributes: mdx_jsx_text_element
                .attributes
                .into_iter()
                .map(AttributeContent::from)
                .collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Link {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl From<mdast::Link> for Link {
    fn from(link: mdast::Link) -> Self {
        Link {
            children: link.children.into_iter().map(Node::from).collect(),
            url: link.url,
            title: link.title,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub enum ReferenceKind {
    Shortcut,
    Collapsed,
    Full,
}

impl From<mdast::ReferenceKind> for ReferenceKind {
    fn from(reference_kind: mdast::ReferenceKind) -> Self {
        match reference_kind {
            mdast::ReferenceKind::Shortcut => ReferenceKind::Shortcut,
            mdast::ReferenceKind::Collapsed => ReferenceKind::Collapsed,
            mdast::ReferenceKind::Full => ReferenceKind::Full,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct LinkReference {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    #[serde(rename = "referenceType")]
    pub reference_kind: ReferenceKind,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl From<mdast::LinkReference> for LinkReference {
    fn from(link_reference: mdast::LinkReference) -> Self {
        LinkReference {
            children: link_reference
                .children
                .into_iter()
                .map(Node::from)
                .collect(),
            reference_kind: link_reference.reference_kind.into(),
            identifier: link_reference.identifier,
            label: link_reference.label,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Strong {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::Strong> for Strong {
    fn from(strong: mdast::Strong) -> Self {
        Strong {
            children: strong.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Text {
    pub value: String,
}

impl From<mdast::Text> for Text {
    fn from(text: mdast::Text) -> Self {
        Text { value: text.value }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Code {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
}

impl From<mdast::Code> for Code {
    fn from(code: mdast::Code) -> Self {
        Code {
            value: code.value,
            lang: code.lang,
            meta: code.meta,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Math {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
}

impl From<mdast::Math> for Math {
    fn from(math: mdast::Math) -> Self {
        Math {
            value: math.value,
            meta: math.meta,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct MdxFlowExpression {
    pub value: String,
}

impl From<mdast::MdxFlowExpression> for MdxFlowExpression {
    fn from(mdx_flow_expression: mdast::MdxFlowExpression) -> Self {
        MdxFlowExpression {
            value: mdx_flow_expression.value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Heading {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    pub depth: u8,
}

impl From<mdast::Heading> for Heading {
    fn from(heading: mdast::Heading) -> Self {
        Heading {
            children: heading.children.into_iter().map(Node::from).collect(),
            depth: heading.depth,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Table {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    pub align: Vec<AlignKind>,
}

impl From<mdast::Table> for Table {
    fn from(table: mdast::Table) -> Self {
        Table {
            children: table.children.into_iter().map(Node::from).collect(),
            align: table.align.into_iter().map(AlignKind::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub enum AlignKind {
    Left,
    Right,
    Center,
    None,
}

impl From<mdast::AlignKind> for AlignKind {
    fn from(align_kind: mdast::AlignKind) -> Self {
        match align_kind {
            mdast::AlignKind::Left => AlignKind::Left,
            mdast::AlignKind::Right => AlignKind::Right,
            mdast::AlignKind::Center => AlignKind::Center,
            mdast::AlignKind::None => AlignKind::None,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct ThematicBreak {}

impl From<mdast::ThematicBreak> for ThematicBreak {
    fn from(_: mdast::ThematicBreak) -> Self {
        ThematicBreak {}
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct TableRow {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::TableRow> for TableRow {
    fn from(table_row: mdast::TableRow) -> Self {
        TableRow {
            children: table_row.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct TableCell {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::TableCell> for TableCell {
    fn from(table_cell: mdast::TableCell) -> Self {
        TableCell {
            children: table_cell.children.into_iter().map(Node::from).collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct ListItem {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
    pub spread: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
}

impl From<mdast::ListItem> for ListItem {
    fn from(list_item: mdast::ListItem) -> Self {
        ListItem {
            children: list_item.children.into_iter().map(Node::from).collect(),
            spread: list_item.spread,
            checked: list_item.checked,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Definition {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl From<mdast::Definition> for Definition {
    fn from(definition: mdast::Definition) -> Self {
        Definition {
            url: definition.url,
            title: definition.title,
            identifier: definition.identifier,
            label: definition.label,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Paragraph {
    #[schema(no_recursion)]
    pub children: Vec<Node>,
}

impl From<mdast::Paragraph> for Paragraph {
    fn from(paragraph: mdast::Paragraph) -> Self {
        Paragraph {
            children: paragraph.children.into_iter().map(Node::from).collect(),
        }
    }
}
