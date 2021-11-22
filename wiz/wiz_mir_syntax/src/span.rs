pub const DUMMY_SPAN: Span = Span {
    index: 0,
    length: 0,
};

/// Token position and length in source.
pub struct Span {
    /// Token index.
    index: usize,
    /// Token length
    length: usize,
}
