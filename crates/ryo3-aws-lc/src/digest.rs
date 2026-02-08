use aws_lc_rs::digest::{Context, Digest};
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::types::PyHexDigest;
use ryo3_core::{PyAsciiString, RyMutex};

// SHA1,
// SHA224,
// SHA256,
// SHA384,
// SHA512,
// SHA512_256,
// SHA3_256,
// SHA3_384,
// SHA3_512,

// struct PyHexDigest<const N: usize>([u8; N]);
// const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";

// #[inline]
// fn encode_hex_ref<const N: usize, const S: usize>(bytes: &[u8; N]) -> [u8; S] {
//     debug_assert!(S == N * 2, "S != N * 2");
//     let mut out = [0u8; S];
//     for i in 0..N {
//         let b = bytes[i];
//         out[i * 2] = HEX_CHARS_LOWER[(b >> 4) as usize];
//         out[i * 2 + 1] = HEX_CHARS_LOWER[(b & 0x0f) as usize];
//     }
//     out
// }

// impl<'py> IntoPyObject<'py> for PyHexDigest< { aws_lc_rs::digest::SHA256_OUTPUT_LEN * 2 }> {
//     type Target = PyString;
//     type Output = Bound<'py, Self::Target>;
//     type Error = std::convert::Infallible;

//     #[inline]
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         Ok(PyString::new(py, core::str::from_utf8(&self.0).unwrap()))
//     }
// }

////////////////////

trait PyAlgorithm {
    /// digest size in bytes
    const OUTPUT_LEN: usize;
    /// block size in bytes
    const BLOCK_LEN: usize;
    /// hex digest size in characters (2 chars per byte)
    const HEX_DIGEST_LEN: usize = Self::OUTPUT_LEN * 2;
    /// the name of the digest algorithm
    const NAME: &'static str;
    fn algorithm() -> &'static aws_lc_rs::digest::Algorithm;
}

struct PyContext<A: PyAlgorithm> {
    ctx: Context,
    _algorithm: std::marker::PhantomData<A>,
}

impl<C: PyAlgorithm> PyContext<C> {
    fn new() -> Self {
        Self {
            ctx: Context::new(C::algorithm()),
            _algorithm: std::marker::PhantomData,
        }
    }

    fn new_with_data(data: &[u8]) -> Self {
        let mut ctx = Context::new(C::algorithm());
        ctx.update(data);
        Self {
            ctx,
            _algorithm: std::marker::PhantomData,
        }
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.ctx.update(data);
    }

    #[inline]
    fn finish(&self) -> Digest {
        self.ctx.clone().finish()
    }

    #[inline]
    #[must_use]
    fn algorithm(&self) -> &'static aws_lc_rs::digest::Algorithm {
        self.ctx.algorithm()
    }

    fn from_context(ctx: Context) -> Self {
        // this was weird to figure out but I guess I have to cmp the pointers
        debug_assert!(
            ctx.algorithm() as *const _ == C::algorithm() as *const _,
            "Context algorithm does not match PyAlgorithm"
        );
        Self {
            ctx,
            _algorithm: std::marker::PhantomData,
        }
    }
}

struct PyMutexContext<A: PyAlgorithm>(RyMutex<PyContext<A>>);

impl<A: PyAlgorithm> PyMutexContext<A> {
    fn new() -> Self {
        Self(RyMutex::new(PyContext::new()))
    }

    fn new_with_data(data: &[u8]) -> Self {
        Self(RyMutex::new(PyContext::new_with_data(data)))
    }

    fn py_lock(&self) -> PyResult<std::sync::MutexGuard<'_, PyContext<A>>> {
        self.0.py_lock()
    }
}

// ============================================================================
// MADNESS FROM FUGUE STATE OF MACRO WRITING
// ============================================================================

/// Macro for defining the `PyAlgorithm` structs
///
/// This is the outbout I worked back from:
/// ```rust
/// struct PySha256Algorithm;
/// impl PyAlgorithm for PySha256Algorithm {
///     const OUTPUT_LEN: usize = (aws_lc_rs::digest::SHA256_OUTPUT_LEN);
///     const BLOCK_LEN: usize = (512 / 8);
///     const HEX_DIGEST_LEN: usize = Self::OUTPUT_LEN * 2;
///     const NAME: &'static str = "sha256";
///     fn algorithm() -> &'static aws_lc_rs::digest::Algorithm {
///         &aws_lc_rs::digest::SHA256
///     }
/// }
/// ```

