//! The `Span` type which represents a span of text.

/// A span of text in a source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

    /// Creates a span from a start and an exclusive end offset.
    pub const fn from_range(start: usize, end: usize) -> Self {
        Self {
            start,
            length: end - start,
        }
    }

    pub const fn start(&self) -> usize {
        self.start
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

    /// The smallest span that covers both `self` and `other`.
    pub const fn to(self, other: Span) -> Span {
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };
        let end = if self.end() > other.end() {
            self.end()
        } else {
            other.end()
        };
        Span {
            start,
            length: end - start,
        }
    }
}
