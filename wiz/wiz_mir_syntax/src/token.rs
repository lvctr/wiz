use crate::span::{Span, DUMMY_SPAN};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CommentKind {
    Line,
    Block,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinOpToken {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,
    Shl,
    Shr,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum LitKind {
    Bool,
    Byte,
    Char,
    Integer,
    Float,
    Str,
    StrRaw,
    ByteStr,
    ByteStrRaw,
    Err,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DelimToken {
    /// A round parenthesis (i.e., `(` or `)`).
    Paren,
    /// A square bracket (i.e., `[` or `]`).
    Bracket,
    /// A curly brace (i.e., `{` or `}`).
    Brace,
    /// An empty delimiter.
    NoDelim,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Lit {
    pub kind: LitKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenKind {
    /* Expression-operator symbols. */
    Eq,
    Lt,
    Le,
    EqEq,
    Ne,
    Ge,
    Gt,
    AndAnd,
    OrOr,
    Not,
    Tilde,
    BinOp(BinOpToken),
    BinOpEq(BinOpToken),

    /* Structural symbols */
    At,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEq,
    Comma,
    Semi,
    Colon,
    ModSep,
    RArrow,
    LArrow,
    FatArrow,
    Pound,
    Dollar,
    Question,
    /// Used by proc macros for representing lifetimes, not generated by lexer right now.
    SingleQuote,
    /// An opening delimiter (e.g., `{`).
    OpenDelim(DelimToken),
    /// A closing delimiter (e.g., `}`).
    CloseDelim(DelimToken),

    /* Literals */
    Literal(Lit),

    /// Identifier token.
    /// Do not forget about `NtIdent` when you want to match on identifiers.
    /// It's recommended to use `Token::(ident,uninterpolate,uninterpolated_span)` to
    /// treat regular and interpolated identifiers in the same way.
    Ident(/* is_raw */ bool),
    /// Lifetime identifier token.
    /// Do not forget about `NtLifetime` when you want to match on lifetime identifiers.
    /// It's recommended to use `Token::(lifetime,uninterpolate,uninterpolated_span)` to
    /// treat regular and interpolated lifetime identifiers in the same way.
    Lifetime,

    /// TODO: AST struct
    Interpolated,

    /// A doc comment token.
    /// `Symbol` is the doc comment's data excluding its "quotes" (`///`, `/**`, etc)
    /// similarly to symbols in string literal tokens.
    DocComment(CommentKind, AttrStyle),

    Eof,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AttrStyle {
    Inner,
    Outer,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn dummy() -> Self {
        Self::new(TokenKind::Question, DUMMY_SPAN)
    }

    pub fn glue(&self, joint: &Token) -> Option<Token> {
        let kind = match self.kind {
            TokenKind::Eq => match joint.kind {
                TokenKind::Eq => TokenKind::EqEq,
                TokenKind::Gt => TokenKind::FatArrow,
                _ => return None,
            },
            TokenKind::Lt => match joint.kind {
                TokenKind::Eq => TokenKind::Le,
                TokenKind::Lt => TokenKind::BinOp(BinOpToken::Shl),
                TokenKind::Le => TokenKind::BinOpEq(BinOpToken::Shl),
                TokenKind::BinOp(BinOpToken::Minus) => TokenKind::LArrow,
                _ => return None,
            },
            TokenKind::Gt => match joint.kind {
                TokenKind::Eq => TokenKind::Ge,
                TokenKind::Gt => TokenKind::BinOp(BinOpToken::Shr),
                TokenKind::Ge => TokenKind::BinOpEq(BinOpToken::Shr),
                _ => return None,
            },
            TokenKind::Not => match joint.kind {
                TokenKind::Eq => TokenKind::Ne,
                _ => return None,
            },
            TokenKind::BinOp(op) => match joint.kind {
                TokenKind::Eq => TokenKind::BinOpEq(op),
                TokenKind::BinOp(BinOpToken::And) if op == BinOpToken::And => TokenKind::AndAnd,
                TokenKind::BinOp(BinOpToken::Or) if op == BinOpToken::Or => TokenKind::OrOr,
                TokenKind::Gt if op == BinOpToken::Minus => TokenKind::RArrow,
                _ => return None,
            },
            TokenKind::Dot => match joint.kind {
                TokenKind::Dot => TokenKind::DotDot,
                TokenKind::DotDot => TokenKind::DotDotDot,
                _ => return None,
            },
            TokenKind::DotDot => match joint.kind {
                TokenKind::Dot => TokenKind::DotDotDot,
                TokenKind::Eq => TokenKind::DotDotEq,
                _ => return None,
            },
            TokenKind::Colon => match joint.kind {
                TokenKind::Colon => TokenKind::ModSep,
                _ => return None,
            },
            TokenKind::SingleQuote => match joint.kind {
                TokenKind::Ident(false) => TokenKind::Lifetime,
                _ => return None,
            },

            TokenKind::Le
            | TokenKind::EqEq
            | TokenKind::Ne
            | TokenKind::Ge
            | TokenKind::AndAnd
            | TokenKind::OrOr
            | TokenKind::Tilde
            | TokenKind::BinOpEq(..)
            | TokenKind::At
            | TokenKind::DotDotDot
            | TokenKind::DotDotEq
            | TokenKind::Comma
            | TokenKind::Semi
            | TokenKind::ModSep
            | TokenKind::RArrow
            | TokenKind::LArrow
            | TokenKind::FatArrow
            | TokenKind::Pound
            | TokenKind::Dollar
            | TokenKind::Question
            | TokenKind::OpenDelim(..)
            | TokenKind::CloseDelim(..)
            | TokenKind::Literal(..)
            | TokenKind::Ident(..)
            | TokenKind::Lifetime
            | TokenKind::Interpolated
            | TokenKind::DocComment(..)
            | TokenKind::Eof => return None,
        };

        Some(Token::new(kind, self.span.to(&joint.span)))
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Spacing {
    Alone,
    Joint,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenTree {
    /// A single token.
    Token(Token),
    /// A delimited sequence of token trees.
    Delimited(DelimSpan, DelimToken, TokenStream),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DelimSpan {
    pub open: Span,
    pub close: Span,
}

pub type TreeAndSpacing = (TokenTree, Spacing);

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct TokenStream(pub Vec<TreeAndSpacing>);

impl TokenStream {
    pub fn new(stream: Vec<TreeAndSpacing>) -> Self {
        Self(stream)
    }
}
