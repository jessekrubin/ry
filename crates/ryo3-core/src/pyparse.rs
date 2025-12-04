use pyo3::prelude::*;

use crate::map_py_value_err;

pub trait PyFromStr: Sized {
    /// Parse from a string (basically FromStr but maps errors to PyResult)
    fn py_from_str(ob: &str) -> PyResult<Self>
    where
        Self: Sized;
}

/// Blanket impl for any type that implements FromStr
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

/// Trait for parsing from Python objects (str or bytes)
pub trait PyParse: Sized {
    /// Parse from a string/bytes
    fn py_parse(ob: &Bound<'_, PyAny>) -> PyResult<Self>
    where
        Self: Sized;
}

/// Blanket impl for any type that implements PyFromStr
impl<T> PyParse for T
where
    T: PyFromStr,
{
    #[inline]
    fn py_parse(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // TODO (non-fugue-state-jesse): add support for bytearray/memview/buffer protocol?
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
