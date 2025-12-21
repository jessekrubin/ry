use crate::{JiffSignedDuration, JiffSpan};
use jiff::{Span, SpanRelativeTo};
use pyo3::prelude::*;
use ryo3_core::map_py_value_err;

impl<'py> IntoPyObject<'py> for JiffSpan {
    type Target = pyo3::types::PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffSpan {
    type Target = pyo3::types::PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let signed_duration = self
            .0
            .to_duration(SpanRelativeTo::days_are_24_hours())
            .map(JiffSignedDuration::from)
            .map_err(map_py_value_err)?;
        signed_duration.into_pyobject(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffSpan {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let signed_duration = obj.extract::<JiffSignedDuration>()?;
        let span: Span = signed_duration.0.try_into().map_err(map_py_value_err)?;
        Ok(Self::from(span))
    }
}
