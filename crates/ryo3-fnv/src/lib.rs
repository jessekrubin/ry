#![doc = include_str!("../README.md")]
use std::hash::Hasher;

use ::fnv as fnv_rs;
use pyo3::types::{PyModule, PyTuple};

use pyo3::{intern, prelude::*, IntoPyObjectExt};
use pyo3::{wrap_pyfunction, PyResult};

#[pyclass(name = "FnvHasher", module = "ryo3")]
pub struct PyFnvHasher {
    pub hasher: fnv_rs::FnvHasher,
}

#[pymethods]
impl PyFnvHasher {
    #[new]
    #[pyo3(signature = (s = None, *, key = None))]
    fn py_new(s: Option<ryo3_bytes::PyBytes>, key: Option<u64>) -> Self {
        match (key, s) {
            (Some(k), Some(s)) => {
                let mut hasher = fnv_rs::FnvHasher::with_key(k);
                hasher.write(s.as_ref());
                Self { hasher }
            }
            (Some(k), None) => Self {
                hasher: fnv_rs::FnvHasher::with_key(k),
            },
            (None, Some(s)) => {
                let mut hasher = fnv_rs::FnvHasher::default();
                hasher.write(s.as_ref());
                Self { hasher }
            }
            (None, None) => Self {
                hasher: fnv_rs::FnvHasher::default(),
            },
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            [
                py.None().into_bound_py_any(py)?,
                self.hasher.finish().into_bound_py_any(py)?,
            ],
        )
    }

    fn __str__(&self) -> String {
        format!("fnv1a<{:x}>", self.hasher.finish())
    }

    fn __repr__(&self) -> String {
        format!("fnv1a<{:x}>", self.hasher.finish())
    }

    #[classattr]
    fn name(py: Python<'_>) -> String {
        let a = intern!(py, "fnv1a");
        a.to_string()
    }

    fn digest(&self) -> u64 {
        self.hasher.finish()
    }

    fn hexdigest(&self) -> String {
        // format hex string lowercase
        format!("{:x}", self.hasher.finish())
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&mut self, s: ryo3_bytes::PyBytes) {
        self.hasher.write(s.as_ref());
    }

    fn copy(&self) -> Self {
        Self {
            hasher: fnv_rs::FnvHasher::with_key(self.hasher.finish()),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (s, key = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn fnv1a(s: ryo3_bytes::PyBytes, key: Option<u64>) -> PyResult<PyFnvHasher> {
    Ok(PyFnvHasher {
        hasher: if let Some(k) = key {
            let mut hasher = fnv_rs::FnvHasher::with_key(k);
            hasher.write(s.as_ref());
            hasher
        } else {
            let mut hasher = fnv_rs::FnvHasher::default();
            hasher.write(s.as_ref());
            hasher
        },
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFnvHasher>()?;
    m.add_function(wrap_pyfunction!(self::fnv1a, m)?)?;
    Ok(())
}
