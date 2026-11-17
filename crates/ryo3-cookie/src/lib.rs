#![doc = include_str!("../README.md")]
mod ry_cookie;
mod types;
use pyo3::prelude::*;
pub use ry_cookie::RyCookie;
pub use types::{PyCookieExpiration, PyCookieSameSite};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyCookie>()?;
    Ok(())
}
