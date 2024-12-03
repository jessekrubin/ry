use crate::JiffTimeZone;
use pyo3::{pyclass, FromPyObject};

// #[derive(Debug, FromPyObject)]
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) enum RyInTz {
    Str(String),
    JiffTimezone(JiffTimeZone),
}

impl RyInTz {
    pub(crate) fn tz_string(&self) -> Option<&str> {
        match self {
            RyInTz::Str(s) => Some(s),
            RyInTz::JiffTimezone(tz) => tz.0.iana_name(),
        }
    }
}
