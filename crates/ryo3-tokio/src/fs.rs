//! python `tokio::fs` module

use pyo3::exceptions::PyNotADirectoryError;
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;
use ryo3_bytes::{extract_bytes_ref_str, PyBytes};
use std::path::{Path, PathBuf};

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn read_async<'py>(py: Python<'py>, pth: PathBuf) -> PyResult<Bound<'py, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let fbytes = tokio::fs::read(pth)
            .await
            .map(ryo3_bytes::PyBytes::from)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("read_async - {:?}", e))
            });
        fbytes
        // let b = ryo3_bytes::PyBytes::from(fbytes);
        // b
    })
}
#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_async<'py>(
    py: Python<'py>,
    fspath: PathBuf,
    // b: &Bound<'py, PyAny>,
    b: PyBytes,
) -> PyResult<Bound<'py, PyAny>> {
    // let bref = extract_bytes_ref_str(b)?;
    // let write_res = std::fs::write(fspath.as_ref(), bref);
    // match write_res {
    //     Ok(()) => Ok(bref.len()),
    //     Err(e) => Err(PyNotADirectoryError::new_err(format!(
    //         "write_bytes - parent: {fspath} - {e}"
    //     ))),
    // }

    // let bref = extract_bytes_ref_str(b)?;
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let bref: &[u8] = b.as_ref();
        let len = bref.len();
        let write_res = tokio::fs::write(fspath, b).await.map(|_| len).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("write_async - {:?}", e))
        });
        // write_res.map(|len| len.into_py(py))
        Ok(())

        // py.None().into_py_any(py)
    })
}
