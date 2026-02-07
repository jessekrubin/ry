use crate::types::PyQuery;
use crate::types::{BasicAuth, Timeout};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
use reqwest::header::{HeaderMap, HeaderValue};
use ryo3_http::{PyHeadersLike, PyHttpVersion};
use ryo3_macro_rules::{py_type_err, py_value_err, py_value_error, pytodo};
use std::convert::Into;
use std::time::Duration;

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
    #[allow(dead_code)]
    Multipart(bool), // placeholder
    None,
}

impl<'py, const BLOCKING: bool> FromPyObject<'_, 'py> for ReqwestKwargs<BLOCKING> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        let dict = obj.cast_exact::<PyDict>()?;

        // body parts...
        let body = dict.get_item(intern!(py, "body"))?;
        let json = dict.get_item(intern!(py, "json"))?;
        let form = dict.get_item(intern!(py, "form"))?;
        let multipart = dict.get_item(intern!(py, "multipart"))?;

        // let query: PyResult<Option<String>> =
        let query: Option<PyQuery> = dict
            .get_item(intern!(py, "query"))?
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
                let py_body = body.extract::<crate::body::PyBody>()?;
                match py_body {
                    crate::body::PyBody::Bytes(bs) => PyReqwestBody::Bytes(bs.into_inner()),
                    crate::body::PyBody::Stream(s) => {
                        // using an async stream with blocking client is a no-go (yo)
                        if BLOCKING {
                            if s.is_async() {
                                return py_type_err!(
                                    "cannot use async stream body with blocking client"
                                );
                            }
                            PyReqwestBody::Stream(s)
                        } else {
                            PyReqwestBody::Stream(s)
                        }
                    }
                }
            }
            (None, Some(json), None, None) => {
                let b = ryo3_json::to_vec(&json)?;
                PyReqwestBody::Json(b)
            }
            (None, None, Some(form), None) => {
                let py_any_serializer = ryo3_serde::PyAnySerializer::new(form.as_borrowed(), None);
                let url_encoded_form = serde_urlencoded::to_string(py_any_serializer)
                    .map_err(|e| py_value_error!("failed to serialize form data: {e}"))?;
                PyReqwestBody::Form(url_encoded_form)
            }
            (None, None, None, Some(_multipart)) => {
                pytodo!("multipart not implemented (yet)")
            }
            (None, None, None, None) => PyReqwestBody::None,
        };

        let timeout = dict
            .get_item(intern!(py, "timeout"))?
            .map(|t| t.extract::<Timeout>())
            .transpose()?
            .map(Duration::from);
        let headers = dict
            .get_item(intern!(py, "headers"))?
            .map(|h| h.extract::<PyHeadersLike>())
            .transpose()?
            .map(HeaderMap::try_from)
            .transpose()?;
        let bearer_auth: Option<PyBackedStr> = dict
            .get_item(intern!(py, "bearer_auth"))?
            .map(|b| b.extract())
            .transpose()?;
        let version: Option<PyHttpVersion> = dict
            .get_item(intern!(py, "version"))?
            .map(|v| v.extract())
            .transpose()?;
        Ok(Self {
            body,
            headers,
            query,
            timeout,
            basic_auth: dict
                .get_item(intern!(obj.py(), "basic_auth"))?
                .map(|b| b.extract())
                .transpose()?,
            bearer_auth,
            version,
        })
    }
}

// ===========================================================================
// REQWEST KWARGS BUILDER TODO?
// ===========================================================================

// pub(crate) struct ReqwestKwargsBuilder<const BLOCKING: bool = false> {
//     headers: Option<PyHeadersLike>,
//     query: Option<PyQuery>,
//     body: Option<PyBody>,
//     json: Option<PyRequestJson>,
//     form: Option<String>,
//     multipart: Option<String>,
//     timeout: Option<Timeout>,
//     basic_auth: Option<BasicAuth>,
//     bearer_auth: Option<PyBackedStr>,
//     version: Option<PyHttpVersion>,
// }

// macro_rules! impl_reqwest_kwargs_builder_field {
//     ($field:ident, $ty:ty) => {
//         fn $field(self, $field: Option<$ty>) -> Self {
//             Self { $field, ..self }
//         }
//     };
// }
// impl<const BLOCKING: bool> ReqwestKwargsBuilder<BLOCKING> {
//     fn new() -> Self {
//         Self {
//             headers: None,
//             query: None,
//             body: None,
//             json: None,
//             form: None,
//             multipart: None,
//             timeout: None,
//             basic_auth: None,
//             bearer_auth: None,
//             version: None,
//         }
//     }
//     impl_reqwest_kwargs_builder_field!(headers, PyHeadersLike);
//     impl_reqwest_kwargs_builder_field!(query, PyQuery);
//     impl_reqwest_kwargs_builder_field!(body, PyBody);
//     impl_reqwest_kwargs_builder_field!(json, PyRequestJson);
//     impl_reqwest_kwargs_builder_field!(form, String);
//     impl_reqwest_kwargs_builder_field!(multipart, String);
//     impl_reqwest_kwargs_builder_field!(timeout, Timeout);
//     impl_reqwest_kwargs_builder_field!(basic_auth, BasicAuth);
//     impl_reqwest_kwargs_builder_field!(bearer_auth, PyBackedStr);
//     impl_reqwest_kwargs_builder_field!(version, PyHttpVersion);

//     fn request_body(&mut self) -> PyResult<PyReqwestBody> {
//         let parts = (
//             self.body.take(),
//             self.json.take(),
//             self.form.take(),
//             self.multipart.is_some(), // TODO: impl
//         );
//         match parts {
//             (Some(_), Some(_), _, _)
//             | (Some(_), _, Some(_), _)
//             | (Some(_), _, _, true)
//             | (_, Some(_), Some(_), _)
//             | (_, Some(_), _, true)
//             | (_, _, Some(_), true) => {
//                 py_value_err!("body, json, form, multipart are mutually exclusive")
//             }
//             (Some(body), None, None, false) => match body {
//                 PyBody::Bytes(b) => Ok(PyReqwestBody::Bytes(b.into())),
//                 PyBody::Stream(s) => {
//                     if BLOCKING && s.is_async() {
//                         return py_type_err!("cannot use async stream body with blocking client");
//                     }
//                     Ok(PyReqwestBody::Stream(s))
//                 }
//             },
//             (None, Some(json), None, false) => Ok(PyReqwestBody::Json(json.into())),
//             (None, None, Some(form), false) => Ok(PyReqwestBody::Form(form)),
//             (None, None, None, true) => {
//                 pytodo!("multipart not implemented (yet)");
//             }
//             (None, None, None, false) => Ok(PyReqwestBody::None),
//         }
//     }

//     fn build(self) -> ReqwestKwargs<BLOCKING> {
//         let mut slf = self; // TODO: fix this?
//         let body = slf.request_body().unwrap_or(PyReqwestBody::None);
//         ReqwestKwargs {
//             headers: slf.headers.map(Into::into),
//             query: slf.query,
//             body,
//             timeout: slf.timeout.map(Into::into),
//             basic_auth: slf.basic_auth,
//             bearer_auth: slf.bearer_auth,
//             version: slf.version,
//         }
//     }
// }
