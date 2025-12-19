use crate::errors::map_reqwest_err;
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use futures_util::stream::{BoxStream, Fuse};
use pyo3::exceptions::{PyStopAsyncIteration, PyStopIteration};
use pyo3::{IntoPyObjectExt, prelude::*};
use reqwest::StatusCode;
use ryo3_bytes::PyBytes as RyBytes;
use std::sync::Arc;
use tokio::sync::Mutex;

// This whole response iterator was a difficult thing to figure out.
//
// NOTE: I (jesse) am pretty proud of this. I was struggling to get the
//       async-iterator thingy to work bc rust + async is quite hard, but
//       after lots and lots and lots of trial and error this works!
//
// clippy says this is too long and complicated to just sit in the struct def
//
// ---
//
// # UPDATE [2025-06-06] ~ Switch to using BoxStream:
//    In the obstore library, Kyle Barron (et al) use `BoxStream` and fuse the
//    stream which seems very smart.
//    REF: https://github.com/developmentseed/obstore/blob/50782ed782a15185a936d435d13ca0a7969154ae/obstore/src/get.rs#L219
//
// The inner stream type changes from:
// type OldStreamType = `Arc<Mutex<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>>>`
// to:
// type NewStreamType = `Arc<Mutex<Fuse<BoxStream<'static, Result<Bytes, reqwest::Error>>>>>`

type AsyncResponseStreamInner = Arc<Mutex<Fuse<BoxStream<'static, Result<Bytes, reqwest::Error>>>>>;

#[pyclass(name = "ResponseStream", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyResponseStream {
    status: StatusCode,
    pub(crate) stream: AsyncResponseStreamInner,
}

impl RyResponseStream {
    pub(crate) fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let bstream = response.bytes_stream();
        let stream: BoxStream<'static, Result<Bytes, reqwest::Error>> = Box::pin(bstream);

        let stream = Arc::new(Mutex::new(stream.fuse()));
        Self {
            status,
            stream: stream as AsyncResponseStreamInner,
        }
    }
}

impl std::fmt::Display for RyResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseStream<{}>", self.status)
    }
}

#[pymethods]
impl RyResponseStream {
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            match guard.next().await {
                Some(Ok(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Some(Err(e)) => Err(map_reqwest_err(e)),
                // I totally forgot that this was a thing and that I couldn't just return None
                None => Err(PyStopAsyncIteration::new_err("response-stream-end")),
            }
        })
    }

    #[pyo3(signature = (n=1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            let mut items = Vec::with_capacity(n);
            for _ in 0..n {
                match guard.next().await {
                    Some(Ok(bytes)) => items.push(RyBytes::from(bytes)),
                    Some(Err(e)) => return Err(map_reqwest_err(e)),
                    None => break,
                }
            }
            Ok(items)
        })
    }

    #[pyo3(signature = (*, join=false))]
    fn collect<'py>(&self, py: Python<'py>, join: bool) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        if join {
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let mut guard = stream.lock().await;
                let mut bytes_mut = BytesMut::new();
                while let Some(item) = guard.next().await {
                    match item {
                        Ok(bytes) => bytes_mut.extend_from_slice(&bytes),
                        Err(e) => return Err(map_reqwest_err(e)),
                    }
                }
                let bytes = bytes_mut.freeze();
                let py_bytes = RyBytes::from(bytes);
                Ok(py_bytes)
            })
        } else {
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let mut guard = stream.lock().await;
                let mut items = Vec::new();
                while let Some(item) = guard.next().await {
                    match item {
                        Ok(bytes) => items.push(RyBytes::from(bytes)),
                        Err(e) => return Err(map_reqwest_err(e)),
                    }
                }
                Ok(items)
            })
        }
    }
}

#[cfg(feature = "experimental-async")]
#[pyclass(
    name = "AsyncResponseStream",
    frozen,
    immutable_type,
    skip_from_py_object
)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyAsyncResponseStream {
    status: StatusCode,
    pub(crate) stream: AsyncResponseStreamInner,
}

#[cfg(feature = "experimental-async")]
impl RyAsyncResponseStream {
    pub(crate) fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let bstream = response.bytes_stream();
        let stream: BoxStream<'static, Result<Bytes, reqwest::Error>> = Box::pin(bstream);

        let stream = Arc::new(Mutex::new(stream.fuse()));
        Self {
            status,
            stream: stream as AsyncResponseStreamInner,
        }
    }
}

