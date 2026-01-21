//! `PyHttpStatus` class
//!
//! Caches instances to avoid duplicating them when you are say scraping the
//! web and getting a ton of 200s or 404s or whatever...
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyString, PyTuple};
use ryo3_core::PyAsciiStr;
use ryo3_core::py_type_err;
use ryo3_core::{PyAsciiString, py_value_error};

// is the plural of status "stati" or "statuses"? who knows. literally nothing
// I can do to find that answer online.
/// cache with pyoncelocks of `PyHttpStatus` for statuses/stati(?) 100-599
static STATUS_CACHE: [PyOnceLock<Py<PyHttpStatus>>; 500] = [const { PyOnceLock::new() }; 500];

fn py_cached_http_status(py: Python<'_>, status: http::StatusCode) -> PyResult<Py<PyHttpStatus>> {
    let code = status.as_u16() as usize;
    if (100..=599).contains(&code) {
        let cell = &STATUS_CACHE[code - 100];
        cell.get_or_try_init(py, || Py::new(py, PyHttpStatus(status)))
            .map(|obj| obj.clone_ref(py))
    } else {
        // fuckit no cache
        Py::new(py, PyHttpStatus(status))
    }
}

#[pyclass(name = "HttpStatus", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Copy)]
pub struct PyHttpStatus(pub(crate) http::StatusCode);

impl PyHttpStatus {
    pub fn from_status_code_cached(py: Python<'_>, status: http::StatusCode) -> PyResult<Py<Self>> {
        py_cached_http_status(py, status)
    }
}

#[pymethods]
#[expect(clippy::trivially_copy_pass_by_ref)]
impl PyHttpStatus {
    #[new]
    #[pyo3(signature = (code))]
    fn py_new(py: Python<'_>, code: u16) -> PyResult<Py<Self>> {
        let code = http::StatusCode::from_u16(code)
            .map_err(|_| py_value_error!("Invalid HTTP status code: {code}"))?;
        py_cached_http_status(py, code)
    }

