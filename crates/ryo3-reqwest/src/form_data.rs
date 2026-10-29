//! multipart/form-data ~ `FormData` et al
//!
//! This [`FormData`] implementation aims to provide a javascript-like API for handling multipart/form-data.
//!
//! The actual pyclass structs are faux-multipart forms; they don't
//! actually build the multipart form but assemble the entries for `reqwest` to
//! consume when sending a request.
//!
//! FormPart-values are coerced by type, not value:
//! - `str`/`int`/`float`/`bool` -> text
//! - bytes-like -> binary
//! - `os.PathLike` -> file
//! - `FormPart` -> itself
//! - `list` -> repeated field (dict/pairs only)
//!
//! REF: <https://developer.mozilla.org/en-US/docs/Web/API/FormData>
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bytes::Bytes;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyIterator, PyList, PyString};
use reqwest::header::HeaderMap;
use reqwest::multipart::Form;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::py_dict::KwargsIter;
use ryo3_core::sync::RyRwLock;
use ryo3_http::{PyHeaders, PyHeadersLike};
use ryo3_macro_rules::{py_key_err, py_type_err, py_type_error, py_value_err, py_value_error};

use crate::errors::map_reqwest_err;
use crate::utf8_bytes::Utf8Bytes;

// ============================================================================
// FormPart
// ============================================================================

/// python wrapper for single multipart/form-data part/entry

#[pyclass(name = "FormPart", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RyFormPart(Arc<PartInner>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartInner {
    value: PartValue,
    filename: Option<String>,
    mime: Option<mime::Mime>,
    headers: Option<HeaderMap>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PartValue {
    Text(Utf8Bytes),
    Bytes(Bytes),
    File(PathBuf),
}

impl PartInner {
    fn new(value: PartValue) -> Self {
        let (filename, mime) = match &value {
            PartValue::File(p) => (
                p.file_name().map(|f| f.to_string_lossy().into_owned()),
                Some(mime_guess::from_path(p).first_or_octet_stream()),
            ),
            _ => (None, None),
        };
        Self {
            value,
            filename,
            mime,
            headers: None,
        }
    }

    fn plain_text(&self) -> Option<&str> {
        match &self.value {
            PartValue::Text(s)
                if self.filename.is_none() && self.mime.is_none() && self.headers.is_none() =>
            {
                Some(s.as_str())
            }
            _ => None,
        }
    }

    /// build the reqwest part ~ opens files
    fn to_part(&self) -> PyResult<reqwest::multipart::Part> {
        let mut part = match &self.value {
            PartValue::Text(s) => {
                let b = s.as_bytes();
                reqwest::multipart::Part::stream_with_length(b.clone(), b.len() as u64)
            }
            PartValue::Bytes(b) => {
                reqwest::multipart::Part::stream_with_length(b.clone(), b.len() as u64)
            }
            PartValue::File(p) => {
                let file = std::fs::File::open(p).map_err(|e| io_err_with_path(e, p))?;
                let len = file.metadata().map(|m| m.len());
                let file = tokio::fs::File::from_std(file);
                match len {
                    Ok(len) => reqwest::multipart::Part::stream_with_length(file, len),
                    Err(_) => reqwest::multipart::Part::stream(file),
                }
            }
        };
        if let Some(filename) = &self.filename {
            part = part.file_name(filename.clone());
        }
        if let Some(mime) = &self.mime {
            part = part.mime_str(mime.as_ref()).map_err(map_reqwest_err)?;
        }
        if let Some(headers) = &self.headers {
            part = part.headers(headers.clone());
        }
        Ok(part)
    }
}

impl RyFormPart {
    /// apply overrides ~ in place if the part is not shared
    fn with_kwargs(mut self, kwargs: Option<FormPartKwargs>) -> Self {
        let Some(FormPartKwargs {
            filename,
            mime,
            headers,
        }) = kwargs
        else {
            return self;
        };
        if filename.is_none() && mime.is_none() && headers.is_none() {
            return self;
        }
        let inner = Arc::make_mut(&mut self.0);
        if filename.is_some() {
            inner.filename = filename;
        }
        if mime.is_some() {
            inner.mime = mime;
        }
        if headers.is_some() {
            inner.headers = headers;
        }
        self
    }
}

/// `filename`/`mime`/`headers` kwargs for `FormPart`/`append`/`set`
#[derive(Default)]
struct FormPartKwargs {
    filename: Option<String>,
    mime: Option<mime::Mime>,
    headers: Option<HeaderMap>,
}

