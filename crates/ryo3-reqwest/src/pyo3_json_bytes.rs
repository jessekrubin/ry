use bytes::Bytes;
use jiter::map_json_error;
use pyo3::prelude::*;
use ryo3_jiter::JiterParseOptions;

pub(crate) struct Pyo3JsonBytes {
    pub bytes: Bytes,
    pub options: JiterParseOptions,
}

impl Pyo3JsonBytes {
    pub(crate) fn new(buf: Bytes, options: JiterParseOptions) -> Self {
        Self {
            bytes: buf,
            options,
        }
    }
}
impl From<(Bytes, JiterParseOptions)> for Pyo3JsonBytes {
    fn from(value: (Bytes, JiterParseOptions)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl From<Bytes> for Pyo3JsonBytes {
    fn from(value: Bytes) -> Self {
        Self::new(value, JiterParseOptions::default())
    }
}

impl<'py> IntoPyObject<'py> for Pyo3JsonBytes {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let b_u8 = &self.bytes[..];
        let parser = self.options.parser();
        parser
            .python_parse(py, b_u8)
            .map_err(|e| map_json_error(b_u8, &e))
    }
}
