#![doc = include_str!("../README.md")]
use std::hash::Hasher;

use pyo3::types::{PyModule, PyString, PyTuple};

use pyo3::{IntoPyObjectExt, intern, prelude::*};
use pyo3::{PyResult, wrap_pyfunction};

use fnv::FnvHasher;
use std::sync::Mutex;

#[pyclass(name = "FnvHasher", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFnvHasher(pub Mutex<FnvHasher>);

impl PyFnvHasher {
    fn finish(&self) -> u64 {
        self.0.lock().expect("Failed to lock hasher").finish()
    }
}

impl From<FnvHasher> for PyFnvHasher {
    fn from(hasher: FnvHasher) -> Self {
        Self(Mutex::new(hasher))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FnvKey(u64);

impl Default for FnvKey {
    fn default() -> Self {
        FnvKey(0xcbf29ce484222325)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for FnvKey {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(n) = obj.extract::<u64>() {
            Ok(FnvKey(n))
        } else if let Ok(b) = obj.extract::<[u8; 8]>() {
            let key = u64::from_be_bytes(b);
            Ok(FnvKey(key))
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

#[pymethods]
impl PyFnvHasher {
    #[new]
    #[pyo3(signature = (s = None, *, key =  None))]
    fn py_new(py: Python<'_>, s: Option<ryo3_bytes::PyBytes>, key: Option<FnvKey>) -> Self {
        py.detach(|| match (key, s) {
            (Some(k), Some(s)) => {
                let mut hasher = FnvHasher::with_key(k.into());
                hasher.write(s.as_ref());
                Self::from(hasher)
            }
            (Some(k), None) => Self::from(FnvHasher::with_key(k.into())),
            (None, Some(s)) => {
                let mut hasher = FnvHasher::default();
                hasher.write(s.as_ref());
                Self::from(hasher)
            }
            (None, None) => Self::from(FnvHasher::default()),
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
        kw.set_item(pyo3::intern!(py, "key"), self.finish())?;
        PyTuple::new(py, [args.into_bound_py_any(py)?, kw.into_bound_py_any(py)?])
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "fnv1a")
    }

    fn intdigest(&self) -> u64 {
        self.finish()
    }

    fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        let bytes = self.finish().to_be_bytes();
        pyo3::types::PyBytes::new(py, &bytes)
    }

    fn hexdigest(&self) -> String {
        // format hex string lowercase
        format!("{:x}", self.finish())
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, py: Python<'_>, s: ryo3_bytes::PyBytes) -> PyResult<()> {
        py.detach(|| {
            if let Ok(mut h) = self.0.lock() {
                h.write(s.as_ref());
                Ok(())
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to lock hasher in update",
                ))
            }
        })
    }

    fn copy(&self) -> Self {
        Self::from(FnvHasher::with_key(self.finish()))
    }
}

#[pyfunction]
#[pyo3(signature = (s = None, key = None))]
pub fn fnv1a(py: Python<'_>, s: Option<ryo3_bytes::PyBytes>, key: Option<FnvKey>) -> PyFnvHasher {
    PyFnvHasher::py_new(py, s, key)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFnvHasher>()?;
    m.add_function(wrap_pyfunction!(self::fnv1a, m)?)?;
    Ok(())
}

impl std::fmt::Display for PyFnvHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fnv1a<{:x}>", self.finish())
    }
}
