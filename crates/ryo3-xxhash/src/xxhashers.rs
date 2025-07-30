use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use ryo3_core::PyLock;
use std::sync::Mutex;
use xxhash_rust::xxh3::{Xxh3, Xxh3Builder};
use xxhash_rust::xxh32::Xxh32;
use xxhash_rust::xxh64::Xxh64;

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
    #[pyo3(signature = (b = None, seed = 0))]
    fn py_new(b: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> Self {
        match b {
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
    fn update(&self, b: ryo3_bytes::PyBytes) -> PyResult<()> {
        let mut hasher = self.hasher.py_lock()?;
        hasher.update(b.as_ref());
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
#[pyo3(signature = (s = None, seed = 0))]
pub fn xxh64(s: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> PyResult<PyXxh64> {
    Ok(PyXxh64::py_new(s, seed))
}

#[pyclass(name = "Xxh3", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh3 {
    seed: u64,
    hasher: Mutex<Xxh3>,
}

#[pymethods]
impl PyXxh3 {
    #[new]
    #[pyo3(signature = (b = None, seed = 0, secret = None))]
    fn py_new(
        b: Option<ryo3_bytes::PyBytes>,
        seed: Option<u64>,
        secret: Option<[u8; 192]>,
    ) -> Self {
        let seed = seed.unwrap_or(0);
        let h = match secret {
            Some(s) => Xxh3Builder::new().with_seed(seed).with_secret(s).build(),
            None => Xxh3Builder::new().with_seed(seed).build(),
        };
        match b {
            Some(s) => {
                let mut hasher = h;
                hasher.update(s.as_ref());
                Self {
                    seed,
                    hasher: Mutex::new(hasher),
                }
            }
            None => Self {
                seed,
                hasher: Mutex::new(h),
            },
        }
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        self.hasher
            .py_lock()
            .map(|hasher| format!("xxh3<{:x}>", hasher.digest()))
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "xxh3")
    }

    #[classattr]
    fn digest_size() -> usize {
        16
    }

    #[classattr]
    fn block_size() -> usize {
        64
    }

    #[getter]
    fn seed(&self) -> u64 {
        self.seed
    }

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.hasher.py_lock().map(|h| h.digest())?;
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u64> {
        self.hasher.py_lock().map(|h| h.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        let digest = self.intdigest()?;
        Ok(format!("{digest:016x}"))
    }

    fn digest128<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.intdigest128()?;
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest128(&self) -> PyResult<u128> {
        self.hasher.py_lock().map(|h| h.digest128())
    }

    fn hexdigest128(&self) -> PyResult<String> {
        let digest = self.intdigest128()?;
        Ok(format!("{digest:032x}"))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, b: ryo3_bytes::PyBytes) -> PyResult<()> {
        // self.hasher.update(b.as_ref());
        let mut hasher = self.hasher.py_lock()?;
        hasher.update(b.as_ref());
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
        h.reset();
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (s = None, seed = 0, secret = None))]
pub fn xxh3(
    s: Option<ryo3_bytes::PyBytes>,
    seed: Option<u64>,
    secret: Option<[u8; 192]>,
) -> PyXxh3 {
    PyXxh3::py_new(s, seed, secret)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh32>()?;
    m.add_class::<PyXxh64>()?;
    m.add_class::<PyXxh3>()?;
    m.add_function(wrap_pyfunction!(xxh32, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3, m)?)?;
    Ok(())
}
