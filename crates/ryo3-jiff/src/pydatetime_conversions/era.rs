use crate::jiff_types::JiffEra;
use jiff::civil::Era;
use pyo3::prelude::*;
use pyo3::types::PyString;

const JIFF_ERA_STRINGS: &str = "'BCE', 'CE'";

impl FromPyObject<'_> for JiffEra {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffEra> {
        // downcast to string...
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
                "bce" => Ok(JiffEra(Era::BCE)),
                "ce" => Ok(JiffEra(Era::CE)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid era: {s} (options: {JIFF_ERA_STRINGS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Invalid type for era",
            ))
        }
    }
}
