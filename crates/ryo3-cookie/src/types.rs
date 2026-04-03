//! py-types/extractors/converters

use pyo3::{IntoPyObjectExt, intern, prelude::*, types::PyString};
use ryo3_core::{py_type_err, py_value_err};
use ryo3_jiff::RyTimestamp;

// ------------------------------------------------------------------------
// py-same-site
// ------------------------------------------------------------------------

/// Python newtype for `cookie::SameSite` w/ `IntoPyObject` and `FromPyObject`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyCookieSameSite(cookie::SameSite);

impl PyCookieSameSite {
    pub const LAX: Self = Self(cookie::SameSite::Lax);
    pub const STRICT: Self = Self(cookie::SameSite::Strict);
    pub const NONE: Self = Self(cookie::SameSite::None);
}

impl<'py> IntoPyObject<'py> for &PyCookieSameSite {
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

impl<'py> IntoPyObject<'py> for PyCookieSameSite {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyCookieSameSite {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "Lax" | "lax" => Ok(Self::LAX),
                "Strict" | "strict" => Ok(Self::STRICT),
                "None" | "none" => Ok(Self::NONE),
                _ => {
                    py_value_err!("Invalid SameSite value: {s} (options: 'Lax', 'Strict', 'None')")
                }
            }
        } else {
            py_type_err!("Invalid SameSite value (options: 'Lax', 'Strict', 'None')")
        }
    }
}

impl From<cookie::SameSite> for PyCookieSameSite {
    #[inline]
    fn from(s: cookie::SameSite) -> Self {
        Self(s)
    }
}

impl From<PyCookieSameSite> for cookie::SameSite {
    #[inline]
    fn from(s: PyCookieSameSite) -> Self {
        s.0
    }
}

// ------------------------------------------------------------------------
// py-expiration
// ------------------------------------------------------------------------
/// Python newtype for `cookie::Expiration` w/ `IntoPyObject` and `FromPyObject`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyCookieExpiration(cookie::Expiration);

impl PyCookieExpiration {
    // pub(crate) fn to_pydatetime<'py>(
    //     &self,
    //     py: Python<'py>,
    //     odt: cookie::Expiration,
    // ) -> PyResult<Bound<'py, pyo3::types::PyDateTime>> {
    //     match odt {
    //         cookie::Expiration::Session => panic!(
    //             "PyCookieExpiration::offsetdatetime2pydatetime should never be called with Expiration::Session"
    //         ),
    //         cookie::Expiration::DateTime(dt) => {
    //             use pyo3::types::{PyDateTime, PyDelta, PyTzInfo};
    //             let off = dt.offset();
    //             // pyo3::types::PyDateTime::new(
    //             // pyo3::Python::with_gil(|py| py),
    //             let seconds_offset = off.whole_seconds();
    //             let td = PyDelta::new(py, 0, seconds_offset, 0, true)?;
    //             let tzinfo = PyTzInfo::fixed_offset(py, td)?;
    //             PyDateTime::new(
    //                 py,
    //                 dt.year(),
    //                 dt.month() as u8,
    //                 dt.day(),
    //                 dt.hour(),
    //                 dt.minute(),
    //                 dt.second(),
    //                 dt.nanosecond() / 1000, // convert nanoseconds to microseconds
    //                 Some(tzinfo.cast()?),
    //             )
    //         }
    //     }
    // }
}

impl From<cookie::Expiration> for PyCookieExpiration {
    #[inline]
    fn from(e: cookie::Expiration) -> Self {
        Self(e)
    }
}

impl From<PyCookieExpiration> for cookie::Expiration {
    #[inline]
    fn from(e: PyCookieExpiration) -> Self {
        e.0
    }
}

impl<'py> IntoPyObject<'py> for PyCookieExpiration {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            cookie::Expiration::Session => Ok(py.None().into_pyobject(py)?),
            cookie::Expiration::DateTime(dt) => {
                let seconds = dt.unix_timestamp();
                let nanos = i32::try_from(dt.nanosecond()).expect(
                    "wenodis: jiff::Timestamp nanoseconds always -999_999_999..=999_999_999 & time::OffsetDateTime nanoseconds always 0..=999_999_999 => infallible"
                );
                RyTimestamp::new(seconds, nanos)
                    .expect("wenodis: this is infallible (i think)")
                    .into_bound_py_any(py)
            }
        }
    }
}
