use bytes::Bytes;
use jiter::{map_json_error, PythonParse};
use pyo3::prelude::*;
//
// pub(crate) struct Pyo3Bytes(pub Bytes);
//
// impl Pyo3Bytes {
//     pub fn new(buf: Bytes) -> Self {
//         Self(buf)
//     }
// }
// impl From<Bytes> for Pyo3Bytes {
//     fn from(value: Bytes) -> Self {
//         Self::new(value)
//     }
// }
//
// impl<'py> IntoPyObject<'py> for Pyo3Bytes {
//     type Target = PyBytes;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         Ok(PyBytes::new(py, &self.0[..]))
//     }
// }

pub(crate) struct Pyo3JsonBytes(pub Bytes);

impl Pyo3JsonBytes {
    pub fn new(buf: Bytes) -> Self {
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
        let parse_builder = PythonParse {
            allow_inf_nan: true,
            cache_mode: ::jiter::StringCacheMode::All,
            partial_mode: ::jiter::PartialMode::Off,
            catch_duplicate_keys: false,
            float_mode: ::jiter::FloatMode::Float,
            // cache_mode = StringCacheMode::All,
            // partial_mode = PartialMode::Off,
            // catch_duplicate_keys = false,
            // float_mode = FloatMode::Float
        };
        // let b = slf.body.as_ref().unwrap();
        // let r =

        parse_builder
            .python_parse(py, b_u8)
            .map_err(|e| map_json_error(b_u8, &e))
        // r
    }
}
