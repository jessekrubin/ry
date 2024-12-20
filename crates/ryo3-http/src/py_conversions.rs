//! http python conversions
//!
//! TODO: figure out how to `intern!()` the strings...
use pyo3::prelude::*;
use pyo3::types::PyString;

pub struct HttpMethod(pub http::Method);

impl<'py> IntoPyObject<'py> for HttpMethod {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}
impl<'py> IntoPyObject<'py> for &HttpMethod {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            http::Method::GET => Ok("GET"),
            http::Method::POST => Ok("POST"),
            http::Method::PUT => Ok("PUT"),
            http::Method::DELETE => Ok("DELETE"),
            http::Method::HEAD => Ok("HEAD"),
            http::Method::OPTIONS => Ok("OPTIONS"),
            http::Method::CONNECT => Ok("CONNECT"),
            http::Method::PATCH => Ok("PATCH"),
            http::Method::TRACE => Ok("TRACE"),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "UNSUPPORTED HTTP METHOD".to_string(),
            )),
        }?;

        #[cfg(Py_LIMITED_API)]
        {
            Ok(PyString::new(py, s).into_any())
        }
        #[cfg(not(Py_LIMITED_API))]
        {
            Ok(PyString::new(py, s))
        }
    }
}

const HTTP_METHOD_STRINGS: &str =
    "'GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS', 'CONNECT', 'PATCH', 'TRACE'";

impl FromPyObject<'_> for HttpMethod {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<HttpMethod> {
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_uppercase();
            match s.as_str() {
                "GET" => Ok(HttpMethod(http::Method::GET)),
                "POST" => Ok(HttpMethod(http::Method::POST)),
                "PUT" => Ok(HttpMethod(http::Method::PUT)),
                "DELETE" => Ok(HttpMethod(http::Method::DELETE)),
                "HEAD" => Ok(HttpMethod(http::Method::HEAD)),
                "OPTIONS" => Ok(HttpMethod(http::Method::OPTIONS)),
                "CONNECT" => Ok(HttpMethod(http::Method::CONNECT)),
                "PATCH" => Ok(HttpMethod(http::Method::PATCH)),
                "TRACE" => Ok(HttpMethod(http::Method::TRACE)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid HTTP method: {s} (options: {HTTP_METHOD_STRINGS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Invalid unit: {ob} (options: {HTTP_METHOD_STRINGS})"
            )))
        }
    }
}
//
