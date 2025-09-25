//! http python conversions
//!
//! TODO: figure out how to `intern!()` the strings...

use crate::HttpHeaderNameRef;
use crate::http_types::{HttpHeaderName, HttpHeaderValue, HttpMethod, HttpVersion};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::{IntoPyObjectExt, intern};

impl<'py> IntoPyObject<'py> for &HttpMethod {
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
        Ok(b)
    }
}

impl<'py> IntoPyObject<'py> for HttpMethod {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

const HTTP_METHOD_STRINGS: &str = "'GET'/'get', 'POST'/'post', 'PUT'/'put', 'DELETE'/'delete', 'HEAD'/'head', 'OPTIONS'/'options', 'CONNECT'/'connect', 'PATCH'/'patch', 'TRACE'/'trace'";

impl FromPyObject<'_> for HttpMethod {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "GET" | "get" => Ok(Self(http::Method::GET)),
                "POST" | "post" => Ok(Self(http::Method::POST)),
                "PUT" | "put" => Ok(Self(http::Method::PUT)),
                "DELETE" | "delete" => Ok(Self(http::Method::DELETE)),
                "HEAD" | "head" => Ok(Self(http::Method::HEAD)),
                "OPTIONS" | "options" => Ok(Self(http::Method::OPTIONS)),
                "CONNECT" | "connect" => Ok(Self(http::Method::CONNECT)),
                "PATCH" | "patch" => Ok(Self(http::Method::PATCH)),
                "TRACE" | "trace" => Ok(Self(http::Method::TRACE)),
                _ => Err(PyErr::new::<PyValueError, _>(format!(
                    "Invalid HTTP method: {s} (options: {HTTP_METHOD_STRINGS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Invalid method: {ob} (options: {HTTP_METHOD_STRINGS})"
            )))
        }
    }
}

// ============================================================================
// HTTP VERSION
// ============================================================================
impl<'py> IntoPyObject<'py> for &HttpVersion {
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
        Ok(b)
    }
}

