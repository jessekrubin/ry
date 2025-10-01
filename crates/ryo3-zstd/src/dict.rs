use pyo3::prelude::*;
use ryo3_bytes::PyBytes;
use ryo3_macro_rules::pytodo;
use zstd_safe::CDict;

#[pyclass(name = "ZstdDict", frozen)]
pub struct PyZstdDict {
    pub dict: Vec<u8>,
    pub level: i32,
}

#[pymethods]
impl PyZstdDict {
    #[new]
    #[pyo3(signature = (dict_content, /, *, is_raw=false))]
    fn py_new(dict_content: PyBytes, is_raw: bool) -> PyResult<Self> {

        let cdict = CDict::try_create(dict_content.as_ref(), 0)?;
        let dict = dict_content.as_ref().to_vec();
        let level = level.unwrap_or(0);
        Self { dict, level }
    }

    // static methods...
    #[staticmethod]
    fn from_continuous(sample_data: PyBytes, sample_sizes: Vec<usize>, max_size: usize) {
        // Implementation for creating a dictionary from continuous samples
        // This is a placeholder; actual implementation would involve calling Zstd functions
        pytodo!()
    }

    #[staticmethod]
    fn from_files(file_paths: Vec<&str>, max_size: usize) {
        // Implementation for creating a dictionary from files
        // This is a placeholder; actual implementation would involve reading files and calling Zstd functions
        pytodo!()
    }

    #[staticmethod]
    fn from_sample_iterator(sample_data: Vec<PyBytes>, max_size: usize) {
        // Implementation for creating a dictionary from continuous samples
        // This is a placeholder; actual implementation would involve calling Zstd functions
        pytodo!()
    }

    #[staticmethod]
    fn from_samples(sample_data: Vec<PyBytes>, sample_sizes: Vec<usize>, max_size: usize) {
        // Implementation for creating a dictionary from continuous samples
        // This is a placeholder; actual implementation would involve calling Zstd functions
        pytodo!()
    }
}