macro_rules! define_py_algorithm {
    (
        py_algorithm = $py_algorithm:ident,
        algorithm = $algorithm:expr,
        output_len = $output_len:expr,
        block_len = $block_len:expr,
        name = $name:expr
    ) => {
        struct $py_algorithm;
        impl PyAlgorithm for $py_algorithm {
            const OUTPUT_LEN: usize = $output_len;
            const BLOCK_LEN: usize = $block_len;
            const HEX_DIGEST_LEN: usize = Self::OUTPUT_LEN * 2;
            const NAME: &'static str = $name;
            fn algorithm() -> &'static aws_lc_rs::digest::Algorithm {
                &$algorithm
            }
        }
    };
}
// [
//   {
//     "name": "sha1",
//     "output_len": 20,
//     "block_len": 64
//   },
//   {
//     "name": "sha224",
//     "output_len": 28,
//     "block_len": 64
//   },
//   {
//     "name": "sha256",
//     "output_len": 32,
//     "block_len": 64
//   },
//   {
//     "name": "sha384",
//     "output_len": 48,
//     "block_len": 128
//   },
//   {
//     "name": "sha3_256",
//     "output_len": 32,
//     "block_len": 136
//   },
//   {
//     "name": "sha3_384",
//     "output_len": 48,
//     "block_len": 104
//   },
//   {
//     "name": "sha3_512",
//     "output_len": 64,
//     "block_len": 72
//   },
//   {
//     "name": "sha512",
//     "output_len": 64,
//     "block_len": 128
//   },
//   {
//     "name": "sha512_256",
//     "output_len": 32,
//     "block_len": 128
//   }
// ]

pub(crate) const SHA1_OUTPUT_LEN: usize = 20;
pub(crate) const SHA1_OUTPUT_LEN_HEX: usize = SHA1_OUTPUT_LEN * 2;
pub(crate) const SHA1_BLOCK_LEN: usize = 64;

pub(crate) const SHA224_OUTPUT_LEN: usize = 28;
pub(crate) const SHA224_OUTPUT_LEN_HEX: usize = SHA224_OUTPUT_LEN * 2;
pub(crate) const SHA224_BLOCK_LEN: usize = 64;

pub(crate) const SHA256_OUTPUT_LEN: usize = 32;
pub(crate) const SHA256_OUTPUT_LEN_HEX: usize = SHA256_OUTPUT_LEN * 2;
pub(crate) const SHA256_BLOCK_LEN: usize = 64;

pub(crate) const SHA384_OUTPUT_LEN: usize = 48;
pub(crate) const SHA384_OUTPUT_LEN_HEX: usize = SHA384_OUTPUT_LEN * 2;
pub(crate) const SHA384_BLOCK_LEN: usize = 128;

pub(crate) const SHA3_256_OUTPUT_LEN: usize = 32;
pub(crate) const SHA3_256_OUTPUT_LEN_HEX: usize = SHA3_256_OUTPUT_LEN * 2;
pub(crate) const SHA3_256_BLOCK_LEN: usize = 136;

pub(crate) const SHA3_384_OUTPUT_LEN: usize = 48;
pub(crate) const SHA3_384_OUTPUT_LEN_HEX: usize = SHA3_384_OUTPUT_LEN * 2;
pub(crate) const SHA3_384_BLOCK_LEN: usize = 104;

pub(crate) const SHA3_512_OUTPUT_LEN: usize = 64;
pub(crate) const SHA3_512_OUTPUT_LEN_HEX: usize = SHA3_512_OUTPUT_LEN * 2;
pub(crate) const SHA3_512_BLOCK_LEN: usize = 72;

pub(crate) const SHA512_OUTPUT_LEN: usize = 64;
pub(crate) const SHA512_OUTPUT_LEN_HEX: usize = SHA512_OUTPUT_LEN * 2;
pub(crate) const SHA512_BLOCK_LEN: usize = 128;

