//! Response stream impls for ryo3-reqwest
//!
//! TODO: deduplicate logic and code to reduce copy-paste between different versions
//! TODO: consider making a common trait for the different response stream types
//! TODO:
//!
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

#[derive(Clone)]
struct ResponseStreamInner {
    status: StatusCode,
    min_read_size: usize,
    pub(crate) stream: AsyncResponseStreamInner,
}

#[pyclass(name = "ResponseStream", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyResponseStream {
    inner: ResponseStreamInner,
}

impl ResponseStreamInner {
    pub(crate) fn from_response(response: reqwest::Response, min_read_size: usize) -> Self {
        let status = response.status();
        let bstream = response.bytes_stream();
        let stream: BoxStream<'static, Result<Bytes, reqwest::Error>> = Box::pin(bstream);

        let stream = Arc::new(Mutex::new(stream.fuse()));
        Self {
            status,
            min_read_size,
            stream: stream as AsyncResponseStreamInner,
        }
    }

    #[inline]
    async fn next(&self) -> Result<Option<Bytes>, reqwest::Error> {
        if self.min_read_size == 0 {
            next_bytes(&self.stream).await
        } else {
            let mut guard = self.stream.lock().await;
            let mut chunks = Vec::with_capacity(8);
            let mut total_size = 0;
            while total_size < self.min_read_size {
                match guard.next().await {
                    Some(Ok(bytes)) => {
                        total_size += bytes.len();
                        chunks.push(bytes);
                    }
                    Some(Err(e)) => return Err(e),
                    None => break,
                }
            }
            if chunks.is_empty() {
                Ok(None)
            } else {
                let bytes = if chunks.len() == 1 {
                    chunks
                        .into_iter()
                        .next()
                        .expect("wenodis: just checked len>0")
                } else {
                    // TODO: possibly just take the first chunk and then extend_from_slice the rest o the chunks
                    let mut bytes_mut = BytesMut::with_capacity(total_size);
                    for chunk in chunks {
                        bytes_mut.extend_from_slice(&chunk);
                    }
                    bytes_mut.freeze()
                };
                Ok(Some(bytes))
            }
        }
    }

    /// py-next helper
    #[inline]
    async fn py_anext(&self) -> PyResult<Option<ryo3_bytes::PyBytes>> {
        let res = self.next().await;
        match res {
            Ok(Some(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
            Ok(None) => Err(PyStopAsyncIteration::new_err("response-stream-end")),
            Err(e) => Err(map_reqwest_err(e)),
        }
    }
}

impl RyResponseStream {
    pub(crate) fn from_response(response: reqwest::Response, min_read_size: usize) -> Self {
        let inner = ResponseStreamInner::from_response(response, min_read_size);
        Self { inner }
    }
}

impl std::fmt::Display for RyResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseStream<{}>", self.inner.status)
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
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { inner.py_anext().await })
    }

    #[pyo3(signature = (n=1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.inner.stream.clone();
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
        let stream = self.inner.stream.clone();
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
    inner: ResponseStreamInner,
}

#[cfg(feature = "experimental-async")]
impl RyAsyncResponseStream {
    pub(crate) fn from_response(response: reqwest::Response, min_read_size: usize) -> Self {
        let inner = ResponseStreamInner::from_response(response, min_read_size);
        Self { inner }
    }
}

#[cfg(feature = "experimental-async")]
impl std::fmt::Display for RyAsyncResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncResponseStream<{}>", self.inner.status)
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
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { inner.py_anext().await })
    }

    // async fn _anext( &self) -> PyResult<RyBytes> {
    //     let res = self.inner.next().await;
    //     match res {
    //         Ok(Some(bytes)) => Ok(RyBytes::from(bytes)),
    //         Ok(None) => Err(PyStopAsyncIteration::new_err("response-stream-end")),
    //         Err(e) => Err(map_reqwest_err(e)),
    //     }
    // }

    #[pyo3(signature = (n=1))]
    async fn take(&self, n: usize) -> PyResult<Vec<RyBytes>> {
        use ryo3_macro_rules::py_runtime_error;
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        let stream = self.inner.stream.clone();
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
        let stream = self.inner.stream.clone();
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
    inner: ResponseStreamInner,
}

impl RyBlockingResponseStream {
    pub(crate) fn from_response(response: reqwest::Response, min_read_size: usize) -> Self {
        let inner = ResponseStreamInner::from_response(response, min_read_size);
        Self { inner }
    }
}

#[pymethods]
impl RyBlockingResponseStream {
    fn __repr__(&self) -> String {
        format!("BlockingResponseStream<{}>", self.inner.status)
    }

    fn __iter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<RyBytes> {
        let stream = self.inner.stream.clone();
        let a = py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(next_bytes(&stream))
                .map_err(map_reqwest_err)
        })?;
        match a {
            Some(bytes) => Ok(RyBytes::from(bytes)),
            None => Err(PyStopIteration::new_err("response-stream-end")),
        }
    }

    #[pyo3(signature = (n=1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<Vec<RyBytes>> {
        let stream = self.inner.stream.clone();
        let items = py
            .detach(|| {
                pyo3_async_runtimes::tokio::get_runtime()
                    .block_on(take_bytes(&stream, n))
                    .map_err(map_reqwest_err)
            })
            .map(|bytes_vec| bytes_vec.into_iter().map(RyBytes::from).collect())?;
        Ok(items)
    }

    #[pyo3(signature = (*, join=false))]
    fn collect<'py>(&self, py: Python<'py>, join: bool) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.inner.stream.clone();
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        if join {
            let py_bytes = py.detach(|| {
                rt.block_on(collect_bytes_join(&stream))
                    .map(RyBytes::from)
                    .map_err(map_reqwest_err)
            })?;
            py_bytes.into_bound_py_any(py)
        } else {
            let py_bytes_vec: Vec<RyBytes> = py.detach(|| {
                rt.block_on(collect_bytes_vec(&stream))
                    .map(|bytes_vec| bytes_vec.into_iter().map(RyBytes::from).collect())
                    .map_err(map_reqwest_err)
            })?;
            py_bytes_vec.into_bound_py_any(py)
        }
    }
}

#[inline]
async fn next_bytes(stream: &AsyncResponseStreamInner) -> Result<Option<Bytes>, reqwest::Error> {
    let mut guard = stream.lock().await;
    match guard.next().await {
        Some(Ok(bytes)) => Ok(Some(bytes)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

#[inline]
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

#[inline]
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

#[inline]
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
