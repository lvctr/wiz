pub mod error;
pub mod lexer;
pub mod parser;

pub fn maybe_file_to_parser() {}

pub fn maybe_file_to_stream() {}

pub fn stream_to_parser() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
