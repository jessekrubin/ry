use std::convert::Into;
use std::time::Duration;

use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
use reqwest::header::{HeaderMap, HeaderValue};
use ryo3_http::{PyHeadersLike, PyHttpVersion};
use ryo3_macro_rules::{py_type_err, py_value_err, py_value_error, pytodo};
use ryo3_std::time::PyTimeout;

use crate::types::{BasicAuth, PyQuery};

pub(crate) struct ReqwestKwargs<const BLOCKING: bool = false> {
    headers: Option<HeaderMap>,
    query: Option<PyQuery>,
    body: PyReqwestBody,
    timeout: Option<Duration>,
    basic_auth: Option<BasicAuth>,
    bearer_auth: Option<PyBackedStr>,
    version: Option<PyHttpVersion>,
}

pub(crate) type BlockingReqwestKwargs = ReqwestKwargs<true>;

impl<const BLOCKING: bool> ReqwestKwargs<BLOCKING> {
    /// Apply the kwargs to the `reqwest::RequestBuilder`
    #[inline]
    pub(crate) fn apply(self, req: reqwest::RequestBuilder) -> PyResult<reqwest::RequestBuilder> {
        let mut req = req;

        // headers
        if let Some(headers) = self.headers {
            req = req.headers(headers);
        }

        // query
        if let Some(query) = self.query {
            // temp hack we know that the query is already url-encoded so we
            // decode it and then re-encode it...
            let decoded: Vec<(&str, &str)> = serde_urlencoded::from_str(query.as_ref())
                .map_err(|err| py_value_error!("failed to decode query params: {err}"))?;
            req = req.query(&decoded);
        }

        // body
        req = match self.body {
            PyReqwestBody::Bytes(b) => req.body(b),
            PyReqwestBody::Stream(s) => req.body(s),
            PyReqwestBody::Json(j) => req.body(j).header(
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ),
            PyReqwestBody::Form(f) => req.body(f).header(
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            ),
            PyReqwestBody::Multipart(_m) => {
                pytodo!("multipart not implemented (yet)");
            }
            PyReqwestBody::None => req,
        };

        // timeout
        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout);
        }

        // basic auth
        if let Some(basic_auth) = self.basic_auth {
            req = req.basic_auth(basic_auth.username(), basic_auth.password());
        }

        // bearer auth
        if let Some(token) = self.bearer_auth {
            req = req.bearer_auth(token);
        }

        // version
        if let Some(version) = self.version {
            req = req.version(version.into());
        }

        Ok(req)
    }
}

#[derive(Debug)]
enum PyReqwestBody {
    Bytes(bytes::Bytes),
    Stream(crate::body::PyBodyStream),
    Json(Vec<u8>),
    Form(String),
    #[expect(dead_code)]
    Multipart(bool), // placeholder
    None,
}

#[inline]
fn extract_body_from_py_body<const BLOCKING: bool>(
    body: Borrowed<'_, '_, PyAny>,
) -> PyResult<PyReqwestBody> {
    let py_body = body.extract::<crate::body::PyBody>()?;
    match py_body {
        crate::body::PyBody::Bytes(bs) => Ok(PyReqwestBody::Bytes(bs.into_inner())),
        crate::body::PyBody::Stream(s) => {
            if BLOCKING && s.is_async() {
                return py_type_err!("cannot use async stream body with blocking client");
            }
            Ok(PyReqwestBody::Stream(s))
        }
    }
}

#[inline]
fn extract_form_body(form: Borrowed<'_, '_, PyAny>) -> PyResult<PyReqwestBody> {
    let py_any_serializer = ryo3_serde::PyAnySerializer::new(form, None);
    let url_encoded_form = serde_urlencoded::to_string(py_any_serializer)
        .map_err(|e| py_value_error!("failed to serialize form data: {e}"))?;
    Ok(PyReqwestBody::Form(url_encoded_form))
}

