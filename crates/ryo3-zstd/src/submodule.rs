//! DEPRECATED `ry.zstd` submodule ~ remove in v0.0.109
//!
//! functions warn via `#[pyo3(warn(...))]`; the old constants are served by a
//! module `__getattr__` (only called for names missing from the module dict)
//! so reading them warns too
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyAttributeError, PyDeprecationWarning};
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use zstd::zstd_safe;

use crate::compression_level::PyCompressionLevel;
use crate::oneshot;

#[pyfunction]
#[pyo3(
    signature = (data, level = PyCompressionLevel::default()),
    text_signature = "(data, level=3)",
    warn(
        message = "`ry.zstd.compress` is deprecated; use `ry.zstd_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
fn compress(py: Python<'_>, data: ReadableBuffer, level: PyCompressionLevel) -> PyResult<RyBytes> {
    oneshot::zstd_compress(py, data, level)
}

#[pyfunction]
#[pyo3(warn(
    message = "`ry.zstd.decompress` is deprecated; use `ry.zstd_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
fn decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    oneshot::zstd_decompress(py, data)
}

#[pyfunction]
#[pyo3(warn(
    message = "`ry.zstd.decode` is deprecated; use `ry.zstd_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
fn decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    oneshot::zstd_decompress(py, data)
}

#[pyfunction]
#[pyo3(warn(
    message = "`ry.zstd.unzstd` is deprecated; use `ry.zstd_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
fn unzstd(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    oneshot::zstd_decompress(py, data)
}

#[pyfunction]
#[pyo3(
    name = "is_zstd",
    warn(
        message = "`ry.zstd.is_zstd` is deprecated; use `ry.is_zstd` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
fn submod_is_zstd(data: ReadableBuffer) -> bool {
    oneshot::is_zstd(data)
}

/// old constant -> (value, replacement for the warning)
fn deprecated_constant(py: Python<'_>, name: &str) -> PyResult<Option<(Py<PyAny>, &'static str)>> {
    let found = match name {
        "__zstd_version__" => (
            zstd_safe::version_string().into_py_any(py)?,
            "`ry.ZSTD_VERSION`",
        ),
        "VERSION_MAJOR" => (
            zstd_safe::VERSION_MAJOR.into_py_any(py)?,
            "`ry.ZSTD_VERSION_INFO`",
        ),
        "VERSION_MINOR" => (
            zstd_safe::VERSION_MINOR.into_py_any(py)?,
            "`ry.ZSTD_VERSION_INFO`",
        ),
        "VERSION_RELEASE" => (
            zstd_safe::VERSION_RELEASE.into_py_any(py)?,
            "`ry.ZSTD_VERSION_INFO`",
        ),
        "VERSION_NUMBER" => (
            zstd_safe::VERSION_NUMBER.into_py_any(py)?,
            "`ry.ZSTD_VERSION_INFO`",
        ),
        "CLEVEL_DEFAULT" => (
            zstd_safe::CLEVEL_DEFAULT.into_py_any(py)?,
            "the `ry.zstd_compress` default (`level=3`)",
        ),
        "MAGICNUMBER" => (zstd_safe::MAGICNUMBER.into_py_any(py)?, "`ry.is_zstd`"),
        "MAGIC_DICTIONARY" => (zstd_safe::MAGIC_DICTIONARY.into_py_any(py)?, "nothing"),
        "MAGIC_SKIPPABLE_MASK" => (zstd_safe::MAGIC_SKIPPABLE_MASK.into_py_any(py)?, "nothing"),
        "MAGIC_SKIPPABLE_START" => (zstd_safe::MAGIC_SKIPPABLE_START.into_py_any(py)?, "nothing"),
        "BLOCKSIZELOG_MAX" => (zstd_safe::BLOCKSIZELOG_MAX.into_py_any(py)?, "nothing"),
        "BLOCKSIZE_MAX" => (zstd_safe::BLOCKSIZE_MAX.into_py_any(py)?, "nothing"),
        "CONTENTSIZE_ERROR" => (zstd_safe::CONTENTSIZE_ERROR.into_py_any(py)?, "nothing"),
        "CONTENTSIZE_UNKNOWN" => (zstd_safe::CONTENTSIZE_UNKNOWN.into_py_any(py)?, "nothing"),
        _ => return Ok(None),
    };
    Ok(Some(found))
}

#[pyfunction]
fn __getattr__(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    let Some((value, replacement)) = deprecated_constant(py, name)? else {
        return Err(PyAttributeError::new_err(format!(
            "module 'ry.zstd' has no attribute '{name}'"
        )));
    };
    let msg = if replacement == "nothing" {
        format!("`ry.zstd.{name}` is deprecated with no replacement [removal: v0.0.109]")
    } else {
        format!("`ry.zstd.{name}` is deprecated; use {replacement} instead [removal: v0.0.109]")
    };
    let category = py.get_type::<PyDeprecationWarning>();
    let c_msg = std::ffi::CString::new(msg)?;
    PyErr::warn(py, &category, &c_msg, 1)?;
    Ok(value)
}

pub fn pysubmod_register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compress, m)?)?;
    m.add_function(wrap_pyfunction!(decompress, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(unzstd, m)?)?;
    m.add_function(wrap_pyfunction!(submod_is_zstd, m)?)?;
    m.add_function(wrap_pyfunction!(__getattr__, m)?)?;
    Ok(())
}