impl FromPyObject<'_, '_> for FormPartKwargs {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        let mut kwargs = Self::default();
        for kwarg in KwargsIter::new(obj.cast_exact::<PyDict>()?) {
            let (key, value) = kwarg?;
            match key {
                "filename" => kwargs.filename = value.extract()?,
                "mime" => {
                    if let Some(s) = value.extract::<Option<&str>>()? {
                        let mime = s
                            .parse()
                            .map_err(|e| py_value_error!("invalid mime type '{s}': {e}"))?;
                        kwargs.mime = Some(mime);
                    }
                }
                "headers" => {
                    kwargs.headers = value
                        .extract::<Option<PyHeadersLike>>()?
                        .map(PyHeadersLike::into_header_map);
                }
                _ => return py_type_err!("unexpected keyword argument: {key}"),
            }
        }
        Ok(kwargs)
    }
}

// TODO: Move this to `ryo3-core`?
/// Copy pasta io error handling for Python w/ a fielpath
fn io_err_with_path(e: std::io::Error, path: &Path) -> PyErr {
    match e.raw_os_error() {
        Some(errno) => PyOSError::new_err((errno, e.to_string(), path.to_path_buf())),
        None => e.into(),
    }
}

impl FromPyObject<'_, '_> for PartValue {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = obj.cast::<PyString>() {
            Utf8Bytes::try_from(s.to_owned()).map(Self::Text)
        } else if let Ok(b) = obj.cast_exact::<PyBool>() {
            let s = if b.is_true() { "true" } else { "false" };
            Ok(Self::Text(Utf8Bytes::from_static(s)))
        } else if obj.is_instance_of::<PyInt>() || obj.is_instance_of::<PyFloat>() {
            Utf8Bytes::try_from(obj.str()?).map(Self::Text)
        } else if let Ok(buf) = obj.extract::<ReadableBuffer>() {
            // TODO: besure we are zero-copy
            Ok(Self::Bytes(buf.into_bytes()))
        } else if let Ok(p) = obj.extract::<PathBuf>() {
            Ok(Self::File(p))
        } else {
            let tname = obj.get_type().name()?;
            py_type_err!(
                "invalid form-data value type '{tname}' (expected str, int, float, bool, bytes-like, os.PathLike or FormPart)"
            )
        }
    }
}

impl FromPyObject<'_, '_> for RyFormPart {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        if let Ok(part) = obj.cast_exact::<Self>() {
            Ok(part.get().clone())
        } else {
            obj.extract().map(|v| Self(Arc::new(PartInner::new(v))))
        }
    }
}

/// `str` for plain text parts, otherwise the `FormPart`
struct FormValue<'a>(&'a RyFormPart);

impl<'py> IntoPyObject<'py> for FormValue<'_> {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        match self.0.0.plain_text() {
            Some(s) => s.into_bound_py_any(py),
            None => self.0.clone().into_bound_py_any(py),
        }
    }
}

impl std::fmt::Display for RyFormPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &*self.0;
        f.write_str("FormPart(")?;
        match &inner.value {
            PartValue::Text(s) => write!(f, "{:?}", s.as_str())?,
            PartValue::Bytes(b) => write!(f, "{}", RyBytes::new(b.clone()))?,
            PartValue::File(p) => write!(f, "Path({:?})", p.to_string_lossy())?,
        }
        if let Some(filename) = &inner.filename {
            write!(f, ", filename={filename:?}")?;
        }
        if let Some(mime) = &inner.mime {
            write!(f, ", mime={mime:?}")?;
        }
        if let Some(headers) = &inner.headers {
            write!(f, ", headers={:#}", PyHeaders::from(headers.clone()))?;
        }
        f.write_char(')')
    }
}

#[pymethods]
impl RyFormPart {
    #[new]
    #[pyo3(
        signature = (value, /, **kwargs),
        text_signature = "(value, /, *, filename=None, mime=None, headers=None)"
    )]
    fn py_new(value: Self, kwargs: Option<FormPartKwargs>) -> Self {
        value.with_kwargs(kwargs)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    /// `str` | `Bytes` | `pathlib.Path`
    #[getter]
    fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        match &self.0.value {
            PartValue::Text(s) => s.as_str().into_bound_py_any(py),
            PartValue::Bytes(b) => RyBytes::new(b.clone()).into_bound_py_any(py),
            PartValue::File(p) => p.into_bound_py_any(py),
        }
    }

    #[getter]
    fn filename(&self) -> Option<&str> {
        self.0.filename.as_deref()
    }

    #[getter]
    fn mime(&self) -> Option<&str> {
        self.0.mime.as_ref().map(AsRef::as_ref)
    }

    #[getter]
    fn headers(&self) -> Option<PyHeaders> {
        self.0.headers.clone().map(PyHeaders::from)
    }
}