    #[expect(clippy::trivially_copy_pass_by_ref)]
    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, [self.0.as_u16()])
    }

    #[must_use]
    fn __str__(&self) -> PyAsciiStr<'_> {
        PyAsciiStr::from(self.0.as_str())
    }

    #[must_use]
    fn __repr__(&self) -> PyAsciiString {
        format!("{self:?}").into()
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
        } else if let Ok(i) = other.extract::<u16>() {
            match i {
                status => match op {
                    CompareOp::Eq => Ok(self.0.as_u16() == status),
                    CompareOp::Ne => Ok(self.0.as_u16() != status),
                    CompareOp::Lt => Ok(self.0.as_u16() < status),
                    CompareOp::Le => Ok(self.0.as_u16() <= status),
                    CompareOp::Gt => Ok(self.0.as_u16() > status),
                    CompareOp::Ge => Ok(self.0.as_u16() >= status),
                },
            }
        } else {
            match op {
                CompareOp::Eq => Ok(false),
                CompareOp::Ne => Ok(true),
                _ => py_type_err!("http-status-code-invalid-comparison"),
            }
        }
    }

    // ========================================================================
    // CLASS ATTRS
    // ------------------------------------------------------------------------
    // The following was generated crudely and could be done with a macro but meh
    // ========================================================================

    // <CLASS-ATTRS>
    /// 100 ~ Continue
    #[expect(non_snake_case)]
    #[classattr]
    fn CONTINUE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::CONTINUE)
    }

    /// 101 ~ Switching Protocols
    #[expect(non_snake_case)]
    #[classattr]
    fn SWITCHING_PROTOCOLS(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::SWITCHING_PROTOCOLS)
    }

    /// 102 ~ Processing
    #[expect(non_snake_case)]
    #[classattr]
    fn PROCESSING(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PROCESSING)
    }

    /// 103 ~ Early Hints
    #[expect(non_snake_case)]
    #[classattr]
    fn EARLY_HINTS(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::EARLY_HINTS)
    }

    /// 200 ~ OK
    #[expect(non_snake_case)]
    #[classattr]
    fn OK(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::OK)
    }

    /// 201 ~ Created
    #[expect(non_snake_case)]
    #[classattr]
    fn CREATED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::CREATED)
    }

    /// 202 ~ Accepted
    #[expect(non_snake_case)]
    #[classattr]
    fn ACCEPTED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::ACCEPTED)
    }

    /// 203 ~ Non Authoritative Information
    #[expect(non_snake_case)]
    #[classattr]
    fn NON_AUTHORITATIVE_INFORMATION(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NON_AUTHORITATIVE_INFORMATION)
    }

    /// 204 ~ No Content
    #[expect(non_snake_case)]
    #[classattr]
    fn NO_CONTENT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NO_CONTENT)
    }

    /// 205 ~ Reset Content
    #[expect(non_snake_case)]
    #[classattr]
    fn RESET_CONTENT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::RESET_CONTENT)
    }

    /// 206 ~ Partial Content
    #[expect(non_snake_case)]
    #[classattr]
    fn PARTIAL_CONTENT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PARTIAL_CONTENT)
    }

    /// 207 ~ Multi-Status
    #[expect(non_snake_case)]
    #[classattr]
    fn MULTI_STATUS(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::MULTI_STATUS)
    }

    /// 208 ~ Already Reported
    #[expect(non_snake_case)]
    #[classattr]
    fn ALREADY_REPORTED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::ALREADY_REPORTED)
    }

    /// 226 ~ IM Used
    #[expect(non_snake_case)]
    #[classattr]
    fn IM_USED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::IM_USED)
    }

    /// 300 ~ Multiple Choices
    #[expect(non_snake_case)]
    #[classattr]
    fn MULTIPLE_CHOICES(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::MULTIPLE_CHOICES)
    }

    /// 301 ~ Moved Permanently
    #[expect(non_snake_case)]
    #[classattr]
    fn MOVED_PERMANENTLY(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::MOVED_PERMANENTLY)
    }

    /// 302 ~ Found
    #[expect(non_snake_case)]
    #[classattr]
    fn FOUND(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::FOUND)
    }

    /// 303 ~ See Other
    #[expect(non_snake_case)]
    #[classattr]
    fn SEE_OTHER(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::SEE_OTHER)
    }

    /// 304 ~ Not Modified
    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_MODIFIED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NOT_MODIFIED)
    }

    /// 305 ~ Use Proxy
    #[expect(non_snake_case)]
    #[classattr]
    fn USE_PROXY(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::USE_PROXY)
    }

    /// 307 ~ Temporary Redirect
    #[expect(non_snake_case)]
    #[classattr]
    fn TEMPORARY_REDIRECT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::TEMPORARY_REDIRECT)
    }

    /// 308 ~ Permanent Redirect
    #[expect(non_snake_case)]
    #[classattr]
    fn PERMANENT_REDIRECT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PERMANENT_REDIRECT)
    }

    /// 400 ~ Bad Request
    #[expect(non_snake_case)]
    #[classattr]
    fn BAD_REQUEST(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::BAD_REQUEST)
    }

    /// 401 ~ Unauthorized
    #[expect(non_snake_case)]
    #[classattr]
    fn UNAUTHORIZED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::UNAUTHORIZED)
    }

    /// 402 ~ Payment Required
    #[expect(non_snake_case)]
    #[classattr]
    fn PAYMENT_REQUIRED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PAYMENT_REQUIRED)
    }

    /// 403 ~ Forbidden
    #[expect(non_snake_case)]
    #[classattr]
    fn FORBIDDEN(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::FORBIDDEN)
    }

    /// 404 ~ Not Found
    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_FOUND(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NOT_FOUND)
    }

    /// 405 ~ Method Not Allowed
    #[expect(non_snake_case)]
    #[classattr]
    fn METHOD_NOT_ALLOWED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::METHOD_NOT_ALLOWED)
    }

    /// 406 ~ Not Acceptable
    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_ACCEPTABLE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NOT_ACCEPTABLE)
    }

    /// 407 ~ Proxy Authentication Required
    #[expect(non_snake_case)]
    #[classattr]
    fn PROXY_AUTHENTICATION_REQUIRED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }

    /// 408 ~ Request Timeout
    #[expect(non_snake_case)]
    #[classattr]
    fn REQUEST_TIMEOUT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::REQUEST_TIMEOUT)
    }

    /// 409 ~ Conflict
    #[expect(non_snake_case)]
    #[classattr]
    fn CONFLICT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::CONFLICT)
    }

    /// 410 ~ Gone
    #[expect(non_snake_case)]
    #[classattr]
    fn GONE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::GONE)
    }

    /// 411 ~ Length Required
    #[expect(non_snake_case)]
    #[classattr]
    fn LENGTH_REQUIRED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::LENGTH_REQUIRED)
    }

    /// 412 ~ Precondition Failed
    #[expect(non_snake_case)]
    #[classattr]
    fn PRECONDITION_FAILED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PRECONDITION_FAILED)
    }

    /// 413 ~ Payload Too Large
    #[expect(non_snake_case)]
    #[classattr]
    fn PAYLOAD_TOO_LARGE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PAYLOAD_TOO_LARGE)
    }

    /// 414 ~ URI Too Long
    #[expect(non_snake_case)]
    #[classattr]
    fn URI_TOO_LONG(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::URI_TOO_LONG)
    }

    /// 415 ~ Unsupported Media Type
    #[expect(non_snake_case)]
    #[classattr]
    fn UNSUPPORTED_MEDIA_TYPE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }

    /// 416 ~ Range Not Satisfiable
    #[expect(non_snake_case)]
    #[classattr]
    fn RANGE_NOT_SATISFIABLE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::RANGE_NOT_SATISFIABLE)
    }

    /// 417 ~ Expectation Failed
    #[expect(non_snake_case)]
    #[classattr]
    fn EXPECTATION_FAILED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::EXPECTATION_FAILED)
    }

    /// 418 ~ I'm a teapot
    #[expect(non_snake_case)]
    #[classattr]
    fn IM_A_TEAPOT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::IM_A_TEAPOT)
    }

    /// 421 ~ Misdirected Request
    #[expect(non_snake_case)]
    #[classattr]
    fn MISDIRECTED_REQUEST(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::MISDIRECTED_REQUEST)
    }

    /// 422 ~ Unprocessable Entity
    #[expect(non_snake_case)]
    #[classattr]
    fn UNPROCESSABLE_ENTITY(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::UNPROCESSABLE_ENTITY)
    }

    /// 423 ~ Locked
    #[expect(non_snake_case)]
    #[classattr]
    fn LOCKED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::LOCKED)
    }

    /// 424 ~ Failed Dependency
    #[expect(non_snake_case)]
    #[classattr]
    fn FAILED_DEPENDENCY(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::FAILED_DEPENDENCY)
    }

    /// 425 ~ Too Early
    #[expect(non_snake_case)]
    #[classattr]
    fn TOO_EARLY(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::TOO_EARLY)
    }

    /// 426 ~ Upgrade Required
    #[expect(non_snake_case)]
    #[classattr]
    fn UPGRADE_REQUIRED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::UPGRADE_REQUIRED)
    }

    /// 428 ~ Precondition Required
    #[expect(non_snake_case)]
    #[classattr]
    fn PRECONDITION_REQUIRED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::PRECONDITION_REQUIRED)
    }

    /// 429 ~ Too Many Requests
    #[expect(non_snake_case)]
    #[classattr]
    fn TOO_MANY_REQUESTS(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::TOO_MANY_REQUESTS)
    }

    /// 431 ~ Request Header Fields Too Large
    #[expect(non_snake_case)]
    #[classattr]
    fn REQUEST_HEADER_FIELDS_TOO_LARGE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
    }

    /// 451 ~ Unavailable For Legal Reasons
    #[expect(non_snake_case)]
    #[classattr]
    fn UNAVAILABLE_FOR_LEGAL_REASONS(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
    }

    /// 500 ~ Internal Server Error
    #[expect(non_snake_case)]
    #[classattr]
    fn INTERNAL_SERVER_ERROR(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// 501 ~ Not Implemented
    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_IMPLEMENTED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NOT_IMPLEMENTED)
    }

    /// 502 ~ Bad Gateway
    #[expect(non_snake_case)]
    #[classattr]
    fn BAD_GATEWAY(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::BAD_GATEWAY)
    }

    /// 503 ~ Service Unavailable
    #[expect(non_snake_case)]
    #[classattr]
    fn SERVICE_UNAVAILABLE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::SERVICE_UNAVAILABLE)
    }

    /// 504 ~ Gateway Timeout
    #[expect(non_snake_case)]
    #[classattr]
    fn GATEWAY_TIMEOUT(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::GATEWAY_TIMEOUT)
    }

    /// 505 ~ HTTP Version Not Supported
    #[expect(non_snake_case)]
    #[classattr]
    fn HTTP_VERSION_NOT_SUPPORTED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }

    /// 506 ~ Variant Also Negotiates
    #[expect(non_snake_case)]
    #[classattr]
    fn VARIANT_ALSO_NEGOTIATES(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::VARIANT_ALSO_NEGOTIATES)
    }

    /// 507 ~ Insufficient Storage
    #[expect(non_snake_case)]
    #[classattr]
    fn INSUFFICIENT_STORAGE(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::INSUFFICIENT_STORAGE)
    }

    /// 508 ~ Loop Detected
    #[expect(non_snake_case)]
    #[classattr]
    fn LOOP_DETECTED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::LOOP_DETECTED)
    }

    /// 510 ~ Not Extended
    #[expect(non_snake_case)]
    #[classattr]
    fn NOT_EXTENDED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NOT_EXTENDED)
    }

    /// 511 ~ Network Authentication Required
    #[expect(non_snake_case)]
    #[classattr]
    fn NETWORK_AUTHENTICATION_REQUIRED(py: Python<'_>) -> PyResult<Py<Self>> {
        py_cached_http_status(py, ::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
    // </CLASS-ATTRS>
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
        (103, "Early Hints"),

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

impl std::fmt::Debug for PyHttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpStatus({})", self.0.as_str())
    }
}
