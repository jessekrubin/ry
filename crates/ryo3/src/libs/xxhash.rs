use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{wrap_pyfunction, PyResult, Python};
use xxhash_rust::const_xxh3::xxh3_128_with_seed as const_xxh3_128_with_seed;
use xxhash_rust::const_xxh3::xxh3_64_with_seed as const_xxh3_64_with_seed;
use xxhash_rust::const_xxh32::xxh32 as const_xxh32;
use xxhash_rust::const_xxh64::xxh64 as const_xxh64;
use xxhash_rust::xxh3::{Xxh3, Xxh3Builder};
use xxhash_rust::xxh32::Xxh32;
use xxhash_rust::xxh64::Xxh64;

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh32_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u32>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh32(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh32_intdigest(b: &[u8], seed: Option<u32>) -> PyResult<u32> {
    Ok(const_xxh32(b, seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh32_hexdigest(b: &[u8], seed: Option<u32>) -> PyResult<String> {
    Ok(format!("{:08x}", const_xxh32(b, seed.unwrap_or(0))))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh64_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh64(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh64_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u64> {
    Ok(const_xxh64(b, seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh64_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    Ok(format!("{:016x}", const_xxh64(b, seed.unwrap_or(0))))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_64_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh3_64_with_seed(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_64_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u64> {
    Ok(const_xxh3_64_with_seed(b, seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_64_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:016x}",
        const_xxh3_64_with_seed(b, seed.unwrap_or(0))
    ))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_128_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh3_128_with_seed(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_128_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u128> {
    Ok(const_xxh3_128_with_seed(b, seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_128_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:032x}",
        const_xxh3_128_with_seed(b, seed.unwrap_or(0))
    ))
}

// =======
// ALIASES
// =======
#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_digest<'a>(py: Python<'a>, b: &'a [u8], seed: Option<u64>) -> PyResult<Bound<'a, PyBytes>> {
    xxh3_64_digest(py, b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u64> {
    xxh3_64_intdigest(b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh3_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    xxh3_64_hexdigest(b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh128_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    xxh3_128_digest(py, b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh128_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u128> {
    xxh3_128_intdigest(b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
fn xxh128_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    xxh3_128_hexdigest(b, seed)
}

#[pyclass(name = "Xxh32")]
pub struct PyXxh32 {
    seed: u32,
    pub hasher: Xxh32,
}

#[pymethods]
impl PyXxh32 {
    #[new]
    #[pyo3(signature = (b = None, seed = None))]
    fn new(b: Option<&[u8]>, seed: Option<u32>) -> Self {
        match b {
            Some(s) => {
                let seed = seed.unwrap_or(0);
                let mut hasher = Xxh32::new(seed);
                hasher.update(s);
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

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("xxh32".to_string())
    }

    #[getter]
    fn seed(&self) -> PyResult<u32> {
        Ok(self.seed)
    }

    fn digest<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyBytes>> {
        let digest = self.hasher.digest();
        Ok(PyBytes::new_bound(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u32> {
        Ok(self.hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:08x}", self.hasher.digest()))
    }

    fn update(&mut self, s: &[u8]) -> PyResult<()> {
        self.hasher.update(s);
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
pub fn xxh32(s: Option<&[u8]>, seed: Option<u32>) -> PyResult<PyXxh32> {
    Ok(PyXxh32::new(s, seed))
}

/// Python-Xxh64 hasher
#[pyclass(name = "Xxh64")]
pub struct PyXxh64 {
    seed: u64,
    pub hasher: Xxh64,
}

#[pymethods]
impl PyXxh64 {
    /// Create a new Xxh64 hasher
    #[new]
    #[pyo3(signature = (b = None, seed = 0))]
    fn new(b: Option<&[u8]>, seed: Option<u64>) -> Self {
        match b {
            Some(s) => {
                let mut hasher = Xxh64::new(seed.unwrap_or(0));
                hasher.update(s);
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
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("xxh64".to_string())
    }

    #[getter]
    fn seed(&self) -> PyResult<u64> {
        Ok(self.seed)
    }
    fn digest<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyBytes>> {
        let digest = self.hasher.digest();
        Ok(PyBytes::new_bound(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u64> {
        Ok(self.hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:016x}", self.hasher.digest()))
    }

    fn update(&mut self, b: &[u8]) -> PyResult<()> {
        self.hasher.update(b);
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
pub fn xxh64(s: Option<&[u8]>, seed: Option<u64>) -> PyResult<PyXxh64> {
    Ok(PyXxh64::new(s, seed))
}

#[pyclass]
pub struct PyXxh3 {
    seed: u64,
    pub hasher: Xxh3,
}

#[pymethods]
impl PyXxh3 {
    #[new]
    #[pyo3(signature = (b = None, seed = 0, secret = None))]
    fn new(b: Option<&[u8]>, seed: Option<u64>, secret: Option<[u8; 192]>) -> Self {
        let seed = seed.unwrap_or(0);
        let h = match secret {
            Some(s) => Xxh3Builder::new().with_seed(seed).with_secret(s).build(),
            None => Xxh3Builder::new().with_seed(seed).build(),
        };
        match b {
            Some(s) => {
                let mut hasher = h.clone();
                hasher.update(s);
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

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok("xxh3".to_string())
    }

    #[getter]
    fn seed(&self) -> PyResult<u64> {
        Ok(self.seed)
    }

    fn digest<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyBytes>> {
        let digest = self.hasher.digest();
        Ok(PyBytes::new_bound(py, &digest.to_be_bytes()))
    }

    fn intdigest(&self) -> PyResult<u64> {
        Ok(self.hasher.digest())
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(format!("{:016x}", self.hasher.digest()))
    }

    fn digest128<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyBytes>> {
        let digest = self.hasher.digest128();
        Ok(PyBytes::new_bound(py, &digest.to_be_bytes()))
    }

    fn intdigest128(&self) -> PyResult<u128> {
        Ok(self.hasher.digest128())
    }

    fn hexdigest128(&self) -> PyResult<String> {
        Ok(format!("{:032x}", self.hasher.digest128()))
    }

    fn update(&mut self, b: &[u8]) -> PyResult<()> {
        self.hasher.update(b);
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
pub fn xxh3(s: Option<&[u8]>, seed: Option<u64>, secret: Option<[u8; 192]>) -> PyResult<PyXxh3> {
    Ok(PyXxh3::new(s, seed, secret))
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(xxh32_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh3_64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh3_128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_hexdigest, m)?)?;

    // aliases
    m.add_function(wrap_pyfunction!(xxh3_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_hexdigest, m)?)?;

    m.add_class::<PyXxh32>()?;
    m.add_class::<PyXxh64>()?;
    m.add_class::<PyXxh3>()?;
    m.add_function(wrap_pyfunction!(xxh32, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3, m)?)?;
    Ok(())
}
