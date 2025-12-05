use ryo3_core::types::{PyDigest, PyHexDigest};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, intern, pyclass, pyfunction, pymethods, wrap_pyfunction};
use ryo3_core::PyMutex;
use twox_hash::XxHash3_128;

#[pyclass(name = "xxh3_128", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3.xxhash"))]
pub struct PyXxHash3_128 {
    seed: u64,
    hasher: PyMutex<XxHash3_128>,
}

#[pymethods]
impl PyXxHash3_128 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0, secret = None))]
    fn py_new(
        data: Option<ryo3_bytes::PyBytes>,
        seed: u64,
        secret: Option<[u8; 192]>,
    ) -> PyResult<Self> {
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
                    hasher: PyMutex::new(hasher),
                })
            }
            None => Ok(Self {
                seed,
                hasher: PyMutex::new(hasher),
            }),
        }
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

    fn digest(&self) -> PyResult<PyDigest<u128>> {
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
    fn update(&self, py: Python<'_>, data: ryo3_bytes::PyBytes) -> PyResult<()> {
        py.detach(|| {
            let mut hasher = self.hasher.py_lock()?;
            hasher.write(data.as_ref());
            Ok(())
        })
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
        *h = XxHash3_128::with_seed(seed.unwrap_or(self.seed));
        Ok(())
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    #[pyo3(signature = (data, *, seed = 0, secret = None))]
    fn oneshot(
        py: Python<'_>,
        data: ryo3_bytes::PyBytes,
        seed: u64,
        secret: Option<ryo3_bytes::PyBytes>,
    ) -> PyResult<u128> {
        py.detach(|| {
            if let Some(secret) = secret {
                twox_hash::XxHash3_128::oneshot_with_seed_and_secret(
                    seed,
                    secret.as_ref(),
                    data.as_ref(),
                )
                .map_err(|e| PyValueError::new_err(format!("invalid secret: {e}")))
            } else {
                Ok(twox_hash::XxHash3_128::oneshot_with_seed(
                    seed,
                    data.as_ref(),
                ))
            }
        })
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh3_128_digest(data: ryo3_bytes::PyBytes, seed: u64) -> PyDigest<u128> {
    XxHash3_128::oneshot_with_seed(seed, data.as_ref()).into()
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh3_128_intdigest(data: ryo3_bytes::PyBytes, seed: u64) -> PyResult<u128> {
    Ok(twox_hash::XxHash3_128::oneshot_with_seed(
        seed,
        data.as_ref(),
    ))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh3_128_hexdigest(data: ryo3_bytes::PyBytes, seed: u64) -> PyHexDigest<u128> {
    let digest = twox_hash::XxHash3_128::oneshot_with_seed(seed, data.as_ref());
    PyHexDigest(digest)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh128_digest(data: ryo3_bytes::PyBytes, seed: u64) -> PyDigest<u128> {
    twox_hash::XxHash3_128::oneshot_with_seed(seed, data.as_ref()).into()
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh128_intdigest(data: ryo3_bytes::PyBytes, seed: u64) -> PyResult<u128> {
    xxh3_128_intdigest(data, seed)
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = 0))]
pub fn xxh128_hexdigest(data: ryo3_bytes::PyBytes, seed: u64) -> PyHexDigest<u128> {
    xxh3_128_hexdigest(data, seed)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_class::<PyXxHash3_128>()?;

    m.add_function(wrap_pyfunction!(xxh3_128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_intdigest, m)?)?;

    // aliases
    m.add_function(wrap_pyfunction!(xxh128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_intdigest, m)?)?;

    // alias xxh3_128 as xxh128 (matches `xxhash` python package)
    let attr = m.getattr(intern!(py, "xxh3_128"))?;
    m.add(intern!(py, "xxh128"), attr)?;

    Ok(())
}
