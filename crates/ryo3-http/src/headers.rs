use std::fmt::Display;
use std::ops::Deref;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};

use http::header::HeaderMap;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use ryo3_core::macros::{py_runtime_error, py_value_error};
use ryo3_core::sync::RyRwLock;

use crate::http_types::{
    PyHttpHeaderMapRef, PyHttpHeaderName, PyHttpHeaderNameRef, PyHttpHeaderValue,
    PyHttpHeaderValueRef,
};
use crate::{PyHeadersLike, PyHttpHeaderMap};

#[pyclass(name = "Headers", frozen, immutable_type, mapping, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyHeaders(pub Arc<RyRwLock<HeaderMap, false>>);

impl Deref for PyHeaders {
    type Target = Arc<RyRwLock<HeaderMap, false>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PyHeaders {
    #[inline]
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, HeaderMap> {
        self.0.py_read()
    }

    #[inline]
    pub(crate) fn with_read<R>(&self, f: impl FnOnce(&HeaderMap) -> R) -> R {
        let inner = self.read();
        f(&inner)
    }

    #[inline]
    fn write(&self) -> RwLockWriteGuard<'_, HeaderMap> {
        self.0.py_write()
    }

    #[must_use]
    pub fn clone_header_map(&self) -> HeaderMap {
        self.read().clone()
    }

    #[must_use]
    pub fn into_header_map(self) -> HeaderMap {
        match Arc::try_unwrap(self.0) {
            Ok(lock) => lock
                .0
                .into_inner()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
            Err(shared) => shared.py_read().clone(),
        }
    }

    fn py_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        PyHttpHeaderMapRef(&self.read()).into_pyobject(py)
    }

    #[cfg(feature = "pydantic")]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        use pyo3::BoundObject;
        let py = value.py();
        if let Ok(headers) = value.cast_exact::<Self>() {
            Ok(headers.as_borrowed().into_bound())
        } else {
            let headers_like = value.extract::<PyHeadersLike>()?;
            Py::new(py, Self::from(headers_like)).map(|headers| headers.into_bound(py))
        }
    }
}

impl Display for PyHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.read();
        if f.alternate() {
            write!(f, "{inner:?}")
        } else {
            write!(f, "Headers({inner:?})")
        }
    }
}

impl PartialEq for PyHeaders {
    fn eq(&self, other: &Self) -> bool {
        *(self.read()) == *(other.read())
    }
}

impl Eq for PyHeaders {}

impl From<HeaderMap> for PyHeaders {
    fn from(hm: HeaderMap) -> Self {
        Self(Arc::new(RyRwLock::new(hm)))
    }
}

impl From<PyHttpHeaderMap> for PyHeaders {
    fn from(hm: PyHttpHeaderMap) -> Self {
        Self::from(HeaderMap::from(hm))
    }
}

impl From<PyHeaders> for HeaderMap {
    fn from(h: PyHeaders) -> Self {
        h.into_header_map()
    }
}

impl From<Arc<RyRwLock<HeaderMap, false>>> for PyHeaders {
    fn from(hm: Arc<RyRwLock<HeaderMap, false>>) -> Self {
        Self(hm)
    }
}

