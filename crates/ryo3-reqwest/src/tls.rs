use std::sync::OnceLock;

use pyo3::prelude::*;
use reqwest::tls::CertificateRevocationList;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_macro_rules::py_value_error;
static SYSTEM_CERTS: OnceLock<Vec<reqwest::Certificate>> = OnceLock::new();

/// Load the system root certificates as `reqwest::Certificate`s.
///
/// This is makes client generation much faster.
pub(crate) fn py_load_native_certs() -> &'static [reqwest::Certificate] {
    SYSTEM_CERTS.get_or_init(|| {
        let mut roots = Vec::new();
        let result = rustls_native_certs::load_native_certs();

        if !result.errors.is_empty() {
            for err in result.errors {
                tracing::warn!("failed to load a native cert: {err}");
            }
        }
        for cert in result.certs {
            if let Ok(req_cert) = reqwest::Certificate::from_der(cert.as_ref()) {
                roots.push(req_cert);
            } else {
                tracing::debug!("failed to parse a native cert");
            }
        }
        roots
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CertificateKind {
    Der,
    Pem,
}

#[derive(Debug, Clone)]
#[pyclass(name = "Certificate", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyCertificate {
    pub(crate) kind: CertificateKind,
    pub(crate) bin: bytes::Bytes,
    pub(crate) cert: ::reqwest::Certificate,
}

impl RyCertificate {
    pub fn inner(&self) -> &::reqwest::Certificate {
        &self.cert
    }

    fn from_der(der: &[u8]) -> PyResult<Self> {
        ::reqwest::Certificate::from_der(der)
            .map(|cert| Self {
                kind: CertificateKind::Der,
                bin: bytes::Bytes::copy_from_slice(der),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create certificate from DER: {}", e))
    }

    fn from_pem(pem: &[u8]) -> PyResult<Self> {
        ::reqwest::Certificate::from_pem(pem)
            .map(|cert| Self {
                kind: CertificateKind::Pem,
                bin: bytes::Bytes::copy_from_slice(pem),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create certificate from PEM: {}", e))
    }

    fn from_pem_bundle(pem_bundle: &[u8]) -> PyResult<Vec<Self>> {
        ::reqwest::Certificate::from_pem_bundle(pem_bundle)
            .map(|certs| {
                certs
                    .into_iter()
                    .map(|cert| Self {
                        kind: CertificateKind::Pem,
                        bin: bytes::Bytes::copy_from_slice(pem_bundle),
                        cert,
                    })
                    .collect()
            })
            .map_err(|e| py_value_error!("Failed to create certificate from PEM bundle: {}", e))
    }
}

impl std::hash::Hash for RyCertificate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.bin.hash(state);
    }
}

#[pymethods]
impl RyCertificate {
    #[new]
    fn py_new() -> PyResult<Self> {
        Err(py_value_error!(
            "Cannot create Certificate directly; use from_der or from_pem"
        ))
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        pyo3::types::PyBytes::new(py, self.bin.as_ref())
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(name = "from_der")]
    #[staticmethod]
    fn py_from_der(der: ReadableBuffer) -> PyResult<Self> {
        Self::from_der(der.as_ref())
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(name = "from_pem")]
    #[staticmethod]
    fn py_from_pem(pem: ReadableBuffer) -> PyResult<Self> {
        Self::from_pem(pem.as_ref())
    }

    #[pyo3(name = "from_pem_bundle")]
    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn py_from_pem_bundle(pem_bundle: ReadableBuffer) -> PyResult<Vec<Self>> {
        Self::from_pem_bundle(pem_bundle.as_ref())
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }
}

impl std::fmt::Display for RyCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            CertificateKind::Der => {
                write!(
                    f,
                    "Certificate<DER; {:p}>",
                    std::ptr::from_ref::<Self>(self)
                )
            }
            CertificateKind::Pem => {
                write!(
                    f,
                    "Certificate<PEM; {:p}>",
                    std::ptr::from_ref::<Self>(self)
                )
            }
        }
    }
}

impl PartialEq for RyCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.bin == other.bin
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for RyCertificate {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(cert) = obj.cast_exact::<Self>() {
            Ok(cert.get().clone())
        } else if let Ok(b) = obj.extract::<RyBytes>() {
            if let Ok(cert) = Self::from_pem(b.as_ref()) {
                Ok(cert)
            } else if let Ok(cert) = Self::from_der(b.as_ref()) {
                Ok(cert)
            } else {
                Err(py_value_error!(
                    "Failed to parse bytes as PEM or DER certificate"
                ))
            }
        } else {
            Err(py_value_error!("Expected Certificate object"))
        }
    }
}

// ==== CERTIFICATE REVOCATION LIST ====

#[derive(Debug)]
#[pyclass(name = "CertificateRevocationList", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyCertificateRevocationList {
    pub(crate) bin: bytes::Bytes,
    pub(crate) crl: ::reqwest::tls::CertificateRevocationList,
}

impl RyCertificateRevocationList {
    fn from_pem(pem: &[u8]) -> PyResult<Self> {
        ::reqwest::tls::CertificateRevocationList::from_pem(pem)
            .map(|crl| Self {
                bin: bytes::Bytes::copy_from_slice(pem),
                crl,
            })
            .map_err(|e| {
                py_value_error!("Failed to create CertificateRevocationList from PEM: {}", e)
            })
    }

