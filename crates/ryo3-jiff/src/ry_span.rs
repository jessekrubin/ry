use crate::internal::RySpanRelativeTo;
use crate::ry_signed_duration::RySignedDuration;
use jiff::Span;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "Span", module = "ryo3")]
pub struct RySpan(pub(crate) Span);

#[pymethods]
impl RySpan {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(Span::new()))
    }

    fn to_string(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __str__(&self) -> String {
        format!("Span<{}>", self.0.to_string())
    }

    fn __repr__(&self) -> String {
        format!("Span<{}>", self.0.to_string())
    }

    fn __neg__(&self) -> PyResult<Self> {
        Ok(Self(self.0.negate()))
    }

    fn __invert__(&self) -> PyResult<Self> {
        Ok(Self(self.0.negate()))
    }

    // fn to_jiff_duration<'a, R: Into<SpanRelativeTo<'a>>>(
    //     &self,
    //     relative: R,
    // ) -> PyResult<RySignedDuration> {
    //     let a = self.0.to_jiff_duration(relative.into());
    //     RySignedDuration()
    // }
    fn to_jiff_duration(&self, relative: RySpanRelativeTo) -> PyResult<RySignedDuration> {
        match relative {
            RySpanRelativeTo::Zoned(z) => self
                .0
                .to_jiff_duration(&z.0)
                .map(|d| RySignedDuration(d))
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            RySpanRelativeTo::Date(d) => self
                .0
                .to_jiff_duration(d.0)
                .map(|d| RySignedDuration(d))
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            RySpanRelativeTo::DateTime(dt) => self
                .0
                .to_jiff_duration(dt.0)
                .map(|d| RySignedDuration(d))
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
        }
    }
}

impl From<Span> for RySpan {
    fn from(span: Span) -> Self {
        Self(span)
    }
}
