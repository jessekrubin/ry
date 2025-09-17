#![allow(non_snake_case)]

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyString, PyTuple};

#[pyclass(name = "HttpStatus", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Copy, Debug)]
pub struct PyHttpStatus(pub http::StatusCode);

#[pymethods]
#[expect(clippy::trivially_copy_pass_by_ref)]
impl PyHttpStatus {
    #[new]
    #[pyo3(signature = (code))]
    pub(crate) fn py_new(code: u16) -> PyResult<Self> {
        Ok(Self(http::StatusCode::from_u16(code).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (code={code})"))
        })?))
    }

    #[expect(clippy::trivially_copy_pass_by_ref)]
    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, [self.0.as_u16()])
    }

    #[must_use]
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    fn __int__(&self) -> u16 {
        self.0.as_u16()
    }

    #[must_use]
    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> u16 {
        self.0.as_u16()
    }

    #[must_use]
    #[getter]
    fn canonical_reason<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.0.as_u16())
    }

    #[getter]
    #[must_use]
    fn reason<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.0.as_u16())
    }

    #[getter]
    #[must_use]
    fn is_informational(&self) -> bool {
        self.0.is_informational()
    }

    #[getter]
    #[must_use]
    fn is_success(&self) -> bool {
        self.0.is_success()
    }

    #[getter]
    #[must_use]
    fn is_redirect(&self) -> bool {
        self.0.is_redirection()
    }

    #[getter]
    #[must_use]
    fn is_redirection(&self) -> bool {
        self.0.is_redirection()
    }

    #[getter]
    #[must_use]
    fn is_client_error(&self) -> bool {
        self.0.is_client_error()
    }

    #[getter]
    #[must_use]
    fn is_server_error(&self) -> bool {
        self.0.is_server_error()
    }

    #[getter]
    #[must_use]
    fn is_error(&self) -> bool {
        self.0.is_server_error() || self.0.is_client_error()
    }

    #[getter]
    #[must_use]
    fn is_ok(&self) -> bool {
        self.0.is_success()
    }

    #[getter]
    #[must_use]
    fn ok(&self) -> bool {
        self.0.is_success()
    }

    #[must_use]
    fn __hash__(&self) -> u64 {
        u64::from(self.0.as_u16())
    }

    #[must_use]
    fn __bool__(&self) -> bool {
        self.0.is_success()
    }

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
        if let Ok(status_downcast_gucci) = other.cast_exact::<Self>() {
            let status = status_downcast_gucci.get();
            match op {
                CompareOp::Eq => Ok(self.0 == status.0),
                CompareOp::Ne => Ok(self.0 != status.0),
                CompareOp::Lt => Ok(self.0 < status.0),
                CompareOp::Le => Ok(self.0 <= status.0),
                CompareOp::Gt => Ok(self.0 > status.0),
                CompareOp::Ge => Ok(self.0 >= status.0),
            }
        } else {
            let status_extract_res = other.extract::<u16>();
            match status_extract_res {
                Ok(status) => match op {
                    CompareOp::Eq => Ok(self.0.as_u16() == status),
                    CompareOp::Ne => Ok(self.0.as_u16() != status),
                    CompareOp::Lt => Ok(self.0.as_u16() < status),
                    CompareOp::Le => Ok(self.0.as_u16() <= status),
                    CompareOp::Gt => Ok(self.0.as_u16() > status),
                    CompareOp::Ge => Ok(self.0.as_u16() >= status),
                },
                Err(_) => match op {
                    CompareOp::Eq => Ok(false),
                    CompareOp::Ne => Ok(true),
                    _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "http-status-code-invalid-comparison".to_string(),
                    )),
                },
            }
        }
    }

    // ========================================================================
    // CLASS ATTRS
    // ------------------------------------------------------------------------
    // The following was generated crudely and could be done with a macro but meh
    // ========================================================================

    #[expect(non_snake_case)]
    #[classattr]
    fn CONTINUE() -> Self {
        Self(http::StatusCode::CONTINUE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SWITCHING_PROTOCOLS() -> Self {
        Self(http::StatusCode::SWITCHING_PROTOCOLS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PROCESSING() -> Self {
        Self(http::StatusCode::PROCESSING)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn OK() -> Self {
        Self(http::StatusCode::OK)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn CREATED() -> Self {
        Self(http::StatusCode::CREATED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ACCEPTED() -> Self {
        Self(http::StatusCode::ACCEPTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NON_AUTHORITATIVE_INFORMATION() -> Self {
        Self(http::StatusCode::NON_AUTHORITATIVE_INFORMATION)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NO_CONTENT() -> Self {
        Self(http::StatusCode::NO_CONTENT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn RESET_CONTENT() -> Self {
        Self(http::StatusCode::RESET_CONTENT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PARTIAL_CONTENT() -> Self {
        Self(http::StatusCode::PARTIAL_CONTENT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MULTI_STATUS() -> Self {
        Self(http::StatusCode::MULTI_STATUS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ALREADY_REPORTED() -> Self {
        Self(http::StatusCode::ALREADY_REPORTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn IM_USED() -> Self {
        Self(http::StatusCode::IM_USED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MULTIPLE_CHOICES() -> Self {
        Self(http::StatusCode::MULTIPLE_CHOICES)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MOVED_PERMANENTLY() -> Self {
        Self(http::StatusCode::MOVED_PERMANENTLY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn FOUND() -> Self {
        Self(http::StatusCode::FOUND)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SEE_OTHER() -> Self {
        Self(http::StatusCode::SEE_OTHER)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_MODIFIED() -> Self {
        Self(http::StatusCode::NOT_MODIFIED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn USE_PROXY() -> Self {
        Self(http::StatusCode::USE_PROXY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn TEMPORARY_REDIRECT() -> Self {
        Self(http::StatusCode::TEMPORARY_REDIRECT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PERMANENT_REDIRECT() -> Self {
        Self(http::StatusCode::PERMANENT_REDIRECT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn BAD_REQUEST() -> Self {
        Self(http::StatusCode::BAD_REQUEST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNAUTHORIZED() -> Self {
        Self(http::StatusCode::UNAUTHORIZED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PAYMENT_REQUIRED() -> Self {
        Self(http::StatusCode::PAYMENT_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn FORBIDDEN() -> Self {
        Self(http::StatusCode::FORBIDDEN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_FOUND() -> Self {
        Self(http::StatusCode::NOT_FOUND)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn METHOD_NOT_ALLOWED() -> Self {
        Self(http::StatusCode::METHOD_NOT_ALLOWED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_ACCEPTABLE() -> Self {
        Self(http::StatusCode::NOT_ACCEPTABLE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PROXY_AUTHENTICATION_REQUIRED() -> Self {
        Self(http::StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn REQUEST_TIMEOUT() -> Self {
        Self(http::StatusCode::REQUEST_TIMEOUT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn CONFLICT() -> Self {
        Self(http::StatusCode::CONFLICT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn GONE() -> Self {
        Self(http::StatusCode::GONE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LENGTH_REQUIRED() -> Self {
        Self(http::StatusCode::LENGTH_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PRECONDITION_FAILED() -> Self {
        Self(http::StatusCode::PRECONDITION_FAILED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PAYLOAD_TOO_LARGE() -> Self {
        Self(http::StatusCode::PAYLOAD_TOO_LARGE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn URI_TOO_LONG() -> Self {
        Self(http::StatusCode::URI_TOO_LONG)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSUPPORTED_MEDIA_TYPE() -> Self {
        Self(http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn RANGE_NOT_SATISFIABLE() -> Self {
        Self(http::StatusCode::RANGE_NOT_SATISFIABLE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn EXPECTATION_FAILED() -> Self {
        Self(http::StatusCode::EXPECTATION_FAILED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn IM_A_TEAPOT() -> Self {
        Self(http::StatusCode::IM_A_TEAPOT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MISDIRECTED_REQUEST() -> Self {
        Self(http::StatusCode::MISDIRECTED_REQUEST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNPROCESSABLE_ENTITY() -> Self {
        Self(http::StatusCode::UNPROCESSABLE_ENTITY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCKED() -> Self {
        Self(http::StatusCode::LOCKED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn FAILED_DEPENDENCY() -> Self {
        Self(http::StatusCode::FAILED_DEPENDENCY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn TOO_EARLY() -> Self {
        Self(http::StatusCode::TOO_EARLY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UPGRADE_REQUIRED() -> Self {
        Self(http::StatusCode::UPGRADE_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PRECONDITION_REQUIRED() -> Self {
        Self(http::StatusCode::PRECONDITION_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn TOO_MANY_REQUESTS() -> Self {
        Self(http::StatusCode::TOO_MANY_REQUESTS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn REQUEST_HEADER_FIELDS_TOO_LARGE() -> Self {
        Self(http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNAVAILABLE_FOR_LEGAL_REASONS() -> Self {
        Self(http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn INTERNAL_SERVER_ERROR() -> Self {
        Self(http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_IMPLEMENTED() -> Self {
        Self(http::StatusCode::NOT_IMPLEMENTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn BAD_GATEWAY() -> Self {
        Self(http::StatusCode::BAD_GATEWAY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SERVICE_UNAVAILABLE() -> Self {
        Self(http::StatusCode::SERVICE_UNAVAILABLE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn GATEWAY_TIMEOUT() -> Self {
        Self(http::StatusCode::GATEWAY_TIMEOUT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn HTTP_VERSION_NOT_SUPPORTED() -> Self {
        Self(http::StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn VARIANT_ALSO_NEGOTIATES() -> Self {
        Self(http::StatusCode::VARIANT_ALSO_NEGOTIATES)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn INSUFFICIENT_STORAGE() -> Self {
        Self(http::StatusCode::INSUFFICIENT_STORAGE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOOP_DETECTED() -> Self {
        Self(http::StatusCode::LOOP_DETECTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_EXTENDED() -> Self {
        Self(http::StatusCode::NOT_EXTENDED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NETWORK_AUTHENTICATION_REQUIRED() -> Self {
        Self(http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
}

macro_rules! status_code_match {
    ($py:expr, $code:expr, {
        $(
            ($num:literal, $msg:literal)
        ),* $(,)?
    }) => {
        match $code {
            $(
                $num => Some(intern!($py, $msg)),
            )*
            _ => None,
        }
    };
}

pub fn status_code_pystring(py: Python<'_>, status_code: u16) -> Option<&Bound<'_, PyString>> {
    status_code_match!(py, status_code, {
        // 1xx
        (100, "Continue"),
        (101, "Switching Protocols"),
        (102, "Processing"),

        // 2xx
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (203, "Non Authoritative Information"), // should or should not be `Non-Authoritative`?
        (204, "No Content"),
        (205, "Reset Content"),
        (206, "Partial Content"),
        (207, "Multi-Status"),
        (208, "Already Reported"),
        (226, "IM Used"),

        // 3xx
        (300, "Multiple Choices"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (303, "See Other"),
        (304, "Not Modified"),
        (305, "Use Proxy"),
        (307, "Temporary Redirect"),
        (308, "Permanent Redirect"),

        // 4xx
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (402, "Payment Required"),
        (403, "Forbidden"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (406, "Not Acceptable"),
        (407, "Proxy Authentication Required"),
        (408, "Request Timeout"),
        (409, "Conflict"),
        (410, "Gone"),
        (411, "Length Required"),
        (412, "Precondition Failed"),
        (413, "Payload Too Large"),
        (414, "URI Too Long"),
        (415, "Unsupported Media Type"),
        (416, "Range Not Satisfiable"),
        (417, "Expectation Failed"),
        (418, "I'm a teapot"),
        (421, "Misdirected Request"),
        (422, "Unprocessable Entity"),
        (423, "Locked"),
        (424, "Failed Dependency"),
        (425, "Too Early"),
        (426, "Upgrade Required"),
        (428, "Precondition Required"),
        (429, "Too Many Requests"),
        (431, "Request Header Fields Too Large"),
        (451, "Unavailable For Legal Reasons"),

        // 5xx
        (500, "Internal Server Error"),
        (501, "Not Implemented"),
        (502, "Bad Gateway"),
        (503, "Service Unavailable"),
        (504, "Gateway Timeout"),
        (505, "HTTP Version Not Supported"),
        (506, "Variant Also Negotiates"),
        (507, "Insufficient Storage"),
        (508, "Loop Detected"),
        (510, "Not Extended"),
        (511, "Network Authentication Required"),
    })
}
