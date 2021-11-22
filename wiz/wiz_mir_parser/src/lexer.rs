pub mod token_trees;
mod string_reader;
mod token_stream_builder;

use crate::error::PResult;
use wiz_mir_syntax::token::TokenStream;
use crate::lexer::string_reader::StringReader;
use crate::lexer::token_trees::TokenTreeReader;

pub fn parse_token_trees(src: &str, start_position:usize, ) -> PResult<TokenStream> {
    let tt_reader = TokenTreeReader::from(StringReader {
        start_position,
        position: start_position,
        end_src_index: src.len(),
        src
    });
    Ok(TokenStream::default())
}
