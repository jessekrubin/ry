type JsonSerOpt = u8;
const JSON_SER_FMT: JsonSerOpt = 1 << 0;
const JSON_SER_SORT_KEYS: JsonSerOpt = 1 << 1;
const JSON_SER_APPEND_NEWLINE: JsonSerOpt = 1 << 2;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct JsonOptions(JsonSerOpt);

impl JsonOptions {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub(crate) const fn with_sort_keys(self, sort_keys: bool) -> Self {
        if sort_keys {
            Self(self.0 | JSON_SER_SORT_KEYS)
        } else {
            self
        }
    }

    #[inline]
    pub(crate) const fn with_append_newline(self, append_newline: bool) -> Self {
        if append_newline {
            Self(self.0 | JSON_SER_APPEND_NEWLINE)
        } else {
            self
        }
    }

    #[inline]
    pub(crate) const fn with_fmt(self, fmt: bool) -> Self {
        if fmt {
            Self(self.0 | JSON_SER_FMT)
        } else {
            self
        }
    }

    #[inline]
    pub(crate) const fn fmt(self) -> bool {
        self.0 & JSON_SER_FMT != 0
    }

    #[inline]
    pub(crate) const fn sort_keys(self) -> bool {
        self.0 & JSON_SER_SORT_KEYS != 0
    }

    #[inline]
    pub(crate) const fn append_newline(self) -> bool {
        self.0 & JSON_SER_APPEND_NEWLINE != 0
    }
}