    fn from_pem_bundle(pem: &[u8]) -> PyResult<Vec<Self>> {
        ::reqwest::tls::CertificateRevocationList::from_pem_bundle(pem)
            .map(|crls| {
                crls.into_iter()
                    .map(|crl| Self {
                        bin: bytes::Bytes::copy_from_slice(pem),
                        crl,
                    })
                    .collect()
            })
            .map_err(|e| {
                py_value_error!(
                    "Failed to create CertificateRevocationList from PEM bundle: {}",
                    e
                )
            })
    }
}

impl std::hash::Hash for RyCertificateRevocationList {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bin.hash(state);
    }
}

impl Clone for RyCertificateRevocationList {
    fn clone(&self) -> Self {
        Self {
            bin: self.bin.clone(),
            crl: self.into(),
        }
    }
}

#[pymethods]
impl RyCertificateRevocationList {
    #[new]
    #[expect(clippy::needless_pass_by_value)]
    fn py_new(pem: ReadableBuffer) -> PyResult<Self> {
        Self::from_pem(pem.as_ref())
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        pyo3::types::PyBytes::new(py, self.bin.as_ref())
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(name = "from_pem")]
    #[staticmethod]
    fn py_from_pem(pem: ReadableBuffer) -> PyResult<Self> {
        ::reqwest::tls::CertificateRevocationList::from_pem(pem.as_ref())
            .map(|crl| Self {
                bin: pem.copy_to_bytes(),
                crl,
            })
            .map_err(|e| {
                py_value_error!("Failed to create CertificateRevocationList from PEM: {}", e)
            })
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(name = "from_pem_bundle")]
    #[staticmethod]
    fn py_from_pem_bundle(pem_bundle: ReadableBuffer) -> PyResult<Vec<Self>> {
        Self::from_pem_bundle(pem_bundle.as_ref())
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }
}

impl std::fmt::Display for RyCertificateRevocationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CertificateRevocationList<{:p}>",
            std::ptr::from_ref::<Self>(self)
        )
    }
}

impl PartialEq for RyCertificateRevocationList {
    fn eq(&self, other: &Self) -> bool {
        self.bin == other.bin
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for RyCertificateRevocationList {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(cert) = obj.cast_exact::<Self>() {
            Ok(cert.get().clone())
        } else if let Ok(b) = obj.extract::<ReadableBuffer>() {
            Self::py_from_pem(b)
        } else {
            Err(py_value_error!("Expected CertificateRevocationList object"))
        }
    }
}

impl From<&RyCertificateRevocationList> for CertificateRevocationList {
    fn from(ry_crl: &RyCertificateRevocationList) -> Self {
        Self::from_pem(ry_crl.bin.as_ref())
            .expect("wenodis: it already was constructed from valid pem")
    }
}

impl From<RyCertificateRevocationList> for CertificateRevocationList {
    fn from(ry_crl: RyCertificateRevocationList) -> Self {
        ry_crl.crl
    }
}

// ==== IDENTITY ====

#[derive(Debug, Clone)]
#[pyclass(name = "Identity", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyIdentity {
    pub(crate) bin: bytes::Bytes,
    pub(crate) cert: ::reqwest::Identity,
}

impl RyIdentity {
    pub fn inner(&self) -> &::reqwest::Identity {
        &self.cert
    }

    fn from_pem(pem: &[u8]) -> PyResult<Self> {
        ::reqwest::Identity::from_pem(pem)
            .map(|cert| Self {
                bin: bytes::Bytes::copy_from_slice(pem),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create Identity from PEM: {}", e))
    }
}

impl std::hash::Hash for RyIdentity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bin.hash(state);
    }
}

#[pymethods]
impl RyIdentity {
    #[new]
    #[expect(clippy::needless_pass_by_value)]
    fn py_new(pem: ReadableBuffer) -> PyResult<Self> {
        Self::from_pem(pem.as_ref())
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        pyo3::types::PyBytes::new(py, self.bin.as_ref())
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.bin == other.bin
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.bin != other.bin
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(name = "from_pem")]
    #[staticmethod]
    fn py_from_pem(pem: ReadableBuffer) -> PyResult<Self> {
        Self::from_pem(pem.as_ref())
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }
}

impl std::fmt::Display for RyIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity<PEM; {:p}>", std::ptr::from_ref::<Self>(self))
    }
}

impl PartialEq for RyIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.bin == other.bin
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for RyIdentity {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(cert) = obj.cast_exact::<Self>() {
            Ok(cert.get().clone())
        } else if let Ok(b) = obj.extract::<ReadableBuffer>() {
            Self::py_from_pem(b)
        } else {
            Err(py_value_error!("Expected Identity object"))
        }
    }
}

macro_rules! impl_as_ref_bytes {
    ($ty:ty) => {
        impl AsRef<[u8]> for $ty {
            fn as_ref(&self) -> &[u8] {
                self.bin.as_ref()
            }
        }
    };
}

impl_as_ref_bytes!(RyCertificate);
impl_as_ref_bytes!(RyCertificateRevocationList);
impl_as_ref_bytes!(RyIdentity);
