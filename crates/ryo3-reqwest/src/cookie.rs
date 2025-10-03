use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_macro_rules::py_value_error;
use ryo3_macro_rules::{py_value_err, pytodo};
use ryo3_std::time::PyDuration;

#[pyclass(name = "Cookie", frozen)]
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
        max_age: Option<PyDuration>,
        partitioned: Option<bool>,
        path: Option<String>,
        permanent: bool,
        removal: bool,
        same_site: Option<PySameSite>,
        secure: Option<bool>,
    ) -> PyResult<Self> {
        let mut cb = cookie::Cookie::build((name, value));

        if let Some(domain) = domain {
            cb = cb.domain(domain);
        }
        if let Some(_expires) = expires {
            pytodo!("handle expires");
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
            cb = cb.same_site(same_site.0);
        }

        if let Some(secure) = secure {
            cb = cb.secure(secure);
        }

        Ok(Self(cb.into()))
    }

    // ------------------------------------------------------------------------
    // STATIC/"CLASS" METHODS
    // ------------------------------------------------------------------------

    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        match cookie::Cookie::parse(s.to_string()) {
            Ok(c) => Ok(Self(c.into_owned())),
            Err(e) => py_value_err!("failed to parse cookie: {e}"),
        }
    }

    #[staticmethod]
    fn parse_encoded(s: &str) -> PyResult<Self> {
        match cookie::Cookie::parse_encoded(s.to_string()) {
            Ok(c) => Ok(Self(c.into_owned())),
            Err(e) => py_value_err!("failed to parse cookie: {e}"),
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
    #[expect(clippy::unused_self)]
    fn expires(&self) -> PyResult<()> {
        pytodo!("handle expires getter");
    }

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
    fn same_site(&self) -> Option<PySameSite> {
        self.0.same_site().map(PySameSite)
    }

    /// Return the secure of the cookie | None
    #[getter]
    fn secure(&self) -> Option<bool> {
        self.0.secure()
    }
}

impl std::fmt::Debug for PyCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cookie(\"{}\", \"{}\"", self.0.name(), self.0.value())?;
        // this order...
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

// ------------------------------------------------------------------------
// same site
// ------------------------------------------------------------------------
struct PySameSite(cookie::SameSite);

impl<'py> IntoPyObject<'py> for &PySameSite {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            cookie::SameSite::Lax => intern!(py, "Lax"),
            cookie::SameSite::Strict => intern!(py, "Strict"),
            cookie::SameSite::None => intern!(py, "None"),
        };
        let b = s.as_borrowed();
        Ok(b)
    }
}

impl<'py> IntoPyObject<'py> for PySameSite {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl FromPyObject<'_> for PySameSite {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "Lax" | "lax" => Ok(Self(cookie::SameSite::Lax)),
                "Strict" | "strict" => Ok(Self(cookie::SameSite::Strict)),
                "None" | "none" => Ok(Self(cookie::SameSite::None)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid SameSite value: {s} (options: 'Lax', 'Strict', 'None')"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Invalid SameSite value: {ob} (options: 'Lax', 'Strict', 'None')"
            )))
        }
    }
}
