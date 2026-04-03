use pyo3::{BoundObject, prelude::*};
use ryo3_core::{py_value_err, py_value_error, pytodo};
use ryo3_std::time::PyDuration;

use crate::{PyCookieSameSite, types::PyCookieExpiration};

#[pyclass(name = "Cookie", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyCookie(pub(crate) ::cookie::Cookie<'static>);

impl From<::cookie::Cookie<'static>> for PyCookie {
    fn from(value: ::cookie::Cookie<'static>) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyCookie {
    #[new]
    #[pyo3(signature = (
        name,
        value,
        *,
        domain = None,
        expires = None,
        http_only = None,
        max_age = None,
        partitioned = None,
        path = None,
        permanent = false,
        removal = false,
        same_site = None,
        secure = None,
    ))]
    #[expect(clippy::too_many_arguments)]
    fn py_new(
        name: String,
        value: String,
        domain: Option<String>,
        expires: Option<i64>,
        http_only: Option<bool>,
        max_age: Option<&PyDuration>,
        partitioned: Option<bool>,
        path: Option<String>,
        permanent: bool,
        removal: bool,
        same_site: Option<PyCookieSameSite>,
        secure: Option<bool>,
    ) -> PyResult<Self> {
        let mut cb = cookie::Cookie::build((name, value));

        if let Some(domain) = domain {
            cb = cb.domain(domain);
        }
        if let Some(_expires) = expires {
            pytodo!("handle expires input");
        }

        if let Some(http_only) = http_only {
            cb = cb.http_only(http_only);
        }

        if let Some(max_age) = max_age {
            let ma = max_age
                .0
                .try_into()
                .map_err(|e| py_value_error!("invalid max_age duration: {e}"))?;

            cb = cb.max_age(ma);
        }

        if let Some(partitioned) = partitioned {
            cb = cb.partitioned(partitioned);
        }

        if let Some(path) = path {
            cb = cb.path(path);
        }

        if permanent {
            cb = cb.permanent();
        }

        if removal {
            cb = cb.removal();
        }

        if let Some(same_site) = same_site {
            cb = cb.same_site(same_site.into());
        }

        if let Some(secure) = secure {
            cb = cb.secure(secure);
        }

        Ok(Self(cb.into()))
    }

    // ------------------------------------------------------------------------
    // STATIC "CLASS" METHODS (aka "constructors")
    // ------------------------------------------------------------------------
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        use ryo3_core::PyFromStr;
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyParse;
        Self::py_parse(s)
    }

    #[staticmethod]
    fn parse_encoded(s: &str) -> PyResult<Self> {
        match cookie::Cookie::parse_encoded(s) {
            Ok(c) => Ok(Self(c.into_owned())),
            Err(e) => py_value_err!("failed to parse cookie: {e}"),
        }
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(cookie) = value.cast_exact::<Self>() {
            Ok(cookie.as_borrowed().into_bound())
        } else {
            Self::parse(value).map(|cookie| cookie.into_pyobject(py))?
        }
    }

    // ------------------------------------------------------------------------
    // STR/REPR/FORMAT
    // ------------------------------------------------------------------------
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn encoded(&self) -> String {
        self.0.encoded().to_string()
    }

    fn stripped(&self) -> String {
        self.0.stripped().to_string()
    }

    fn encoded_stripped(&self) -> String {
        self.0.encoded().stripped().to_string()
    }

    fn stripped_encoded(&self) -> String {
        self.0.stripped().encoded().to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.0 != other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.to_string().hash(&mut hasher);
        hasher.finish()
    }

    // ------------------------------------------------------------------------
    // GETTERS
    // ------------------------------------------------------------------------

    /// Return the name of the cookie
    #[getter]
    fn name(&self) -> &str {
        self.0.name()
    }

    /// Return the value of the cookie
    #[getter]
    fn value(&self) -> &str {
        self.0.value()
    }

    /// Return the value with surrounding double-quotes trimmed.
    #[getter]
    fn value_trimmed(&self) -> &str {
        self.0.value_trimmed()
    }

    /// Return tuple of (name, value)
    #[getter]
    fn name_value(&self) -> (&str, &str) {
        self.0.name_value()
    }

    /// Return tuple of (name, value with surrounding double-quotes trimmed)
    #[getter]
    fn name_value_trimmed(&self) -> (&str, &str) {
        self.0.name_value_trimmed()
    }

    /// Return the domain of the cookie | None
    #[getter]
    fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Return the expires of the cookie | None
    #[getter]
    fn expires(&self) -> Option<PyCookieExpiration> {
        self.0.expires().map(PyCookieExpiration::from)
    }

    // TODO pydatetime conversion?
    // ```rust
    // #[getter]
    // fn expires_pydatetime(&self, py: Python<'_>) -> PyResult<Option<PyCookieExpiration>> {
    //     let expires = self.0.expires();
    //     if let Some(expires) = expires {
    //         let py_dt = PyCookieExpiration::offsetdatetime2pydatetime(self, py, expires)?;
    //         Ok(Some(PyCookieExpiration::from(expires)))
    //     } else {
    //         Ok(None)
    //     }
    // }
    // ```

    /// Return the `http_only` of the cookie | None
    #[getter]
    fn http_only(&self) -> Option<bool> {
        self.0.http_only()
    }

    /// Return the `max_age` of the cookie as Duration | None
    #[getter]
    fn max_age(&self) -> Option<PyDuration> {
        self.0.max_age().map(|d| d.unsigned_abs().into())
    }

    /// Return the partitioned of the cookie | None
    #[getter]
    fn partitioned(&self) -> Option<bool> {
        self.0.partitioned()
    }

    /// Return the path of the cookie | None
    #[getter]
    fn path(&self) -> Option<&str> {
        self.0.path()
    }

    /// Return the `same_site` of the cookie 'Lax' | 'Strict' | 'None' | None
    #[getter]
    fn same_site(&self) -> Option<PyCookieSameSite> {
        self.0.same_site().map(PyCookieSameSite::from)
    }

    /// Return the secure of the cookie | None
    #[getter]
    fn secure(&self) -> Option<bool> {
        self.0.secure()
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================
    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, Self>> {
        Self::from_any(value).map_err(|e| py_value_error!("Cookie validation error: {e}"))
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

impl std::fmt::Debug for PyCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cookie(\"{}\", \"{}\"", self.0.name(), self.0.value())?;
        // this is the kw ordering...
        // expires = None,
        // http_only = None,
        // max_age = None,
        // partitioned = None,
        // path = None,
        // permanent = false,
        // removal = false,
        // same_site = None,
        // secure = None,
        if let Some(domain) = self.0.domain() {
            write!(f, ", domain=\"{domain}\"")?;
        }

        if let Some(expires) = self.0.expires() {
            write!(f, ", expires={expires:?}")?;
        }

        if let Some(http_only) = self.0.http_only() {
            if http_only {
                write!(f, ", http_only=True")?;
            } else {
                write!(f, ", http_only=False")?;
            }
        }

        if let Some(max_age) = self.max_age() {
            write!(f, ", max_age={max_age:?}")?;
        }

        if let Some(partitioned) = self.0.partitioned() {
            if partitioned {
                write!(f, ", partitioned=True")?;
            } else {
                write!(f, ", partitioned=False")?;
            }
        }

        if let Some(path) = self.0.path() {
            write!(f, ", path=\"{path}\"")?;
        }

        if let Some(same_site) = self.0.same_site() {
            let s = match same_site {
                cookie::SameSite::Lax => "Lax",
                cookie::SameSite::Strict => "Strict",
                cookie::SameSite::None => "None",
            };
            write!(f, ", same_site=\"{s}\"")?;
        }
        if let Some(secure) = self.0.secure() {
            if secure {
                write!(f, ", secure=True")?;
            } else {
                write!(f, ", secure=False")?;
            }
        }
        write!(f, ")")
    }
}

impl std::str::FromStr for PyCookie {
    type Err = cookie::ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = cookie::Cookie::parse(s)?;
        Ok(Self(c.into_owned()))
    }
}

#[cfg(feature = "pydantic")]
impl ryo3_pydantic::GetPydanticCoreSchemaCls for PyCookie {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3::types::{PyDict, PyTuple};
        use ryo3_pydantic::interns;

        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let schema = core_schema.call_method(interns::str_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
            args,
            Some(&serialization_kwargs),
        )
    }
}
