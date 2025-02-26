use pyo3::types::{PyBytes, PyModule, PyModuleMethods};
use pyo3::{pyclass, pyfunction, pymethods, wrap_pyfunction, Bound, PyResult, Python};
use xxhash_rust::xxh3::{Xxh3, Xxh3Builder};
use xxhash_rust::xxh32::Xxh32;
use xxhash_rust::xxh64::Xxh64;

#[pyclass(name = "Xxh32", module = "ryo3")]
pub struct PyXxh32 {
    seed: u32,
    pub hasher: Xxh32,
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
                Self { seed, hasher }
            }
            None => Self {
                seed: seed.unwrap_or(0),
                hasher: Xxh32::new(seed.unwrap_or(0)),
            },
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("xxh32<{:x}>", self.hasher.digest()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("xxh32<{:x}>", self.hasher.digest()))
    }

    #[classattr]
    fn name() -> PyResult<String> {
        Ok("xxh32".to_string())
    }

    #[getter]
    fn seed(&self) -> PyResult<u32> {
        Ok(self.seed)
    }

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.hasher.digest();
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u32> {
        Ok(self.hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:08x}", self.hasher.digest()))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&mut self, s: ryo3_bytes::PyBytes) -> PyResult<()> {
        self.hasher.update(s.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        Ok(Self {
            hasher: self.hasher.clone(),
            seed: self.seed,
        })
    }

    #[pyo3(signature = (seed = None))]
    fn reset(&mut self, seed: Option<u32>) -> PyResult<()> {
        self.hasher.reset(seed.unwrap_or(self.seed));
        Ok(())
    }
}

/// Create a new Xxh32 hasher
#[pyfunction]
#[pyo3(signature = (s = None, seed = 0))]
pub fn xxh32(s: Option<ryo3_bytes::PyBytes>, seed: Option<u32>) -> PyResult<PyXxh32> {
    Ok(PyXxh32::py_new(s, seed))
}

/// Python-Xxh64 hasher
#[pyclass(name = "Xxh64", module = "ryo3")]
pub struct PyXxh64 {
    seed: u64,
    pub hasher: Xxh64,
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
                Self { seed, hasher }
            }
            None => Self {
                seed: seed.unwrap_or(0),
                hasher: Xxh64::new(seed.unwrap_or(0)),
            },
        }
    }

    /// Return the string representation of the hasher
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("xxh64<{:x}>", self.hasher.digest()))
    }

    /// Return the string representation of the hasher
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("xxh64<{:x}>", self.hasher.digest()))
    }

    /// Return the name of the hasher ('xxh64')
    #[classattr]
    fn name() -> PyResult<String> {
        Ok("xxh64".to_string())
    }

    #[getter]
    fn seed(&self) -> PyResult<u64> {
        Ok(self.seed)
    }
    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.hasher.digest();
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u64> {
        Ok(self.hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:016x}", self.hasher.digest()))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&mut self, b: ryo3_bytes::PyBytes) -> PyResult<()> {
        self.hasher.update(b.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        Ok(Self {
            hasher: self.hasher.clone(),
            seed: self.seed,
        })
    }

    #[pyo3(signature = (seed = None))]
    fn reset(&mut self, seed: Option<u64>) -> PyResult<()> {
        self.hasher.reset(seed.unwrap_or(self.seed));
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (s = None, seed = 0))]
pub fn xxh64(s: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> PyResult<PyXxh64> {
    Ok(PyXxh64::py_new(s, seed))
}

#[pyclass(name = "Xxh3", module = "ryo3")]
pub struct PyXxh3 {
    seed: u64,
    pub hasher: Xxh3,
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
                let mut hasher = h.clone();
                hasher.update(s.as_ref());
                Self { seed, hasher }
            }
            None => Self { seed, hasher: h },
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("xxh3<{:x}>", self.hasher.digest()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("xxh3<{:x}>", self.hasher.digest()))
    }

    #[classattr]
    fn name() -> String {
        "xxh3".to_string()
    }

    #[getter]
    fn seed(&self) -> PyResult<u64> {
        Ok(self.seed)
    }

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.hasher.digest();
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u64> {
        Ok(self.hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:016x}", self.hasher.digest()))
    }

    fn digest128<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let digest = self.hasher.digest128();
        Ok(PyBytes::new(py, &digest.to_be_bytes()))
    }

    fn intdigest128(&self) -> PyResult<u128> {
        Ok(self.hasher.digest128())
    }

    fn hexdigest128(&self) -> PyResult<String> {
        Ok(format!("{:032x}", self.hasher.digest128()))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&mut self, b: ryo3_bytes::PyBytes) -> PyResult<()> {
        self.hasher.update(b.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        Ok(Self {
            hasher: self.hasher.clone(),
            seed: self.seed,
        })
    }

    fn reset(&mut self) -> PyResult<()> {
        self.hasher.reset();
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (s = None, seed = 0, secret = None))]
pub fn xxh3(
    s: Option<ryo3_bytes::PyBytes>,
    seed: Option<u64>,
    secret: Option<[u8; 192]>,
) -> PyResult<PyXxh3> {
    Ok(PyXxh3::py_new(s, seed, secret))
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
