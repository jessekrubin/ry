#![expect(clippy::needless_pass_by_value)]
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, pyfunction, wrap_pyfunction};
use ryo3_core::PyLock;
use std::sync::Mutex;
use xxhash_rust::xxh64::Xxh64;

/// Python-Xxh64 hasher
#[pyclass(name = "Xxh64", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh64 {
    seed: u64,
    hasher: Mutex<Xxh64>,
}

#[pymethods]
impl PyXxh64 {
    /// Create a new Xxh64 hasher
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0))]
    fn py_new(data: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> Self {
        match data {
            Some(s) => {
                let mut hasher = Xxh64::new(seed.unwrap_or(0));
                hasher.update(s.as_ref());
                let seed = seed.unwrap_or(0);
                let hasher = Mutex::new(hasher);
                Self { seed, hasher }
            }
            None => Self {
                seed: seed.unwrap_or(0),
                hasher: Mutex::new(Xxh64::new(seed.unwrap_or(0))),
            },
        }
    }

    /// Return the string representation of the hasher
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    /// Return the string representation of the hasher
    fn __repr__(&self) -> PyResult<String> {
        let hasher = self.hasher.py_lock()?;
        let digest = hasher.digest();
        Ok(format!("xxh64<{digest:x}>"))
    }

    /// Return the name of the hasher ('xxh64')
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

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.hasher.py_lock().map(|hasher| hasher.digest())?;
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u64> {
        self.hasher.py_lock().map(|hasher| hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        let digest = self.intdigest()?;
        Ok(format!("{digest:016x}"))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, data: ryo3_bytes::PyBytes) -> PyResult<()> {
        let mut hasher = self.hasher.py_lock()?;
        hasher.update(data.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        let hasher = self.hasher.py_lock()?;
        Ok(Self {
            hasher: Mutex::new(hasher.clone()),
            seed: self.seed,
        })
    }

    #[pyo3(signature = (seed = None))]
    fn reset(&self, seed: Option<u64>) -> PyResult<()> {
        let mut hasher = self.hasher.py_lock()?;
        hasher.reset(seed.unwrap_or(self.seed));
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (data = None, seed = 0))]
pub fn xxh64(data: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> PyResult<PyXxh64> {
    Ok(PyXxh64::py_new(data, seed))
}

// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_digest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = xxhash_rust::xxh64::xxh64(data.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
    Ok(xxhash_rust::xxh64::xxh64(data.as_ref(), seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:016x}",
        xxhash_rust::xxh64::xxh64(data.as_ref(), seed.unwrap_or(0))
    ))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh64>()?;
    m.add_function(wrap_pyfunction!(xxh64, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_intdigest, m)?)?;
    Ok(())
}
