#[derive(Debug, Clone, Copy)]
pub struct JiterParseOptions {
    pub allow_inf_nan: bool,
    pub cache_mode: ::jiter::StringCacheMode,
    pub partial_mode: ::jiter::PartialMode,
    pub catch_duplicate_keys: bool,
}

impl JiterParseOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_inf_nan: false,
            cache_mode: ::jiter::StringCacheMode::All,
            partial_mode: ::jiter::PartialMode::Off,
            catch_duplicate_keys: false,
        }
    }

    #[must_use]
    pub const fn with_allow_inf_nan(mut self, allow_inf_nan: bool) -> Self {
        self.allow_inf_nan = allow_inf_nan;
        self
    }

    #[must_use]
    pub const fn with_cache_mode(mut self, cache_mode: ::jiter::StringCacheMode) -> Self {
        self.cache_mode = cache_mode;
        self
    }

    #[must_use]
    pub const fn with_partial_mode(mut self, partial_mode: ::jiter::PartialMode) -> Self {
        self.partial_mode = partial_mode;
        self
    }

    #[must_use]
    pub const fn with_catch_duplicate_keys(mut self, catch_duplicate_keys: bool) -> Self {
        self.catch_duplicate_keys = catch_duplicate_keys;
        self
    }
}

impl Default for JiterParseOptions {
    fn default() -> Self {
        Self::new()
    }
}
