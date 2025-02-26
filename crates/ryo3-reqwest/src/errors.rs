use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::PyErr;
use ryo3_http::PyHttpStatus;
use ryo3_url::PyUrl;

#[pyclass(extends=PyException, module="ry.ryo3", name="ReqwestError")]
#[derive(Debug)]
pub struct RyReqwestError(pub Option<reqwest::Error>);

#[pymethods]
impl RyReqwestError {
    #[expect(unused_variables)]
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn py_new<'py>(args: &Bound<'py, PyTuple>, kwargs: Option<&Bound<'py, PyDict>>) -> Self {
        Self(None)
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

    pub fn is_body(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_body()
        } else {
            false
        }
    }

    pub fn is_builder(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_builder()
        } else {
            false
        }
    }

    pub fn is_connect(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_connect()
        } else {
            false
        }
    }

    pub fn is_decode(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_decode()
        } else {
            false
        }
    }

    pub fn is_redirect(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_redirect()
        } else {
            false
        }
    }

    pub fn is_request(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_request()
        } else {
            false
        }
    }

    pub fn is_status(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_status()
        } else {
            false
        }
    }

    pub fn is_timeout(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_timeout()
        } else {
            false
        }
    }

    pub fn status(&self) -> Option<PyHttpStatus> {
        if let Some(e) = &self.0 {
            e.status().map(PyHttpStatus)
        } else {
            None
        }
    }

    pub fn url(&self) -> Option<PyUrl> {
        if let Some(e) = &self.0 {
            e.url().map(|url| PyUrl(url.clone()))
        } else {
            None
        }
    }

    pub fn with_url<'py>(mut slf: PyRefMut<'py, Self>, url: &PyUrl) -> PyRefMut<'py, Self> {
        if let Some(e) = &mut slf.0 {
            let mut url = url.0.clone();
            e.url_mut().replace(&mut url);
        }
        slf
    }

    pub fn without_url(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        // take the error
        let err = slf.0.take();
        if let Some(e) = err {
            // baboom put it back
            slf.0 = Some(e.without_url());
        }
        slf
    }
}

impl From<RyReqwestError> for PyErr {
    fn from(e: RyReqwestError) -> Self {
        // map_reqwest_err(e)
        if let Some(e) = e.0 {
            PyErr::new::<RyReqwestError, _>(format!("{e:?}"))
        } else {
            PyErr::new::<RyReqwestError, _>("RyReqwestError(None)")
        }
    }
}

pub(crate) fn map_reqwest_err(e: reqwest::Error) -> PyErr {
    let e = RyReqwestError(Some(e));
    e.into()
}
