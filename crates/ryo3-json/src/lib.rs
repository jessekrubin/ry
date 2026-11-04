#![doc = include_str!("../README.md")]

pub mod orjson;
mod ser_opts;
mod serialize;
mod transcode;

use pyo3::prelude::*;
pub use ryo3_json_ser as ser;
pub use serialize::{PyJsonDtype, dumps, py_to_vec, stringify};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(orjson::orjson_default, m)?)?;
    m.add_function(wrap_pyfunction!(serialize::stringify, m)?)?;
    m.add_class::<RyJson>()?;
    Ok(())
}

/// `ry.JSON` namespace ~ a faux submodule
///
/// This was a submodule but it was weird and I dont really like the
/// submodule pattern w/ pyo3
///
/// Functions are `#[classattr]`s in the class dict, which has the
/// benefit of not having to copy duplicate signatures around
#[pyclass(name = "JSON", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Copy)]
pub struct RyJson;

#[pymethods]
impl RyJson {
    #[classattr]
    #[pyo3(name = "JSONDecodeError")]
    fn json_decode_error(py: Python<'_>) -> Py<pyo3::types::PyType> {
        py.get_type::<ryo3_jiter::RyJSONDecodeError>().unbind()
    }

    #[classattr]
    fn parse(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(ryo3_jiter::parse, py)?.into_any().unbind())
    }

    #[classattr]
    fn loads(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(ryo3_jiter::loads, py)?.into_any().unbind())
    }

    #[classattr]
    fn stringify(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(serialize::stringify, py)?
            .into_any()
            .unbind())
    }

    #[classattr]
    fn dumps(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(serialize::dumps, py)?.into_any().unbind())
    }

    #[classattr]
    fn minify(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(transcode::minify, py)?.into_any().unbind())
    }

    #[classattr]
    fn fmt(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(transcode::fmt, py)?.into_any().unbind())
    }

    #[classattr]
    fn cache_clear(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(ryo3_jiter::cache_clear, py)?
            .into_any()
            .unbind())
    }

    #[classattr]
    fn cache_usage(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(ryo3_jiter::cache_usage, py)?
            .into_any()
            .unbind())
    }
}
