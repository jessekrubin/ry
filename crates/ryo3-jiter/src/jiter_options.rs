use jiter::{FloatMode, PartialMode, PythonParse, StringCacheMode};

#[derive(Debug, Clone, Copy)]
pub struct JiterParseOptions {
    pub allow_inf_nan: bool,
    pub cache_mode: StringCacheMode,
    pub partial_mode: PartialMode,
    pub catch_duplicate_keys: bool,
    pub float_mode: FloatMode,
}

impl Default for JiterParseOptions {
    fn default() -> Self {
        JiterParseOptions {
            allow_inf_nan: false,
            cache_mode: StringCacheMode::All,
            partial_mode: PartialMode::Off,
            catch_duplicate_keys: false,
            float_mode: FloatMode::Float,
        }
    }
}

impl Into<PythonParse> for JiterParseOptions {
    fn into(self) -> PythonParse {
        PythonParse {
            allow_inf_nan: self.allow_inf_nan,
            cache_mode: self.cache_mode,
            partial_mode: self.partial_mode,
            catch_duplicate_keys: self.catch_duplicate_keys,
            float_mode: self.float_mode,
        }
    }
}

impl JiterParseOptions {
    pub fn build_python_parse(&self) -> PythonParse {
        PythonParse {
            allow_inf_nan: self.allow_inf_nan,
            cache_mode: self.cache_mode,
            partial_mode: self.partial_mode,
            catch_duplicate_keys: self.catch_duplicate_keys,
            float_mode: self.float_mode,
        }
    }
}
