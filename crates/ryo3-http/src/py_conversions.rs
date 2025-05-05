//! http python conversions
//!
//! TODO: figure out how to `intern!()` the strings...

use crate::http_types::{HttpHeaderName, HttpHeaderValue, HttpMethod, HttpVersion};
use pyo3::exceptions::PyValueError;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

impl<'py> IntoPyObject<'py> for &HttpMethod {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            http::Method::GET => Ok(intern!(py, "GET")),
            http::Method::POST => Ok(intern!(py, "POST")),
            http::Method::PUT => Ok(intern!(py, "PUT")),
            http::Method::DELETE => Ok(intern!(py, "DELETE")),
            http::Method::HEAD => Ok(intern!(py, "HEAD")),
            http::Method::OPTIONS => Ok(intern!(py, "OPTIONS")),
            http::Method::CONNECT => Ok(intern!(py, "CONNECT")),
            http::Method::PATCH => Ok(intern!(py, "PATCH")),
            http::Method::TRACE => Ok(intern!(py, "TRACE")),
            _ => Err(PyErr::new::<PyValueError, _>(
                "UNSUPPORTED HTTP METHOD".to_string(),
            )),
        }?;
        let b = s.as_borrowed();
        #[cfg(Py_LIMITED_API)]
        {
            Ok(b.into_any())
        }
        #[cfg(not(Py_LIMITED_API))]
        {
            Ok(b)
        }
    }
}

impl<'py> IntoPyObject<'py> for HttpMethod {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
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

// ============================================================================
// HTTP VERSION
// ============================================================================
impl<'py> IntoPyObject<'py> for &HttpVersion {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            http::Version::HTTP_09 => intern!(py, "HTTP/0.9"),
            http::Version::HTTP_10 => intern!(py, "HTTP/1.0"),
            http::Version::HTTP_11 => intern!(py, "HTTP/1.1"),
            http::Version::HTTP_2 => intern!(py, "HTTP/2"),
            http::Version::HTTP_3 => intern!(py, "HTTP/3"),
            _ => unreachable!(),
        };
        let b = s.as_borrowed();
        #[cfg(Py_LIMITED_API)]
        {
            Ok(b.into_any())
        }
        #[cfg(not(Py_LIMITED_API))]
        {
            Ok(b)
        }
    }
}

impl<'py> IntoPyObject<'py> for HttpVersion {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

const HTTP_VERSION_STRING: &str = "Invalid HTTP version ~ must be one of 'HTTP/0.9'|'0.9', 'HTTP/1.0'|'HTTP/1'|'1.0'|'1', 'HTTP/1.1'|'1.1', 'HTTP/2.0'|'HTTP/2'|'2.0'|'2'|'2.2', 'HTTP/3.0'|'HTTP/3'|'3.0'|'3'";
impl FromPyObject<'_> for HttpVersion {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<HttpVersion> {
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string();
            match s.as_str().to_ascii_uppercase().as_str() {
                "HTTP/0.9" | "0.9" => Ok(HttpVersion(http::Version::HTTP_09)),
                "HTTP/1.0" | "HTTP/1" | "1.0" | "1" => Ok(HttpVersion(http::Version::HTTP_10)),
                "HTTP/1.1" | "1.1" => Ok(HttpVersion(http::Version::HTTP_11)),
                "HTTP/2.0" | "HTTP/2" | "2.0" | "2" => Ok(HttpVersion(http::Version::HTTP_2)),
                "HTTP/3" | "HTTP/3.0" | "3.0" | "3" => Ok(HttpVersion(http::Version::HTTP_3)),
                _ => Err(PyErr::new::<PyValueError, _>(HTTP_VERSION_STRING)),
            }
        } else if let Ok(i) = ob.extract::<u8>() {
            match i {
                0 => Ok(HttpVersion(http::Version::HTTP_09)),
                10 => Ok(HttpVersion(http::Version::HTTP_10)),
                1 | 11 => Ok(HttpVersion(http::Version::HTTP_11)),
                2|20 => Ok(HttpVersion(http::Version::HTTP_2)),
                3|30 => Ok(HttpVersion(http::Version::HTTP_3)),
                _ => Err(PyErr::new::<PyValueError, _>(
                    "Invalid HTTP version: {i} (options: 0= HTPP/0.0, 1 | 10 = HTTP/1.0, 11 = HTTP/1.1, 2 | 20 = HTTP/2.0, 3 | 30 = HTTP/3.0)"
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Invalid unit: {ob} (options: {HTTP_VERSION_STRING})"
            )))
        }
    }
}
// ============================================================================
// HttpHeaderName
// ============================================================================

pub(crate) fn header_name_to_pystring<'py>(
    py: Python<'py>,
    name: &http::HeaderName,
) -> Bound<'py, PyString> {
    let s = name.as_str();
    PyString::new(py, s)
}

// pub(crate) fn pystring_to_header_name<'py>(py: Python<'py> , s:&Bound<'py, PyString>) -> PyResult<http::HeaderName> {
//     http::HeaderName::from_bytes(s.as_bytes())
//         .map_err(|e| PyValueError::new_err(format!("invalid-header-name: {e}")))
// }

impl<'py> IntoPyObject<'py> for &HttpHeaderName {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = self.0.as_str();
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

impl<'py> IntoPyObject<'py> for HttpHeaderName {
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

impl FromPyObject<'_> for HttpHeaderName {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<HttpHeaderName> {
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string();
            http::HeaderName::from_bytes(s.as_bytes())
                .map(HttpHeaderName)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-name: {e}")))
        } else if let Ok(pyb) = ob.downcast::<PyBytes>() {
            let s = pyb.as_bytes();
            http::HeaderName::from_bytes(s)
                .map(HttpHeaderName)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-name: {e}")))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid-header-name".to_string(),
            ))
        }
    }
}

// ============================================================================
// HttpHeaderValue
// ============================================================================

pub(crate) fn header_value_to_pystring<'py>(
    py: Python<'py>,
    value: &http::HeaderValue,
) -> PyResult<Bound<'py, PyString>> {
    let s = value
        .to_str()
        .map_err(|e| PyValueError::new_err(format!("{e}")))?;
    Ok(PyString::new(py, s))
}

impl<'py> IntoPyObject<'py> for &HttpHeaderValue {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        header_value_to_pystring(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for HttpHeaderValue {
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

impl FromPyObject<'_> for HttpHeaderValue {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<HttpHeaderValue> {
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string();
            http::HeaderValue::from_str(&s)
                .map(HttpHeaderValue::from)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-value: {e}")))
        } else if let Ok(pyb) = ob.downcast::<PyBytes>() {
            let s = pyb.as_bytes();
            http::HeaderValue::from_bytes(s)
                .map(HttpHeaderValue::from)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-value: {e}")))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid-header-value".to_string(),
            ))
        }
    }
}
