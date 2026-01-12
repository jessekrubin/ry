//! reqwest Body implementations for Python body types
//!
//! This was a huge pain in the ass to figure out but I think I got it.
//!
//! The sync/async iterable implementation is based primarily off of the
//! impl in `obstore`'s 'put' methods
//!
//! AFAICT `pyo3_async_runtimes::tokio::into_stream_v1` is generally faster
//! than `pyo3_async_runtimes::tokio::into_stream_v2` as well as allows for
//! saner error handling.
//!
//! the `future_utils::ready` macro is very nifty
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::stream::BoxStream;
use futures_util::ready;
use pyo3::prelude::*;
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_macro_rules::py_type_err;

pub(crate) struct PyBodySyncStream(Py<PyAny>);
pub(crate) struct PyBodyAsyncStream(BoxStream<'static, PyResult<Py<PyAny>>>);
pub(crate) enum PyBodyStream {
    Sync(PyBodySyncStream),
    Async(PyBodyAsyncStream),
}

// #[cfg(feature = "experimental-async")]
impl PyBodyStream {
    pub(crate) fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }
}

#[derive(Debug)]
pub(crate) enum PyBody {
    Stream(PyBodyStream),
    Bytes(RyBytes),
}

impl std::fmt::Debug for PyBodyStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(_) => f.debug_struct("PyBodyStream::Sync").finish(),
            Self::Async(_) => f.debug_struct("PyBodyStream::Async").finish(),
        }
    }
}

impl Iterator for PyBodySyncStream {
    type Item = Result<RyBytes, PyErr>;

    fn next(&mut self) -> Option<Self::Item> {
        Python::attach(|py| {
            let result = self.0.call_method0(py, pyo3::intern!(py, "__next__"));
            match result {
                Ok(obj) => Some(obj.extract::<RyBytes>(py)),
                Err(e) => {
                    if e.is_instance_of::<pyo3::exceptions::PyStopIteration>(py) {
                        None
                    } else {
                        Some(Err(e))
                    }
                }
            }
        })
    }
}

impl futures_util::stream::Stream for PyBodySyncStream {
    type Item = PyResult<RyBytes>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Python::attach(
            |py| match self.0.call_method0(py, pyo3::intern!(py, "__next__")) {
                Ok(val) => Poll::Ready(Some(val.extract::<RyBytes>(py))),
                Err(e) => {
                    if e.is_instance_of::<pyo3::exceptions::PyStopIteration>(py) {
                        Poll::Ready(None)
                    } else {
                        Poll::Ready(Some(Err(e)))
                    }
                }
            },
        )
    }
}

impl futures_util::stream::Stream for PyBodyAsyncStream {
    type Item = PyResult<RyBytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // this ready macro is pretty swag
        match ready!(self.0.as_mut().poll_next(cx)) {
            Some(Ok(obj)) => {
                let r = Python::attach(|py| obj.bind(py).extract::<RyBytes>());
                Poll::Ready(Some(r))
            }
            Some(Err(e)) => {
                let is_stop = Python::attach(|py| {
                    e.is_instance_of::<pyo3::exceptions::PyStopAsyncIteration>(py)
                });
                if is_stop {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Err(e)))
                }
            }
            None => Poll::Ready(None),
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for PyBody {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let py = obj.py();
        // TODO: dedupe these interned strings
        if let Ok(buffer) = obj.extract::<RyBytes>() {
            Ok(Self::Bytes(buffer))
        } else if obj.hasattr(pyo3::intern!(py, "__aiter__"))? {
            let inner_iter = obj.call_method0(pyo3::intern!(py, "__aiter__"))?;
            let stream = pyo3_async_runtimes::tokio::into_stream_v1(inner_iter)?;
            let boxed_stream: BoxStream<'static, PyResult<Py<PyAny>>> = Box::pin(stream);
            Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream(
                boxed_stream,
            ))))
        } else if obj.hasattr(pyo3::intern!(py, "__anext__"))? {
            let stream = pyo3_async_runtimes::tokio::into_stream_v1(obj.to_owned())?;
            let boxed_stream: BoxStream<'static, PyResult<Py<PyAny>>> = Box::pin(stream);
            Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream(
                boxed_stream,
            ))))
        } else if obj.hasattr(pyo3::intern!(py, "__iter__"))? {
            let iter_obj = obj.call_method0(pyo3::intern!(py, "__iter__"))?;
            let sync_stream = PyBodySyncStream(iter_obj.into());
            Ok(Self::Stream(PyBodyStream::Sync(sync_stream)))
        } else if obj.hasattr(pyo3::intern!(py, "__next__"))? {
            let sync_stream = PyBodySyncStream(obj.into());
            Ok(Self::Stream(PyBodyStream::Sync(sync_stream)))
        } else {
            py_type_err!("Expected bytes-like object or an async or sync iterable for request body")
        }
    }
}

// ----------------------------------------------------------------------------
// INTO-BODY
// ----------------------------------------------------------------------------
impl From<PyBodyAsyncStream> for reqwest::Body {
    fn from(val: PyBodyAsyncStream) -> Self {
        Self::wrap_stream(val)
    }
}

impl From<PyBodySyncStream> for reqwest::Body {
    fn from(val: PyBodySyncStream) -> Self {
        Self::wrap_stream(val)
    }
}

impl From<PyBodyStream> for reqwest::Body {
    fn from(val: PyBodyStream) -> Self {
        match val {
            PyBodyStream::Sync(s) => s.into(),
            PyBodyStream::Async(s) => s.into(),
        }
    }
}

impl From<PyBody> for reqwest::Body {
    fn from(val: PyBody) -> Self {
        match val {
            PyBody::Bytes(b) => Self::from(b.into_inner()),
            PyBody::Stream(s) => s.into(),
        }
    }
}
