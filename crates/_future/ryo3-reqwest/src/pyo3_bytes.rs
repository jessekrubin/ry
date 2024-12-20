use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub(crate) struct Pyo3Bytes(pub Bytes);

impl Pyo3Bytes {
    pub fn new(buf: Bytes) -> Self {
        Self(buf)
    }

    // pub fn new_multiple(buffers: Vec<Bytes>) -> Self {
    //     Self(buffers)
    // }
}

// TODO: return buffer protocol object? This isn't possible on an array of Bytes, so if you want to
// support the buffer protocol in the future (e.g. for get_range) you may need to have a separate
// wrapper of Bytes
impl<'py> IntoPyObject<'py> for Pyo3Bytes {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // Copy all internal Bytes objects into a single PyBytes
        // Since our inner callback is infallible, this will only panic on out of memory
        Ok(PyBytes::new(py, &self.0[..]))
        // for buf in self.0.iter() {
    }
}
