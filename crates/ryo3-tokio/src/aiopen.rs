use crate::fs::PyDirEntryAsync;
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;

use pyo3::types::PyDict;
use pyo3::IntoPyObjectExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncSeekExt;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy)]
struct OpenOptions {
    append: bool,
    create: bool,
    create_new: bool,
    read: bool,
    truncate: bool,
    write: bool,
}

struct PyAsyncFileInner {
    file: Option<BufReader<File>>,
    path: PathBuf,
    open_options: OpenOptions,
}

impl PyAsyncFileInner {
    async fn open(&mut self) -> PyResult<()> {
        let opts = &self.open_options;
        let mut open_opts = tokio::fs::OpenOptions::new();
        opts.apply_to(&mut open_opts);

        // if opts.read {
        //     open_opts.read(true);
        // }
        // if opts.write {
        //     open_opts.write(false);
        // }
        // if opts.append {
        //     open_opts.append(true);
        // }
        // if opts.create {
        //     open_opts.create(true);
        // }
        // if opts.truncate {
        //     open_opts.truncate(true);
        // }
        let file_res = open_opts.open(&self.path).await;
        println!("res {:?}", file_res);
        let file = file_res.map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!(
                "Failed to open file {}: {}",
                self.path.display(),
                e
            ))
        })?;
        println!("Opened file: {:?}", file);
        self.file = Some(BufReader::new(file));
        Ok(())
    }

    async fn close(&mut self) -> PyResult<()> {
        let mut file = self
            .file
            .take()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
        file.flush()
            .await
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(())
    }

    async fn read_all(&mut self) -> PyResult<Vec<u8>> {
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .await
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(buf)
    }

    async fn read(&mut self, buf: &mut [u8]) -> PyResult<usize> {
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
        file.read(buf)
            .await
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    async fn readline(&mut self) -> PyResult<Option<Vec<u8>>> {
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
        let mut buf = Vec::new();
        let bytes_read = file.read_until(b'\n', &mut buf).await.map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to read line: {}", e))
        })?;
        if bytes_read == 0 {
            Ok(None)
        } else {
            Ok(Some(buf))
        }
    }

    async fn write(&mut self, buf: &[u8]) -> PyResult<()> {
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
        file.write_all(buf)
            .await
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
}

#[pyclass(name = "AsyncFile", module = "ry", frozen)]
pub struct PyAsyncFile {
    inner: Arc<Mutex<Option<PyAsyncFileInner>>>,
    // path: PathBuf,
    // open_options: OpenOptions,
}

#[pymethods]
impl PyAsyncFile {
    #[new]
    pub fn py_new(p: PathBuf, mode: Option<&str>) -> PyResult<Self> {
        let path = PathBuf::from(p);

        let mode = mode.unwrap_or("r");
        let open_options = OpenOptions::from_mode_string(&mode)?;
        let inner = PyAsyncFileInner {
            file: None,
            path,
            open_options,
        };
        Ok(PyAsyncFile {
            inner: Arc::new(Mutex::new(Some(inner))),
        })
    }

    fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut locked = inner.lock().await;

            let inner_ref = locked
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;

