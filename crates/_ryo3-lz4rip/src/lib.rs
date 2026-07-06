#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use lz4rip::block::{
    compress, compress_into_with_dict, decompress_into, decompress_into_with_dict,
    get_maximum_output_size,
};
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};
mod py_frame_info;
mod py_lz4_dict_compressor;
use lz4rip::frame::FrameEncoder;
pub use py_frame_info::{PyBlockMode, PyBlockSize, PyFrameInfo};
pub use py_lz4_dict_compressor::PyLz4DictCompressor;
use ryo3_core::macros::{py_value_err, py_value_error};

fn compression_error(err: impl std::fmt::Display) -> PyErr {
    py_value_error!("LZ4 compression failed: {err}")
}

fn decompression_error(err: impl std::fmt::Display) -> PyErr {
    py_value_error!("LZ4 decompression failed: {err}")
}

#[pyfunction]
#[pyo3(signature = (data, *, block = false, dictionary = None, dict_id = None, frame_info = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn lz4_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    block: bool,
    dictionary: Option<ReadableBuffer>,
    dict_id: Option<u32>,
    frame_info: Option<py_frame_info::PyFrameInfo>,
) -> PyResult<RyBytes> {
    let input = data.as_ref();
    let dict = dictionary.as_ref().map(AsRef::as_ref);
    if block && frame_info.is_some() {
        return py_value_err!("frame_info is not applicable when block=True");
    }
    if block {
        py.detach(|| {
            if let Some(dict) = dict {
                let mut output = vec![0; get_maximum_output_size(input.len())];
                let size =
                    compress_into_with_dict(input, &mut output, dict).map_err(compression_error)?;
                output.truncate(size);
                Ok(output.into())
            } else {
                Ok(compress(input).into())
            }
        })
    } else {
        py.detach(|| {
            let mut output = Vec::new();
            let mut encoder = if let Some(dict) = dict {
                FrameEncoder::with_dictionary(
                    &mut output,
                    dict,
                    dict_id.unwrap_or(0),
                    frame_info.map(py_frame_info::PyFrameInfo::into_inner),
                )
                .map_err(compression_error)?
            } else {
                FrameEncoder::new(&mut output)
            };
            encoder.write_all(input).map_err(compression_error)?;
            encoder.finish().map_err(compression_error)?;
            Ok(output.into())
        })
    }
}

#[pyfunction]
#[pyo3(signature = (data, uncompressed_size, *, block = false, dictionary = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn lz4_decompress(
    py: Python<'_>,
    data: ReadableBuffer,
    uncompressed_size: Option<usize>,
    block: bool,
    dictionary: Option<ReadableBuffer>,
) -> PyResult<RyBytes> {
    let input = data.as_ref();
    let dict = dictionary.as_ref().map(AsRef::as_ref);

    if block {
        py.detach(|| {
            let mut output = vec![0; uncompressed_size.unwrap_or(0)];
            let size = if let Some(dict) = dict {
                decompress_into_with_dict(input, &mut output, dict)
            } else {
                decompress_into(input, &mut output)
            }
            .map_err(decompression_error)?;
            output.truncate(size);
            Ok(output.into())
        })
    } else {
        py.detach(|| {
            let mut output = vec![0; uncompressed_size.unwrap_or(0)];
            let mut decoder = lz4rip::frame::FrameDecoder::new(input);
            decoder
                .read_exact(&mut output)
                .map_err(decompression_error)?;
            Ok(output.into())
        })
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lz4_compress, m)?)?;
    m.add_function(wrap_pyfunction!(lz4_decompress, m)?)?;
    m.add_class::<PyLz4DictCompressor>()?;
    Ok(())
}
