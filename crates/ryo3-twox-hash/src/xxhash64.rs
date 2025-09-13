use crate::py_digest::{PyDigest, PyHexDigest};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, pyfunction, wrap_pyfunction};
use ryo3_core::PyLock;
use std::hash::Hasher;
use std::sync::Mutex;
use twox_hash::XxHash64 as XxHash3_64;

#[pyclass(name = "xxh64", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3.xxhash"))]
pub struct PyXxHash64 {
    seed: u64,
    hasher: Mutex<XxHash3_64>,
}

#[pymethods]
impl PyXxHash64 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0))]
    fn py_new(data: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(0);
        let hasher = XxHash3_64::with_seed(seed);
        match data {
            Some(s) => {
                let mut hasher = hasher;
                hasher.write(s.as_ref());
                Self {
                    seed,
                    hasher: Mutex::new(hasher),
                }
            }
            None => Self {
                seed,
                hasher: Mutex::new(hasher),
            },
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        self.hasher
            .py_lock()
            .map(|hasher| format!("xxh64<{:x}>", hasher.finish()))
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "xxh64")
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
    fn seed(&self) -> u64 {
        self.seed
    }

    fn digest(&self) -> PyResult<PyDigest<u64>> {
        let digest = self.hasher.py_lock().map(|h| h.finish())?;

        Ok(PyDigest(digest))
    }

    fn intdigest(&self) -> PyResult<u64> {
        self.hasher.py_lock().map(|h| h.finish())
    }

    fn hexdigest(&self) -> PyResult<PyHexDigest<u64>> {
        let digest = self.intdigest()?;
        Ok(PyHexDigest(digest))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, data: ryo3_bytes::PyBytes) -> PyResult<()> {
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
        *h = XxHash3_64::with_seed(self.seed);
        Ok(())
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    #[pyo3(signature = (data, *, seed = None))]
    fn oneshot(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> u64 {
        twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref())
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_digest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyDigest<u64> {
    let digest = twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref());
    PyDigest(digest)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> u64 {
    twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref())
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u64> {
    twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref()).into()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxHash64>()?;
    m.add_function(wrap_pyfunction!(xxh64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_intdigest, m)?)?;
    Ok(())
}
