use crate::PyUrl;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{Bound, FromPyObject, PyAny, PyErr, PyResult};

pub struct UrlLike(pub url::Url);

impl FromPyObject<'_> for crate::UrlLike {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        crate::extract_url(ob).map(crate::UrlLike)
    }
}

pub fn extract_url(ob: &Bound<PyAny>) -> PyResult<url::Url> {
    if let Ok(url) = ob.downcast::<PyUrl>() {
        let url = url.borrow();
        Ok(url.0.clone())
    } else if let Ok(s) = ob.downcast::<PyString>()?.to_str() {
        url::Url::parse(s)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={ob})")))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected str or URL object",
        ))
    }
}