#[pymethods]
impl PyHeaders {
    #[new]
    #[pyo3(signature = (headers = None, /, **kwargs))]
    fn py_new(headers: Option<PyHeadersLike>, kwargs: Option<PyHeadersLike>) -> Self {
        match (headers, kwargs) {
            (Some(headers), Some(kwargs)) => {
                let mut headers_map = headers.into_header_map();
                headers_map.extend(kwargs.into_header_map());
                Self::from(headers_map)
            }
            (Some(headers), None) => {
                let headers_map = headers.into_header_map();
                Self::from(headers_map)
            }
            (None, Some(kwargs)) => {
                let kw_headers = kwargs.into_header_map();
                Self::from(kw_headers)
            }
            (None, None) => Self::from(HeaderMap::new()),
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let dict = self.py_dict(py)?;
        PyTuple::new(py, vec![dict])
    }

    /// Return struct Debug-string
    #[must_use]
    fn __dbg__(&self) -> String {
        format!("{self:?}")
    }

    #[must_use]
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[must_use]
    fn __len__(&self) -> usize {
        self.with_read(http::HeaderMap::len)
    }

    #[must_use]
    fn __eq__(&self, other: &Self) -> bool {
        self.with_read(|inner| inner == &*(other.read()))
    }

    #[must_use]
    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    #[must_use]
    fn __contains__(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn __getitem__(&self, key: &str) -> Option<PyHttpHeaderValue> {
        self.with_read(|inner| inner.get(key).map(PyHttpHeaderValue::from))
    }

    fn __setitem__(&self, key: PyHttpHeaderName, value: PyHttpHeaderValue) -> PyResult<()> {
        self.insert(key, value)?;
        Ok(())
    }

    fn __delitem__(&self, key: PyHttpHeaderName) {
        self.remove(key);
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        self.keys(py)
    }

    // ========================================================================
    // Methods of `HeaderMap`:
    // ========================================================================
    // - `append`: impl via `try_append`
    // - `capacity`:
    // - `clear`:
    // - `contains_key`:
    // - `entry`:
    // - `get`:
    // - `get_all`:
    // - `insert`:
    // - `is_empty`:
    // - `iter`:
    // - `keys`:
    // - `keys_len`:
    // - `len`:
    // - `remove`:
    // - `try_append`: `append`
    // - `try_entry`: `entry`
    // - `try_insert`: `insert`
    // - `values`:
    // - TBD
    //     - `drain`
    //     - `get_mut`
    //     - `iter_mut`
    //     - `reserve`
    //     - `try_with_capacity`
    //     - `values_mut`
    //     - `with_capacity`

    fn append(&self, key: PyHttpHeaderName, value: PyHttpHeaderValue) -> PyResult<bool> {
        self.write()
            .try_append(key.0, value.0)
            .map_err(|e| py_runtime_error!("header-append-error: {e}"))
    }

    #[getter]
    fn is_flat(&self) -> bool {
        let inner = self.read();
        inner.len() == inner.keys_len()
    }

    fn clear(&self) {
        self.write().clear();
    }

    #[must_use]
    fn contains_key(&self, key: &str) -> bool {
        self.read().contains_key(key)
    }

    fn get(&self, key: &str) -> Option<PyHttpHeaderValue> {
        self.read().get(key).map(PyHttpHeaderValue::from)
    }

    fn get_all<'py>(&'py self, py: Python<'py>, key: &str) -> PyResult<Bound<'py, PyList>> {
        let map = self.read();
        let hvals = map
            .get_all(key)
            .iter()
            .map(PyHttpHeaderValueRef::from)
            .collect::<Vec<_>>();
        PyList::new(py, hvals)
    }

    fn insert(
        &self,
        key: PyHttpHeaderName,
        value: PyHttpHeaderValue,
    ) -> PyResult<Option<PyHttpHeaderValue>> {
        self.write()
            .try_insert(key.0, value.0)
            .map_err(|e| py_runtime_error!("header-insert-error: {e}"))
            .map(|v| v.map(PyHttpHeaderValue::from))
    }

    #[must_use]
    fn is_empty(&self) -> bool {
        self.read().is_empty()
    }

    #[must_use]
    fn __bool__(&self) -> bool {
        !self.read().is_empty()
    }

    fn keys<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let map = self.read();
        PyList::new(py, map.keys().map(PyHttpHeaderNameRef).collect::<Vec<_>>())
    }

    #[must_use]
    fn keys_len(&self) -> usize {
        self.read().keys_len()
    }

    #[must_use]
    fn len(&self) -> usize {
        self.read().len()
    }

    fn remove(&self, key: PyHttpHeaderName) -> Option<PyHttpHeaderValue> {
        self.write().remove(key.0).map(PyHttpHeaderValue::from)
    }

    fn pop(&self, key: PyHttpHeaderName) -> Option<PyHttpHeaderValue> {
        self.remove(key)
    }

    fn values<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let map = self.read();
        let values = map.values().map(PyHttpHeaderValueRef).collect::<Vec<_>>();
        PyList::new(py, values)
    }

    #[pyo3(signature = (headers, *, append = false))]
    fn update(&self, headers: PyHeadersLike, append: bool) -> PyResult<()> {
        match headers {
            PyHeadersLike::Headers(other) => {
                let other_inner = other.read();
                let mut inner = self.write();
                if append {
                    update_headers_append(&other_inner, &mut inner)
                } else {
                    update_headers_insert(&other_inner, &mut inner)
                }
            }
            PyHeadersLike::Map(other) => {
                let hm = other.into();
                let mut inner = self.write();
                if append {
                    update_headers_append(&hm, &mut inner)
                } else {
                    update_headers_insert(&hm, &mut inner)
                }
            }
        }
        .map_err(|e| map_max_size_reached_to_pyerr(&e))
    }

