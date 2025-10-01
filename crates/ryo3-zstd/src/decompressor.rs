
use pyo3::prelude::*;

use pyo3::prelude::*;
use ryo3_bytes::PyBytes;

#[pyclass(name = "ZstdDecompressor")]
pub struct PyZstdDecompressor {
    dict : Option<PyZstdDict>,
    options: Option<zstd::stream::DecompressOptions>,
}

#[pymethods]
impl PyZstdDict {
    #[new]
    #[pyo3(signature = (dict_content, /, *, is_raw=false))]
    fn py_new(dict_content: PyBytes, is_raw: bool) -> Self {
        let dict = dict_content.as_ref().to_vec();
        let level = level.unwrap_or(0);
        Self { dict, level }
    }
}
