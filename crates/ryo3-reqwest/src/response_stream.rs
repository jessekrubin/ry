use crate::errors::map_reqwest_err;
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use futures_util::stream::{BoxStream, Fuse};
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use reqwest::StatusCode;
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

#[pyclass(name = "ResponseStream", frozen)]
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

#[pymethods]
impl RyResponseStream {
    fn __repr__(&self) -> String {
        format!("ResponseStream<{}; >", self.status)
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
                    Some(Ok(bytes)) => items.push(ryo3_bytes::PyBytes::from(bytes)),
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
                let py_bytes = ryo3_bytes::PyBytes::from(bytes);
                Ok(py_bytes)
            })
        } else {
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let mut guard = stream.lock().await;
                let mut items = Vec::new();
                while let Some(item) = guard.next().await {
                    match item {
                        Ok(bytes) => items.push(ryo3_bytes::PyBytes::from(bytes)),
                        Err(e) => return Err(map_reqwest_err(e)),
                    }
                }
                Ok(items)
            })
        }
    }
}
