use crate::{JiffRoundMode, JiffUnit};
use jiff::TimestampRound;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "TimestampRound", module = "ryo3", frozen)]
pub struct RyTimestampRound {
    pub smallest: JiffUnit,
    pub mode: JiffRoundMode,
    pub increment: i64,
    // internal
    pub(crate) round: TimestampRound,
}

#[pymethods]
impl RyTimestampRound {
    #[new]
    #[pyo3(signature = (smallest=None, mode=None, increment=1))]
    pub fn py_new(
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> PyResult<Self> {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::HalfExpand));
        let round = TimestampRound::new()
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        Ok(Self {
            smallest,
            mode,
            increment,
            round,
        })
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!(
            "TimestampRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.smallest, self.mode, self.increment
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.smallest == other.smallest
            && self.increment == other.increment
    }

    #[pyo3(signature = (smallest=None, mode=None, increment=None))]
    fn replace(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let smallest = smallest.unwrap_or(self.smallest);
        let mode = mode.unwrap_or(self.mode);
        let increment = increment.unwrap_or(self.increment);
        let round = TimestampRound::new()
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        Self {
            smallest,
            mode,
            increment,
            round,
        }
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    fn _smallest(&self) -> JiffUnit {
        self.smallest
    }

    fn _mode(&self) -> JiffRoundMode {
        self.mode
    }

    fn _increment(&self) -> i64 {
        self.increment
    }
}
