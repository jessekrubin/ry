use crate::{JiffSignedDuration, JiffSpan};
use jiff::{Span, SpanRelativeTo};
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};

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
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
        signed_duration.into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffSpan {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let signed_duration = ob.extract::<JiffSignedDuration>()?;
        let span: Span = signed_duration.0.try_into()?;
        Ok(Self::from(span))
    }
}