pub(crate) const SHA512_256_OUTPUT_LEN: usize = 32;
pub(crate) const SHA512_256_OUTPUT_LEN_HEX: usize = SHA512_256_OUTPUT_LEN * 2;
pub(crate) const SHA512_256_BLOCK_LEN: usize = 128;

// SHA1
define_py_algorithm!(
    py_algorithm = PySha1Algorithm,
    algorithm = aws_lc_rs::digest::SHA1_FOR_LEGACY_USE_ONLY,
    output_len = SHA1_OUTPUT_LEN, // 20
    block_len = SHA1_BLOCK_LEN,   // 64
    name = "sha1"
);
// SHA224
define_py_algorithm!(
    py_algorithm = PySha224Algorithm,
    algorithm = aws_lc_rs::digest::SHA224,
    output_len = SHA224_OUTPUT_LEN, // 28
    block_len = SHA224_BLOCK_LEN,   // 64
    name = "sha224"
);
// SHA256
define_py_algorithm!(
    py_algorithm = PySha256Algorithm,
    algorithm = aws_lc_rs::digest::SHA256,
    output_len = SHA256_OUTPUT_LEN,
    block_len = SHA256_BLOCK_LEN, // 64
    name = "sha256"
);
// SHA384
define_py_algorithm!(
    py_algorithm = PySha384Algorithm,
    algorithm = aws_lc_rs::digest::SHA384,
    output_len = SHA384_OUTPUT_LEN,
    block_len = SHA384_BLOCK_LEN, // 128
    name = "sha384"
);
// SHA512
define_py_algorithm!(
    py_algorithm = PySha512Algorithm,
    algorithm = aws_lc_rs::digest::SHA512,
    output_len = SHA512_OUTPUT_LEN,
    block_len = SHA512_BLOCK_LEN, // 128
    name = "sha512"
);

// SHA512_256
define_py_algorithm!(
    py_algorithm = PySha512_256Algorithm,
    algorithm = aws_lc_rs::digest::SHA512_256,
    output_len = SHA512_256_OUTPUT_LEN,
    block_len = SHA512_256_BLOCK_LEN, // 128
    name = "sha512_256"
);

// SHA3_256
define_py_algorithm!(
    py_algorithm = PySha3_256Algorithm,
    algorithm = aws_lc_rs::digest::SHA3_256,
    output_len = SHA3_256_OUTPUT_LEN,
    block_len = SHA3_256_BLOCK_LEN, // 136
    name = "sha3_256"
);

// SHA3_384
define_py_algorithm!(
    py_algorithm = PySha3_384Algorithm,
    algorithm = aws_lc_rs::digest::SHA3_384,
    output_len = SHA3_384_OUTPUT_LEN,
    block_len = SHA3_384_BLOCK_LEN, // 104
    name = "sha3_384"
);
// SHA3_512,
define_py_algorithm!(
    py_algorithm = PySha3_512Algorithm,
    algorithm = aws_lc_rs::digest::SHA3_512,
    output_len = SHA3_512_OUTPUT_LEN,
    block_len = SHA3_512_BLOCK_LEN, // 72
    name = "sha3_512"
);

// ============================================================================

#[pyclass(name = "sha256", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySha256(PyMutexContext<PySha256Algorithm>);

