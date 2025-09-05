use pyo3::types::{PyBytes, PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, intern, pyclass, pyfunction, pymethods, wrap_pyfunction};
use ryo3_core::PyLock;
use std::sync::Mutex;
use xxhash_rust::xxh3::{Xxh3, Xxh3Builder, xxh3_64_with_seed, xxh3_128_with_seed};
// ============================================================================

#[pyclass(name = "Xxh3", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh3 {
    seed: u64,
    hasher: Mutex<Xxh3>,
}

#[pymethods]
impl PyXxh3 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0, secret = None))]
    fn py_new(
        data: Option<ryo3_bytes::PyBytes>,
        seed: Option<u64>,
        secret: Option<[u8; 192]>,
    ) -> Self {
        let seed = seed.unwrap_or(0);
        let h = match secret {
            Some(s) => Xxh3Builder::new().with_seed(seed).with_secret(s).build(),
            None => Xxh3Builder::new().with_seed(seed).build(),
        };
        match data {
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
    fn update(&self, data: ryo3_bytes::PyBytes) -> PyResult<()> {
        // self.hasher.update(b.as_ref());
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

    fn reset(&self) -> PyResult<()> {
        let mut h = self.hasher.py_lock()?;
        h.reset();
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (data = None, *, seed = 0, secret = None))]
pub fn xxh3(
    data: Option<ryo3_bytes::PyBytes>,
    seed: Option<u64>,
    secret: Option<[u8; 192]>,
) -> PyXxh3 {
    PyXxh3::py_new(data, seed, secret)
}

// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_64_digest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = xxh3_64_with_seed(data.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_64_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
    Ok(xxh3_64_with_seed(data.as_ref(), seed.unwrap_or(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:016x}",
        xxh3_64_with_seed(data.as_ref(), seed.unwrap_or(0))
    ))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_128_digest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = xxh3_128_with_seed(data.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_128_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u128> {
    Ok(xxh3_128_with_seed(data.as_ref(), seed.unwrap_or(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_128_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:032x}",
        xxh3_128_with_seed(data.as_ref(), seed.unwrap_or(0))
    ))
}

// =======
// ALIASES
// =======
#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_digest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = xxh3_64_with_seed(data.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
    xxh3_64_intdigest(data, seed)
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    xxh3_64_hexdigest(data, seed)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh128_digest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = xxh3_128_with_seed(data.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh128_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u128> {
    xxh3_128_intdigest(data, seed)
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh128_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    xxh3_128_hexdigest(data, seed)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh3>()?;
    m.add_function(wrap_pyfunction!(xxh3, m)?)?;

    m.add_function(wrap_pyfunction!(xxh3_128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_intdigest, m)?)?;

    // aliases
    m.add_function(wrap_pyfunction!(xxh128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_intdigest, m)?)?;
    Ok(())
}
