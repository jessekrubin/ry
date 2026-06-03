use pyo3::prelude::*;

use crate::{PyCastExactOpt, map_py_value_err, pystr_read_fast};

pub trait PyFromStr: Sized {
    /// Parse from a string (basically `FromStr` but maps errors to `PyResult`)
    fn py_from_str(ob: &str) -> PyResult<Self>;
}

/// Blanket impl for any type that implements `FromStr`
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
    fn py_parse(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self>;
}

/// Blanket impl for any type that implements `PyFromStr`
impl<T> PyParse for T
where
    T: PyFromStr,
{
    #[inline]
    fn py_parse(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        // TODO (non-fugue-state-jesse): add support for bytearray/memview/buffer
        // protocol?
        if let Some(s) = ob.cast_exact_opt::<pyo3::types::PyString>() {
            let s = pystr_read_fast(s)?;
            T::py_from_str(s).map_err(map_py_value_err)
        } else if let Some(b) = ob.cast_exact_opt::<pyo3::types::PyBytes>() {
            let s = std::str::from_utf8(b.as_bytes())?;
            T::py_from_str(s).map_err(map_py_value_err)
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected a str or bytes object",
            ))
        }
    }
}

pub struct PyFromStrArg<T>(T);

impl<T> PyFromStrArg<T> {
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'a, 'py, T> FromPyObject<'a, 'py> for PyFromStrArg<T>
where
    T: PyFromStr,
{
    type Error = PyErr;

    #[inline]
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        // TODO (non-fugue-state-jesse): add support for bytearray/memview/buffer
        // protocol?
        if let Some(s) = obj.cast_exact_opt::<pyo3::types::PyString>() {
            let s = pystr_read_fast(s)?;
            return T::py_from_str(s).map_err(map_py_value_err).map(Self);
        }

        // extract subclass
        if let Ok(s) = obj.extract::<&str>() {
            return T::py_from_str(s).map_err(map_py_value_err).map(Self);
        }

        Err(pyo3::exceptions::PyTypeError::new_err("Expected a str"))
    }
}

// TODO: expand this to `PyFromStrArg`
pub struct PyParseArg<T>(T);

impl<'a, 'py, T> FromPyObject<'a, 'py> for PyParseArg<T>
where
    T: PyParse,
{
    type Error = PyErr;

    #[inline]
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        PyParse::py_parse(obj).map(Self)
    }
}

impl<T> PyParseArg<T> {
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}
