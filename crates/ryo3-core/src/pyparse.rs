use pyo3::prelude::*;

use crate::map_py_value_err;

pub trait PyFromStr: Sized {
    /// Parse from a string
    fn py_from_str(ob: &str) -> PyResult<Self>
    where
        Self: Sized;
}

impl<T> PyFromStr for T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    #[inline]
    fn py_from_str(ob: &str) -> PyResult<Self> {
        T::from_str(ob).map_err(map_py_value_err)
    }
}

pub trait PyParse: Sized {
    /// Parse from a string/bytes
    fn py_parse(ob: &Bound<'_, PyAny>) -> PyResult<Self>
    where
        Self: Sized;
}

impl<T> PyParse for T
where
    T: PyFromStr,
    // T: std::str::FromStr,
    // T::Err: std::fmt::Display,
{
    #[inline]
    fn py_parse(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.cast_exact::<pyo3::types::PyString>() {
            let s = s.to_str()?;
            T::py_from_str(s).map_err(map_py_value_err)
        } else if let Ok(b) = ob.cast_exact::<pyo3::types::PyBytes>() {
            let a = String::from_utf8_lossy(b.as_bytes());
            T::py_from_str(&a).map_err(map_py_value_err)
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected a str or bytes object",
            ))
        }
    }
}
