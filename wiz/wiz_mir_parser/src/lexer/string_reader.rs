pub struct StringReader<'a> {
    /// Start reading src at this position
    pub(crate) start_position: usize,
    /// Current reading position
    pub(crate) position: usize,
    /// Stop reading src at this position
    pub(crate) end_src_index: usize,
    /// Target src
    pub(crate) src: &'a str,
}
