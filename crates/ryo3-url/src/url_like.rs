use crate::PyUrl;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{Bound, FromPyObject, PyAny, PyErr, PyResult};

pub struct UrlLike(pub url::Url);

impl<'py> FromPyObject<'_, 'py> for UrlLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(url) = obj.cast_exact::<PyUrl>() {
            let url = url.borrow();
            Ok(Self(url.0.clone()))
        } else if let Ok(s) = obj.cast::<PyString>()?.to_str() {
            let url = url::Url::parse(s).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={s})"))
            })?;
            Ok(Self(url))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected str or URL object",
            ))
        }
    }
}

pub fn extract_url(ob: &Bound<'_, PyAny>) -> PyResult<url::Url> {
    if let Ok(url) = ob.cast::<PyUrl>() {
        let url = url.borrow();
        Ok(url.0.clone())
    } else if let Ok(s) = ob.cast::<PyString>()?.to_str() {
        url::Url::parse(s)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={ob})")))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected str or URL object",
        ))
    }
}
