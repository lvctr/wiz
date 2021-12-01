#[cfg(test)]
mod tests;

use crate::error::{ParseError, PResult};
use std::vec::IntoIter;
use wiz_mir_syntax::span::DUMMY_SPAN;
use wiz_mir_syntax::syntax;
use wiz_mir_syntax::token::{Spacing, Token, TokenKind, TokenStream, TokenTree, TreeAndSpacing};

struct Parser {
    pub stream: IntoIter<TreeAndSpacing>,
    pub token: TokenTree,
    pub prev_token: TokenTree,
    pub token_spacing: Spacing,
}

impl Parser {
    pub fn parse(&mut self) -> PResult<syntax::File> {
        let start = self.token.span().clone();
        let attrs = self.parse_attributes()?;
        let mut items = vec![];
        while !self.stream.is_empty() {
            self.bump();
            items.push(self.parse_item()?);
        }
        Ok(syntax::File {
            attrs,
            items,
            span: start.to(&self.token.span()),
        })
    }
    /// Consume line that start with `#!`
    fn parse_file_attribute(&mut self) -> PResult<Vec<()>> {
        match &self.token {
            TokenTree::Token(token) => {
                match token.kind {
                    TokenKind::Pound => {
                        // TODO
                        Err(ParseError::from(format!("Unsupported syntax #")))
                    }
                    _ => {Ok(vec![])}
                }
            }
            TokenTree::Delimited(_, _, _) => { Ok(vec![]) }
        }
    }

    fn parse_attributes(&mut self) -> PResult<Vec<()>> {
        match &self.token {
            TokenTree::Token(token) => {
                match token {
                    &_ => { Ok(vec![]) }
                }
            }
            TokenTree::Delimited(_, _, _) => { Ok(vec![]) }
        }
    }

    fn parse_item(&mut self) -> PResult<syntax::Item> {
        Ok(syntax::Item {
            id: 0,
            attrs: self.parse_attributes()?,
            visibility: (),
            kind: syntax::ItemKind::Struct(syntax::VariantData { fields: vec![] }),
            span: DUMMY_SPAN,
        })
    }

    fn parse_statement(&mut self) -> PResult<syntax::Statement> {
        Ok(syntax::Statement {
            kind: syntax::StatementKind::Expression,
        })
    }

    fn bump(&mut self) {
        self.prev_token = self.token.clone();
        let (token, token_spacing) = self.stream.next().unwrap();
        self.token = token;
        self.token_spacing = token_spacing;
    }
}

impl From<TokenStream> for Parser {
    fn from(stream: TokenStream) -> Self {
        Self {
            stream: stream.0.into_iter(),
            token: TokenTree::Token(Token::dummy()),
            prev_token: TokenTree::Token(Token::dummy()),
            token_spacing: Spacing::Alone,
        }
    }
}
