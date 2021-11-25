use crate::span::Span;

#[derive(Debug, Eq, PartialEq, Clone)]
struct File {
    pub attrs: Vec<()>,
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Item {
    pub id: usize,
    pub attrs: Vec<()>,
    pub visibility: (),
    pub kind: ItemKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ItemKind {
    Struct,
    Function,
    Const,
    Static,
}
