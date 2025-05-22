#![allow(non_snake_case)]

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyString, PyTuple};

#[pyclass(name = "HttpStatus", module = "ry.ryo3.http", frozen)]
#[derive(Clone, Debug)]
pub struct PyHttpStatus(pub http::StatusCode);

#[pymethods]
impl PyHttpStatus {
    #[new]
    #[pyo3(signature = (code))]
    fn py_new(code: u16) -> PyResult<Self> {
        Ok(Self(http::StatusCode::from_u16(code).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (code={code})"))
        })?))
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, [self.0.as_u16()])
    }

    #[must_use]
    pub fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    pub fn __int__(&self) -> u16 {
        self.0.as_u16()
    }

    #[must_use]
    pub fn to_py(&self) -> u16 {
        self.0.as_u16()
    }

    #[must_use]
    pub fn _canonical_reason_orig(&self) -> Option<&'static str> {
        self.0.canonical_reason()
    }

    #[must_use]
    #[getter]
    pub fn canonical_reason<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.0.as_u16())
    }

    #[getter]
    #[must_use]
    pub fn reason<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.0.as_u16())
    }

    #[getter]
    #[must_use]
    pub fn is_informational(&self) -> bool {
        self.0.is_informational()
    }

    #[getter]
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.0.is_success()
    }

    #[getter]
    #[must_use]
    pub fn is_redirect(&self) -> bool {
        self.0.is_redirection()
    }

    #[getter]
    #[must_use]
    pub fn is_redirection(&self) -> bool {
        self.0.is_redirection()
    }

    #[getter]
    #[must_use]
    pub fn is_client_error(&self) -> bool {
        self.0.is_client_error()
    }

    #[getter]
    #[must_use]
    pub fn is_server_error(&self) -> bool {
        self.0.is_server_error()
    }

    #[getter]
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.0.is_server_error() || self.0.is_client_error()
    }

    #[getter]
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.0.is_success()
    }

    #[getter]
    #[must_use]
    pub fn ok(&self) -> bool {
        self.0.is_success()
    }

    #[must_use]
    pub fn __hash__(&self) -> u64 {
        u64::from(self.0.as_u16())
    }

    #[must_use]
    pub fn __bool__(&self) -> bool {
        self.0.is_success()
    }

    pub fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
        if let Ok(status_downcast_gucci) = other.downcast::<PyHttpStatus>() {
            let status = status_downcast_gucci.extract::<PyHttpStatus>()?;
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
    fn CONTINUE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::CONTINUE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SWITCHING_PROTOCOLS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::SWITCHING_PROTOCOLS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PROCESSING() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PROCESSING)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn OK() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::OK)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn CREATED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::CREATED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ACCEPTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::ACCEPTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NON_AUTHORITATIVE_INFORMATION() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NON_AUTHORITATIVE_INFORMATION)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NO_CONTENT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NO_CONTENT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn RESET_CONTENT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::RESET_CONTENT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PARTIAL_CONTENT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PARTIAL_CONTENT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MULTI_STATUS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MULTI_STATUS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ALREADY_REPORTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::ALREADY_REPORTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn IM_USED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::IM_USED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MULTIPLE_CHOICES() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MULTIPLE_CHOICES)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MOVED_PERMANENTLY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MOVED_PERMANENTLY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn FOUND() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::FOUND)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SEE_OTHER() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::SEE_OTHER)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_MODIFIED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_MODIFIED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn USE_PROXY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::USE_PROXY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn TEMPORARY_REDIRECT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::TEMPORARY_REDIRECT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PERMANENT_REDIRECT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PERMANENT_REDIRECT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn BAD_REQUEST() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::BAD_REQUEST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNAUTHORIZED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNAUTHORIZED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PAYMENT_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PAYMENT_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn FORBIDDEN() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::FORBIDDEN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_FOUND() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_FOUND)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn METHOD_NOT_ALLOWED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::METHOD_NOT_ALLOWED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_ACCEPTABLE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_ACCEPTABLE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PROXY_AUTHENTICATION_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn REQUEST_TIMEOUT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::REQUEST_TIMEOUT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn CONFLICT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::CONFLICT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn GONE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::GONE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LENGTH_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::LENGTH_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PRECONDITION_FAILED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PRECONDITION_FAILED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PAYLOAD_TOO_LARGE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PAYLOAD_TOO_LARGE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn URI_TOO_LONG() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::URI_TOO_LONG)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSUPPORTED_MEDIA_TYPE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn RANGE_NOT_SATISFIABLE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::RANGE_NOT_SATISFIABLE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn EXPECTATION_FAILED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::EXPECTATION_FAILED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn IM_A_TEAPOT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::IM_A_TEAPOT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MISDIRECTED_REQUEST() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MISDIRECTED_REQUEST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNPROCESSABLE_ENTITY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNPROCESSABLE_ENTITY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCKED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::LOCKED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn FAILED_DEPENDENCY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::FAILED_DEPENDENCY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn TOO_EARLY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::TOO_EARLY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UPGRADE_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UPGRADE_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn PRECONDITION_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PRECONDITION_REQUIRED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn TOO_MANY_REQUESTS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::TOO_MANY_REQUESTS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn REQUEST_HEADER_FIELDS_TOO_LARGE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNAVAILABLE_FOR_LEGAL_REASONS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn INTERNAL_SERVER_ERROR() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_IMPLEMENTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_IMPLEMENTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn BAD_GATEWAY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::BAD_GATEWAY)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SERVICE_UNAVAILABLE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::SERVICE_UNAVAILABLE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn GATEWAY_TIMEOUT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::GATEWAY_TIMEOUT)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn HTTP_VERSION_NOT_SUPPORTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn VARIANT_ALSO_NEGOTIATES() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::VARIANT_ALSO_NEGOTIATES)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn INSUFFICIENT_STORAGE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::INSUFFICIENT_STORAGE)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOOP_DETECTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::LOOP_DETECTED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_EXTENDED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_EXTENDED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NETWORK_AUTHENTICATION_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
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

pub fn status_code_pystring(py: Python, status_code: u16) -> Option<&Bound<'_, PyString>> {
    status_code_match!(py, status_code, {
        (100, "Continue"),
        (101, "Switching Protocols"),
        (102, "Processing"),

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

        (300, "Multiple Choices"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (303, "See Other"),
        (304, "Not Modified"),
        (305, "Use Proxy"),
        (307, "Temporary Redirect"),
        (308, "Permanent Redirect"),

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