// ============================================================================
// FormData
// ============================================================================

type FormEntry = (String, RyFormPart);
type FormEntries = Arc<Vec<FormEntry>>;

#[pyclass(name = "FormData", frozen, immutable_type, mapping)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyFormData {
    entries: RyRwLock<FormEntries, false>,
    percent_encode: Option<FormPercentEncode>,
}

impl RyFormData {
    fn new(entries: FormEntries, percent_encode: Option<FormPercentEncode>) -> Self {
        Self {
            entries: RyRwLock::new(entries),
            percent_encode,
        }
    }

    // TODO: make actual iterable?
    fn entries(&self) -> FormEntries {
        Arc::clone(&self.entries.py_read())
    }

    /// cow-like mutate the entries
    fn mutate<R>(&self, f: impl FnOnce(&mut Vec<FormEntry>) -> R) -> R {
        f(Arc::make_mut(&mut self.entries.py_write()))
    }

    /// remove all values under `name` returning the number of removed entries
    fn remove(&self, name: &str) -> usize {
        self.mutate(|entries| {
            let len = entries.len();
            entries.retain(|(n, _)| n != name);
            len - entries.len()
        })
    }
}

/// `FormData` | `dict[str, value | list[value]]` | iterable of
/// `(str, value | list[value])` pairs ~ also what a request holds
#[derive(Debug, Default)]
pub(crate) struct FormDataLike {
    entries: FormEntries,
    percent_encode: Option<FormPercentEncode>,
}

impl FormDataLike {
    /// build the reqwest form ~ opens files
    pub(crate) fn into_form(self) -> PyResult<Form> {
        let form = match self.percent_encode {
            Some(pe) => pe.apply(Form::new()),
            None => Form::new(),
        };
        self.entries.iter().try_fold(form, |form, (name, part)| {
            Ok(form.part(name.clone(), part.0.to_part()?))
        })
    }
}

impl From<&RyFormData> for FormDataLike {
    fn from(fd: &RyFormData) -> Self {
        Self {
            entries: fd.entries(),
            percent_encode: fd.percent_encode,
        }
    }
}

/// push `value` under `name` ~ a `list` is a repeated field
fn push_entry(
    entries: &mut Vec<FormEntry>,
    name: String,
    value: &Bound<'_, PyAny>,
) -> PyResult<()> {
    if let Ok(list) = value.cast::<PyList>() {
        entries.reserve(list.len());
        for item in list {
            entries.push((name.clone(), item.extract()?));
        }
    } else {
        entries.push((name, value.extract()?));
    }
    Ok(())
}

const FORM_DATA_LIKE_EXPECTED: &str = "expected FormData, dict or iterable of (name, value) pairs";

impl FromPyObject<'_, '_> for FormDataLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        if let Ok(fd) = obj.cast_exact::<RyFormData>() {
            return Ok(Self::from(fd.get()));
        }
        let mut entries = Vec::new();
        if let Ok(dict) = obj.cast::<PyDict>() {
            entries.reserve(dict.len());
            for (k, v) in dict.iter() {
                push_entry(&mut entries, k.extract()?, &v)?;
            }
        } else if obj.is_instance_of::<PyString>() {
            // str is iterable but never what anyone means here
            return py_type_err!("{FORM_DATA_LIKE_EXPECTED}; got str");
        } else {
            for pair in obj.try_iter()? {
                let (name, value): (String, Bound<'_, PyAny>) = pair?
                    .extract()
                    .map_err(|_| py_type_error!("{FORM_DATA_LIKE_EXPECTED}"))?;
                push_entry(&mut entries, name, &value)?;
            }
        }
        Ok(Self {
            entries: Arc::new(entries),
            percent_encode: None,
        })
    }
}

impl std::fmt::Display for RyFormData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FormData([")?;
        for (i, (name, part)) in self.entries().iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            match part.0.plain_text() {
                Some(s) => write!(f, "({name:?}, {s:?})")?,
                None => write!(f, "({name:?}, {part})")?,
            }
        }
        f.write_char(']')?;
        if let Some(pe) = self.percent_encode {
            write!(f, ", percent_encode={:?}", pe.as_str())?;
        }
        f.write_char(')')
    }
}

