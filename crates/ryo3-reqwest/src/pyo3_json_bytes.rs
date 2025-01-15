use bytes::Bytes;
use jiter::{map_json_error, PythonParse};
use pyo3::prelude::*;
pub(crate) struct Pyo3JsonBytes(pub Bytes);

impl Pyo3JsonBytes {
    pub(crate) fn new(buf: Bytes) -> Self {
        Self(buf)
    }
}

impl From<Bytes> for Pyo3JsonBytes {
    fn from(value: Bytes) -> Self {
        Self::new(value)
    }
}

impl<'py> IntoPyObject<'py> for Pyo3JsonBytes {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let b_u8 = &self.0[..];
        let parser = PythonParse {
            allow_inf_nan: true,
            cache_mode: ::jiter::StringCacheMode::All,
            partial_mode: ::jiter::PartialMode::Off,
            catch_duplicate_keys: false,
            float_mode: ::jiter::FloatMode::Float,
        };
        parser
            .python_parse(py, b_u8)
            .map_err(|e| map_json_error(b_u8, &e))
    }
}
