#![doc = include_str!("../README.md")]
pub mod block;
pub mod constants;
#[cfg(feature = "frame")]
pub mod frame;
pub use error::Error;
use pyo3::prelude::*;

pub type Ryo3Lz4ripResult<T, E = error::Error> = std::result::Result<T, E>;

pub mod error {
    use pyo3::prelude::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RyLz4Error {
        /// input too (damn) large for a u32-le size prefix
        BlockPrefixTooBig,
        /// input too (damn) short for u32-le size prefix (4 bytes)
        BlockPrefixMissing { input_len: usize },
        /// size prefix claims more than the theoretical max compression ratio
        /// REF: <https://github.com/lz4/lz4/blob/dev/doc/lz4_Block_format.md>
        BlockPrefixFishy { size: usize, input_len: usize },
        /// size prefix dont match da real decompressed size
        BlockPrefixMismatch { expected: usize, actual: usize },
        /// frame header declares a content-size that doesn't fit in `usize`
        /// (only reachable on <64-bit targets)
        FrameContentSizeTooBig { declared: u64 },
    }

    #[derive(Debug)]
    pub enum Error {
        // --------------------------------------------------------------------
        // ryo3-lz4rip errors
        // --------------------------------------------------------------------
        Ry(RyLz4Error),

        // --------------------------------------------------------------------
        // foreign errors
        // --------------------------------------------------------------------
        Io(std::io::Error),
        BlockCompress(lz4rip::block::CompressError),
        BlockDecompress(lz4rip::block::DecompressError),
        #[cfg(feature = "frame")]
        FrameError(lz4rip::frame::Error),
    }

    impl std::fmt::Display for RyLz4Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::BlockPrefixTooBig => {
                    write!(f, "block size prefix too big; input must be < u32::MAX")
                }
                Self::BlockPrefixMissing { input_len } => {
                    write!(
                        f,
                        "input too short for u32-le size prefix (got {input_len} bytes); pass `size` for raw blocks"
                    )
                }
                Self::BlockPrefixFishy { size, input_len } => {
                    write!(
                        f,
                        "size prefix ({size}) impossibly large for {input_len}-byte input"
                    )
                }
                Self::BlockPrefixMismatch { expected, actual } => {
                    write!(
                        f,
                        "size prefix ({expected}) != decompressed size ({actual})"
                    )
                }
                Self::FrameContentSizeTooBig { declared } => {
                    write!(
                        f,
                        "frame declares content-size ({declared}) larger than usize::MAX"
                    )
                }
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Ry(e) => write!(f, "{e}"),
                Self::Io(e) => write!(f, "io error: {e}"),
                Self::BlockCompress(e) => write!(f, "block compression error: {e}"),
                Self::BlockDecompress(e) => write!(f, "block decompression error: {e}"),
                #[cfg(feature = "frame")]
                Self::FrameError(e) => write!(f, "frame error: {e}"),
            }
        }
    }

    impl From<RyLz4Error> for Error {
        fn from(err: RyLz4Error) -> Self {
            Self::Ry(err)
        }
    }

    impl From<std::io::Error> for Error {
        fn from(err: std::io::Error) -> Self {
            Self::Io(err)
        }
    }

    impl From<lz4rip::block::CompressError> for Error {
        fn from(err: lz4rip::block::CompressError) -> Self {
            Self::BlockCompress(err)
        }
    }

    impl From<lz4rip::block::DecompressError> for Error {
        fn from(err: lz4rip::block::DecompressError) -> Self {
            Self::BlockDecompress(err)
        }
    }

    #[cfg(feature = "frame")]
    impl From<lz4rip::frame::Error> for Error {
        fn from(err: lz4rip::frame::Error) -> Self {
            match err {
                lz4rip::frame::Error::IoError(e) => Self::Io(e),
                lz4rip::frame::Error::CompressionError(e) => Self::BlockCompress(e),
                lz4rip::frame::Error::DecompressionError(e) => Self::BlockDecompress(e),
                _ => Self::FrameError(err),
            }
        }
    }

    impl From<Error> for PyErr {
        fn from(err: Error) -> Self {
            match err {
                Error::Io(e) => e.into(),
                _ => pyo3::exceptions::PyValueError::new_err(err.to_string()),
            }
        }
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "frame")]
    {
        frame::pymod_add(m)?;
    }
    m.add_function(wrap_pyfunction!(block::lz4_compress_block, m)?)?;
    m.add_function(wrap_pyfunction!(block::lz4_decompress_block, m)?)?;
    m.add_class::<block::PyLz4BlockCompressor>()?;
    m.add_class::<block::PyLz4BlockDecompressor>()?;
    Ok(())
}
