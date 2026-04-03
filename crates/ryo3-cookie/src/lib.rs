#![doc = include_str!("../README.md")]
mod py_cookie;
mod types;
pub use py_cookie::PyCookie;
use pyo3::prelude::*;
pub use types::{PyCookieExpiration, PyCookieSameSite};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCookie>()?;
    Ok(())
}
