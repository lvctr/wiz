use crate::error::PResult;
use wiz_mir_syntax::span::DUMMY_SPAN;
use wiz_mir_syntax::syntax;
use wiz_mir_syntax::token::{Spacing, Token};

struct Parser {
    pub token: Token,
    pub prev_token: Token,
    pub token_spacing: Spacing,
    reach_eof: bool,
}

impl Parser {
    pub fn parse(&mut self) -> PResult<syntax::File> {
        let start = self.token.span.clone();
        let mut items = vec![];
        while self.reach_eof {
            items.push(self.parse_item()?);
        }
        Ok(syntax::File {
            attrs: vec![],
            items,
            span: start.to(&self.token.span),
        })
    }

    fn parse_item(&mut self) -> PResult<syntax::Item> {
        Ok(syntax::Item {
            id: 0,
            attrs: vec![],
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
}