#[pymethods]
impl RyFormData {
    #[new]
    #[pyo3(signature = (data = None, /, *, percent_encode = None))]
    fn py_new(data: Option<FormDataLike>, percent_encode: Option<FormPercentEncode>) -> Self {
        let data = data.unwrap_or_default();
        Self::new(data.entries, percent_encode.or(data.percent_encode))
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __len__(&self) -> usize {
        self.entries.py_read().len()
    }

    fn __contains__(&self, name: &str) -> bool {
        self.entries.py_read().iter().any(|(n, _)| n == name)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.percent_encode == other.percent_encode && self.entries() == other.entries()
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        self.get(py, name, None)?
            .map_or_else(|| py_key_err!("{name}"), Ok)
    }

    fn __setitem__(&self, name: String, value: RyFormPart) {
        self.set(name, value, None);
    }

    fn __delitem__(&self, name: &str) -> PyResult<()> {
        if self.remove(name) > 0 {
            Ok(())
        } else {
            py_key_err!("{name}")
        }
    }

    /// names w/ duplicates (js `keys()`)
    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.keys(py)?.try_iter()
    }

    fn __copy__(&self) -> Self {
        self.copy()
    }

    /// parts are immutable so shallow == deep
    fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        self.copy()
    }

    fn copy(&self) -> Self {
        Self::new(self.entries(), self.percent_encode)
    }

    #[getter]
    fn percent_encode(&self) -> Option<&'static str> {
        self.percent_encode.map(FormPercentEncode::as_str)
    }

    #[pyo3(
        signature = (name, value, /, **kwargs),
        text_signature = "($self, name, value, /, *, filename=None, mime=None, headers=None)"
    )]
    fn append(&self, name: String, value: RyFormPart, kwargs: Option<FormPartKwargs>) {
        let part = value.with_kwargs(kwargs);
        self.mutate(|entries| entries.push((name, part)));
    }

    /// replace the first value under `name` and drop the rest (or append)
    #[pyo3(
        signature = (name, value, /, **kwargs),
        text_signature = "($self, name, value, /, *, filename=None, mime=None, headers=None)"
    )]
    fn set(&self, name: String, value: RyFormPart, kwargs: Option<FormPartKwargs>) {
        let mut part = Some(value.with_kwargs(kwargs));
        self.mutate(|entries| {
            entries.retain_mut(|(n, p)| *n != name || part.take().map(|v| *p = v).is_some());
            if let Some(part) = part {
                entries.push((name, part));
            }
        });
    }

    /// remove all values under `name` ~ no error if missing (js)
    fn delete(&self, name: &str) {
        self.remove(name);
    }

    #[pyo3(signature = (name, default = None))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        default: Option<Bound<'py, PyAny>>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.entries().iter().find(|(n, _)| n == name) {
            Some((_, part)) => FormValue(part).into_bound_py_any(py).map(Some),
            None => Ok(default),
        }
    }

    fn getall<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyList>> {
        let entries = self.entries();
        PyList::new(
            py,
            entries
                .iter()
                .filter(|(n, _)| n == name)
                .map(|(_, p)| FormValue(p)),
        )
    }

    fn keys<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        PyList::new(py, self.entries().iter().map(|(n, _)| n))
    }

    fn values<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        PyList::new(py, self.entries().iter().map(|(_, p)| FormValue(p)))
    }

    fn items<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        PyList::new(py, self.entries().iter().map(|(n, p)| (n, FormValue(p))))
    }
}

// ============================================================================
// percent-encode
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FormPercentEncode {
    PathSegment,
    AttrChars,
    Noop,
}

impl FormPercentEncode {
    fn apply(self, form: Form) -> Form {
        match self {
            Self::PathSegment => form.percent_encode_path_segment(),
            Self::AttrChars => form.percent_encode_attr_chars(),
            Self::Noop => form.percent_encode_noop(),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::PathSegment => "path-segment",
            Self::AttrChars => "attr-chars",
            Self::Noop => "noop",
        }
    }
}

const FORM_PERCENT_ENCODE_ACCEPTED: &str = "'path-segment', 'attr-chars', 'noop'";

impl FromPyObject<'_, '_> for FormPercentEncode {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        match ob.extract::<&str>() {
            Ok("path-segment") => Ok(Self::PathSegment),
            Ok("attr-chars") => Ok(Self::AttrChars),
            Ok("noop") => Ok(Self::Noop),
            Ok(s) => py_value_err!(
                "invalid percent-encode '{s}' (options: {FORM_PERCENT_ENCODE_ACCEPTED})"
            ),
            Err(_) => py_type_err!(
                "invalid percent-encode, expected a string (options: {FORM_PERCENT_ENCODE_ACCEPTED})"
            ),
        }
    }
}
