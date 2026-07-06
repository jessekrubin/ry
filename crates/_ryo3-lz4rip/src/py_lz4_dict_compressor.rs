use lz4rip::block::DictCompressor;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

#[pyclass(name = "Lz4DictCompressor", module = "ry.ryo3")]
#[derive(Debug)]
pub struct PyLz4DictCompressor {
    inner: DictCompressor,
}

#[pymethods]
impl PyLz4DictCompressor {
    #[new]
    #[expect(clippy::needless_pass_by_value)]
    fn new(dictionary: ReadableBuffer) -> Self {
        Self {
            inner: DictCompressor::new(dictionary.as_ref()),
        }
    }

    #[expect(clippy::needless_pass_by_value)]
    fn compress(&mut self, py: Python<'_>, data: ReadableBuffer) -> RyBytes {
        let input = data.as_ref();
        py.detach(|| self.inner.compress(input)).into()
    }
}
