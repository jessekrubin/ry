use parking_lot::Mutex;
use pyo3::PyErr;
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

#[pyclass(extends=PyException, module="ry.ryo3", name="ReqwestError", frozen)]
#[derive(Debug)]
pub struct RyReqwestError(pub Arc<Mutex<Option<reqwest::Error>>>);

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
        if let Some(e) = &(*self.0.lock()) {
            e.is_body()
        } else {
            false
        }
    }

    fn is_builder(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_builder()
        } else {
            false
        }
    }

    fn is_connect(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_connect()
        } else {
            false
        }
    }

    fn is_decode(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_decode()
        } else {
            false
        }
    }

    fn is_redirect(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_redirect()
        } else {
            false
        }
    }

    fn is_request(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_request()
        } else {
            false
        }
    }

    fn is_status(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_status()
        } else {
            false
        }
    }

    fn is_timeout(&self) -> bool {
        if let Some(e) = &(*self.0.lock()) {
            e.is_timeout()
        } else {
            false
        }
    }

    fn status(&self) -> Option<PyHttpStatus> {
        if let Some(e) = &(*self.0.lock()) {
            e.status().map(PyHttpStatus)
        } else {
            None
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

impl From<RyReqwestError> for PyErr {
    fn from(e: RyReqwestError) -> Self {
        let value = e.0.lock().take();
        if let Some(e) = value {
            Self::new::<RyReqwestError, _>(format!("{e} ~ {e:?}"))
        } else {
            Self::new::<RyReqwestError, _>("RyReqwestError(None)")
        }
    }
}

pub(crate) fn map_reqwest_err(e: reqwest::Error) -> PyErr {
    let e = RyReqwestError(Arc::new(Mutex::new(Some(e))));
    e.into()
}
