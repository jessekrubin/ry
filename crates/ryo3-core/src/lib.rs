mod errors;
pub mod pystring;
pub mod static_str;
pub mod types;

pub use errors::FeatureNotEnabledError;

#[inline]
#[must_use]
pub fn py_bool2str(val: bool) -> &'static str {
    if val {
        static_str::PY_TRUE
    } else {
        static_str::PY_FALSE
    }
}
