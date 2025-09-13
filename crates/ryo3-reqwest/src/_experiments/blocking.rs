use pyo3::PyResult;
use pyo3::prelude::*;

use ::jiter::{PythonParse, map_json_error};
use bytes::Bytes;
use pyo3::Bound;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes, PyDict};
use reqwest::StatusCode;
use ryo3_url::PyUrl;

#[pyclass]
#[pyo3(name = "BlockingResponse")]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyBlockingResponse {
    // Store the response in an Option so we can take ownership later.
    /// The actual response which will be consumed when read
    res: Option<reqwest::blocking::Response>,

    /// The body stored as bytes in the ob
    body: Option<Bytes>,
    // ========================================================================
    /// das status code
    status_code: StatusCode,
    /// das headers
    headers: reqwest::header::HeaderMap,
    /// das url
    url: reqwest::Url,
    /// das content length -- if it exists (tho it might not and/or be
    /// different if the response is compressed)
    content_length: Option<u64>,
}
impl From<reqwest::blocking::Response> for RyBlockingResponse {
    fn from(res: reqwest::blocking::Response) -> Self {
        Self {
            status_code: res.status(),
            headers: res.headers().clone(),
            url: res.url().clone(),
            content_length: res.content_length(),
            body: None,
            res: Some(res),
        }
    }
}
impl RyBlockingResponse {
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
        }
    }
}

#[pymethods]
impl RyBlockingResponse {
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
            pydict.set_item(k, v)?;
        }
        Ok(pydict)
    }

    /// Return the content length of the response, if it is known or `None`.
    #[getter]
    fn content_length(&self) -> Option<u64> {
        self.content_length
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

    fn json(mut slf: PyRefMut<'_, Self>) -> PyResult<Bound<'_, PyAny>> {
        slf.read_body()?;

        let parse_builder = PythonParse {
            allow_inf_nan: true,
            cache_mode: jiter::StringCacheMode::All,
            partial_mode: jiter::PartialMode::Off,
            catch_duplicate_keys: false,
            float_mode: jiter::FloatMode::Float,
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
#[pyo3(name = "Client", module = "ry.ryo3")]
#[derive(Debug)]
pub struct RyClient(reqwest::blocking::Client);

#[pymethods]
impl RyClient {
    #[new]
    fn new() -> Self {
        Self(reqwest::blocking::Client::new())
    }

    fn get(&self, url: &str) -> PyResult<RyBlockingResponse> {
        self.0
            .get(url)
            .send()
            .map(RyBlockingResponse::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }

    fn post(&self, url: &str, body: &str) -> PyResult<RyBlockingResponse> {
        self.0
            .post(url)
            .body(body.to_string())
            .send()
            .map(RyBlockingResponse::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }

    fn put(&self, url: &str, body: &str) -> PyResult<RyBlockingResponse> {
        self.0
            .put(url)
            .body(body.to_string())
            .send()
            .map(RyBlockingResponse::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }

    fn patch(&self, url: &str, body: &str) -> PyResult<RyBlockingResponse> {
        self.0
            .patch(url)
            .body(body.to_string())
            .send()
            .map(RyBlockingResponse::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }

    fn delete(&self, url: &str) -> PyResult<RyBlockingResponse> {
        self.0
            .delete(url)
            .send()
            .map(RyBlockingResponse::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }

    fn head(&self, url: &str) -> PyResult<RyBlockingResponse> {
        self.0
            .head(url)
            .send()
            .map(RyBlockingResponse::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }
}
