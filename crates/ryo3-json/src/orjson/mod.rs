use pyo3::exceptions::PyImportError;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;

use crate::serialize::{self, PyJsonDtype};

static ORJSON_FRAGMENT: PyOnceLock<Py<PyAny>> = PyOnceLock::new();

fn orjson_fragment<'py>(py: Python<'py>, obj: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    let fragment = ORJSON_FRAGMENT.import(py, "orjson", "Fragment").map_err(|e| {
        let emsg = format!(
            "`orjson` not found/importable; install w/ `pip install orjson` or `uv add orjson` ~ ERR: {e}",
        );
        PyImportError::new_err(emsg)
    })?;
    fragment.call1((obj,))
}

/// Function to be used as/with `orjson.dumps(obj, default=orjson_default)`
#[pyfunction]
pub fn orjson_default<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    // serialize (MAKING SURE IT IS A PYBYTES) and make an `orjson.Fragment`
    serialize::stringify_impl(py, obj, None, false, false, false, PyJsonDtype::PyBytes)
        .and_then(|v| orjson_fragment(py, v))
}
