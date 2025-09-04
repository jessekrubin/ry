#![allow(clippy::needless_pass_by_value)]
use pyo3::types::{PyBytes, PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python, pyfunction, wrap_pyfunction};

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_core::PyLock;
use std::sync::Mutex;
use xxhash_rust::xxh32::Xxh32;

#[pyclass(name = "Xxh32", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh32 {
    seed: u32,
    hasher: Mutex<Xxh32>,
}

#[pymethods]
impl PyXxh32 {
    #[new]
    #[pyo3(signature = (b = None, seed = None))]
    fn py_new(b: Option<ryo3_bytes::PyBytes>, seed: Option<u32>) -> Self {
        match b {
            Some(s) => {
                let seed = seed.unwrap_or(0);
                let mut hasher = Xxh32::new(seed);
                hasher.update(s.as_ref());

                Self {
                    seed,
                    hasher: Mutex::new(hasher),
                }
            }
            None => Self {
                seed: seed.unwrap_or(0),
                hasher: Mutex::new(Xxh32::new(seed.unwrap_or(0))),
            },
        }
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        let hasher = self.hasher.py_lock()?;
        Ok(format!("xxh32<{:x}>", hasher.digest()))
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

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let hasher = self.hasher.py_lock()?;
        let digest = hasher.digest();
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u32> {
        self.hasher.py_lock().map(|hasher| hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        self.hasher
            .py_lock()
            .map(|hasher| format!("{:08x}", hasher.digest()))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, s: ryo3_bytes::PyBytes) -> PyResult<()> {
        let mut hasher = self.hasher.py_lock()?;
        hasher.update(s.as_ref());
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
    fn reset(&self, seed: Option<u32>) -> PyResult<()> {
        let mut hasher = self.hasher.py_lock()?;
        hasher.reset(seed.unwrap_or(self.seed));
        Ok(())
    }
}

/// Create a new xxh32 hasher
#[pyfunction]
#[pyo3(signature = (s = None, seed = 0))]
pub fn xxh32(s: Option<ryo3_bytes::PyBytes>, seed: Option<u32>) -> PyXxh32 {
    PyXxh32::py_new(s, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh32_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u32>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = xxhash_rust::xxh32::xxh32(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh32_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u32>) -> PyResult<u32> {
    Ok(xxhash_rust::xxh32::xxh32(b.as_ref(), seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh32_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u32>) -> PyResult<String> {
    Ok(format!(
        "{:08x}",
        xxhash_rust::xxh32::xxh32(b.as_ref(), seed.unwrap_or(0))
    ))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh32>()?;
    m.add_function(wrap_pyfunction!(xxh32, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_intdigest, m)?)?;
    Ok(())
}
