use crate::ry_timestamp::RyTimestamp;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::TimestampDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "TimestampDifference", module = "ryo3")]
pub struct RyTimestampDifference(pub(crate) TimestampDifference);

impl From<TimestampDifference> for RyTimestampDifference {
    fn from(value: TimestampDifference) -> Self {
        RyTimestampDifference(value)
    }
}

#[pymethods]
impl RyTimestampDifference {
    #[new]
    #[pyo3(
       signature = (timestamp, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    pub fn py_new(
        timestamp: &RyTimestamp,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let mut d_diff = TimestampDifference::new(timestamp.0);
        if let Some(smallest) = smallest {
            d_diff = d_diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            d_diff = d_diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            d_diff = d_diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            d_diff = d_diff.increment(increment);
        }
        RyTimestampDifference(d_diff)
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        RyTimestampDifference(self.0.smallest(unit.0))
    }

    fn largest(&self, unit: JiffUnit) -> Self {
        RyTimestampDifference(self.0.largest(unit.0))
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        RyTimestampDifference(self.0.mode(mode.0))
    }

    fn increment(&self, increment: i64) -> Self {
        RyTimestampDifference(self.0.increment(increment))
    }
}
#[derive(Debug, Clone, FromPyObject)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum IntoTimestampDifferenceTuple {
    UnitTimestamp(JiffUnit, RyTimestamp),
    UnitZoned(JiffUnit, RyZoned),
}

impl From<IntoTimestampDifferenceTuple> for TimestampDifference {
    fn from(val: IntoTimestampDifferenceTuple) -> Self {
        match val {
            IntoTimestampDifferenceTuple::UnitTimestamp(unit, date) => {
                TimestampDifference::from((unit.0, date.0))
            }
            IntoTimestampDifferenceTuple::UnitZoned(unit, zoned) => {
                TimestampDifference::from((unit.0, zoned.0))
            }
        }
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum IntoTimestampDifference {
    RyTimestampDifference(RyTimestampDifference),
    Zoned(RyZoned),
    Timestamp(RyTimestamp),
    TimestampDifferenceTuple(IntoTimestampDifferenceTuple),
}

impl From<IntoTimestampDifference> for TimestampDifference {
    fn from(val: IntoTimestampDifference) -> Self {
        match val {
            IntoTimestampDifference::RyTimestampDifference(d_diff) => d_diff.0,
            IntoTimestampDifference::Zoned(zoned) => TimestampDifference::from(zoned.0),
            IntoTimestampDifference::Timestamp(date) => TimestampDifference::from(date.0),
            IntoTimestampDifference::TimestampDifferenceTuple(tuple) => tuple.into(),
        }
    }
}
