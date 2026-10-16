use pyo3::prelude::*;
use ryo3_macro_rules::{py_type_err, py_value_err};

use crate::JiffFriendlyDesignator;

const JIFF_FRIENDLY_DESIGNATOR_ACCEPTED: &str =
    "'human'/'human-time', 'short', 'compact', or 'verbose'";

impl<'py> FromPyObject<'_, 'py> for JiffFriendlyDesignator {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "human-time" | "human" => Ok(Self::HUMAN_TIME),
                "short" => Ok(Self::SHORT),
                "compact" => Ok(Self::COMPACT),
                "verbose" => Ok(Self::VERBOSE),
                _ => py_value_err!(
                    "Invalid designator: {s} (options: {JIFF_FRIENDLY_DESIGNATOR_ACCEPTED})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for designator, expected a string (options: {JIFF_FRIENDLY_DESIGNATOR_ACCEPTED})"
            )
        }
    }
}
