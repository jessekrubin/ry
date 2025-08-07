use pyo3::exceptions::PyImportError;
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyDict;

static ORJSON_DUMPS: GILOnceCell<Py<PyAny>> = GILOnceCell::new();
static ORJSON_FRAGMENT: GILOnceCell<Py<PyAny>> = GILOnceCell::new();
// static ORJSON_OPT_APPEND_NEWLINE: GILOnceCell<Py<PyInt>> = GILOnceCell::new();
// static ORJSON_OPT_OPT_INDENT_2: GILOnceCell<Py<PyInt>> = GILOnceCell::new();

#[pyfunction(
    signature = (obj, **kwargs)
)]
pub fn dumps<'py>(
    py: Python<'py>,
    obj: Bound<'py, PyAny>,
    kwargs: Option<&Bound<'py, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let dumps = ORJSON_DUMPS.import(py, "orjson", "dumps").map_err(|e| {
        let emsg = format!(
            "`orjson` not found/importable; install w/ `pip install orjson` or `uv add orjson` ~ ERR: {e}",
        );
        PyImportError::new_err(emsg)
    })?;
    if let Some(kwargs) = kwargs {
        dumps.call((obj,), Some(kwargs))
    } else {
        dumps.call1((obj,))
    }
}

pub fn fragment<'py>(py: Python<'py>, obj: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
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
    crate::serialize::stringify(py, obj, None, false, false, false, true)
        .map_err(|e| PyImportError::new_err(format!("Failed to serialize with orjson: {e}")))
        .and_then(|v| fragment(py, v))
}

#[pymodule(gil_used = false, name = "oj", submodule, module = "ry.ryo3")]
pub fn oj(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}
