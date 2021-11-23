pub struct StringReader<'a> {
    /// Start reading src at this position
    start_position: usize,
    /// Current reading position
    position: usize,
    /// Stop reading src at this position
    end_src_index: usize,
    /// Target src
    src: &'a str,
}

impl<'a> StringReader<'a> {
    pub fn new(src: &'a str, start_position: usize, end_src_index: usize) -> Self {
        Self {
            start_position,
            position: start_position,
            end_src_index,
            src,
        }
    }
}
