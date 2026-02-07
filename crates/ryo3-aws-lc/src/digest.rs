#![doc = include_str!("../README.md")]
use aws_lc_rs::digest::{Context, Digest, SHA256, SHA512};
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};
use ryo3_bytes::PyBytes as RyBytes;
use std::sync::Mutex;

struct PyAwsLcDigest<const SIZE: usize>(Digest);

impl<'py, const SIZE: usize> pyo3::IntoPyObject<'py> for PyAwsLcDigest<SIZE> {
    type Target = pyo3::types::PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        pyo3::types::PyBytes::new_with(py, SIZE, |b| {
            b.copy_from_slice(self.0.as_ref());
            Ok(())
        })
    }
}

#[pyclass(name = "sha256", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySha256(Mutex<Context>);

impl From<Context> for PySha256 {
    fn from(hasher: Context) -> Self {
        Self(Mutex::new(hasher))
    }
}

impl PySha256 {
    fn lock(&self) -> PyResult<std::sync::MutexGuard<'_, Context>> {
        self.0.lock().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to lock hasher: {e}"))
        })
    }

    fn finish(&self) -> PyResult<Digest> {
        let ctx = self.lock()?;
        Ok(ctx.clone().finish())
    }
}

#[pymethods]
impl PySha256 {
    #[new]
    #[pyo3(
        signature = (data = None, *),
        text_signature = "(data=None, /)",
    )]
    fn py_new(py: Python<'_>, data: Option<RyBytes>) -> Self {
        py.detach(|| match data {
            Some(b) => {
                let mut hasher = Context::new(&SHA256);
                hasher.update(b.as_ref());
                Self::from(hasher)
            }
            None => Self::from(Context::new(&SHA256)),
        })
    }

    #[classattr]
    fn digest_size() -> usize {
        SHA256.output_len()
    }

    #[classattr]
    fn block_size() -> usize {
        SHA256.block_len()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        pyo3::intern!(py, "sha256")
    }

    fn digest(&self) -> PyResult<PyAwsLcDigest<32>> {
        let digest = self.finish()?;
        Ok(PyAwsLcDigest(digest))
    }

    // fn hexdigest<'a>(&'a self) -> PyResult<PyHexDigest<String>> {
    //     let digest = self.finish()?;
    //     let b: &[u8; 32] = digest.as_ref().try_into().expect("sha256 digest size");
    //     Ok(PyHexDigest::from(b))
    // }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, data: RyBytes) -> PyResult<()> {
        let mut ctx = self.lock()?;
        ctx.update(data.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        let ctx = self.lock()?;
        Ok(Self::from(ctx.clone()))
    }

    // #[expect(clippy::needless_pass_by_value)]
    // #[pyo3(
    //     signature = (data, *, key = FnvKey::default()),
    //     text_signature = "(data, *, key=0xcbf29ce484222325)",
    // )]
    // #[staticmethod]
    // fn oneshot(data: ryo3_bytes::PyBytes, key: FnvKey) -> u64 {
    //     fnv1a_oneshot(data.as_ref(), key.into())
    // }
}

impl std::fmt::Display for PySha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        eprintln!("Display called on PySha256 ~ TODO make good");
        write!(f, "sha256(...)")
    }
}

#[pyclass(name = "sha512", frozen, immutable_type, skip_from_py_object)]
// #[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySha512(Mutex<Context>);

impl From<Context> for PySha512 {
    fn from(hasher: Context) -> Self {
        Self(Mutex::new(hasher))
    }
}

impl PySha512 {
    fn lock(&self) -> PyResult<std::sync::MutexGuard<'_, Context>> {
        self.0.lock().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to lock hasher: {e}"))
        })
    }

    fn finish(&self) -> PyResult<Digest> {
        let ctx = self.lock()?;
        Ok(ctx.clone().finish())
    }
}

// #[inline]
// fn fnv1a_oneshot(bytes: &[u8], key: u64) -> u64 {
//     bytes.iter().fold(key, |hash, &byte| {
//         let hash = hash ^ u64::from(byte);
//         hash.wrapping_mul(0x0100_0000_01b3)
//     })
// }

#[pymethods]
impl PySha512 {
    #[new]
    #[pyo3(
        signature = (data = None, *),
        text_signature = "(data=None, /)",
    )]
    fn py_new(py: Python<'_>, data: Option<RyBytes>) -> Self {
        py.detach(|| match data {
            Some(b) => {
                let mut hasher = Context::new(&SHA512);
                hasher.update(b.as_ref());
                Self::from(hasher)
            }
            None => Self::from(Context::new(&SHA512)),
        })
    }

    #[classattr]
    fn digest_size() -> usize {
        SHA512.output_len()
    }

    #[classattr]
    fn block_size() -> usize {
        SHA512.block_len()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        pyo3::intern!(py, "sha512")
    }

    fn digest<'s>(&'s self) -> PyResult<PyAwsLcDigest<64>> {
        let digest = self.finish()?;
        Ok(PyAwsLcDigest(digest))
    }

    // fn hexdigest<'a>(&'a self) -> PyResult<PyHexDigest<String>> {
    //     let digest = self.finish()?;
    //     let b: &[u8; 64] = digest.as_ref().try_into().expect("sha512 digest size");
    //     Ok(PyHexDigest::from(b))
    // }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, data: RyBytes) -> PyResult<()> {
        let mut ctx = self.lock()?;
        ctx.update(data.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        let ctx = self.lock()?;
        Ok(Self::from(ctx.clone()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(
        signature = (data, /),
        text_signature = "(data, /)",
    )]
    #[staticmethod]
    fn oneshot(data: RyBytes) -> PyResult<PyAwsLcDigest<64>> {
        let mut hasher = Context::new(&SHA512);
        hasher.update(data.as_ref());
        let digest = hasher.finish();
        Ok(PyAwsLcDigest(digest))
    }
}

impl std::fmt::Display for PySha512 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        eprintln!("Display called on PySha512 ~ TODO make good");
        write!(f, "sha512(...)")
    }
}

// ============================================================================
// REGISTER CLASSES
// ============================================================================
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySha256>()?;
    m.add_class::<PySha512>()?;
    Ok(())
}
