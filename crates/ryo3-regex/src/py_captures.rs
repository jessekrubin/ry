use pyo3::prelude::*;

#[pyclass(module = "ryo3")]
pub struct PyCaptures {
    // owned data extracted from captures
    pub groups: Vec<Option<String>>,

    // named groups go in hashmap
    pub named: std::collections::HashMap<String, Option<String>>,
}

#[pymethods]
impl PyCaptures {
    fn groups(&self) -> Vec<Option<String>> {
        self.groups.clone()
    }

    fn groupdict(&self) -> std::collections::HashMap<String, Option<String>> {
        self.named.clone()
    }

    fn __str__(&self) -> String {
        format!("Captures({:?})", self.groups)
    }

    fn get(&self, i: usize) -> Option<&str> {
        self.groups
            .get(i)
            .map(|m| m.as_ref().map(|s| s.as_str()).unwrap_or_default())
    }

    fn __getitem__<'py>(&self, i: &Bound<'py, PyAny>) -> PyResult<Option<&str>> {
        if let Ok(index) = i.extract::<usize>() {
            // Indexing by number
            Ok(self.groups.get(index).and_then(|m| m.as_deref()))
        } else if let Ok(key) = i.extract::<String>() {
            // Indexing by name
            Ok(self.named.get(&key).and_then(|m| m.as_deref()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected an integer or string"
            ))
        }
    }
}
