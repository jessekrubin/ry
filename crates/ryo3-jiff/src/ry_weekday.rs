use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RyWeekday(pub(crate) jiff::civil::Weekday);

#[pymethods]
impl RyWeekday {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn string(&self) -> &'static str {
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
}
