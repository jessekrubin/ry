use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::sp::done::Done;

#[pyclass]
#[derive(Debug)]
pub struct PyDone {
    done: Done,
}

#[pymethods]
impl PyDone {
    #[new]
    fn new(args: Vec<String>, returncode: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        let d = Done::new(args, returncode, stdout, stderr);
        Self { done: d }
    }

    fn __repr__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.done).unwrap();
        Ok(s)
    }
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    #[getter]
    fn stdout<'py>(&'py self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.done.stdout)
    }

    #[getter]
    fn stderr<'py>(&'py self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.done.stderr)
    }

    #[getter]
    fn returncode(&self) -> i32 {
        self.done.returncode
    }

    #[getter]
    fn args(&self) -> Vec<String> {
        self.done.args.clone()
    }
}

impl From<Done> for PyDone {
    fn from(done: Done) -> Self {
        Self { done }
    }
}
