pub mod traits;

pub type ByteOffset = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceOffset(pub ByteOffset);

impl SourceOffset {
    /// Actual byte offset.
    pub const fn offset(&self) -> ByteOffset {
        self.0
    }

    /// Little utility to help convert 1-based line/column locations into
    /// miette-compatible Spans
    ///
    /// This function is infallible: Giving an out-of-range line/column pair
    /// will return the offset of the last byte in the source.
    pub fn from_location(source: impl AsRef<str>, loc_line: usize, loc_col: usize) -> Self {
        let mut line = 0usize;
        let mut col = 0usize;
        let mut offset = 0usize;
        for char in source.as_ref().chars() {
            if line + 1 >= loc_line && col + 1 >= loc_col {
                break;
            }
            if char == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
            offset += char.len_utf8();
        }

        SourceOffset(offset)
    }
}

/// Span within a [`SourceCode`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceSpan {
    /// The start of the span.
    offset: SourceOffset,
    /// The total length of the span
    length: usize,
}

impl From<ByteOffset> for SourceOffset {
    fn from(bytes: ByteOffset) -> Self {
        SourceOffset(bytes)
    }
}

impl From<std::ops::Range<ByteOffset>> for SourceSpan {
    fn from(range: std::ops::Range<ByteOffset>) -> Self {
        Self {
            offset: range.start.into(),
            length: range.len(),
        }
    }
}

impl SourceSpan {
    /// Create a new [`SourceSpan`].
    pub const fn new(start: SourceOffset, length: usize) -> Self {
        Self {
            offset: start,
            length,
        }
    }

    /// The absolute offset, in bytes, from the beginning of a [`SourceCode`].
    pub const fn offset(&self) -> usize {
        self.offset.offset()
    }

    /// Total length of the [`SourceSpan`], in bytes.
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Whether this [`SourceSpan`] has a length of zero. It may still be useful
    /// to point to a specific point.
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }
}
