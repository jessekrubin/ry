use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_zoned::RyZoned;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum RySpanRelativeTo<'a, 'py> {
    Zoned(Borrowed<'a, 'py, RyZoned>),
    Date(Borrowed<'a, 'py, RyDate>),
    DateTime(Borrowed<'a, 'py, RyDateTime>),

}

impl<'a, 'py> FromPyObject<'a, 'py> for RySpanRelativeTo<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(z) = obj.cast_exact::<RyZoned>() {
            Ok(Self::Zoned(z))
        } else if let Ok(d) = obj.cast_exact::<RyDate>() {
            Ok(Self::Date(d))
        } else if let Ok(dt) = obj.cast_exact::<RyDateTime>() {
            Ok(Self::DateTime(dt))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected ZonedDateTime, DateTime, or Date",
            ))
        }
    }
}
