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
    fn new(s: Option<&[u8]>) -> Self {
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

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("fnv1a<{:x}>", self.hasher.finish()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("fnv1a<{:x}>", self.hasher.finish()))
    }

    #[classattr]
    fn name(py: Python<'_>) -> String {
        intern!(py, "fnv1a").to_string()
    }

    fn digest(&self) -> PyResult<u64> {
        Ok(self.hasher.finish())
    }

    fn hexdigest(&self) -> PyResult<String> {
        // format hex string lowercase
        Ok(format!("{:x}", self.hasher.finish()))
    }

    fn update(&mut self, s: &[u8]) -> PyResult<()> {
        self.hasher.write(s);
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        Ok(Self {
            hasher: fnv_rs::FnvHasher::with_key(self.hasher.finish()),
        })
    }
}

#[pyfunction]
pub fn fnv1a(s: &[u8]) -> PyResult<PyFnvHasher> {
    Ok(PyFnvHasher::new(Some(s)))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFnvHasher>()?;
    m.add_function(wrap_pyfunction!(self::fnv1a, m)?)?;
    Ok(())
}
