const CASE_INSENSITIVE: u8 = 1;
const LITERAL_SEPARATOR: u8 = 1 << 1;
const BACKSLASH_ESCAPE: u8 = 1 << 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GlobOptions(u8);

impl GlobOptions {
    pub(crate) fn new() -> Self {
        Self(0)
    }

    pub(crate) fn case_insensitive(mut self, value: bool) -> Self {
        if value {
            self.0 |= CASE_INSENSITIVE;
        } else {
            self.0 &= !CASE_INSENSITIVE;
        }
        self
    }

    pub(crate) fn literal_separator(mut self, value: bool) -> Self {
        if value {
            self.0 |= LITERAL_SEPARATOR;
        } else {
            self.0 &= !LITERAL_SEPARATOR;
        }
        self
    }

    pub(crate) fn backslash_escape(mut self, value: bool) -> Self {
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

    pub(crate) fn is_literal_separator(self) -> bool {
        self.0 & LITERAL_SEPARATOR != 0
    }

    pub(crate) fn is_backslash_escape(self) -> bool {
        self.0 & BACKSLASH_ESCAPE != 0
    }
}
