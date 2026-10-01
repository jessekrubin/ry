//! Formats for JSON serialization.
#![expect(clippy::inline_always, reason = "perf")]

/// Controls how JSON output is formatted.
pub trait JsonFormat: Sized {
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
pub(crate) struct JsonFormatCompact;

impl JsonFormat for JsonFormatCompact {
    #[inline(always)]
    fn inc(&mut self) {}

    #[inline(always)]
    fn dec(&mut self) {}

    #[inline(always)]
    fn sep(&self, _: &mut Vec<u8>) {}

    #[inline(always)]
    fn indent(&self, _: &mut Vec<u8>) {}
}

pub(crate) struct JsonFormatPretty<const INDENT: usize = 2> {
    depth: u32,
}

impl<const INDENT: usize> JsonFormatPretty<INDENT> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self { depth: 0 }
    }
}

impl<const INDENT: usize> JsonFormat for JsonFormatPretty<INDENT> {
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
        let needed_space = (self.depth as usize) * INDENT;
        output.reserve(needed_space + 1);
        output.push(b'\n');
        for _ in 0..(needed_space) {
            output.push(b' ');
        }
    }
}
