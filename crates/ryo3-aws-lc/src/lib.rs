#![doc = include_str!("../README.md")]
mod digest;

use aws_lc_rs::digest::{Context, Digest, SHA256, SHA512};
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    digest::pymod_add(m)?;
    Ok(())
}
