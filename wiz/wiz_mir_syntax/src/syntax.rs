use crate::span::Span;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct File {
    pub attrs: Vec<()>,
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub id: usize,
    pub attrs: Vec<()>,
    pub visibility: (),
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ItemKind {
    Struct(VariantData),
    Union(VariantData),
    Function(),
    Const(),
    Static(),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct VariantData {
    fields: Vec<Field>
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Field {
    pub id: usize,
    pub attrs: Vec<()>,
    pub visibility: (),
    pub span: Span,
    pub identifier: String,
    pub ty: (),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StatementKind {
    Expression,
    WhileLoop,
    Return,
}
