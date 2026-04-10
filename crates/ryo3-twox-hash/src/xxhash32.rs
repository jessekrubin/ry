use std::hash::Hasher;

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_bytes::ReadableBuffer;
use ryo3_core::types::{PyDigest, PyHexDigest};
use ryo3_core::{PyAsciiString, RyMutex};
use twox_hash::XxHash32;

#[pyclass(name = "xxh32", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3.xxhash"))]
pub struct PyXxHash32 {
    seed: u32,
    hasher: RyMutex<XxHash32, true>,
}

#[pymethods]
impl PyXxHash32 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0))]
    fn py_new(data: Option<ReadableBuffer>, seed: u32) -> Self {
        let hasher = XxHash32::with_seed(seed);
        match data {
            Some(s) => {
                let mut hasher = hasher;
                hasher.write(s.as_ref());
                Self {
                    seed,
                    hasher: hasher.into(),
                }
            }
            None => Self {
                seed,
                hasher: hasher.into(),
            },
        }
    }

    fn __repr__(&self) -> PyResult<PyAsciiString> {
        self.hasher
            .py_lock()
            .map(|hasher| format!("xxh32<{:x}>", hasher.finish()).into())
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "xxh32")
    }

    #[classattr]
    fn digest_size() -> usize {
        4
    }

    #[classattr]
    fn block_size() -> usize {
        16
    }

    #[getter]
    fn seed(&self) -> u32 {
        self.seed
    }

    #[getter]
    fn length(&self) -> PyResult<u64> {
        self.hasher.py_lock().map(|h| h.total_len())
    }

    fn digest(&self) -> PyResult<PyDigest<u32>> {
        let digest = self.hasher.py_lock().map(|h| h.finish_32())?;

        Ok(PyDigest(digest))
    }

    fn intdigest(&self) -> PyResult<u32> {
        self.hasher.py_lock().map(|h| h.finish_32())
    }

    fn hexdigest(&self) -> PyResult<PyHexDigest<u32>> {
        let digest = self.intdigest()?;
        Ok(PyHexDigest(digest))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, py: Python<'_>, data: ReadableBuffer) -> PyResult<()> {
        let slice = data.as_ref();
        py.detach(|| {
            let mut hasher = self.hasher.py_lock()?;
            hasher.write(slice);
            Ok(())
        })
    }

    fn copy(&self) -> PyResult<Self> {
        let hasher = self.hasher.py_lock()?;
        Ok(Self {
            hasher: hasher.clone().into(),
            seed: self.seed,
        })
    }

    #[pyo3(signature = (*, seed = None))]
    fn reset(&self, seed: Option<u32>) -> PyResult<()> {
        let mut h = self.hasher.py_lock()?;
        *h = XxHash32::with_seed(seed.unwrap_or(self.seed));
        Ok(())
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    #[pyo3(signature = (data, *, seed = 0))]
    fn oneshot(py: Python<'_>, data: ReadableBuffer, seed: u32) -> PyDigest<u32> {
        let slice = data.as_ref();
        py.detach(|| XxHash32::oneshot(seed, slice).into())
    }
}

// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================
#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh32_digest(py: Python<'_>, data: ReadableBuffer, seed: u32) -> PyDigest<u32> {
    let slice = data.as_ref();
    py.detach(|| XxHash32::oneshot(seed, slice).into())
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh32_intdigest(py: Python<'_>, data: ReadableBuffer, seed: u32) -> u32 {
    let slice = data.as_ref();
    py.detach(|| XxHash32::oneshot(seed, slice))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh32_hexdigest(py: Python<'_>, data: ReadableBuffer, seed: u32) -> PyHexDigest<u32> {
    let slice = data.as_ref();
    py.detach(|| XxHash32::oneshot(seed, slice).into())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxHash32>()?;
    m.add_function(wrap_pyfunction!(xxh32_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_intdigest, m)?)?;
    Ok(())
}
