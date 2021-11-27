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
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ItemKind {
    Struct,
    Function,
    Const,
    Static,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Statement {
    kind: StatementKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StatementKind {}
