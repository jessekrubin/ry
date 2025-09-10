#![expect(clippy::needless_pass_by_value)]
use crate::py_digest::{PyDigest, PyHexDigest};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, pyfunction, wrap_pyfunction};
use ryo3_core::PyLock;
use std::hash::Hasher;
use std::sync::Mutex;
use twox_hash::XxHash32;

#[pyclass(name = "xxh32", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh32 {
    seed: u32,
    hasher: Mutex<XxHash32>,
}

#[pymethods]
impl PyXxh32 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0))]
    fn py_new(data: Option<ryo3_bytes::PyBytes>, seed: Option<u32>) -> PyResult<Self> {
        let seed = seed.unwrap_or(0);
        let hasher = XxHash32::with_seed(seed);
        match data {
            Some(s) => {
                let mut hasher = hasher;
                hasher.write(s.as_ref());
                Ok(Self {
                    seed,
                    hasher: Mutex::new(hasher),
                })
            }
            None => Ok(Self {
                seed,
                hasher: Mutex::new(hasher),
            }),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        self.hasher
            .py_lock()
            .map(|hasher| format!("xxh32<{:x}>", hasher.finish()))
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "xxh32")
    }

    #[classattr]
    fn digest_size() -> usize {
        8
    }

    #[classattr]
    fn block_size() -> usize {
        32
    }

    #[getter]
    fn seed(&self) -> u32 {
        self.seed
    }

    fn digest<'py>(&self) -> PyResult<PyDigest<u32>> {
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
    fn update(&self, data: ryo3_bytes::PyBytes) -> PyResult<()> {
        // self.hasher.update(b.as_ref());
        let mut hasher = self.hasher.py_lock()?;
        hasher.write(data.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        let hasher = self.hasher.py_lock()?;
        Ok(Self {
            hasher: Mutex::new(hasher.clone()),
            seed: self.seed,
        })
    }

    fn reset(&self) -> PyResult<()> {
        let mut h = self.hasher.py_lock()?;
        *h = XxHash32::with_seed(self.seed);
        Ok(())
    }
}

// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================
#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh32_digest(data: ryo3_bytes::PyBytes, seed: Option<u32>) -> PyDigest<u32> {
    let digest = twox_hash::XxHash32::oneshot(seed.unwrap_or(0), data.as_ref());
    PyDigest(digest)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh32_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u32>) -> u32 {
    twox_hash::XxHash32::oneshot(seed.unwrap_or(0), data.as_ref())
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh32_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u32>) -> PyHexDigest<u32> {
    twox_hash::XxHash32::oneshot(seed.unwrap_or(0), data.as_ref()).into()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh32>()?;
    m.add_function(wrap_pyfunction!(xxh32_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_intdigest, m)?)?;
    Ok(())
}
