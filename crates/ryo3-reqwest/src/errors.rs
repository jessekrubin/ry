use parking_lot::Mutex;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use ryo3_http::PyHttpStatus;
use ryo3_url::PyUrl;
use std::sync::Arc;

/// macro for crate use only to return `Response already consumed` error
#[macro_export]
macro_rules! pyerr_response_already_consumed {
    () => {
        ::pyo3::exceptions::PyValueError::new_err("Response already consumed")
    };
}

#[derive(Debug)]
#[pyclass(extends=PyException, module="ry.ryo3", name="ReqwestError", frozen, immutable_type, skip_from_py_object)]
pub struct RyReqwestError(pub Arc<Mutex<Option<reqwest::Error>>>);

impl From<reqwest::Error> for RyReqwestError {
    fn from(e: reqwest::Error) -> Self {
        Self(Arc::new(Mutex::new(Some(e))))
    }
}

#[pymethods]
impl RyReqwestError {
    #[expect(unused_variables)]
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn py_new<'py>(args: &Bound<'py, PyTuple>, kwargs: Option<&Bound<'py, PyDict>>) -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    fn __dbg__(&self) -> String {
        format!("{:?}", self.0)
    }

    // Methods of reqwest::Error
    // - is_body
    // - is_builder
    // - is_connect
    // - is_decode
    // - is_redirect
    // - is_request
    // - is_status
    // - is_timeout
    // - status
    // - url
    // - url_mut
    // - with_url
    // - without_url

    fn is_body(&self) -> bool {
        self.0.lock().as_ref().is_some_and(reqwest::Error::is_body)
    }

    fn is_builder(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_builder)
    }

    fn is_connect(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_connect)
    }

    fn is_decode(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_decode)
    }

    fn is_redirect(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_redirect)
    }

    fn is_request(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_request)
    }

    fn is_status(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_status)
    }

    fn is_timeout(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(reqwest::Error::is_timeout)
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Option<Py<PyHttpStatus>>> {
        if let Some(e) = &(*self.0.lock()) {
            e.status()
                .map(|status| PyHttpStatus::from_status_code_cached(py, status))
                .transpose()
        } else {
            Ok(None)
        }
    }

    fn url(&self) -> Option<PyUrl> {
        if let Some(e) = &(*self.0.lock()) {
            e.url().map(|url| PyUrl::new(url.clone()))
        } else {
            None
        }
    }

    fn with_url<'py>(slf: PyRef<'py, Self>, url: &PyUrl) -> PyRef<'py, Self> {
        // take the error
        let err = slf.0.lock().take();
        if let Some(e) = err {
            // baboom put it back
            slf.0.lock().replace(e.with_url(url.as_ref().clone()));
        }
        slf
    }

    fn without_url(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        // take the error
        let err = slf.0.lock().take();
        if let Some(e) = err {
            // baboom put it back sans url
            slf.0.lock().replace(e.without_url());
        }
        slf
    }
}

impl From<RyReqwestError> for pyo3::PyErr {
    fn from(e: RyReqwestError) -> Self {
        let value = e.0.lock().take();
        if let Some(e) = value {
            Self::new::<RyReqwestError, _>(format!("{e} ~ {e:?}"))
        } else {
            Self::new::<RyReqwestError, _>("RyReqwestError(None)")
        }
    }
}

/// Maps a `reqwest::Error` to a `pyo3::PyErr`, handling the case where the
/// Python interpreter is shutting down
///
/// Should prevent panics during interpreter shutdown when background threads
pub(crate) fn map_reqwest_err(e: reqwest::Error) -> pyo3::PyErr {
    #[expect(unsafe_code)]
    if unsafe { pyo3::ffi::Py_IsInitialized() } == 0 {
        loop {
            std::thread::park();
        }
    }
    let maybe_pyerr = Python::try_attach(|_py| {
        let req_err = RyReqwestError::from(e);
        pyo3::PyErr::from(req_err)
    });
    if maybe_pyerr.is_none() {
        tracing::warn!("Interpreter died while processing error. Parking thread.");
        loop {
            std::thread::park();
        }
    }
    maybe_pyerr.expect("no-way-jose")
}