macro_rules! define_py_hasher {
    (
        py_struct = $py_struct:ident,
        py_name = $name:expr,
        algorithm = $algorithm:ty,
        output_len = $output_len:expr,
        output_len_hex = $output_len_hex:expr,
        block_len = $block_len:expr
    ) => {
        #[pyclass(name = $name, frozen, immutable_type, skip_from_py_object)]
        #[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
        pub struct $py_struct(PyMutexContext<$algorithm>);

        impl $py_struct {
            fn digest_bytes(&self) -> PyResult<[u8; $output_len]> {
                let ctx = self.0.py_lock()?;
                let digest = ctx.finish();
                Ok(digest
                    .as_ref()
                    .try_into()
                    .expect(concat!($name, " digest size mismatch")))
            }
        }

        impl std::fmt::Display for $py_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let selfptr = std::ptr::from_ref(self);
                write!(f, "{}<{}>", $name, selfptr as usize)
            }
        }

        #[pymethods]
        impl $py_struct {
            #[new]
            #[pyo3(signature = (data = None, *), text_signature = "(data=None, /)")]
            fn py_new(py: Python<'_>, data: Option<RyBytes>) -> Self {
                py.detach(|| match data {
                    Some(b) => Self(PyMutexContext::new_with_data(b.as_ref())),
                    None => Self(PyMutexContext::new()),
                })
            }

            #[classattr]
            fn digest_size() -> usize {
                <$algorithm as PyAlgorithm>::OUTPUT_LEN
            }

            #[classattr]
            fn block_size() -> usize {
                <$algorithm as PyAlgorithm>::BLOCK_LEN
            }

            #[classattr]
            fn name(py: Python<'_>) -> &Bound<'_, PyString> {
                pyo3::intern!(py, <$algorithm as PyAlgorithm>::NAME)
            }

            fn __repr__(&self) -> PyAsciiString {
                format!("{self}").into()
            }

            fn digest(&self) -> PyResult<PyAwsLcRsDigest<$output_len>> {
                let ctx = self.0.py_lock()?;
                let digest = ctx.finish();
                Ok(PyAwsLcRsDigest(digest))
            }

            fn hexdigest(&self) -> PyResult<PyHexDigest<[u8; $output_len_hex]>> {
                let bytes = self.digest_bytes()?;
                Ok(PyHexDigest::from(&bytes))
            }

            // #[expect(clippy::needless_pass_by_value)]
            fn update(&self, data: RyBytes) -> PyResult<()> {
                let mut ctx = self.0.py_lock()?;
                ctx.update(data.as_ref());
                Ok(())
            }

            fn copy(&self) -> PyResult<Self> {
                let ctx = self.0.py_lock()?;
                Ok(Self(PyMutexContext(RyMutex::new(PyContext::from_context(
                    ctx.ctx.clone(),
                )))))
            }

            #[staticmethod]
            #[allow(clippy::needless_pass_by_value)]
            fn oneshot(data: RyBytes) -> PyResult<PyAwsLcRsDigest<$output_len>> {
                // weird <> syntax works...
                let mut ctx = Context::new(<$algorithm as PyAlgorithm>::algorithm());
                ctx.update(data.as_ref());
                let digest = ctx.finish();
                Ok(PyAwsLcRsDigest(digest))
            }
        }
    };
}

define_py_hasher!(
    py_struct = PySha1,
    py_name = "sha1",
    algorithm = PySha1Algorithm,
    output_len = SHA1_OUTPUT_LEN,
    output_len_hex = SHA1_OUTPUT_LEN_HEX,
    block_len = SHA1_BLOCK_LEN
);

define_py_hasher!(
    py_struct = PySha224,
    py_name = "sha224",
    algorithm = PySha224Algorithm,
    output_len = SHA224_OUTPUT_LEN,
    output_len_hex = SHA224_OUTPUT_LEN_HEX,
    block_len = SHA224_BLOCK_LEN
);

// NOT USED -- The sha256 struct and stuff are the non-macro'd versions
// that I used to actually write the macro
//
// define_py_hasher!(
//     py_struct = PySha256,
//     py_name = "sha256",
//     algorithm = PySha256Algorithm,
//     output_len = SHA256_OUTPUT_LEN,
//     output_len_hex = SHA256_OUTPUT_LEN_HEX,
//     block_len = SHA256_BLOCK_LEN
// );

define_py_hasher!(
    py_struct = PySha384,
    py_name = "sha384",
    algorithm = PySha384Algorithm,
    output_len = SHA384_OUTPUT_LEN,
    output_len_hex = SHA384_OUTPUT_LEN_HEX,
    block_len = SHA384_BLOCK_LEN
);

define_py_hasher!(
    py_struct = PySha512,
    py_name = "sha512",
    algorithm = PySha512Algorithm,
    output_len = SHA512_OUTPUT_LEN,
    output_len_hex = SHA512_OUTPUT_LEN_HEX,
    block_len = SHA512_BLOCK_LEN
);

define_py_hasher!(
    py_struct = PySha512_256,
    py_name = "sha512_256",
    algorithm = PySha512_256Algorithm,
    output_len = SHA512_256_OUTPUT_LEN,
    output_len_hex = SHA512_256_OUTPUT_LEN_HEX,
    block_len = SHA512_256_BLOCK_LEN
);

