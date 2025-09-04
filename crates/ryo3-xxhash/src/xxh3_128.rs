use crate::py_digest::{PyDigest, PyHexDigest};
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes, PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, intern, pyclass, pyfunction, pymethods, wrap_pyfunction};
use ryo3_core::PyLock;
use std::sync::Mutex;
use twox_hash::XxHash3_128;
// use xxhash_rust::xxh3::{Xxh3, Xxh3Builder, xxh3_64_with_seed, xxh3_128_with_seed};
// ============================================================================

#[pyclass(name = "xxh3_128", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh3_128 {
    seed: u64,
    hasher: Mutex<XxHash3_128>,
}

#[pymethods]
impl PyXxh3_128 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0, secret = None))]
    fn py_new(
        data: Option<ryo3_bytes::PyBytes>,
        seed: Option<u64>,
        secret: Option<[u8; 192]>,
    ) -> PyResult<Self> {
        let seed = seed.unwrap_or(0);
        let hasher = if let Some(s) = secret {
            XxHash3_128::with_seed_and_secret(seed, s)
                .map_err(|_| PyValueError::new_err("Secret must be exactly 192 bytes long"))
        } else {
            Ok(XxHash3_128::with_seed(seed))
        }?;
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
            .map(|hasher| format!("xxh3<{:x}>", hasher.finish_128()))
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "xxh3_128")
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

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<PyDigest<u128>> {
        let digest = self.hasher.py_lock().map(|h| h.finish_128())?;
        Ok(PyDigest(digest))
    }

    fn intdigest(&self) -> PyResult<u128> {
        self.hasher.py_lock().map(|h| h.finish_128())
    }

    fn hexdigest(&self) -> PyResult<PyHexDigest<u128>> {
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
        *h = XxHash3_128::with_seed(self.seed);
        Ok(())
    }
}

// #[pyfunction]
// #[pyo3(signature = (data = None, *, seed = 0, secret = None))]
// pub fn xxh3_128(
//     data: Option<ryo3_bytes::PyBytes>,
//     seed: Option<u64>,
//     secret: Option<[u8; 192]>,
// ) -> PyResult<PyXxh3_128> {
//     PyXxh3_128::py_new(data, seed, secret)
// }
//
// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================

// #[expect(clippy::needless_pass_by_value)]
// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_64_digest(
//     py: Python<'_>,
//     data: ryo3_bytes::PyBytes,
//     seed: Option<u64>,
// ) -> PyDigest<u64> {
//     let digest = twox_hash::XxHash3_64::oneshot_with_seed(seed.unwrap_or(0), data.as_ref());
//     PyDigest(digest)
// }

// #[expect(clippy::needless_pass_by_value)]
// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_64_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> u64 {
//     twox_hash::XxHash3_64::oneshot_with_seed(seed.unwrap_or(0), data.as_ref())
// }

// #[expect(clippy::needless_pass_by_value)]
// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u64> {
//     twox_hash::XxHash3_64::oneshot_with_seed(seed.unwrap_or(0), data.as_ref()).into()
// }

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_128_digest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyDigest<u128> {
    XxHash3_128::oneshot_with_seed(seed.unwrap_or(0), data.as_ref()).into()
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_128_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u128> {
    Ok(twox_hash::XxHash3_128::oneshot_with_seed(
        seed.unwrap_or(0),
        data.as_ref(),
    ))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh3_128_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u128> {
    let digest = twox_hash::XxHash3_128::oneshot_with_seed(seed.unwrap_or(0), data.as_ref());
    PyHexDigest(digest)
}

// =======
// ALIASES
// =======
// #[expect(clippy::needless_pass_by_value)]
// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_digest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyDigest<u64> {
//     let digest = twox_hash::XxHash3_64::oneshot_with_seed(seed.unwrap_or(0), data.as_ref());
//     PyDigest(digest)
// }

// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> u64 {
//     xxh3_64_intdigest(data, seed)
// }
//
// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u64> {
//     xxh3_64_hexdigest(data, seed)
// }

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh128_digest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyDigest<u128> {
    twox_hash::XxHash3_128::oneshot_with_seed(seed.unwrap_or(0), data.as_ref()).into()
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh128_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u128> {
    xxh3_128_intdigest(data, seed)
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh128_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u128> {
    xxh3_128_hexdigest(data, seed)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh3_128>()?;
    // m.add_function(wrap_pyfunction!(, m)?)?;

    m.add_function(wrap_pyfunction!(xxh3_128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_intdigest, m)?)?;
    // m.add_function(wrap_pyfunction!(xxh3_64_digest, m)?)?;
    // m.add_function(wrap_pyfunction!(xxh3_64_hexdigest, m)?)?;
    // m.add_function(wrap_pyfunction!(xxh3_64_intdigest, m)?)?;
    //
    // // aliases
    m.add_function(wrap_pyfunction!(xxh128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_intdigest, m)?)?;
    // m.add_function(wrap_pyfunction!(xxh3_digest, m)?)?;
    // m.add_function(wrap_pyfunction!(xxh3_hexdigest, m)?)?;
    // m.add_function(wrap_pyfunction!(xxh3_intdigest, m)?)?;
    Ok(())
}
