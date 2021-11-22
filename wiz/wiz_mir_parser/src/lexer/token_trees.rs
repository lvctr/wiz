use std::collections::HashMap;
use wiz_mir_syntax::span::Span;
use wiz_mir_syntax::token;
use wiz_mir_syntax::token::Token;
use crate::lexer::string_reader::StringReader;

pub struct UnmatchedBrace {
    pub expected_delim: token::DelimToken,
    pub found_delim: Option<token::DelimToken>,
    pub found_span: Span,
    pub unclosed_span: Option<Span>,
    pub candidate_span: Option<Span>,
}

pub struct TokenTreeReader<'a> {
    string_reader: StringReader<'a>,
    token: Token,
    /// Stack of open delimiters and their spans. Used for error message.
    open_braces: Vec<(token::DelimToken, Span)>,
    unmatched_braces: Vec<UnmatchedBrace>,
    /// The type and spans for all braces
    ///
    /// Used only for error recovery when arriving to EOF with mismatched braces.
    matching_delim_spans: Vec<(token::DelimToken, Span, Span)>,
    last_unclosed_found_span: Option<Span>,
    /// Collect empty block spans that might have been auto-inserted by editors.
    last_delim_empty_block_spans: HashMap<token::DelimToken, Span>,
    /// Collect the spans of braces (Open, Close). Used only
    /// for detecting if blocks are empty and only braces.
    matching_block_spans: Vec<(Span, Span)>,
}

impl<'a> From<StringReader<'a>> for TokenTreeReader<'a> {
    fn from(string_reader: StringReader<'a>) -> Self {
        Self {
            string_reader,
            token: Token::dummy(),
            open_braces: vec![],
            unmatched_braces: vec![],
            matching_delim_spans: vec![],
            last_unclosed_found_span: None,
            last_delim_empty_block_spans: Default::default(),
            matching_block_spans: vec![]
        }
    }
}