//! python fnv1a implementation
//!
//! Formerly used the `fnv` crate but it is such a simple crate re-implementing
//! the core logic outweighs pulling in another dep.
use pyo3::types::{PyString, PyTuple};
use pyo3::{IntoPyObjectExt, intern, prelude::*};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::{
    PyAsciiString, RyMutex, py_type_err,
    types::{PyDigest, PyHexDigest},
};
use std::hash::Hasher;

const FNV1A_64_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV1A_64_PRIME: u64 = 0x0100_0000_01b3;

// ============================================================================
// adapted from the `fnv` crate
// ============================================================================
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fnv1aHasher(u64);

impl Default for Fnv1aHasher {
    #[inline]
    fn default() -> Self {
        Self(FNV1A_64_OFFSET)
    }
}

impl Fnv1aHasher {
    /// Create an FNV hasher starting with a state corresponding
    /// to the hash `key`.
    #[inline]
    pub fn with_key(key: u64) -> Self {
        Self(key)
    }
}

impl Hasher for Fnv1aHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let hash = bytes.iter().fold(self.0, |hash, &byte| {
            let hash = hash ^ u64::from(byte);
            hash.wrapping_mul(FNV1A_64_PRIME)
        });
        *self = Self(hash);
    }
}

// ============================================================================
// ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~
// ============================================================================

#[pyclass(name = "fnv1a", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFnv1a(RyMutex<Fnv1aHasher>);

impl PyFnv1a {
    fn lock(&self) -> PyResult<std::sync::MutexGuard<'_, Fnv1aHasher>> {
        self.0.py_lock()
    }

    fn finish(&self) -> PyResult<u64> {
        self.0.py_lock().map(|h| h.finish())
    }
}

impl From<Fnv1aHasher> for PyFnv1a {
    fn from(hasher: Fnv1aHasher) -> Self {
        Self(RyMutex::new(hasher))
    }
}

impl From<u64> for PyFnv1a {
    fn from(key: u64) -> Self {
        Self(RyMutex::new(Fnv1aHasher::with_key(key)))
    }
}

#[inline]
fn fnv1a_oneshot(bytes: &[u8], key: u64) -> u64 {
    bytes.iter().fold(key, |hash, &byte| {
        let hash = hash ^ u64::from(byte);
        hash.wrapping_mul(FNV1A_64_PRIME)
    })
}

#[pymethods]
impl PyFnv1a {
    #[new]
    #[pyo3(
        signature = (data = None, *, key = Fnv1aKey::default()),
        text_signature = "(data=None, *, key=0xcbf29ce484222325)",
    )]
    fn py_new(py: Python<'_>, data: Option<RyBytes>, key: Fnv1aKey) -> Self {
        py.detach(|| match data {
            Some(b) => {
                let mut hasher = Fnv1aHasher::from(key);
                hasher.write(b.as_ref());
                Self::from(hasher)
            }
            None => Self::from(Fnv1aHasher::from(key)),
        })
    }

    #[classattr]
    fn digest_size() -> usize {
        8
    }

    #[classattr]
    fn block_size() -> usize {
        // well fnv ain't blocky and just does a byte at a time
        // so i guess it's just 1?
        1
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::new(py, [py.None().into_bound_py_any(py)?])?;
        let kw = pyo3::types::PyDict::new(py);
        let key = self.finish()?;
        kw.set_item(pyo3::intern!(py, "key"), key)?;
        PyTuple::new(py, [args.into_bound_py_any(py)?, kw.into_bound_py_any(py)?])
    }

    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "fnv1a")
    }

    fn intdigest(&self) -> PyResult<u64> {
        self.finish()
    }

    fn digest(&self) -> PyResult<PyDigest<u64>> {
        self.finish().map(PyDigest::from)
    }

    fn hexdigest(&self) -> PyResult<PyHexDigest<u64>> {
        self.finish().map(PyHexDigest::from)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, py: Python<'_>, data: RyBytes) -> PyResult<()> {
        py.detach(|| {
            let mut h = self.lock()?;
            h.write(data.as_ref());
            Ok(())
        })
    }

    fn copy(&self) -> PyResult<Self> {
        self.finish().map(Self::from)
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(
        signature = (data, *, key = Fnv1aKey::default()),
        text_signature = "(data, *, key=0xcbf29ce484222325)",
    )]
    #[staticmethod]
    fn oneshot(data: RyBytes, key: Fnv1aKey) -> u64 {
        fnv1a_oneshot(data.as_ref(), key.into())
    }
}

impl std::fmt::Display for PyFnv1a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.finish().expect("no-way-jose");
        write!(f, "fnv1a<{key:x}>")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Fnv1aKey(u64);

impl Default for Fnv1aKey {
    fn default() -> Self {
        Self(FNV1A_64_OFFSET)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for Fnv1aKey {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(n) = obj.extract::<u64>() {
            Ok(Self(n))
        } else if let Ok(b) = obj.extract::<[u8; 8]>() {
            let key = u64::from_be_bytes(b);
            Ok(Self(key))
        } else {
            py_type_err!("Key must be an integer or 8-byte bytes-like object")
        }
    }
}

impl From<Fnv1aKey> for u64 {
    fn from(key: Fnv1aKey) -> Self {
        key.0
    }
}

impl From<Fnv1aKey> for Fnv1aHasher {
    fn from(key: Fnv1aKey) -> Self {
        Self::with_key(key.into())
    }
}
