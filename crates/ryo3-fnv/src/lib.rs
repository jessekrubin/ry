#![doc = include_str!("../README.md")]
use std::hash::Hasher;

use pyo3::types::{PyModule, PyString, PyTuple};

use pyo3::{IntoPyObjectExt, intern, prelude::*};
use pyo3::{PyResult, wrap_pyfunction};

use fnv::FnvHasher;
use std::sync::Mutex;

#[pyclass(name = "FnvHasher", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFnvHasher(Mutex<FnvHasher>);

impl PyFnvHasher {
    fn lock(&self) -> PyResult<std::sync::MutexGuard<'_, FnvHasher>> {
        self.0.lock().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to lock hasher: {e}"))
        })
    }

    fn finish(&self) -> PyResult<u64> {
        self.lock().map(|h| h.finish())
    }
}

impl From<FnvHasher> for PyFnvHasher {
    fn from(hasher: FnvHasher) -> Self {
        Self(Mutex::new(hasher))
    }
}

#[pymethods]
impl PyFnvHasher {
    #[new]
    #[pyo3(signature = (b = None, *, key = FnvKey::default()))]
    fn py_new(py: Python<'_>, b: Option<ryo3_bytes::PyBytes>, key: FnvKey) -> Self {
        py.detach(|| match b {
            Some(b) => {
                let mut hasher = FnvHasher::with_key(key.into());
                hasher.write(b.as_ref());
                Self::from(hasher)
            }
            None => Self::from(FnvHasher::with_key(key.into())),
        })
    }

    #[classattr]
    fn digest_size() -> usize {
        8
    }

    #[classattr]
    fn block_size() -> usize {
        // well fnv ain't blocky and just does a byte at a time
        // so i guess it's just 1?
        1
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::new(py, [py.None().into_bound_py_any(py)?])?;
        let kw = pyo3::types::PyDict::new(py);
        let key = self.finish()?;
        kw.set_item(pyo3::intern!(py, "key"), key)?;
        PyTuple::new(py, [args.into_bound_py_any(py)?, kw.into_bound_py_any(py)?])
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "fnv1a")
    }

    fn intdigest(&self) -> PyResult<u64> {
        self.finish()
    }

    fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyBytes>> {
        self.finish()
            .map(|n| pyo3::types::PyBytes::new(py, &(n.to_le_bytes())))
    }

    fn hexdigest(&self) -> PyResult<String> {
        self.finish().map(|n| format!("{n:x}"))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, py: Python<'_>, data: ryo3_bytes::PyBytes) -> PyResult<()> {
        py.detach(|| {
            let mut h = self.lock()?;
            h.write(data.as_ref());
            Ok(())
        })
    }

    fn copy(&self) -> PyResult<Self> {
        self.finish().map(|k| Self::from(FnvHasher::with_key(k)))
    }
}

#[pyfunction]
#[pyo3(signature = (b = None, key = FnvKey::default()))]
pub fn fnv1a(py: Python<'_>, b: Option<ryo3_bytes::PyBytes>, key: FnvKey) -> PyFnvHasher {
    PyFnvHasher::py_new(py, b, key)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFnvHasher>()?;
    m.add_function(wrap_pyfunction!(self::fnv1a, m)?)?;
    Ok(())
}

impl std::fmt::Display for PyFnvHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.finish().expect("no-way-jose");
        write!(f, "fnv1a<{key:x}>")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FnvKey(u64);

impl Default for FnvKey {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for FnvKey {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(n) = obj.extract::<u64>() {
            Ok(Self(n))
        } else if let Ok(b) = obj.extract::<[u8; 8]>() {
            let key = u64::from_be_bytes(b);
            Ok(Self(key))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Key must be an integer or 8-byte bytes-like object",
            ))
        }
    }
}

impl From<FnvKey> for u64 {
    fn from(key: FnvKey) -> Self {
        key.0
    }
}
