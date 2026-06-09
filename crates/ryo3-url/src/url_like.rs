use pyo3::prelude::*;
use ryo3_core::macros::{py_type_err, py_value_error};

use crate::PyUrl;
pub struct UrlLike(pub url::Url);

impl From<UrlLike> for url::Url {
    fn from(ul: UrlLike) -> Self {
        ul.0
    }
}

impl UrlLike {
    fn apply_with_params<'py>(
        &mut self,
        params: &'py Bound<'py, pyo3::types::PyDict>,
    ) -> PyResult<()> {
        let mut query_pairs = self.0.query_pairs_mut();

        for (k, v) in params {
            let k_str: &str = k.extract()?;
            let v_str: &str = v.extract()?;
            query_pairs.append_pair(k_str, v_str);
        }
        Ok(())
    }

    pub fn py_with_params<'py>(
        mut self,
        params: &'py Bound<'py, pyo3::types::PyDict>,
    ) -> PyResult<Self> {
        self.apply_with_params(params)?;
        Ok(self)
    }
}

impl<'py> FromPyObject<'_, 'py> for UrlLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(url) = obj.cast_exact::<PyUrl>() {
            let url = url.borrow();
            Ok(Self(url.0.clone()))
        } else if let Ok(s) = obj.extract::<&str>() {
            url::Url::parse(s)
                .map(Self)
                .map_err(|e| py_value_error!("{e} (url={s})"))
        } else if let Ok(b) = obj.extract::<&[u8]>() {
            let s = std::str::from_utf8(b)
                .map_err(|e| py_value_error!("Invalid UTF-8 sequence: {e} (bytes={b:?})"))?;
            url::Url::parse(s)
                .map(Self)
                .map_err(|e| py_value_error!("{e} (url={s})"))
        } else {
            py_type_err!("Expected str or URL object")
        }
    }
}

impl From<UrlLike> for PyUrl {
    fn from(ul: UrlLike) -> Self {
        ul.0.into()
    }
}

impl std::fmt::Display for UrlLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
