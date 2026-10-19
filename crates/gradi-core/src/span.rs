//! The `Span` type which represents a span of text.

/// A span of text in a source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The start of the span
    start: usize,
    /// The length or amount of bytes in the span
    length: usize,
}

impl Span {
    pub const fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }
    pub const fn len(&self) -> usize {
        self.length
    }

    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub const fn end(&self) -> usize {
        self.start + self.length
    }
}
