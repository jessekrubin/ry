use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::PyErr;
use ryo3_http::PyHttpStatus;
use ryo3_url::PyUrl;
use std::fmt;

#[pyclass(extends=PyException, module="ry.ryo3", name="ReqwestError")]
#[derive(Debug)]
pub struct RyReqwestError(pub Option<reqwest::Error>);

// impl RyReqwestError {
//     fn new(e: reqwest::Error) -> Self {
//         RyReqwestError(e)
//     }
// }

#[pymethods]
impl RyReqwestError {
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn py_new<'py>(args: Bound<'py, PyTuple>, kwargs: Option<Bound<'py, PyDict>>) -> Self {
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

    // pub fn is_decode(&self) -> bool {
    //     self.0.is_decode()
    // }
    //
    // pub fn is_redirect(&self) -> bool {
    //     self.0.is_redirect()
    // }
    //
    // pub fn is_request(&self) -> bool {
    //     self.0.is_request()
    // }
    //
    // pub fn is_status(&self) -> bool {
    //     self.0.is_status()
    // }
    //
    pub fn is_timeout(&self) -> bool {
        if let Some(e) = &self.0 {
            e.is_timeout()
        } else {
            false
        }
    }

    pub fn status(&self) -> Option<PyHttpStatus> {
        if let Some(e) = &self.0 {
            if let Some(status) = e.status() {
                Some(PyHttpStatus(status))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn url(&self) -> Option<PyUrl> {
        if let Some(e) = &self.0 {
            if let Some(url) = e.url() {
                Some(PyUrl(url.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }

    // pub fn with_url<'py>(mut slf: PyRefMut<'py, Self>, url: &PyUrl) -> PyRefMut<'py, Self> {
    //     let mut url = url.0.clone();
    //     slf.0.url_mut().replace(&mut url);
    //     slf
    // }
    //
    // pub fn without_url(mut slf: PyRefMut<'_, Self>) {
    //     slf.0.url_mut().take();
    // }
}
//
// impl fmt::Display for RyReqwestError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "RyReqwestErrorPy: {:?}", self.0)
//     }
// }
//
// #[derive(Error, Debug)]
// #[pyclass]
// pub struct RyReqwestError

// #[derive(Error, Debug)]
// pub enum Ryo3ReqwestError {
//     /// A wrapped [object_store::Error]
//     #[error(transparent)]
//     ReqwestError(#[from] reqwest::Error),
//
//     /// A wrapped [PyErr]
//     #[error(transparent)]
//     PyErr(#[from] PyErr),
//
//     /// A wrapped [std::io::Error]
//     #[error(transparent)]
//     IOError(#[from] std::io::Error),
// }
//
// impl From<Ryo3ReqwestError> for PyErr {
//     fn from(e: Ryo3ReqwestError) -> Self {
//         match e {
//             Ryo3ReqwestError::PyErr(err) => err,
//
//             Ryo3ReqwestError::ReqwestError(ref err) => {
//                 tracing::trace!("ReqwestError: {:?}", err);
//                 ReqwestError::new_err(format!("{:?}", err))
//             }
//             // Ryo3ReqwestError::PyErr(err) => err,
//             Ryo3ReqwestError::IOError(err) => PyIOError::new_err(err.to_string()),
//         }
//     }
// }

// pub(crate) fn map_reqwest_err(e: &Error) -> PyErr {
//     PyValueError::new_err(format!("{e}"))
// }
// pub(crate) fn map_reqwest_err<E>(e: E) -> PyErr
// where
//     E: fmt::Display,
// {
//
//     PyErr::new::<PyValueError, _>(format!("{e}"))
// }

// version taht takes either a reqwest::Error or a reference to a reqwest::Error
pub(crate) fn map_reqwest_err(e: reqwest::Error) -> PyErr {
    let e = RyReqwestError(Some(e));
    e.into()
}

impl From<RyReqwestError> for PyErr {
    fn from(e: RyReqwestError) -> Self {
        // map_reqwest_err(e)
        if let Some(e) = e.0 {
            PyErr::new::<RyReqwestError, _>(format!("{:?}", e))
        } else {
            PyErr::new::<RyReqwestError, _>("RyReqwestError(None)")
        }
    }
}
