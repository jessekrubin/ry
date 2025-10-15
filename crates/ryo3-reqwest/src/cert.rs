use pyo3::prelude::*;
use reqwest::multipart::Part;
use ryo3_macro_rules::py_value_error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CertificateKind {
    Der,
    Pem,
}

#[pyclass(name = "Certificate", frozen)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyCertificate {
    pub(crate) kind: CertificateKind,
    pub(crate) bin: bytes::Bytes,
    pub(crate) cert: ::reqwest::Certificate,
}

#[pymethods]
impl PyCertificate {
    #[staticmethod]
    pub fn from_der(der: ryo3_bytes::PyBytes) -> PyResult<Self> {
        ::reqwest::Certificate::from_der(der.as_ref())
            .map(|cert| Self {
                kind: CertificateKind::Der,
                bin: bytes::Bytes::copy_from_slice(der.as_ref()),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create certificate from DER: {}", e))
    }

    #[staticmethod]
    pub fn from_pem(pem: ryo3_bytes::PyBytes) -> PyResult<Self> {
        ::reqwest::Certificate::from_pem(pem.as_ref())
            .map(|cert| Self {
                kind: CertificateKind::Pem,
                bin: bytes::Bytes::copy_from_slice(pem.as_ref()),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create certificate from PEM: {}", e))
    }

    #[staticmethod]
    pub fn from_pem_bundle(path: ryo3_bytes::PyBytes) -> PyResult<Vec<Self>> {
        ::reqwest::Certificate::from_pem_bundle(path.as_ref())
            .map(|certs| {
                certs
                    .into_iter()
                    .map(|cert| Self {
                        kind: CertificateKind::Pem,

                        bin: bytes::Bytes::copy_from_slice(path.as_ref()),
                        cert,
                    })
                    .collect()
            })
            .map_err(|e| py_value_error!("Failed to create certificate from PEM file: {}", e))
    }

    fn __repr__(&self) -> PyResult<String> {
        match self.kind {
            CertificateKind::Der => Ok(format!("Certificate<DER; {:p}>", self as *const Self)),
            CertificateKind::Pem => Ok(format!("Certificate<PEM; {:p}>", self as *const Self)),
        }
    }
}

impl PartialEq for PyCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.bin == other.bin
    }
}
