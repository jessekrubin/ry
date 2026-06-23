use std::ops::{BitAnd, Deref};

const CASE_INSENSITIVE: u8 = 1;
const LITERAL_SEPARATOR: u8 = 1 << 1;
const BACKSLASH_ESCAPE: u8 = 1 << 2;
pub(crate) const DEFAULT_BACKSLASH_ESCAPE: bool = cfg!(windows);

const DEFAULT_OPTIONS: GlobOptions = GlobOptions(0)
    .case_insensitive(false)
    .literal_separator(false)
    .backslash_escape(DEFAULT_BACKSLASH_ESCAPE);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GlobOptions(u8);

impl BitAnd for GlobOptions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Deref for GlobOptions {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GlobOptions {
    pub(crate) const fn new() -> Self {
        DEFAULT_OPTIONS
    }

    pub(crate) const fn case_insensitive(mut self, value: bool) -> Self {
        if value {
            self.0 |= CASE_INSENSITIVE;
        } else {
            self.0 &= !CASE_INSENSITIVE;
        }
        self
    }

    pub(crate) const fn literal_separator(mut self, value: bool) -> Self {
        if value {
            self.0 |= LITERAL_SEPARATOR;
        } else {
            self.0 &= !LITERAL_SEPARATOR;
        }
        self
    }

    pub(crate) const fn backslash_escape(mut self, value: bool) -> Self {
        if value {
            self.0 |= BACKSLASH_ESCAPE;
        } else {
            self.0 &= !BACKSLASH_ESCAPE;
        }
        self
    }

    pub(crate) const fn is_case_insensitive(self) -> bool {
        self.0 & CASE_INSENSITIVE != 0
    }

    pub(crate) const fn is_literal_separator(self) -> bool {
        self.0 & LITERAL_SEPARATOR != 0
    }

    pub(crate) const fn is_backslash_escape(self) -> bool {
        self.0 & BACKSLASH_ESCAPE != 0
    }

    pub(crate) fn build(self, pattern: &str) -> Result<globset::Glob, globset::Error> {
        globset::GlobBuilder::new(pattern)
            .case_insensitive(self.is_case_insensitive())
            .literal_separator(self.is_literal_separator())
            .backslash_escape(self.is_backslash_escape())
            .build()
    }
}
