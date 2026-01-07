use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::stream::BoxStream;
use futures_util::ready;
use pyo3::prelude::*;
use ryo3_bytes::PyBytes as RyBytes;

pub(crate) enum PyBodyStream {
    Sync(PyBodySyncStream),
    Async(PyBodyAsyncStream),
}

impl std::fmt::Debug for PyBodyStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(_) => f.debug_struct("PyBodyStream::Sync").finish(),
            Self::Async(_) => f.debug_struct("PyBodyStream::Async").finish(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum PyBody {
    Stream(PyBodyStream),
    Bytes(RyBytes),
}

pub(crate) struct PyBodySyncStream {
    obj: Py<PyAny>,
}

pub(crate) struct PyBodyAsyncStream {
    inner: BoxStream<'static, PyResult<Py<PyAny>>>,
}

impl Iterator for PyBodySyncStream {
    type Item = Result<RyBytes, PyErr>;

    fn next(&mut self) -> Option<Self::Item> {
        Python::attach(|py| {
            let result = self.obj.call_method0(py, pyo3::intern!(py, "__next__"));
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
            |py| match self.obj.call_method0(py, pyo3::intern!(py, "__next__")) {
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
        match ready!(self.inner.as_mut().poll_next(cx)) {
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
        // check is str so we don't hit the iterable checks below
        if let Ok(buffer) = obj.extract::<RyBytes>() {
            Ok(Self::Bytes(buffer))
        } else if obj.hasattr(pyo3::intern!(py, "__aiter__"))? {
            let inner_iter = obj.call_method0(pyo3::intern!(py, "__aiter__"))?;
            let s = pyo3_async_runtimes::tokio::into_stream_v1(inner_iter)?;
            Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream {
                inner: Box::pin(s),
            })))
        } else if obj.hasattr(pyo3::intern!(py, "__anext__"))? {
            let s = pyo3_async_runtimes::tokio::into_stream_v1(obj.to_owned())?;

            Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream {
                inner: Box::pin(s),
            })))
        } else if obj.hasattr(pyo3::intern!(py, "__iter__"))? {
            let iter_obj = obj.call_method0(pyo3::intern!(py, "__iter__"))?;
            Ok(Self::Stream(PyBodyStream::Sync(PyBodySyncStream {
                obj: iter_obj.into(),
            })))
        } else if obj.hasattr(pyo3::intern!(py, "__next__"))? {
            Ok(Self::Stream(PyBodyStream::Sync(PyBodySyncStream {
                obj: obj.into(),
            })))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected bytes-like object or an async or sync iterable for request body",
            ))
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
