#![expect(clippy::needless_pass_by_value)]

use crate::py_digest::{PyDigest, PyHexDigest};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyModuleMethods, PyString};
use pyo3::{Bound, PyResult, Python, pyfunction, wrap_pyfunction};
use ryo3_core::PyLock;
use std::hash::Hasher;
use std::sync::Mutex;
use twox_hash::XxHash64 as XxHash3_64;
// use xxhash_rust::xxh64::Xxh64;

// /// Python-Xxh64 hasher
// #[pyclass(name = "Xxh64", module = "ry.ryo3.xxhash", frozen)]
// pub struct PyXxh64 {
//     seed: u64,
//     hasher: Mutex<Xxh64>,
// }
//
// #[pymethods]
// impl PyXxh64 {
//     /// Create a new Xxh64 hasher
//     #[new]
//     #[pyo3(signature = (data = None, *, seed = 0))]
//     fn py_new(data: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> Self {
//         match data {
//             Some(s) => {
//                 let mut hasher = Xxh64::new(seed.unwrap_or(0));
//                 hasher.update(s.as_ref());
//                 let seed = seed.unwrap_or(0);
//                 let hasher = Mutex::new(hasher);
//                 Self { seed, hasher }
//             }
//             None => Self {
//                 seed: seed.unwrap_or(0),
//                 hasher: Mutex::new(Xxh64::new(seed.unwrap_or(0))),
//             },
//         }
//     }
//
//     /// Return the string representation of the hasher
//     fn __str__(&self) -> PyResult<String> {
//         self.__repr__()
//     }
//
//     /// Return the string representation of the hasher
//     fn __repr__(&self) -> PyResult<String> {
//         let hasher = self.hasher.py_lock()?;
//         let digest = hasher.digest();
//         Ok(format!("xxh64<{digest:x}>"))
//     }
//
//     /// Return the name of the hasher ('xxh64')
//     #[classattr]
//     fn name(py: Python<'_>) -> &Bound<'_, PyString> {
//         intern!(py, "xxh64")
//     }
//
//     #[classattr]
//     fn digest_size() -> usize {
//         8
//     }
//
//     #[classattr]
//     fn block_size() -> usize {
//         32
//     }
//
//     #[getter]
//     fn seed(&self) -> u64 {
//         self.seed
//     }
//
//     fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
//         let digest = self.hasher.py_lock().map(|hasher| hasher.digest())?;
//         Ok(PyBytes::new(py, &digest.to_be_bytes()))
//     }
//
//     fn intdigest(&self) -> PyResult<u64> {
//         self.hasher.py_lock().map(|hasher| hasher.digest())
//     }
//
//     fn hexdigest(&self) -> PyResult<String> {
//         let digest = self.intdigest()?;
//         Ok(format!("{digest:016x}"))
//     }
//
//     #[expect(clippy::needless_pass_by_value)]
//     fn update(&self, data: ryo3_bytes::PyBytes) -> PyResult<()> {
//         let mut hasher = self.hasher.py_lock()?;
//         hasher.update(data.as_ref());
//         Ok(())
//     }
//
//     fn copy(&self) -> PyResult<Self> {
//         let hasher = self.hasher.py_lock()?;
//         Ok(Self {
//             hasher: Mutex::new(hasher.clone()),
//             seed: self.seed,
//         })
//     }
//
//     #[pyo3(signature = (seed = None))]
//     fn reset(&self, seed: Option<u64>) -> PyResult<()> {
//         let mut hasher = self.hasher.py_lock()?;
//         hasher.reset(seed.unwrap_or(self.seed));
//         Ok(())
//     }
// }

// #[pyfunction]
// #[pyo3(signature = (data = None, seed = 0))]
// pub fn xxh64(data: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> PyResult<PyXxh64> {
//     Ok(PyXxh64::py_new(data, seed))
// }

#[pyclass(name = "xxh64", module = "ry.ryo3.xxhash", frozen)]
pub struct PyXxh64 {
    seed: u64,
    hasher: Mutex<XxHash3_64>,
}

#[pymethods]
impl PyXxh64 {
    #[new]
    #[pyo3(signature = (data = None, *, seed = 0))]
    fn py_new(data: Option<ryo3_bytes::PyBytes>, seed: Option<u64>) -> PyResult<Self> {
        let seed = seed.unwrap_or(0);
        let hasher = XxHash3_64::with_seed(seed);
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

    fn digest<'py>(&self) -> PyResult<PyDigest<u64>> {
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
        *h = XxHash3_64::with_seed(self.seed);
        Ok(())
    }
}

// ====================================================================================
// ONCE SHOT FUNCTIONS
// ====================================================================================
// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh64_digest(py: Python<'_>, data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyDigest<u64> {
//     let v = twox_hash::xxhash64::Hasher::oneshot(seed.unwrap_or(0), data.as_ref());
//     PyDigest(v)
// }

// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh64_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
// }

// #[pyfunction]
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u64> {
//     let v = xxhash_rust::xxh64::xxh64(data.as_ref(), seed.unwrap_or(0));
//     PyHexDigest(v)
// }
#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_digest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyDigest<u64> {
    let digest = twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref());
    PyDigest(digest)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_intdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> u64 {
    twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref())
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, *, seed = None))]
pub fn xxh64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyHexDigest<u64> {
    twox_hash::XxHash64::oneshot(seed.unwrap_or(0), data.as_ref()).into()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXxh64>()?;
    m.add_function(wrap_pyfunction!(xxh64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_hexdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_intdigest, m)?)?;
    Ok(())
}
