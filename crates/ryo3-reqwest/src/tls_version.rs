use pyo3::prelude::*;
use pyo3::types::PyString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TlsVersion {
    Tlsv1_0,
    Tlsv1_1,
    Tlsv1_2,
    Tlsv1_3,
}

impl From<&TlsVersion> for reqwest::tls::Version {
    fn from(value: &TlsVersion) -> Self {
        match value {
            TlsVersion::Tlsv1_0 => Self::TLS_1_0,
            TlsVersion::Tlsv1_1 => Self::TLS_1_1,
            TlsVersion::Tlsv1_2 => Self::TLS_1_2,
            TlsVersion::Tlsv1_3 => Self::TLS_1_3,
        }
    }
}

impl<'py> IntoPyObject<'py> for TlsVersion {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self {
            Self::Tlsv1_0 => pyo3::intern!(py, "1.0"),
            Self::Tlsv1_1 => pyo3::intern!(py, "1.1"),
            Self::Tlsv1_2 => pyo3::intern!(py, "1.2"),
            Self::Tlsv1_3 => pyo3::intern!(py, "1.3"),
        };
        Ok(s.as_borrowed())
    }
}

const TLS_VERSION_STRINGS: &str = "'1.0', '1.1', '1.2', '1.3'";

impl<'py> FromPyObject<'_, 'py> for TlsVersion {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "1.0" => Ok(Self::Tlsv1_0),
                "1.1" => Ok(Self::Tlsv1_1),
                "1.2" => Ok(Self::Tlsv1_2),
                "1.3" => Ok(Self::Tlsv1_3),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid TLS version: {s} (options: {TLS_VERSION_STRINGS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "TLS version must be a string (options: {TLS_VERSION_STRINGS})"
            )))
        }
    }
}
//
