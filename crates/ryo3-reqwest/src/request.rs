use std::convert::Into;
use std::time::Duration;

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
use reqwest::header::{HeaderMap, HeaderValue};
use ryo3_core::kwargs::KwargsIter;
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
pub(crate) struct ReqwestKwargs2<const BLOCKING: bool = false> {
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
    #[inline]
    pub(crate) fn benchmark_score(&self) -> usize {
        usize::from(self.headers.is_some())
            + usize::from(self.query.is_some())
            + usize::from(!matches!(self.body, PyReqwestBody::None))
            + usize::from(self.timeout.is_some())
            + usize::from(self.basic_auth.is_some())
            + usize::from(self.bearer_auth.is_some())
            + usize::from(self.version.is_some())
    }

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

impl<const BLOCKING: bool> ReqwestKwargs2<BLOCKING> {
    #[inline]
    pub(crate) fn benchmark_score(&self) -> usize {
        usize::from(self.headers.is_some())
            + usize::from(self.query.is_some())
            + usize::from(!matches!(self.body, PyReqwestBody::None))
            + usize::from(self.timeout.is_some())
            + usize::from(self.basic_auth.is_some())
            + usize::from(self.bearer_auth.is_some())
            + usize::from(self.version.is_some())
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
                extract_body_from_py_body::<BLOCKING>(body.as_borrowed())?
            }
            (None, Some(json), None, None) => PyReqwestBody::Json(ryo3_json::to_vec(&json)?),
            (None, None, Some(form), None) => extract_form_body(form.as_borrowed())?,
            (None, None, None, Some(_multipart)) => {
                pytodo!("multipart not implemented (yet)")
            }
            (None, None, None, None) => PyReqwestBody::None,
        };

        let timeout = dict
            .get_item(intern!(py, "timeout"))?
            .map(|t| t.extract::<PyTimeout>())
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

//=-=========================================================================

// mod kwargs_iter {

//     //! BORROWED ITERATORS!!!
//     use pyo3::ffi;
//     use pyo3::prelude::*;
//     use pyo3::types::{PyAny, PyDict};

//     // ----------------------------------------------------------------------------
//     // DICT
//     // ----------------------------------------------------------------------------

//     /// Modified from `pyo3::types::dict`
//     ///
//     /// Big advantage is ref counts arent messed w/ so only use if you know the
//     /// dict is not being modified during iteration
//     pub(crate) struct BorrowedDictIter<'a, 'py> {
//         dict: Borrowed<'a, 'py, PyDict>,
//         ppos: ffi::Py_ssize_t,
//         remaining: usize,
//     }

//     impl<'a, 'py> Iterator for BorrowedDictIter<'a, 'py> {
//         type Item = (Borrowed<'a, 'py, PyAny>, Borrowed<'a, 'py, PyAny>);

//         #[inline]
//         fn next(&mut self) -> Option<Self::Item> {
//             let mut key_ptr: *mut ffi::PyObject = std::ptr::null_mut();
//             let mut val_ptr: *mut ffi::PyObject = std::ptr::null_mut();

//             #[expect(unsafe_code)]
//             // Safety: self.dict lives sufficiently long that the pointer is not dangling
//             if unsafe {
//                 ffi::PyDict_Next(
//                     self.dict.as_ptr(),
//                     &raw mut self.ppos,
//                     &raw mut key_ptr,
//                     &raw mut val_ptr,
//                 )
//             } != 0
//             {
//                 self.remaining -= 1;
//                 let py = self.dict.py();
//                 // Safety:
//                 // - PyDict_Next returns borrowed values
//                 // - we have already checked that `PyDict_Next` succeeded, so we can assume these to be non-null
//                 let map_key = unsafe { Borrowed::from_ptr(py, key_ptr) };
//                 let map_val = unsafe { Borrowed::from_ptr(py, val_ptr) };
//                 Some((map_key, map_val))
//             } else {
//                 None
//             }
//         }

//         #[inline]
//         fn size_hint(&self) -> (usize, Option<usize>) {
//             let len = self.len();
//             (len, Some(len))
//         }

//         #[inline]
//         fn count(self) -> usize
//         where
//             Self: Sized,
//         {
//             self.len()
//         }
//     }

//     impl ExactSizeIterator for BorrowedDictIter<'_, '_> {
//         fn len(&self) -> usize {
//             self.remaining
//         }
//     }

//     impl<'a, 'py> BorrowedDictIter<'a, 'py> {
//         pub(crate) fn new(dict: Borrowed<'a, 'py, PyDict>) -> Self {
//             let len = dict.len();
//             BorrowedDictIter {
//                 dict,
//                 ppos: 0,
//                 remaining: len,
//             }
//         }
//     }
// }

impl<'py, const BLOCKING: bool> FromPyObject<'_, 'py> for ReqwestKwargs2<BLOCKING> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dict = obj.cast_exact::<PyDict>()?;

        let kwargs = KwargsIter::new(dict);
        // use thingy::BorrowedDictIter;

        let mut res = Self {
            body: PyReqwestBody::None,
            headers: None,
            query: None,
            timeout: None,
            basic_auth: None,
            bearer_auth: None,
            version: None,
        };

        let mut body_set = false;
        for (key, value) in kwargs {
            // let key_str: &str = key.extract()?;
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
                    res.body = PyReqwestBody::Json(ryo3_json::to_vec(&value)?);
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
                    res.headers = Some(HeaderMap::from(value.extract::<PyHeadersLike>()?));
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
                _other => {
                    return py_type_err!("unexpected keyword argument: {key}");
                }
            }
        }
        Ok(res)
    }
}

#[pyfunction(signature = (**kwargs))]
pub(crate) fn _bench_extract_reqwest_kwargs(kwargs: Option<ReqwestKwargs>) -> PyResult<usize> {
    Ok(kwargs.as_ref().map_or(0, ReqwestKwargs::benchmark_score))
}

#[pyfunction(signature = (**kwargs))]
pub(crate) fn _bench_extract_reqwest_kwargs2(kwargs: Option<ReqwestKwargs2>) -> PyResult<usize> {
    Ok(kwargs.as_ref().map_or(0, ReqwestKwargs2::benchmark_score))
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
