use crate::JiterParseOptions;
use bytes::Bytes;
use jiter::{map_json_error, PythonParse};
use pyo3::prelude::*;

pub struct JsonBytes {
    bytes: Bytes,
    options: JiterParseOptions,
}

impl JsonBytes {
    pub fn new(buf: Bytes, options: JiterParseOptions) -> Self {
        Self {
            bytes: buf,
            options: JiterParseOptions::default()
        }
    }
    pub fn new_with_options(buf: Bytes, options:JiterParseOptions) -> Self {
        Self {
            bytes: buf,
            options
        }
    }


    fn build_python_parser(&self) -> PythonParse {
        self.options.build_python_parse()
    }
}

impl From<Bytes> for JsonBytes {
    fn from(value: Bytes) -> Self {
        Self {
            bytes: value,
            options: JiterParseOptions::default()
        }
    }
}

impl<'py> IntoPyObject<'py> for JsonBytes {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let b_u8: &[u8] = &self.bytes;
        let parser = self.build_python_parser();
        parser
            .python_parse(py, b_u8)
            .map_err(|e| map_json_error(b_u8, &e))
    }
}
