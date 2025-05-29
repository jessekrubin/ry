use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use pythonize::depythonize;

#[pyfunction]
pub fn stringify_v1<'py>(obj: Bound<'py, PyAny>) -> PyResult<String> {
    let new_sample: serde_json::Value =
        depythonize(&obj).map_err(|e| PyOSError::new_err(format!("Failed to depythonize: {e}")))?;
    serde_json::to_string(&new_sample)
        .map_err(|e| PyOSError::new_err(format!("Failed to jsonify: {e}")))
}
// #[pyfunction]
// pub fn stringify_uh<'py>(obj: Bound<'py, PyAny>) -> PyResult<String> {

//     //  let wrapper =
//     //     match stringify_inner::<serde_json::Value>(&obj) {
//     //         Ok(value) => value,
//     //         Err(e) => return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(e.to_string())),
//     //     };
//     // serde_json::to_string(&wrapper).map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(e.to_string()))

//     let wrapper = Depythonizer::from_object(&obj);

//     serde_json::to_string(&wrapper).map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(e.to_string()))
// }
// lock the gil
// #[pyfunction]
// pub fn stringify_v2<'py>(py: Python<'py>, obj: Bound<'py, PyAny>) -> PyResult<String> {
//     let a = Python::with_gil(|py| {
//         let new_sample: serde_json::Value = depythonize(&obj)
//             .map_err(|e| PyOSError::new_err(format!("Failed to depythonize: {e}")))?;
//         serde_json::to_string(&new_sample)
//             .map_err(|e| PyOSError::new_err(format!("Failed to jsonify: {e}")))
//     });
//     a
// }

//     let new_sample: serde_json::Value = depythonize(&obj)
//         .map_err(|e| PyOSError::new_err(format!("Failed to depythonize: {e}")))?;
//     serde_json::to_string(&new_sample)
//         .map_err(|e| PyOSError::new_err(format!("Failed to jsonify: {e}")))
// }
#[pyfunction]
pub fn stringify_v3<'py>(obj: Bound<'py, PyAny>) -> PyResult<ryo3_bytes::PyBytes> {
    let new_sample: serde_json::Value =
        depythonize(&obj).map_err(|e| PyOSError::new_err(format!("Failed to depythonize: {e}")))?;
    serde_json::to_vec(&new_sample)
        .map(|bytes| ryo3_bytes::PyBytes::from(bytes))
        .map_err(|e| PyOSError::new_err(format!("Failed to jsonify: {e}")))
    // let a = stringify_inner(
    //     &obj,
    // ) .ok_or_else(|| {
    //     PyOSError::new_err("Failed to depythonize the object into serde_json::Value")
    // });
    // .map_err(|e| PyErr::new::<pyo3::exceptions::PyTypeError, _>(e.to_string()))
}

//     >= PyModule::import(py, "orjson").map_err(|e| PyOSError::new_err(format!("Failed to import orjson: {e}")))
// }
#[pyfunction]
pub fn stringify_v4<'py>(py: Python<'py>, obj: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    static ORJSON: GILOnceCell<Py<PyAny>> = GILOnceCell::new();
    let a = ORJSON.import(py, "orjson", "dumps")?;

    let r = a.call1((obj,));
    r
}

//     a.call(args, kwargs)
//      static ZONE_INFO: GILOnceCell<Py< PyModule
//      >> = GILOnceCell::new();
//   ZONE_INFO
//       .import(py, "zoneinfo", "ZoneInfo")
//       .and_then(|obj| obj.call1((tz.iana_name(),)))
// }

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stringify_v1, m)?)?;
    // m.add_function(wrap_pyfunction!(stringify_v2, m)?)?;
    m.add_function(wrap_pyfunction!(stringify_v3, m)?)?;
    m.add_function(wrap_pyfunction!(stringify_v4, m)?)?;
    Ok(())
}
