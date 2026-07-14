//! lz4 dictionary training
use lz4rip::block::DictTrainer;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::macros::py_value_err;

use crate::Error;
use crate::error::RyLz4Error;

#[pyfunction]
#[pyo3(signature = (samples, dict_size = 65_535))]
pub fn lz4_train_dict(
    py: Python<'_>,
    samples: &Bound<'_, PyAny>,
    dict_size: usize,
) -> PyResult<RyBytes> {
    if dict_size == 0 {
        return py_value_err!("dict_size must be positive");
    }
    let mut trainer = DictTrainer::new(dict_size);
    for sample in samples.try_iter()? {
        let sample = sample?;
        let buf = sample.extract::<ReadableBuffer>()?;
        trainer.add_sample(buf.as_ref());
    }
    let samples = trainer.sample_count();
    let dict = py.detach(|| trainer.train());
    if dict.is_empty() {
        return Err(Error::from(RyLz4Error::DictTrainFailed { samples }).into());
    }
    Ok(dict.into())
}
