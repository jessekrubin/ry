use std::path::PathBuf;

use pyo3::prelude::*;
use reqwest::multipart::Form;
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::py_value_err;
use ryo3_http::{PyHeaders, PyHeadersLike};
use ryo3_macro_rules::py_type_err;

use crate::errors::map_reqwest_err;

#[derive(Debug)]
#[pyclass(name = "FormData", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub(crate) struct PyFormData {
    pub(crate) opts: PyFormDataOpts,
    // pub(crate) inner: Option<Form>,
}

#[derive(Debug, Clone)]
pub(crate) struct PyFormDataOpts {
    parts: Vec<PyFormPart>,
    percent_encode: Option<FormPercentEncode>,
}

impl TryFrom<&PyFormData> for reqwest::multipart::Form {
    type Error = PyErr;

    fn try_from(value: &PyFormData) -> Result<Self, Self::Error> {
        value.opts.build_request_multipart_form()
    }
}

impl PyFormDataOpts {
    pub(crate) fn build_request_multipart_form(&self) -> PyResult<Form> {
        let mut form = match self.percent_encode {
            Some(FormPercentEncode::PathSegment) => Form::new().percent_encode_path_segment(),
            Some(FormPercentEncode::AttrChars) => Form::new().percent_encode_attr_chars(),
            Some(FormPercentEncode::Noop) => Form::new().percent_encode_noop(),
            None => Form::new(),
        };
        for part in &self.parts {
            let p = part.build_request_multipart_part()?;
            form = form.part(part.0.opts.name.clone(), p);
        }
        Ok(form)
    }
}

#[pymethods]
impl PyFormData {
    #[new]
    #[pyo3(signature = (*parts, percent_encode = None))]
    fn py_new(parts: PyFormParts, percent_encode: Option<FormPercentEncode>) -> Self {
        let opts = PyFormDataOpts {
            parts: parts.0,
            percent_encode,
        };
        // let inner = opts.build_request_multipart_form()?;
        Self {
            opts,
            // inner: Some(inner),
        }
    }
}

impl Clone for PyFormData {
    fn clone(&self) -> Self {
        Self {
            opts: self.opts.clone(),
            // inner: None,
        }
    }
}

#[derive(Debug, Clone)]
struct PyFormParts(Vec<PyFormPart>);

impl<'py> FromPyObject<'_, 'py> for PyFormParts {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let parts_tuple = obj.cast_exact::<pyo3::types::PyTuple>()?;
        // iter collect and bail on first extraction error
        let mut parts: Vec<PyFormPart> = Vec::with_capacity(parts_tuple.len());
        for part_obj in parts_tuple.iter_borrowed() {
            let p = part_obj.cast_exact::<PyFormPart>()?;
            let r = p.get().0.clone();
            parts.push(PyFormPart(r));
        }
        Ok(Self(parts))
    }
}

#[derive(Debug)]
struct PyFormPartInner {
    // part: Option<reqwest::multipart::Part>,
    opts: PyFormPartOptions,
}

#[pyclass(name = "FormPart", frozen, immutable_type, skip_from_py_object)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFormPart(std::sync::Arc<PyFormPartInner>);

#[derive(Debug, Clone)]
struct PyFormPartOptions {
    name: String,
    body: FormBody,
    filename: Option<String>,
    // TODO: dont use PyHeaders directly here bc it could create a rwlock which is not really needed
    headers: Option<PyHeaders>,
    length: Option<usize>,
    mime: Option<String>,
}

impl PyFormPartOptions {
    fn build_request_multipart_part(&self) -> PyResult<reqwest::multipart::Part> {
        let mut part = match &self.body {
            FormBody::Text(s) => match self.length {
                Some(length) => {
                    reqwest::multipart::Part::stream_with_length(s.clone(), length as u64)
                }
                None => reqwest::multipart::Part::text(s.clone()),
            },
            FormBody::Bytes(b) => {
                // convert to bytes::Bytes
                let b: &bytes::Bytes = b.as_ref();
                match self.length {
                    Some(length) => {
                        reqwest::multipart::Part::stream_with_length(b.clone(), length as u64)
                    }
                    None => reqwest::multipart::Part::stream(b.clone()), // cheam bytes clone
                }
            }
            FormBody::File(p) => pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async { reqwest::multipart::Part::file(p).await })?,
        };
        if let Some(filename) = &self.filename {
            part = part.file_name(filename.clone());
        }
        if let Some(mime) = &self.mime {
            part = part.mime_str(mime).map_err(map_reqwest_err)?;
        }

