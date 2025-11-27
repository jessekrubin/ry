use crate::py_digest::{PyDigest, PyHexDigest};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, pyfunction, wrap_pyfunction};
use ryo3_core::PyMutex;
use std::hash::Hasher;
use twox_hash::XxHash64 as XxHash3_64;

#[pyclass(name = "xxh64", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3.xxhash"))]
pub struct PyXxHash64 {
    seed: u64,
    hasher: PyMutex<XxHash3_64>,
}

#[pymethods]
impl PyXxHash64 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0))]
    fn py_new(data: Option<ryo3_bytes::PyBytes>, seed: u64) -> Self {
        let hasher = XxHash3_64::with_seed(seed);
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
            hasher: hasher.clone().into(),
            seed: self.seed,
        })
    }

    #[pyo3(signature = (*, seed = None))]
    fn reset(&self, seed: Option<u64>) -> PyResult<()> {
        let mut h = self.hasher.py_lock()?;
        *h = XxHash3_64::with_seed(seed.unwrap_or(self.seed));
        Ok(())
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    #[pyo3(signature = (data, *, seed = 0))]
    fn oneshot(py: Python<'_>, data: ryo3_bytes::PyBytes, seed: u64) -> u64 {
        py.detach(|| twox_hash::XxHash64::oneshot(seed, data.as_ref()))
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh64_digest(py: Python<'_>, data: ryo3_bytes::PyBytes, seed: u64) -> PyDigest<u64> {
    py.detach(|| {
        let digest = twox_hash::XxHash64::oneshot(seed, data.as_ref());
        PyDigest(digest)
    })
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh64_intdigest(py: Python<'_>, data: ryo3_bytes::PyBytes, seed: u64) -> u64 {
    py.detach(|| twox_hash::XxHash64::oneshot(seed, data.as_ref()))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh64_hexdigest(py: Python<'_>, data: ryo3_bytes::PyBytes, seed: u64) -> PyHexDigest<u64> {
    py.detach(|| twox_hash::XxHash64::oneshot(seed, data.as_ref()).into())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxHash64>()?;
    m.add_function(wrap_pyfunction!(xxh64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_intdigest, m)?)?;
    Ok(())
}
