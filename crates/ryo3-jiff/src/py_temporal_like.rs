use pyo3::prelude::*;
use ryo3_macro_rules::py_type_err;

use crate::{RyDate, RyDateTime, RyTime, RyTimestamp, RyZoned};

#[derive(Debug, Clone)]
pub(crate) enum PyTermporalTypes<'a, 'py> {
    Date(Borrowed<'a, 'py, RyDate>),
    DateTime(Borrowed<'a, 'py, RyDateTime>),
    Time(Borrowed<'a, 'py, RyTime>),
    Zoned(Borrowed<'a, 'py, RyZoned>),
    Timestamp(Borrowed<'a, 'py, RyTimestamp>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyTermporalTypes<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        macro_rules! try_extract_type {
            ($ty:ty, $variant:ident) => {
                if let Ok(val) = obj.cast_exact::<$ty>() {
                    return Ok(Self::$variant(val));
                }
            };
        }
        try_extract_type!(RyZoned, Zoned);
        try_extract_type!(RyTimestamp, Timestamp);
        try_extract_type!(RyDateTime, DateTime);
        try_extract_type!(RyDate, Date);
        try_extract_type!(RyTime, Time);
        py_type_err!("Expected a Date, DateTime, Time, Timestamp, or ZonedDateTime type")
    }
}
