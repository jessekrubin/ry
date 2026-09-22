//! Formats for JSON serialization.
#![expect(clippy::inline_always, reason = "perf")]

/// Controls how JSON output is formatted.
pub trait Format: Sized {
    #[doc(hidden)]
    fn inc(&mut self);

    #[doc(hidden)]
    fn dec(&mut self);

    #[doc(hidden)]
    fn sep(&self, output: &mut Vec<u8>);

    #[doc(hidden)]
    fn indent(&self, output: &mut Vec<u8>);
}

/// Compact format for JSON.
pub(super) struct Compact;

impl Format for Compact {
    #[inline(always)]
    fn inc(&mut self) {}

    #[inline(always)]
    fn dec(&mut self) {}

    #[inline(always)]
    fn sep(&self, _: &mut Vec<u8>) {}

    #[inline(always)]
    fn indent(&self, _: &mut Vec<u8>) {}
}

/// Pretty printing format for JSON.
pub(super) struct Pretty<'a> {
    indent: &'a [u8],
    depth: usize,
}

impl<'a> Pretty<'a> {
    /// Creates a pretty printing format with default 2 spaces for indentation.
    #[inline]
    pub(super) fn new() -> Self {
        Self::with_indent(b"  ")
    }

    /// Creates a pretty printing format with the given indentation.
    #[inline]
    fn with_indent(s: &'a [u8]) -> Self {
        Self {
            indent: s,
            depth: 0,
        }
    }
}

impl Format for Pretty<'_> {
    #[inline(always)]
    fn inc(&mut self) {
        self.depth += 1;
    }

    #[inline(always)]
    fn dec(&mut self) {
        self.depth -= 1;
    }

    #[inline(always)]
    fn sep(&self, output: &mut Vec<u8>) {
        output.push(b' ');
    }

    #[inline(always)]
    fn indent(&self, output: &mut Vec<u8>) {
        let needed_space = self.depth * self.indent.len() + 1;
        output.reserve(needed_space);
        output.push(b'\n');
        for _ in 0..self.depth {
            output.extend_from_slice(self.indent);
        }
    }
}
