use pyo3::prelude::*;

#[pyclass(name = "Weekday", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RyWeekday(pub(crate) jiff::civil::Weekday);

#[pymethods]
impl RyWeekday {
    #[expect(clippy::trivially_copy_pass_by_ref)]
    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> &'static str {
        match self.0 {
            jiff::civil::Weekday::Sunday => "sunday",
            jiff::civil::Weekday::Monday => "monday",
            jiff::civil::Weekday::Tuesday => "tuesday",
            jiff::civil::Weekday::Wednesday => "wednesday",
            jiff::civil::Weekday::Thursday => "thursday",
            jiff::civil::Weekday::Friday => "friday",
            jiff::civil::Weekday::Saturday => "saturday",
        }
    }

    #[expect(clippy::trivially_copy_pass_by_ref)]
    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
      )
    )]
    fn string(&self) -> &'static str {
        self.py_to_string()
    }
}