        if let Some(headers) = &self.headers {
            let hm = headers.as_ref().read().clone();
            part = part.headers(hm);
        }
        Ok(part)
    }
}

#[derive(Debug)]
pub(crate) enum FormBody {
    // reqwest::multipart::Part::text
    Text(String),
    // reqwest::multipart::Part::bytes
    Bytes(RyBytes),
    // reqwest::multipart::Part::file
    File(PathBuf),
    // reqwest::multipart::Part::stream
    // todo: implement stream body
    // Stream(PyStream),
}

impl Clone for FormBody {
    fn clone(&self) -> Self {
        match self {
            Self::Text(s) => Self::Text(s.clone()),
            Self::Bytes(b) => {
                let b: &bytes::Bytes = b.as_ref();
                Self::Bytes(RyBytes::from(b.clone()))
            }
            Self::File(p) => Self::File(p.clone()),
            // FormBody::Stream(s) => FormBody::Stream(s.clone()),
        }
    }
}

#[pymethods]
impl PyFormPart {
    #[new]
    #[pyo3(signature = (
        name,
        body,
        *,
        filename = None,
        headers = None,
        length = None,
        mime = None,
    ))]
    fn py_new(
        name: String,
        body: FormBody,
        filename: Option<String>,
        headers: Option<PyHeadersLike>,
        length: Option<usize>,
        mime: Option<String>,
    ) -> PyResult<Self> {
        let headers = headers.map(std::convert::TryInto::try_into).transpose()?;
        let inner = PyFormPartInner {
            // part: None,
            opts: PyFormPartOptions {
                name,
                body,
                filename,
                headers,
                length,
                mime,
            },
        };
        Ok(Self(std::sync::Arc::new(inner)))
    }
}

impl PyFormPart {
    pub(crate) fn build_request_multipart_part(&self) -> PyResult<reqwest::multipart::Part> {
        self.0.opts.build_request_multipart_part()
    }
}

impl<'py> FromPyObject<'_, 'py> for FormBody {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<String>() {
            Ok(Self::Text(s))
        } else if let Ok(b) = obj.extract::<RyBytes>() {
            Ok(Self::Bytes(b))
        } else if let Ok(p) = obj.extract::<PathBuf>() {
            Ok(Self::File(p))
        // } else if let Ok(stream) = obj.extract::<PyStream>() {
        // Ok(FormBody::Stream(stream))
        } else {
            py_type_err!("Expected str, bytes, PathBuf, or stream object",)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FormPercentEncode {
    PathSegment,
    AttrChars,
    Noop,
}

impl<'py> IntoPyObject<'py> for FormPercentEncode {
    type Target = pyo3::types::PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self {
            Self::PathSegment => pyo3::intern!(py, "path-segment"),
            Self::AttrChars => pyo3::intern!(py, "attr-chars"),
            Self::Noop => pyo3::intern!(py, "noop"),
        };
        Ok(s.as_borrowed())
    }
}

const FORM_PERCENT_ENCODE_ACCEPTED: &str = "'path-segment', 'attr-chars', 'noop'";

impl<'py> FromPyObject<'_, 'py> for FormPercentEncode {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode.to_ascii_lowercase().as_str() {
                "path-segment" => Ok(Self::PathSegment),
                "attr-chars" => Ok(Self::AttrChars),
                "noop" => Ok(Self::Noop),
                _ => py_value_err!(
                    "Invalid percent-encode, expected a string (options: {FORM_PERCENT_ENCODE_ACCEPTED})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for round mode, expected a string (options: {FORM_PERCENT_ENCODE_ACCEPTED})"
            )
        }
    }
}