impl<'py> IntoPyObject<'py> for HttpVersion {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

const HTTP_VERSION_STRING: &str = "Invalid HTTP version ~ must be one of 'HTTP/0.9'|'0.9', 'HTTP/1.0'|'HTTP/1'|'1.0'|'1', 'HTTP/1.1'|'1.1', 'HTTP/2.0'|'HTTP/2'|'2.0'|'2'|'2.2', 'HTTP/3.0'|'HTTP/3'|'3.0'|'3'";
impl FromPyObject<'_> for HttpVersion {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.cast_exact::<PyString>() {
            let s = s.extract::<&str>()?;
            match s.to_ascii_uppercase().as_str() {
                "HTTP/0.9" | "0.9" => Ok(Self(http::Version::HTTP_09)),
                "HTTP/1.0" | "HTTP/1" | "1.0" | "1" => Ok(Self(http::Version::HTTP_10)),
                "HTTP/1.1" | "1.1" => Ok(Self(http::Version::HTTP_11)),
                "HTTP/2.0" | "HTTP/2" | "2.0" | "2" => Ok(Self(http::Version::HTTP_2)),
                "HTTP/3" | "HTTP/3.0" | "3.0" | "3" => Ok(Self(http::Version::HTTP_3)),
                _ => Err(PyErr::new::<PyValueError, _>(HTTP_VERSION_STRING)),
            }
        } else if let Ok(i) = ob.extract::<u8>() {
            match i {
                0 => Ok(Self(http::Version::HTTP_09)),
                10 => Ok(Self(http::Version::HTTP_10)),
                1 | 11 => Ok(Self(http::Version::HTTP_11)),
                2 | 20 => Ok(Self(http::Version::HTTP_2)),
                3 | 30 => Ok(Self(http::Version::HTTP_3)),
                _ => Err(PyErr::new::<PyValueError, _>(format!(
                    "Invalid HTTP version: {i} (options: 0= HTTP/0.0, 1 | 10 = HTTP/1.0, 11 = HTTP/1.1, 2 | 20 = HTTP/2.0, 3 | 30 = HTTP/3.0)"
                ))),
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
macro_rules! impl_header_name_to_pystring_interned {
    ($(($hdr:ident, $val:expr)),* $(,)?) => {
        pub(crate) fn header_name_to_pystring_maybe<'py>(
            py: Python<'py>,
            name: &http::HeaderName,
        ) -> Option<&'py Bound<'py, PyString>> {
            match name {
                $( &http::header::$hdr => Some(intern!(py, $val)), )*
                _ => None
            }
        }
    };
}

// now invoke it with *only* CONTENT_TYPE, etc.
impl_header_name_to_pystring_interned! {
    (ACCEPT, "accept"),
    (ACCEPT_CHARSET, "accept-charset"),
    (ACCEPT_ENCODING, "accept-encoding"),
    (ACCEPT_LANGUAGE, "accept-language"),
    (ACCEPT_RANGES, "accept-ranges"),
    (ACCESS_CONTROL_ALLOW_CREDENTIALS, "access-control-allow-credentials"),
    (ACCESS_CONTROL_ALLOW_HEADERS, "access-control-allow-headers"),
    (ACCESS_CONTROL_ALLOW_METHODS, "access-control-allow-methods"),
    (ACCESS_CONTROL_ALLOW_ORIGIN, "access-control-allow-origin"),
    (ACCESS_CONTROL_EXPOSE_HEADERS, "access-control-expose-headers"),
    (ACCESS_CONTROL_MAX_AGE, "access-control-max-age"),
    (ACCESS_CONTROL_REQUEST_HEADERS, "access-control-request-headers"),
    (ACCESS_CONTROL_REQUEST_METHOD, "access-control-request-method"),
    (AGE, "age"),
    (ALLOW, "allow"),
    (ALT_SVC, "alt-svc"),
    (AUTHORIZATION, "authorization"),
    (CACHE_CONTROL, "cache-control"),
    (CACHE_STATUS, "cache-status"),
    (CDN_CACHE_CONTROL, "cdn-cache-control"),
    (CONNECTION, "connection"),
    (CONTENT_DISPOSITION, "content-disposition"),
    (CONTENT_ENCODING, "content-encoding"),
    (CONTENT_LANGUAGE, "content-language"),
    (CONTENT_LENGTH, "content-length"),
    (CONTENT_LOCATION, "content-location"),
    (CONTENT_RANGE, "content-range"),
    (CONTENT_SECURITY_POLICY, "content-security-policy"),
    (CONTENT_SECURITY_POLICY_REPORT_ONLY, "content-security-policy-report-only"),
    (CONTENT_TYPE, "content-type"),
    (COOKIE, "cookie"),
    (DNT, "dnt"),
    (DATE, "date"),
    (ETAG, "etag"),
    (EXPECT, "expect"),
    (EXPIRES, "expires"),
    (FORWARDED, "forwarded"),
    (FROM, "from"),
    (HOST, "host"),
    (IF_MATCH, "if-match"),
    (IF_MODIFIED_SINCE, "if-modified-since"),
    (IF_NONE_MATCH, "if-none-match"),
    (IF_RANGE, "if-range"),
    (IF_UNMODIFIED_SINCE, "if-unmodified-since"),
    (LAST_MODIFIED, "last-modified"),
    (LINK, "link"),
    (LOCATION, "location"),
    (MAX_FORWARDS, "max-forwards"),
    (ORIGIN, "origin"),
    (PRAGMA, "pragma"),
    (PROXY_AUTHENTICATE, "proxy-authenticate"),
    (PROXY_AUTHORIZATION, "proxy-authorization"),
    (PUBLIC_KEY_PINS, "public-key-pins"),
    (PUBLIC_KEY_PINS_REPORT_ONLY, "public-key-pins-report-only"),
    (RANGE, "range"),
    (REFERER, "referer"),
    (REFERRER_POLICY, "referrer-policy"),
    (REFRESH, "refresh"),
    (RETRY_AFTER, "retry-after"),
    (SEC_WEBSOCKET_ACCEPT, "sec-websocket-accept"),
    (SEC_WEBSOCKET_EXTENSIONS, "sec-websocket-extensions"),
    (SEC_WEBSOCKET_KEY, "sec-websocket-key"),
    (SEC_WEBSOCKET_PROTOCOL, "sec-websocket-protocol"),
    (SEC_WEBSOCKET_VERSION, "sec-websocket-version"),
    (SERVER, "server"),
    (SET_COOKIE, "set-cookie"),
    (STRICT_TRANSPORT_SECURITY, "strict-transport-security"),
    (TE, "te"),
    (TRAILER, "trailer"),
    (TRANSFER_ENCODING, "transfer-encoding"),
    (USER_AGENT, "user-agent"),
    (UPGRADE, "upgrade"),
    (UPGRADE_INSECURE_REQUESTS, "upgrade-insecure-requests"),
    (VARY, "vary"),
    (VIA, "via"),
    (WARNING, "warning"),
    (WWW_AUTHENTICATE, "www-authenticate"),
    (X_CONTENT_TYPE_OPTIONS, "x-content-type-options"),
    (X_DNS_PREFETCH_CONTROL, "x-dns-prefetch-control"),
    (X_FRAME_OPTIONS, "x-frame-options"),
    (X_XSS_PROTECTION, "x-xss-protection"),
}

pub(crate) fn header_name_to_pystring<'py>(
    py: Python<'py>,
    name: &http::HeaderName,
) -> PyResult<Bound<'py, PyAny>> {
    if let Some(s) = header_name_to_pystring_maybe(py, name) {
        s.into_bound_py_any(py)
    } else {
        let s = name.as_str();
        let s = PyString::new(py, s);
        s.into_bound_py_any(py)
    }
}

impl<'py> IntoPyObject<'py> for &HttpHeaderName {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        header_name_to_pystring(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for HttpHeaderName {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl FromPyObject<'_> for HttpHeaderName {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.cast_exact::<PyString>() {
            let s = s.extract::<&str>()?;
            http::HeaderName::from_bytes(s.as_bytes())
                .map(HttpHeaderName)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-name: {e}")))
        } else if let Ok(pyb) = ob.cast::<PyBytes>() {
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

impl<'py> IntoPyObject<'py> for &HttpHeaderNameRef<'_> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        header_name_to_pystring(py, self.0)
    }
}

impl<'py> IntoPyObject<'py> for HttpHeaderNameRef<'_> {
    // #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    // #[cfg(not(Py_LIMITED_API))]
    // type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        header_name_to_pystring(py, self.0)
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
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        header_value_to_pystring(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for HttpHeaderValue {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl FromPyObject<'_> for HttpHeaderValue {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            http::HeaderValue::from_str(s)
                .map(Self::from)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-value: {e}")))
        } else if let Ok(pyb) = ob.cast::<PyBytes>() {
            let s = pyb.as_bytes();
            http::HeaderValue::from_bytes(s)
                .map(Self::from)
                .map_err(|e| PyValueError::new_err(format!("invalid-header-value: {e}")))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid-header-value".to_string(),
            ))
        }
    }
}