#[cfg(feature = "experimental-async")]
impl std::fmt::Display for RyAsyncResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncResponseStream<{}>", self.status)
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl RyAsyncResponseStream {
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    // CURRENTLY USING OLD VERSION TO AVOID LIFETIME ISSUES
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            match guard.next().await {
                Some(Ok(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Some(Err(e)) => Err(map_reqwest_err(e)),
                // I totally forgot that this was a thing and that I couldn't just return None
                None => Err(PyStopAsyncIteration::new_err("response-stream-end")),
            }
        })
    }

    // // FUTURE: use future_into_py here?
    // async fn __anext__(&self) -> PyResult<RyBytes> {
    //     let stream = self.stream.clone();
    //     // get next item
    //     let rt = pyo3_async_runtimes::tokio::get_runtime();
    //     let r = rt
    //         .spawn(async move {
    //             let mut guard = stream.lock().await;
    //             guard.next().await
    //         })
    //         .await
    //         .map_err(|e| py_runtime_error!("{e}"))?;
    //     match r {
    //         Some(Ok(bytes)) => Ok(RyBytes::from(bytes)),
    //         Some(Err(e)) => Err(map_reqwest_err(e)),
    //         // I totally forgot that this was a thing and that I couldn't just return None
    //         None => Err(PyStopAsyncIteration::new_err("async-response-stream-end")),
    //     }
    // }

    #[pyo3(signature = (n=1))]
    async fn take(&self, n: usize) -> PyResult<Vec<RyBytes>> {
        use ryo3_macro_rules::py_runtime_error;
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        let stream = self.stream.clone();
        rt.spawn(async move {
            let mut guard = stream.lock().await;
            let mut items = Vec::with_capacity(n);
            for _ in 0..n {
                match guard.next().await {
                    Some(Ok(bytes)) => items.push(RyBytes::from(bytes)),
                    Some(Err(e)) => return Err(map_reqwest_err(e)),
                    None => break,
                }
            }
            Ok(items)
        })
        .await
        .map_err(|e| py_runtime_error!("{e}"))?
    }

    async fn collect(&self) -> PyResult<Vec<RyBytes>> {
        use ryo3_macro_rules::py_runtime_error;
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        let stream = self.stream.clone();
        let py_bytes_vec = rt
            .spawn(async move {
                let mut guard = stream.lock().await;
                let mut items = Vec::new();
                while let Some(item) = guard.next().await {
                    match item {
                        Ok(bytes) => items.push(bytes),
                        Err(e) => return Err(e),
                    }
                }
                Ok(items)
            })
            .await
            .map_err(|e| py_runtime_error!("{e}"))?
            .map(|bytes_vec| bytes_vec.into_iter().map(RyBytes::from).collect())
            .map_err(map_reqwest_err)?;
        Ok(py_bytes_vec)
    }
}

#[pyclass(
    name = "BlockingResponseStream",
    frozen,
    immutable_type,
    skip_from_py_object
)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyBlockingResponseStream {
    status: StatusCode,
    pub(crate) stream: AsyncResponseStreamInner,
}

impl RyBlockingResponseStream {
    pub(crate) fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let bstream = response.bytes_stream();
        let stream: BoxStream<'static, Result<Bytes, reqwest::Error>> = Box::pin(bstream);

        let stream = Arc::new(Mutex::new(stream.fuse()));
        Self {
            status,
            stream: stream as AsyncResponseStreamInner,
        }
    }
}

#[pymethods]
impl RyBlockingResponseStream {
    fn __repr__(&self) -> String {
        format!("BlockingResponseStream<{}; >", self.status)
    }

    fn __iter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<ryo3_bytes::PyBytes> {
        let stream = self.stream.clone();
        let a = py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(next_bytes(&stream))
                .map_err(map_reqwest_err)
        })?;
        match a {
            Some(bytes) => Ok(ryo3_bytes::PyBytes::from(bytes)),
            None => Err(PyStopIteration::new_err("response-stream-end")),
        }
    }

    #[pyo3(signature = (n=1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<Vec<ryo3_bytes::PyBytes>> {
        let stream = self.stream.clone();
        let items = py
            .detach(|| {
                pyo3_async_runtimes::tokio::get_runtime()
                    .block_on(take_bytes(&stream, n))
                    .map_err(map_reqwest_err)
            })
            .map(|bytes_vec| {
                bytes_vec
                    .into_iter()
                    .map(ryo3_bytes::PyBytes::from)
                    .collect()
            })?;
        Ok(items)
    }

    #[pyo3(signature = (*, join=false))]
    fn collect<'py>(&self, py: Python<'py>, join: bool) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        if join {
            let py_bytes = py.detach(|| {
                rt.block_on(collect_bytes_join(&stream))
                    .map(ryo3_bytes::PyBytes::from)
                    .map_err(map_reqwest_err)
            })?;
            py_bytes.into_bound_py_any(py)
        } else {
            let py_bytes_vec: Vec<ryo3_bytes::PyBytes> = py.detach(|| {
                rt.block_on(collect_bytes_vec(&stream))
                    .map(|bytes_vec| {
                        bytes_vec
                            .into_iter()
                            .map(ryo3_bytes::PyBytes::from)
                            .collect()
                    })
                    .map_err(map_reqwest_err)
            })?;
            py_bytes_vec.into_bound_py_any(py)
        }
    }
}

async fn next_bytes(stream: &AsyncResponseStreamInner) -> Result<Option<Bytes>, reqwest::Error> {
    let mut guard = stream.lock().await;
    match guard.next().await {
        Some(Ok(bytes)) => Ok(Some(bytes)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

async fn take_bytes(
    stream: &AsyncResponseStreamInner,
    n: usize,
) -> Result<Vec<Bytes>, reqwest::Error> {
    let mut stream = stream.lock().await;
    let mut chunks = Vec::with_capacity(n);
    for _ in 0..n {
        match stream.next().await {
            Some(Ok(bytes)) => chunks.push(bytes),
            Some(Err(e)) => return Err(e),
            None => break,
        }
    }
    Ok(chunks)
}

async fn collect_bytes_join(stream: &AsyncResponseStreamInner) -> Result<Bytes, reqwest::Error> {
    let mut guard = stream.lock().await;
    let mut bytes_mut = BytesMut::new();
    while let Some(item) = guard.next().await {
        match item {
            Ok(bytes) => bytes_mut.extend_from_slice(&bytes),
            Err(e) => return Err(e),
        }
    }
    Ok(bytes_mut.freeze())
}

async fn collect_bytes_vec(
    stream: &AsyncResponseStreamInner,
) -> Result<Vec<Bytes>, reqwest::Error> {
    let mut guard = stream.lock().await;
    let mut items = Vec::new();
    while let Some(item) = guard.next().await {
        match item {
            Ok(bytes) => items.push(bytes),
            Err(e) => return Err(e),
        }
    }
    Ok(items)
}
