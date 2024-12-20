use pyo3::prelude::*;
use pyo3::PyResult;

use ::jiter::{map_json_error, PythonParse};
use bytes::Bytes;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes, PyDict};
use pyo3::Bound;
use reqwest::StatusCode;
use ryo3_url::PyUrl;

#[pyclass]
#[pyo3(name = "Response")]
#[derive(Debug)]
pub struct RyResponse {
    // Store the response in an Option so we can take ownership later.
    status_code: StatusCode,
    headers: reqwest::header::HeaderMap,
    // cookies: reqwest::cookie::CookieJar,
    // version: Option<reqwest::Version>,
    url: reqwest::Url,

    body: Option<Bytes>,

    res: Option<reqwest::blocking::Response>,
}

impl RyResponse {
    fn read_body(&mut self) -> PyResult<()> {
        if let Some(_b) = self.body.as_ref() {
            Ok(())
        } else {
            let res = self
                .res
                .take()
                .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
            let b = res
                .bytes()
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            self.body = Some(b);
            Ok(())
            // Ok(&*b)
        }
    }
}

#[pymethods]
impl RyResponse {
    #[getter]
    fn status_code(&self) -> PyResult<u16> {
        let res = self
            .res
            .as_ref()
            .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
        Ok(res.status().as_u16())
    }
    fn bytes(mut slf: PyRefMut<'_, Self>) -> PyResult<Bound<'_, PyBytes>> {
        slf.read_body()?;
        let b = slf.body.as_ref().ok_or(PyValueError::new_err(
            "Something went wrong.... this should not happen",
        ))?;
        Ok(PyBytes::new(slf.py(), b))

        // match slf.body.as_ref() {
        //     Some(b) => Ok(b.to_vec()),
        //     None => {
        //         // Take ownership of the response, leaving None in place.
        //         let res = slf
        //             .res
        //             .take()
        //             .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
        //
        //         // Now we have full ownership of res, so we can call text() without error.
        //         let b = res
        //             .bytes()
        //             .map_err(|e| PyValueError::new_err(format!("{e}")))?;
        //         // return the b
        //         Ok(b.to_vec())
        //     }
        // }
    }
    #[getter]
    #[pyo3(name = "url")]
    fn py_url(&self) -> PyUrl {
        PyUrl(self.url.clone())
    }

    #[getter]
    #[pyo3(name = "headers")]
    fn py_headers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let c = self.headers.clone();
        let pydict = PyDict::new(py);
        for (name, value) in &c {
            let k = name.to_string();
            let v = value
                .to_str()
                .map(String::from)
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            // .to_str()?.to_string();
            pydict.set_item(k, v)?;
        }
        Ok(pydict)
    }
    fn text(mut slf: PyRefMut<'_, Self>) -> PyResult<String> {
        slf.read_body()?;
        let b = slf.body.as_ref().ok_or(PyValueError::new_err(
            "Something went wrong.... this should not happen",
        ))?;

        let s = String::from_utf8_lossy(b);
        Ok(s.to_string())
    }

    fn __str__(&self) -> String {
        format!("Response: {}", self.status_code)
    }

    fn __repr__(&self) -> String {
        format!("Response: {}", self.status_code)
    }

    // ) -> PyResult<Bound<'py, PyAny>> {

    fn json(mut slf: PyRefMut<'_, Self>) -> PyResult<Bound<'_, PyAny>> {
        slf.read_body()?;

        let parse_builder = PythonParse {
            allow_inf_nan: true,
            cache_mode: ::jiter::StringCacheMode::All,
            partial_mode: ::jiter::PartialMode::Off,
            catch_duplicate_keys: false,
            float_mode: ::jiter::FloatMode::Float,
            // cache_mode = StringCacheMode::All,
            // partial_mode = PartialMode::Off,
            // catch_duplicate_keys = false,
            // float_mode = FloatMode::Float
        };
        let b = slf.body.as_ref().ok_or(PyValueError::new_err(
            "Something went wrong.... this should not happen",
        ))?;
        parse_builder
            .python_parse(slf.py(), b)
            .map_err(|e| map_json_error(b, &e))
    }
}
#[pyclass]
#[pyo3(name = "Client")]
#[derive(Debug)]
pub struct RyClient(reqwest::blocking::Client);

#[pymethods]
impl RyClient {
    #[new]
    fn new() -> Self {
        Self(reqwest::blocking::Client::new())
    }

    // self.request(Method::GET, url)

    fn get(&self, url: &str) -> PyResult<RyResponse> {
        let response = self
            .0
            .get(url)
            .send()
            .map_err(|e| PyValueError::new_err(format!("{e}")))?;

        let url = response.url().clone();
        let headers = response.headers().clone();
        let status_code = response.status();

        Ok(RyResponse {
            status_code,
            headers,
            url,
            body: None,

            res: Some(response),
        })
    }
}

// pub fn pymod_add(py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_class::<RyClient>()?;
//     m.add_class::<RyResponse>()?;
//     Ok(())
// }