    fn __or__(&self, other: PyHeadersLike) -> PyResult<Self> {
        let mut new_map = self.read().clone();
        match other {
            PyHeadersLike::Headers(other) => {
                let other_inner = other.read();
                update_headers_insert(&other_inner, &mut new_map)
            }
            PyHeadersLike::Map(other) => {
                let hm = other.into();
                update_headers_insert(&hm, &mut new_map)
            }
        }
        .map_err(|e| map_max_size_reached_to_pyerr(&e))?;
        Ok(Self::from(new_map))
    }

    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.py_dict(py)
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.to_py(py)
    }

    #[cfg(feature = "json")]
    #[pyo3(signature = (*, fmt = false))]
    fn stringify(&self, fmt: bool) -> PyResult<String> {
        use ryo3_core::macros::py_value_error;
        let inner = self.read();
        if fmt {
            serde_json::to_string_pretty(&crate::http_serde::HttpHeaderMapRef(&inner))
                .map_err(|e| py_value_error!("{e}"))
        } else {
            serde_json::to_string(&crate::http_serde::HttpHeaderMapRef(&inner))
                .map_err(|e| py_value_error!("{e}"))
        }
    }

    #[cfg(not(feature = "json"))]
    #[expect(clippy::unused_self)]
    #[expect(unused_variables)]
    #[pyo3(signature = (*args, **kwargs))]
    fn stringify(
        &self,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<String> {
        Err(::ryo3_core::FeatureNotEnabledError::new_err(
            "ryo3-http: `json` feature not enabled",
        ))
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn from_json(data: &str) -> PyResult<Self> {
        use ryo3_core::macros::py_value_error;

        serde_json::from_str::<crate::PyHttpHeaderMap>(data)
            .map(|e| Self::from(e.0))
            .map_err(|e| py_value_error!("{e}"))
    }

    #[cfg(not(feature = "json"))]
    #[staticmethod]
    fn from_json(_json: &str) -> PyResult<Self> {
        Err(::ryo3_core::FeatureNotEnabledError::new_err(
            "ryo3-http: `json` feature not enabled",
        ))
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================

    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::macros::py_value_error;
        Self::from_any(value).map_err(|e| py_value_error!("Headers validation error: {e}"))
    }

    #[cfg(feature = "pydantic")]
    fn _pydantic_serialize<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.py_dict(py)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }
}

impl std::hash::Hash for PyHeaders {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner = self.read();
        PyHttpHeaderMapRef(&inner).hash(state);
    }
}

impl std::hash::Hash for PyHttpHeaderMapRef<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // sorted keys
        if self.0.is_empty() {
            return;
        }
        let mut keys: Vec<_> = self.0.keys().collect();
        keys.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
        for key in keys {
            key.hash(state);
            let values: Vec<_> = self.0.get_all(key).iter().collect();
            for value in values {
                value.as_bytes().hash(state);
            }
        }
    }
}

#[cfg(feature = "pydantic")]
impl ryo3_pydantic::GetPydanticCoreSchemaCls for PyHeaders {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::interns;

        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let str_schema = core_schema.call_method(interns::str_schema(py), (), None)?;
        let list_schema =
            core_schema.call_method(interns::list_schema(py), (&str_schema,), None)?;
        let value_schema = core_schema.call_method(
            interns::union_schema(py),
            (vec![&str_schema, &list_schema],),
            None,
        )?;
        // should be dict[str, str | list[str]] but fuck is this ugly
        let dict_schema = core_schema.call_method(
            interns::dict_schema(py),
            (&str_schema, &value_schema),
            None,
        )?;

        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let serializer_fn = cls.getattr(interns::_pydantic_serialize(py))?;
        let serializer_kwargs = PyDict::new(py);
        serializer_kwargs.set_item(interns::return_schema(py), &dict_schema)?;
        let serializer_schema = core_schema.call_method(
            interns::plain_serializer_function_ser_schema(py),
            (&serializer_fn,),
            Some(&serializer_kwargs),
        )?;

        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item("json_schema_input_schema", &dict_schema)?;
        plain_validator_kwargs.set_item(interns::serialization(py), &serializer_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

fn map_max_size_reached_to_pyerr(e: &http::header::MaxSizeReached) -> PyErr {
    py_value_error!("header-size-limit-reached: {e}")
}

fn update_headers_append(
    src: &HeaderMap,
    dst: &mut HeaderMap,
) -> Result<(), http::header::MaxSizeReached> {
    for (k, v) in src {
        dst.try_append(k, v.into())?;
    }
    Ok(())
}

fn update_headers_insert(
    src: &HeaderMap,
    dst: &mut HeaderMap,
) -> Result<(), http::header::MaxSizeReached> {
    for (k, v) in src {
        dst.try_insert(k, v.into())?;
    }
    Ok(())
}
