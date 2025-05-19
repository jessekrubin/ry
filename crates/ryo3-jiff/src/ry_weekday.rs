use pyo3::prelude::*;

#[pyclass(name = "Weekday", module = "ry.ryo3", frozen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RyWeekday(pub(crate) jiff::civil::Weekday);

#[pymethods]
impl RyWeekday {
    #[expect(clippy::trivially_copy_pass_by_ref)]
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
