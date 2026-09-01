//! python crc32fast impl
//!
//! This core impl was copy-pasta-ed from the `ryo3-fnv` crate; it is also a
//! streaming one-byte streaming hasher
use std::hash::Hasher;

use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};
use pyo3::{IntoPyObjectExt, intern};
use ryo3_bytes::ReadableBuffer;
use ryo3_core::PyAsciiString;
use ryo3_core::macros::py_type_err;
use ryo3_core::sync::RyMutex;
use ryo3_core::types::{PyDigest, PyHexDigest};

const CRC32_SEED_DEFAULT: u32 = 0;
const HASHLIB_GIL_MINSIZE: usize = 2048;

// ============================================================================
// ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~ PY ~
// ============================================================================

type Crc32Hasher = crc32fast::Hasher;

#[pyclass(name = "crc32", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyCrc32Fast(RyMutex<Crc32Hasher>);

impl PyCrc32Fast {
    fn lock(&self) -> PyResult<std::sync::MutexGuard<'_, Crc32Hasher>> {
        self.0.py_lock()
    }

    fn finish(&self) -> PyResult<u32> {
        let h = self.lock()?;
        Ok(h.clone().finalize())
    }
}

impl From<Crc32Hasher> for PyCrc32Fast {
    fn from(hasher: Crc32Hasher) -> Self {
        Self(RyMutex::new(hasher))
    }
}

impl From<u32> for PyCrc32Fast {
    fn from(seed: u32) -> Self {
        Self(RyMutex::new(Crc32Hasher::new_with_initial(seed)))
    }
}

#[inline]
fn crc32fast_oneshot(bytes: &[u8], seed: u32) -> u32 {
    let mut hasher = Crc32Hasher::new_with_initial(seed);
    hasher.update(bytes);
    hasher.finalize()
}

#[pymethods]
impl PyCrc32Fast {
    #[new]
    #[pyo3(
        signature = (data = None, *, seed = Crc32Seed::default()),
        text_signature = "(data=None, *, seed=0xcbf29ce484222325)",
    )]
    fn py_new(py: Python<'_>, data: Option<ReadableBuffer>, seed: Crc32Seed) -> Self {
        if let Some(b) = data {
            let b = b.as_ref();
            if b.len() > HASHLIB_GIL_MINSIZE {
                py.detach(|| Self::from(crc32fast_oneshot(b, seed.into())))
            } else {
                Self::from(crc32fast_oneshot(b, seed.into()))
            }
        } else {
            Self::from(Crc32Hasher::new_with_initial(seed.into()))
        }
    }

    #[classattr]
    fn digest_size() -> usize {
        4
    }

    #[classattr]
    fn block_size() -> usize {
        1
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        intern!(py, "crc32")
    }

    #[classattr]
    fn default_seed() -> u32 {
        Crc32Seed::default().into()
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::new(py, [py.None().into_bound_py_any(py)?])?;
        let kw = pyo3::types::PyDict::new(py);
        let seed = self.finish()?;
        kw.set_item(pyo3::intern!(py, "seed"), seed)?;
        PyTuple::new(py, [args.into_bound_py_any(py)?, kw.into_bound_py_any(py)?])
    }

    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    fn intdigest(&self) -> PyResult<u32> {
        self.finish()
    }

    fn digest(&self) -> PyResult<PyDigest<u32>> {
        self.finish().map(PyDigest::from)
    }

    fn hexdigest(&self) -> PyResult<PyHexDigest<u32>> {
        self.finish().map(PyHexDigest::from)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, py: Python<'_>, data: ReadableBuffer) -> PyResult<()> {
        let slice = data.as_ref();
        if slice.len() > HASHLIB_GIL_MINSIZE {
            py.detach(|| {
                let mut h = self.lock()?;
                h.write(slice);
                Ok(())
            })
        } else {
            let mut h = self.lock()?;
            h.write(slice);
            Ok(())
        }
    }

    fn copy(&self) -> PyResult<Self> {
        self.finish().map(Self::from)
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(
        signature = (data, *, seed = Crc32Seed::default()),
        text_signature = "(data, *, seed=0)",
    )]
    #[staticmethod]
    fn oneshot(data: ReadableBuffer, seed: Crc32Seed) -> PyDigest<u32> {
        crc32fast_oneshot(data.as_ref(), seed.into()).into()
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(
        signature = (data, *, seed = Crc32Seed::default()),
        text_signature = "(data, *, seed=0)",
    )]
    #[staticmethod]
    fn oneshot_int(data: ReadableBuffer, seed: Crc32Seed) -> u32 {
        crc32fast_oneshot(data.as_ref(), seed.into())
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(
        signature = (data, *, seed = Crc32Seed::default()),
        text_signature = "(data, *, seed=0)",
    )]
    #[staticmethod]
    fn oneshot_hex(data: ReadableBuffer, seed: Crc32Seed) -> PyHexDigest<u32> {
        crc32fast_oneshot(data.as_ref(), seed.into()).into()
    }
}

impl std::fmt::Display for PyCrc32Fast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seed = self.finish().expect("no-way-jose");
        write!(f, "crc32<{seed:x}>")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Crc32Seed(u32);

impl Default for Crc32Seed {
    fn default() -> Self {
        Self(CRC32_SEED_DEFAULT)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for Crc32Seed {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(n) = obj.extract::<u32>() {
            Ok(Self(n))
        } else if let Ok(b) = obj.extract::<[u8; 4]>() {
            let seed = u32::from_be_bytes(b);
            Ok(Self(seed))
        } else {
            py_type_err!("Seed must be an integer or 4-byte bytes-like object")
        }
    }
}

impl From<Crc32Seed> for u32 {
    fn from(seed: Crc32Seed) -> Self {
        seed.0
    }
}

// impl From<Crc32Seed> for Fnv1aHasher {
//     fn from(seed: Crc32Seed) -> Self {
//         Self::with_seed(seed.into())
//     }
// }
