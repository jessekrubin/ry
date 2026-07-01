use lz4rip::block::Compressor;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

#[pyclass(name = "Lz4DictCompressor", module = "ry.ryo3")]
#[derive(Debug)]
pub struct PyLz4DictCompressor {
    inner: Compressor,
}

#[pymethods]
impl PyLz4DictCompressor {
    #[new]
    #[expect(clippy::needless_pass_by_value)]
    fn new(dictionary: ReadableBuffer) -> Self {
        Self {
            inner: Compressor::with_dict(dictionary.as_ref()),
        }
    }

    #[expect(clippy::needless_pass_by_value)]
    fn compress(&mut self, py: Python<'_>, data: ReadableBuffer) -> RyBytes {
        let input = data.as_ref();
        py.detach(|| self.inner.compress(input)).into()
    }
}