            inner_ref.open().await?;
            Ok(())
        })
    }

    // fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    //     let inner = Arc::clone(&self.inner);
    //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
    //         let mut file = inner.lock().await;
    //         if file.is_none() {
    //             let f = File::open(file.as_ref()).await.map_err(PyErr::from)?;
    //
    //
    //
    //         }
    //     })
    // }

    fn __aenter__(slf: Py<Self>, py: Python) -> PyResult<Bound<PyAny>> {
        // call the open method and replace the file
        // // get the self ref...
        // let pyf = slf.get();
        // let inner = Arc::clone(&pyf.inner);

        let inner = Arc::clone(&slf.borrow(py).inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut locked = inner.lock().await;

            let inner_ref = locked
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;

            inner_ref.open().await?;

            Ok(slf)
        })
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let inner_ref = locked
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;
            inner_ref.close().await?;
            Ok(())
        })
    }

    #[pyo3(name = "__aexit__")]
    #[expect(clippy::needless_pass_by_value)]
    pub fn __aexit__<'py>(
        slf: PyRef<Self>,
        py: Python<'py>,
        _exc_type: PyObject,
        _exc_value: PyObject,
        _traceback: PyObject,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&slf.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let inner_ref = locked
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;
            let mut inner_file = inner_ref
                .file
                .take()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;

            inner_file
                .flush()
                .await
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

            inner_ref.file = None;
            Ok(())
        })
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let inner_ref = locked
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;
            let line = inner_ref.readline().await?;
            match line {
                Some(line) => Ok(line),
                None => Err(PyStopAsyncIteration::new_err("End of stream")),
            }
        })
    }

    // fn __aiter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    //
    // }
    //     let inner = Arc::clone(&self.inner);
    //
    //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
    //         let mut locked = inner.lock().await;
    //         let inner_ref = locked
    //             .as_mut()
    //             .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
    //
    //         let file = inner_ref
    //             .file
    //             .as_ref()
    //             .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
    //
    //         // Clone file handle to avoid consuming the original one
    //         let file = file.try_clone().await.map_err(|e| {
    //             pyo3::exceptions::PyIOError::new_err(format!("Failed to clone file: {}", e))
    //         })?;
    //
    //         let reader = tokio::io::BufReader::new(file);
    //         let lines = reader.lines();
    //
    //         Ok(PyAsyncLinesReader {
    //             lines: Arc::new(Mutex::new(Some(lines))),
    //         })
    //     })
    // }

    fn write<'py>(
        &self,
        py: Python<'py>,
        data: ryo3_bytes::PyBytes,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let inner_ref = locked
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;
            inner_ref.write(data.as_ref()).await?;
            Ok(())
        })
    }

    #[pyo3(
        signature = (size = None),
    )]
    pub fn read<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut file = inner.lock().await;
            let locked = file
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("File not opened"))?;
            if let Some(s) = size {
                let mut buf = vec![0; s];
                println!("buf size {:?}", buf.len());
                locked.read(&mut buf).await.map_err(PyErr::from)?;
                let rybytes = ryo3_bytes::PyBytes::from(buf);
                Ok(rybytes)
            } else {
                // println!( "buf NULL {:?}", buf.len());
                let r = locked.read_all().await.map_err(PyErr::from)?;
                println!("bytes {:?}", r);
                let rybytes = ryo3_bytes::PyBytes::from(r);
                Ok(rybytes)
            }
        })
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyAsyncLinesReader {
    lines: Arc<Mutex<Option<tokio::io::Lines<tokio::io::BufReader<File>>>>>,
}

#[pymethods]
impl PyAsyncLinesReader {
    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.lines.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            let lines = guard
                .as_mut()
                .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;

            let getline = lines.next_line().await.map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to read line: {}", e))
            })?;
            match getline {
                Some(line) => Ok(line),
                None => Err(PyStopAsyncIteration::new_err("End of stream")),
            }
        })
    }
}

impl OpenOptions {
    pub fn from_mode_string(mode: &str) -> PyResult<Self> {
        use pyo3::exceptions::PyValueError;

        let mut opts = OpenOptions {
            read: false,
            write: false,
            append: false,
            create: false,
            truncate: false,
            create_new: false,
        };

        match mode {
            "r" => {
                opts.read = true;
            }
            "r+" => {
                opts.read = true;
                opts.write = true;
            }
            "w" => {
                opts.write = true;
                opts.create = true;
                opts.truncate = true;
            }
            "w+" => {
                opts.read = true;
                opts.write = true;
                opts.create = true;
                opts.truncate = true;
            }
            "a" => {
                opts.write = true;
                opts.append = true;
                opts.create = true;
            }
            "a+" => {
                opts.read = true;
                opts.write = true;
                opts.append = true;
                opts.create = true;
            }
            "x" => {
                opts.write = true;
                opts.create_new = true;
            }
            "x+" => {
                opts.read = true;
                opts.write = true;
                opts.create_new = true;
            }
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported open mode: {:?}",
                    mode
                )))
            }
        }

        Ok(opts)
    }

    pub fn apply_to(self, open: &mut tokio::fs::OpenOptions) {
        open.read(self.read);
        open.write(self.write);
        open.append(self.append);
        open.create(self.create);
        open.truncate(self.truncate);
        open.create_new(self.create_new);
    }
}
#[pyfunction(
    signature = (path, mode = None, **kwargs),
)]
pub fn aiopen<'py>(
    py: Python<'py>,
    path: String,
    mode: Option<&str>,
    kwargs: Option<&Bound<'py, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    let path = PathBuf::from(path);
    let py_async_file = PyAsyncFile::py_new(path, mode)?;
    let pyany = py_async_file.into_bound_py_any(py);
    pyany
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAsyncFile>()?;
    m.add_function(wrap_pyfunction!(aiopen, m)?)?;
    Ok(())
}
