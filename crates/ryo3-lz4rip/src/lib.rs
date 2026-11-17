#![doc = include_str!("../README.md")]
pub mod block;
pub mod constants;
pub mod dict;
pub mod error;
#[cfg(feature = "frame")]
pub mod frame;
pub use error::Error;
use pyo3::prelude::*;

pub type Ryo3Lz4ripResult<T, E = error::Error> = std::result::Result<T, E>;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "frame")]
    {
        frame::pymod_add(m)?;
    }
    m.add_function(wrap_pyfunction!(block::lz4_compress_block, m)?)?;
    m.add_function(wrap_pyfunction!(block::lz4_decompress_block, m)?)?;
    m.add_function(wrap_pyfunction!(dict::lz4_train_dict, m)?)?;
    m.add_class::<block::RyLz4BlockCompressor>()?;
    m.add_class::<block::RyLz4BlockDecompressor>()?;
    Ok(())
}
