use ryo3_core::types::{PyDigest, PyHexDigest};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{Bound, PyResult, intern};
use ryo3_core::PyMutex;
use std::hash::Hasher;
use twox_hash::XxHash3_64;

#[pyclass(name = "xxh3_64", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3.xxhash"))]
pub struct PyXxHash3_64 {
    seed: u64,
    hasher: PyMutex<XxHash3_64, true>,
}

#[pymethods]
impl PyXxHash3_64 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0, secret = None))]
    fn py_new(
        data: Option<ryo3_bytes::PyBytes>,
        seed: u64,
        secret: Option<[u8; 192]>,
    ) -> PyResult<Self> {
        let hasher = if let Some(s) = secret {
            XxHash3_64::with_seed_and_secret(seed, s)
                .map_err(|_| PyValueError::new_err("Secret must be exactly 192 bytes long"))
        } else {
            Ok(XxHash3_64::with_seed(seed))
        }?;
        match data {
            Some(s) => {
                let mut hasher = hasher;
                hasher.write(s.as_ref());
                Ok(Self {
                    seed,
                    hasher: hasher.into(),
                })
            }
            None => Ok(Self {
                seed,
                hasher: hasher.into(),
            }),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        self.hasher
            .py_lock()
            .map(|hasher| format!("xxh3<{:x}>", hasher.finish()))
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "xxh3_64")
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

    fn hexdigest(&self) -> PyResult<String> {
        let digest = self.intdigest()?;
        Ok(format!("{digest:016x}"))
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
        *h = XxHash3_64::with_seed(seed.unwrap_or(self.seed));
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
    ) -> PyResult<u64> {
        py.detach(|| {
            if let Some(secret) = secret {
                twox_hash::XxHash3_64::oneshot_with_seed_and_secret(
                    seed,
                    secret.as_ref(),
                    data.as_ref(),
                )
                .map_err(|e| PyValueError::new_err(format!("invalid secret: {e}")))
            } else {
                Ok(twox_hash::XxHash3_64::oneshot_with_seed(
                    seed,
                    data.as_ref(),
                ))
            }
        })
    }
}

// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================

#[pyfunction]
#[pyo3(signature = (data, *, seed = 0, secret = None))]
pub fn xxh3_64_digest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: u64,
    secret: Option<ryo3_bytes::PyBytes>,
) -> PyResult<PyDigest<u64>> {
    PyXxHash3_64::oneshot(py, data, seed, secret).map(PyDigest::from)
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = 0, secret = None))]
pub fn xxh3_64_intdigest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: u64,
    secret: Option<ryo3_bytes::PyBytes>,
) -> PyResult<u64> {
    PyXxHash3_64::oneshot(py, data, seed, secret)
}

#[pyfunction]
#[pyo3(signature = (data, *, seed = 0, secret = None))]
pub fn xxh3_64_hexdigest(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    seed: u64,
    secret: Option<ryo3_bytes::PyBytes>,
) -> PyResult<PyHexDigest<u64>> {
    PyXxHash3_64::oneshot(py, data, seed, secret).map(PyHexDigest::from)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxHash3_64>()?;
    m.add_function(wrap_pyfunction!(xxh3_64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_intdigest, m)?)?;
    Ok(())
}
