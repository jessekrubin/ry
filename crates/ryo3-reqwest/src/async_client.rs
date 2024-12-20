use pyo3::prelude::*;

use crate::errors::map_reqwest_err;
use crate::pyo3_bytes::Pyo3JsonBytes;
use bytes::Bytes;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use reqwest::StatusCode;
use ryo3_bytes::Pyo3Bytes;
use ryo3_url::PyUrl;
use std::borrow::Borrow;

#[pyclass]
#[pyo3(name = "AsyncClient")]
#[derive(Debug, Clone)]
pub struct RyAsyncClient(reqwest::Client);
#[pyclass]
#[pyo3(name = "AsyncResponse")]
#[derive(Debug)]
pub struct RyAsyncResponse {
    // Store the response in an Option so we can take ownership later.
    status_code: StatusCode,
    headers: reqwest::header::HeaderMap,
    // cookies: reqwest::cookie::CookieJar,
    // version: Option<reqwest::Version>,
    url: reqwest::Url,

    body: Option<Bytes>,

    res: Option<reqwest::Response>,
}
impl RyAsyncResponse {
    async fn read_body_async(&mut self) -> Result<(), PyErr> {
        if self.body.is_none() {
            let res = self
                .res
                .take()
                .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
            let b = res
                .bytes()
                .await
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            self.body = Some(b);
        }
        Ok(())
    }
}

#[pymethods]
impl RyAsyncResponse {
    #[getter]
    fn status_code(&self) -> PyResult<u16> {
        Ok(self.status_code.as_u16())
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
        for (name, value) in c.iter() {
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

    fn bytes<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .bytes()
                .await
                .map(Pyo3Bytes::from)
                .map_err(map_reqwest_err)
        })
    }
    fn text<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response.text().await.map_err(map_reqwest_err)
        })
    }

    fn json<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .bytes()
                .await
                .map(Pyo3JsonBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    fn __str__(&self) -> String {
        format!("Response: {}", self.status_code)
    }

    fn __repr__(&self) -> String {
        format!("Response: {}", self.status_code)
    }
}

#[pymethods]
impl RyAsyncClient {
    #[new]
    fn new() -> Self {
        Self(reqwest::Client::new())
    }

    // self.request(Method::GET, url)
    fn get<'py>(&'py mut self, py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.get(&url).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let response = response_future
                .await
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            let r = RyAsyncResponse {
                status_code: response.status(),
                headers: response.headers().clone(),
                url: response.url().clone(),
                body: None,
                res: Some(response),
            };
            Ok(r)
        })
    }

    fn post<'py>(
        &'py mut self,
        py: Python<'py>,
        url: String,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.post(&url).body(body.to_vec()).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let response = response_future
                .await
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            let r = RyAsyncResponse {
                status_code: response.status(),
                headers: response.headers().clone(),
                url: response.url().clone(),
                body: None,
                res: Some(response),
            };
            Ok(r)
        })
    }
}
