use pyo3::exceptions::PyImportError;
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyDict;

static ORJSON_DUMPS: GILOnceCell<Py<PyAny>> = GILOnceCell::new();
// static ORJSON_OPT_APPEND_NEWLINE: GILOnceCell<Py<PyInt>> = GILOnceCell::new();
// static ORJSON_OPT_OPT_INDENT_2: GILOnceCell<Py<PyInt>> = GILOnceCell::new();

#[pyfunction(
    signature = (obj, **kwargs)
)]
pub fn oj_dumps<'py>(
    py: Python<'py>,
    obj: Bound<'py, PyAny>,
    kwargs: Option<&Bound<'py, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let dumps = ORJSON_DUMPS.import(py, "orjson", "dumps").map_err(|e| {
        PyImportError::new_err(
            "orson module not found; install with `pip install orjson` or `uv add orjson`",
        )
    })?;
    if let Some(kwargs) = kwargs {
        dumps.call((obj,), Some(kwargs))
    } else {
        dumps.call1((obj,))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(oj_dumps, m)?)?;
    Ok(())
}
