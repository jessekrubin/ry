#![doc = include_str!("../README.md")]
use std::hash::Hasher;

use ::fnv as fnv_rs;
use pyo3::types::PyModule;
use pyo3::{intern, prelude::*};
use pyo3::{wrap_pyfunction, PyResult};

#[pyclass(name = "FnvHasher", module = "ryo3")]
pub struct PyFnvHasher {
    pub hasher: fnv_rs::FnvHasher,
}

#[pymethods]
impl PyFnvHasher {
    #[new]
    #[pyo3(signature = (s = None))]
    fn py_new(s: Option<&[u8]>) -> Self {
        match s {
            Some(s) => {
                let mut hasher = fnv_rs::FnvHasher::default();
                hasher.write(s);
                Self { hasher }
            }
            None => Self {
                hasher: fnv_rs::FnvHasher::default(),
            },
        }
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

    fn update(&mut self, s: &[u8]) {
        self.hasher.write(s);
    }

    fn copy(&self) -> Self {
        Self {
            hasher: fnv_rs::FnvHasher::with_key(self.hasher.finish()),
        }
    }
}

#[pyfunction]
pub fn fnv1a(s: &[u8]) -> PyResult<PyFnvHasher> {
    Ok(PyFnvHasher::py_new(Some(s)))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFnvHasher>()?;
    m.add_function(wrap_pyfunction!(self::fnv1a, m)?)?;
    Ok(())
}