define_py_hasher!(
    py_struct = PySha3_256,
    py_name = "sha3_256",
    algorithm = PySha3_256Algorithm,
    output_len = SHA3_256_OUTPUT_LEN,
    output_len_hex = SHA3_256_OUTPUT_LEN_HEX,
    block_len = SHA3_256_BLOCK_LEN
);

define_py_hasher!(
    py_struct = PySha3_384,
    py_name = "sha3_384",
    algorithm = PySha3_384Algorithm,
    output_len = SHA3_384_OUTPUT_LEN,
    output_len_hex = SHA3_384_OUTPUT_LEN_HEX,
    block_len = SHA3_384_BLOCK_LEN
);

define_py_hasher!(
    py_struct = PySha3_512,
    py_name = "sha3_512",
    algorithm = PySha3_512Algorithm,
    output_len = SHA3_512_OUTPUT_LEN,
    output_len_hex = SHA3_512_OUTPUT_LEN_HEX,
    block_len = SHA3_512_BLOCK_LEN
);

// ============================================================================
// SHA256 -- implementaiton that is NOT macrod for testing and being able to
//           edit the macro
// ============================================================================
impl PySha256 {
    fn digest_bytes(&self) -> PyResult<[u8; SHA256_OUTPUT_LEN]> {
        let ctx = self.0.py_lock()?;
        let digest = ctx.finish();
        Ok(digest.as_ref().try_into().expect("sha256 digest size"))
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
            Some(b) => Self(PyMutexContext::new_with_data(b.as_ref())),
            None => Self(PyMutexContext::new()),
        })
    }

    #[classattr]
    fn digest_size() -> usize {
        <PySha256Algorithm as PyAlgorithm>::OUTPUT_LEN
    }

    #[classattr]
    fn block_size() -> usize {
        <PySha256Algorithm as PyAlgorithm>::BLOCK_LEN
    }

    #[classattr]
    fn name(py: Python<'_>) -> &Bound<'_, PyString> {
        pyo3::intern!(py, <PySha256Algorithm as PyAlgorithm>::NAME)
    }

    fn digest(&self) -> PyResult<PyAwsLcRsDigest<SHA256_OUTPUT_LEN>> {
        let ctx = self.0.py_lock()?;
        let digest = ctx.finish();
        Ok(PyAwsLcRsDigest(digest))
    }

    fn hexdigest(&self) -> PyResult<PyHexDigest<[u8; SHA256_OUTPUT_LEN_HEX]>> {
        let bytes = self.digest_bytes()?;
        Ok(PyHexDigest::from(&bytes))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn update(&self, data: RyBytes) -> PyResult<()> {
        let mut ctx = self.0.py_lock()?;
        ctx.update(data.as_ref());
        Ok(())
    }

    fn copy(&self) -> PyResult<Self> {
        let ctx = self.0.py_lock()?;
        Ok(Self(PyMutexContext(RyMutex::new(PyContext::from_context(
            ctx.ctx.clone(),
        )))))
    }

    #[staticmethod]
    fn oneshot(data: RyBytes) -> PyResult<PyAwsLcRsDigest<SHA256_OUTPUT_LEN>> {
        let mut ctx = Context::new(PySha256Algorithm::algorithm());
        ctx.update(data.as_ref());
        let digest = ctx.finish();
        Ok(PyAwsLcRsDigest(digest))
    }
}

impl std::fmt::Display for PySha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let selfptr = self as *const _;
        f.write_fmt(core::format_args!("{}<{}>", "sha256", selfptr as usize))
    }
}

// ============================================================================
struct PyAwsLcRsDigest<const SIZE: usize>(Digest);

impl<'py, const SIZE: usize> pyo3::IntoPyObject<'py> for PyAwsLcRsDigest<SIZE> {
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
// ============================================================================
// REGISTER CLASSES
// ============================================================================
pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySha1>()?;
    m.add_class::<PySha224>()?;
    m.add_class::<PySha256>()?;
    m.add_class::<PySha384>()?;
    m.add_class::<PySha3_256>()?;
    m.add_class::<PySha3_384>()?;
    m.add_class::<PySha3_512>()?;
    m.add_class::<PySha512>()?;
    m.add_class::<PySha512_256>()?;
    Ok(())
}
