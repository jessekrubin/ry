use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[pyclass(name = "AsyncFile", module = "ry")]
pub struct PyAsyncFile {
    inner: std::sync::Arc<Mutex<tokio::fs::File>>,
}

#[pymethods]
impl PyAsyncFile {
    fn __aenter__(slf: Py<Self>, py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move { Ok(slf) })
    }

    #[pyo3(name = "__aexit__")]
    pub fn __aexit__<'py>(
        slf: PyRef<Self>,
        py: Python<'py>,
        _exc_type: PyObject,
        _exc_value: PyObject,
        _traceback: PyObject,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&slf.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut file = inner.lock().await;
            file.flush().await.map_err(PyErr::from)?;
            Ok(())
        })
    }

    #[pyo3(
        signature = (size = None),
    )]
    pub fn read<'py>(
        &mut self,
        py: Python<'py>,
        size: Option<usize>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut file = inner.lock().await;
            match size {
                Some(s) => {
                    let mut buf = vec![0; s];
                    file.read_exact(&mut buf).await.map_err(PyErr::from)?;
                    let rybytes = ryo3_bytes::PyBytes::from(buf);
                    return Ok(rybytes);
                }
                None => {
                    let mut buf = vec![];
                    file.read_to_end(&mut buf).await.map_err(PyErr::from)?;
                    let rybytes = ryo3_bytes::PyBytes::from(buf);
                    return Ok(rybytes);
                }
            }
        })
    }

    pub fn write<'py>(
        &mut self,
        py: Python<'py>,
        data: ryo3_bytes::PyBytes,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        let buf: &[u8] = data.as_ref();
        let vec = buf.to_vec();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut file = inner.lock().await;
            file.write_all(&vec).await.map_err(PyErr::from)?;
            Ok(())
        })
    }

    pub fn close<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut f = inner.lock().await;
            f.flush().await.map_err(PyErr::from)?;
            Ok(())
        })
    }
    // pub fn read<'py>(&mut self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
    //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
    //         let mut buf = vec![0; size];
    //         self.inner
    //             .read_exact(&mut buf)
    //             .await
    //             .map_err(|e| e.to_string())?;
    //         Ok(PyBytes::new(py, &buf))
    //     })
    // }

    // pub fn write<'py>(
    //     &mut self,
    //     py: Python<'py>,
    //     data: ryo3_bytes::PyBytes,
    // ) -> PyResult<Bound<'py, PyAny>> {
    //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
    //         let bref: &[u8] = data.as_ref();
    //         self.inner
    //             .write_all(bref)
    //             .await
    //             .map_err(PyErr::from)?;
    //         Ok(())
    //     })

    // }

    // pub fn close(&mut self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
    //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
    //         self.inner.flush().await.map_err(PyErr::from)?;
    //         Ok(())
    //     })
    // }
}

#[pyfunction(
    signature = (path, mode = None),
)]
pub fn aiopen<'py>(
    py: Python<'py>,
    path: String,
    mode: Option<String>,
) -> PyResult<Bound<'py, PyAny>> {
    // let file = aiopen_inner(path, mode);

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let mode = mode.unwrap_or("r".to_string());
        let mod_ref = mode.as_str();
        let file = match mod_ref {
            "r" => File::open(path).await,
            "w" => File::create(path).await,
            "a" => tokio::fs::OpenOptions::new().append(true).open(path).await,
            "r+" => {
                tokio::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(path)
                    .await
            }
            "w+" => {
                tokio::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .await
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unsupported mode: {mode}"
                )))
            }
        }
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(PyAsyncFile {
            inner: Arc::new(Mutex::new(file)),
        })
        // let file = aiopen_inner(path, mode).await?;
        // Ok(file)
    })
}

pub async fn aiopen_inner<'py>(path: &str, mode: Option<&str>) -> PyResult<PyAsyncFile> {
    let mode = mode.unwrap_or("r");
    let file = match mode {
        "r" => File::open(path).await,
        "w" => File::create(path).await,
        "a" => tokio::fs::OpenOptions::new().append(true).open(path).await,
        "r+" => {
            tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .await
        }
        "w+" => {
            tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .await
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Unsupported mode: {mode}"
            )))
        }
    }
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    Ok(PyAsyncFile {
        inner: Arc::new(Mutex::new(file)),
    })
}
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAsyncFile>()?;
    m.add_function(wrap_pyfunction!(aiopen, m)?)?;
    Ok(())
}
