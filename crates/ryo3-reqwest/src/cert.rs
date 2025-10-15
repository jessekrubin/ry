use pyo3::prelude::*;
use ryo3_macro_rules::py_value_error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CertificateKind {
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
    #[new]
    fn py_new() -> PyResult<Self> {
        Err(py_value_error!(
            "Cannot create Certificate directly; use from_der or from_pem"
        ))
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_der(der: ryo3_bytes::PyBytes) -> PyResult<Self> {
        ::reqwest::Certificate::from_der(der.as_ref())
            .map(|cert| Self {
                kind: CertificateKind::Der,
                bin: bytes::Bytes::copy_from_slice(der.as_ref()),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create certificate from DER: {}", e))
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_pem(pem: ryo3_bytes::PyBytes) -> PyResult<Self> {
        ::reqwest::Certificate::from_pem(pem.as_ref())
            .map(|cert| Self {
                kind: CertificateKind::Pem,
                bin: bytes::Bytes::copy_from_slice(pem.as_ref()),
                cert,
            })
            .map_err(|e| py_value_error!("Failed to create certificate from PEM: {}", e))
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_pem_bundle(path: ryo3_bytes::PyBytes) -> PyResult<Vec<Self>> {
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

    fn __repr__(&self) -> String {
        match self.kind {
            CertificateKind::Der => {
                format!("Certificate<DER; {:p}>", std::ptr::from_ref::<Self>(self))
            }
            CertificateKind::Pem => {
                format!("Certificate<PEM; {:p}>", std::ptr::from_ref::<Self>(self))
            }
        }
    }
}

impl PartialEq for PyCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.bin == other.bin
    }
}
