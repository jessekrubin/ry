#![doc = include_str!("../README.md")]
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "UUID", module = "ry.ryo3", frozen)]
pub struct PyUuid(pub(crate) uuid::Uuid);

impl From<uuid::Uuid> for PyUuid {
    fn from(value: uuid::Uuid) -> Self {
        PyUuid(value)
    }
}

#[pymethods]
impl PyUuid {
    #[new]
    fn py_new(hex: &str) -> PyResult<Self> {
        let uuid = uuid::Uuid::parse_str(hex).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyUuid(uuid))
    }


    fn string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        format!("UUID({})", self.string())
    }

    fn to_py(&self) -> uuid::Uuid {
        self.0
    }

}

#[pyfunction]
pub fn uuid4()  -> PyResult<PyUuid> {
    let uuid = uuid::Uuid::new_v4();
    Ok(PyUuid(uuid))
}