#[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
#[inline]
fn extract_kwargs<const BLOCKING: bool>(
    dict: Borrowed<'_, '_, PyDict>,
) -> PyResult<ReqwestKwargs<BLOCKING>> {
    let py = dict.py();

    // body parts...
    let body = dict.get_item(pyo3::intern!(py, "body"))?;
    let json = dict.get_item(pyo3::intern!(py, "json"))?;
    let form = dict.get_item(pyo3::intern!(py, "form"))?;
    let multipart = dict.get_item(pyo3::intern!(py, "multipart"))?;

    let query: Option<PyQuery> = dict
        .get_item(pyo3::intern!(py, "query"))?
        .map(|q| q.extract::<PyQuery>())
        .transpose()?;
    let body: PyReqwestBody = match (body, json, form, multipart) {
        (Some(_), Some(_), _, _)
        | (Some(_), _, Some(_), _)
        | (Some(_), _, _, Some(_))
        | (_, Some(_), Some(_), _)
        | (_, Some(_), _, Some(_))
        | (_, _, Some(_), Some(_)) => {
            return py_value_err!("body, json, form, multipart are mutually exclusive");
        }
        (Some(body), None, None, None) => {
            extract_body_from_py_body::<BLOCKING>(body.as_borrowed())?
        }
        (None, Some(json), None, None) => {
            PyReqwestBody::Json(ryo3_json::to_vec(json.as_borrowed())?)
        }
        (None, None, Some(form), None) => extract_form_body(form.as_borrowed())?,
        (None, None, None, Some(_multipart)) => pytodo!("multipart not implemented (yet)"),
        (None, None, None, None) => PyReqwestBody::None,
    };
    let timeout = dict
        .get_item(pyo3::intern!(py, "timeout"))?
        .map(|t| t.extract::<PyTimeout>())
        .transpose()?
        .map(Duration::from);
    let headers = dict
        .get_item(pyo3::intern!(py, "headers"))?
        .map(|h| h.extract::<PyHeadersLike>())
        .transpose()?
        .map(PyHeadersLike::into_header_map);
    let basic_auth: Option<BasicAuth> = dict
        .get_item(pyo3::intern!(py, "basic_auth"))?
        .map(|b| b.extract())
        .transpose()?;
    let bearer_auth: Option<PyBackedStr> = dict
        .get_item(pyo3::intern!(py, "bearer_auth"))?
        .map(|b| b.extract())
        .transpose()?;
    let version: Option<PyHttpVersion> = dict
        .get_item(pyo3::intern!(py, "version"))?
        .map(|v| v.extract())
        .transpose()?;
    Ok(ReqwestKwargs {
        headers,
        query,
        body,
        timeout,
        basic_auth,
        bearer_auth,
        version,
    })
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
#[inline]
fn extract_kwargs<const BLOCKING: bool>(
    dict: Borrowed<'_, '_, PyDict>,
) -> PyResult<ReqwestKwargs<BLOCKING>> {
    let mut res = ReqwestKwargs {
        body: PyReqwestBody::None,
        headers: None,
        query: None,
        timeout: None,
        basic_auth: None,
        bearer_auth: None,
        version: None,
    };

    let mut body_set = false;
    for kwarg in ryo3_core::py_dict::KwargsIter::new(dict) {
        let (key, value) = kwarg?;
        match key {
            "body" => {
                if body_set {
                    return py_value_err!("body, json, form, multipart are mutually exclusive");
                }
                body_set = true;
                res.body = extract_body_from_py_body::<BLOCKING>(value)?;
            }
            "json" => {
                if body_set {
                    return py_value_err!("body, json, form, multipart are mutually exclusive");
                }
                body_set = true;
                res.body = PyReqwestBody::Json(ryo3_json::to_vec(value)?);
            }
            "form" => {
                if body_set {
                    return py_value_err!("body, json, form, multipart are mutually exclusive");
                }
                body_set = true;
                res.body = extract_form_body(value)?;
            }
            "multipart" => {
                pytodo!("multipart not implemented (yet)");
            }
            "query" => {
                res.query = Some(value.extract()?);
            }
            "headers" => {
                res.headers = Some(
                    value
                        .extract::<PyHeadersLike>()
                        .map(PyHeadersLike::into_header_map)?,
                );
            }
            "timeout" => {
                res.timeout = Some(Duration::from(value.extract::<PyTimeout>()?));
            }
            "basic_auth" => {
                res.basic_auth = Some(value.extract()?);
            }
            "bearer_auth" => {
                res.bearer_auth = Some(value.extract()?);
            }
            "version" => {
                res.version = Some(value.extract()?);
            }
            _ => {
                return py_type_err!("unexpected keyword argument: {key}");
            }
        }
    }
    Ok(res)
}

impl<'py, const BLOCKING: bool> FromPyObject<'_, 'py> for ReqwestKwargs<BLOCKING> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        extract_kwargs(obj.cast_exact::<PyDict>()?)
    }
}
