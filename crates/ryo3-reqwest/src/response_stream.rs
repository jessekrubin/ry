use crate::errors::map_reqwest_err;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt;
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

// This whole response iterator was a difficult thing to figure out.
//
// NOTE: I (jesse) am pretty proud of this. I was struggling to get the
//       async-iterator thingy to work bc rust + async is quite hard, but
//       after lots and lots and lots of trial and error this works!
//
// clippy says this is too long and complicated to just sit in the struct def
type AsyncResponseStreamInner =
    Arc<Mutex<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>>>;
#[pyclass(name = "ResponseStream", module = "ry.ryo3.reqwest", frozen)]
pub struct RyResponseStream {
    pub(crate) stream: AsyncResponseStreamInner,
}

#[pymethods]
impl RyResponseStream {
    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            match guard.as_mut().next().await {
                Some(Ok(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Some(Err(e)) => Err(map_reqwest_err(e)),
                // I totally forgot that this was a thing and that I couldn't just return None
                None => Err(PyStopAsyncIteration::new_err("response-stream-fin")),
            }
        })
    }
}
