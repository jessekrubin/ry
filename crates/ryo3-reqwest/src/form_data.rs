// use pyo3::prelude::*;
// use reqwest::multipart::Form;
//
// #[pyclass]
// #[derive(Debug)]
// pub struct PyFormData {
//     pub(crate) form: reqwest::multipart::Form,
// }
//
// impl From<Form> for PyFormData {
//     fn from(form: Form) -> Self {
//         PyFormData { form }
//     }
// }
//
// #[pymethods]
// impl PyFormData {
//     #[new]
//     fn py_new() -> Self {
//         PyFormData {
//             form: reqwest::multipart::Form::new(),
//         }
//     }
//
//     // fn text(&self, name: &str, value: &str) -> Self {
//     //     let form = self.form.text(name, value);
//     // PyFormData { form }
//     // }
//
//     // fn add(&mut self, name: String, value: String) {
//     //     self.form = self.form.text(name, value);
//     // }
//     //
//     // fn add_file(&mut self, name: String, path: String) -> PyResult<()> {
//     //     let file = reqwest::multipart::Part::file(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
//     //     self.form = self.form.part(name, file);
//     //     Ok(())
//     // }
// }
