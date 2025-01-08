use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;

#[pyclass(name = "HttpStatus", module = "ry.ryo3.http")]
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
    pub fn canonical_reason(&self) -> Option<&'static str> {
        self.0.canonical_reason()
    }

    #[must_use]
    pub fn reason(&self) -> Option<&'static str> {
        self.0.canonical_reason()
    }

    #[must_use]
    pub fn is_informational(&self) -> bool {
        self.0.is_informational()
    }

    #[must_use]
    pub fn is_success(&self) -> bool {
        self.0.is_success()
    }

    #[must_use]
    pub fn is_redirection(&self) -> bool {
        self.0.is_redirection()
    }

    #[must_use]
    pub fn is_client_error(&self) -> bool {
        self.0.is_client_error()
    }

    #[must_use]
    pub fn is_server_error(&self) -> bool {
        self.0.is_server_error()
    }

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
        let downcast_res = other.downcast::<PyHttpStatus>();
        if let Ok(status_downcast_gucci) = downcast_res {
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

    #[allow(non_snake_case)]
    #[classattr]
    fn CONTINUE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::CONTINUE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn SWITCHING_PROTOCOLS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::SWITCHING_PROTOCOLS)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PROCESSING() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PROCESSING)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn OK() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::OK)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn CREATED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::CREATED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ACCEPTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::ACCEPTED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NON_AUTHORITATIVE_INFORMATION() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NON_AUTHORITATIVE_INFORMATION)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NO_CONTENT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NO_CONTENT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn RESET_CONTENT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::RESET_CONTENT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PARTIAL_CONTENT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PARTIAL_CONTENT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MULTI_STATUS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MULTI_STATUS)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ALREADY_REPORTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::ALREADY_REPORTED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn IM_USED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::IM_USED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MULTIPLE_CHOICES() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MULTIPLE_CHOICES)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MOVED_PERMANENTLY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MOVED_PERMANENTLY)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn FOUND() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::FOUND)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn SEE_OTHER() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::SEE_OTHER)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NOT_MODIFIED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_MODIFIED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn USE_PROXY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::USE_PROXY)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn TEMPORARY_REDIRECT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::TEMPORARY_REDIRECT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PERMANENT_REDIRECT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PERMANENT_REDIRECT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn BAD_REQUEST() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::BAD_REQUEST)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UNAUTHORIZED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNAUTHORIZED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PAYMENT_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PAYMENT_REQUIRED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn FORBIDDEN() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::FORBIDDEN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NOT_FOUND() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_FOUND)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn METHOD_NOT_ALLOWED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::METHOD_NOT_ALLOWED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NOT_ACCEPTABLE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_ACCEPTABLE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PROXY_AUTHENTICATION_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn REQUEST_TIMEOUT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::REQUEST_TIMEOUT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn CONFLICT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::CONFLICT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn GONE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::GONE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn LENGTH_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::LENGTH_REQUIRED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PRECONDITION_FAILED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PRECONDITION_FAILED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PAYLOAD_TOO_LARGE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PAYLOAD_TOO_LARGE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn URI_TOO_LONG() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::URI_TOO_LONG)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UNSUPPORTED_MEDIA_TYPE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn RANGE_NOT_SATISFIABLE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::RANGE_NOT_SATISFIABLE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn EXPECTATION_FAILED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::EXPECTATION_FAILED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn IM_A_TEAPOT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::IM_A_TEAPOT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MISDIRECTED_REQUEST() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::MISDIRECTED_REQUEST)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UNPROCESSABLE_ENTITY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNPROCESSABLE_ENTITY)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn LOCKED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::LOCKED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn FAILED_DEPENDENCY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::FAILED_DEPENDENCY)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn TOO_EARLY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::TOO_EARLY)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UPGRADE_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UPGRADE_REQUIRED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PRECONDITION_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::PRECONDITION_REQUIRED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn TOO_MANY_REQUESTS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::TOO_MANY_REQUESTS)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn REQUEST_HEADER_FIELDS_TOO_LARGE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UNAVAILABLE_FOR_LEGAL_REASONS() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn INTERNAL_SERVER_ERROR() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NOT_IMPLEMENTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_IMPLEMENTED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn BAD_GATEWAY() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::BAD_GATEWAY)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn SERVICE_UNAVAILABLE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::SERVICE_UNAVAILABLE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn GATEWAY_TIMEOUT() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::GATEWAY_TIMEOUT)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn HTTP_VERSION_NOT_SUPPORTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn VARIANT_ALSO_NEGOTIATES() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::VARIANT_ALSO_NEGOTIATES)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn INSUFFICIENT_STORAGE() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::INSUFFICIENT_STORAGE)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn LOOP_DETECTED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::LOOP_DETECTED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NOT_EXTENDED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NOT_EXTENDED)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NETWORK_AUTHENTICATION_REQUIRED() -> PyHttpStatus {
        PyHttpStatus(http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
}
