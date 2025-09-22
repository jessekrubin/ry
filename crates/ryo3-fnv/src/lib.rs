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

#[pymethods]
impl PyFnvHasher {
    #[new]
    #[pyo3(signature = (s = None, *, key = None))]
    fn py_new(s: Option<ryo3_bytes::PyBytes>, key: Option<u64>) -> Self {
        match (key, s) {
            (Some(k), Some(s)) => {
                let mut hasher = FnvHasher::with_key(k);
                hasher.write(s.as_ref());
                Self::from(hasher)
            }
            (Some(k), None) => Self::from(FnvHasher::with_key(k)),
            (None, Some(s)) => {
                let mut hasher = FnvHasher::default();
                hasher.write(s.as_ref());
                Self::from(hasher)
            }
            (None, None) => Self::from(FnvHasher::default()),
        }
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

    fn digest(&self) -> ryo3_bytes::PyBytes {
        let bytes = Vec::from(self.finish().to_be_bytes());
        ryo3_bytes::PyBytes::from(bytes)
    }

    fn hexdigest(&self) -> String {
        // format hex string lowercase
        format!("{:x}", self.finish())
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, s: ryo3_bytes::PyBytes) -> PyResult<()> {
        if let Ok(mut h) = self.0.lock() {
            h.write(s.as_ref());
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Failed to lock hasher in update",
            ))
        }
    }

    fn copy(&self) -> Self {
        Self::from(FnvHasher::with_key(self.finish()))
    }
}

#[pyfunction]
#[pyo3(signature = (s = None, key = None))]
pub fn fnv1a(s: Option<ryo3_bytes::PyBytes>, key: Option<u64>) -> PyFnvHasher {
    PyFnvHasher::py_new(s, key)
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
