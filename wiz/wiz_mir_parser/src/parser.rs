use wiz_mir_syntax::span::DUMMY_SPAN;
use wiz_mir_syntax::token::{Spacing, Token};
use wiz_mir_syntax::syntax;
use crate::error::PResult;

struct Parser {
    pub token: Token,
    pub prev_token: Token,
    pub token_spacing: Spacing,
}

impl Parser {
    pub fn parse(&mut self) -> PResult<syntax::File> {
        Ok(syntax::File {
            attrs: vec![],
            items: vec![],
            span: DUMMY_SPAN
        })
    }
}
