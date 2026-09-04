//! Formats for JSON serialization.

use std::io::Write;

use crate::experimental::ser::{Error, Result};

/// Controls how JSON output is formatted.
pub trait Format: Sized {
    #[doc(hidden)]
    fn inc(&mut self);

    #[doc(hidden)]
    fn dec(&mut self);

    #[doc(hidden)]
    fn sep(&self, s: &mut impl Write) -> Result<()>;

    #[doc(hidden)]
    fn indent(&self, s: &mut impl Write) -> Result<()>;
}

/// Compact format for JSON.
pub(super) struct Compact;

impl Format for Compact {
    #[inline(always)]
    fn inc(&mut self) {}

    #[inline(always)]
    fn dec(&mut self) {}

    #[inline(always)]
    fn sep(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn indent(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

/// Pretty printing format for JSON.
pub(super) struct Pretty<'a> {
    indent: &'a str,
    depth: usize,
}

impl<'a> Pretty<'a> {
    /// Creates a pretty printing format with default 2 spaces for indentation.
    #[inline]
    pub(super) fn new() -> Self {
        Self::with_indent("  ")
    }

    /// Creates a pretty printing format with the given indentation.
    #[inline]
    fn with_indent(s: &'a str) -> Self {
        Self {
            indent: s,
            depth: 0,
        }
    }
}

impl Format for Pretty<'_> {
    #[inline(always)]
    fn inc(&mut self) {
        self.depth += 1
    }

    #[inline(always)]
    fn dec(&mut self) {
        self.depth -= 1
    }

    #[inline(always)]
    fn sep(&self, s: &mut impl Write) -> Result<()> {
        match s.write_all(b" ") {
            Ok(_) => Ok(()),
            _ => Err(Error::io()),
        }
    }

    #[inline(always)]
    fn indent(&self, s: &mut impl Write) -> Result<()> {
        if s.write_all(b"\n").is_err() {
            return Err(Error::io());
        }

        for _ in 0..self.depth {
            if s.write_all(self.indent.as_bytes()).is_err() {
                return Err(Error::io());
            }
        }

        Ok(())
    }
}
