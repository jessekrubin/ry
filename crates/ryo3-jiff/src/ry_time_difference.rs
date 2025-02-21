use crate::ry_datetime::RyDateTime;
use crate::ry_time::RyTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::TimeDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "TimeDifference", module = "ryo3", frozen)]
pub struct RyTimeDifference(pub(crate) TimeDifference);

impl From<TimeDifference> for RyTimeDifference {
    fn from(value: TimeDifference) -> Self {
        RyTimeDifference(value)
    }
}

#[pymethods]
impl RyTimeDifference {
    #[new]
    #[pyo3(
       signature = (time, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    pub fn py_new(
        time: &RyTime,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let mut diff = TimeDifference::new(time.0);
        if let Some(smallest) = smallest {
            diff = diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            diff = diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            diff = diff.increment(increment);
        }
        RyTimeDifference(diff)
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        RyTimeDifference(self.0.smallest(unit.0))
    }

    fn largest(&self, unit: JiffUnit) -> Self {
        RyTimeDifference(self.0.largest(unit.0))
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        RyTimeDifference(self.0.mode(mode.0))
    }

    fn increment(&self, increment: i64) -> Self {
        RyTimeDifference(self.0.increment(increment))
    }
}
#[derive(Debug, Clone, FromPyObject)]
#[expect(clippy::enum_variant_names)]
pub(crate) enum IntoTimeDifferenceTuple {
    UnitTime(JiffUnit, RyTime),
    UnitDateTime(JiffUnit, RyDateTime),
    UnitZoned(JiffUnit, RyZoned),
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum IntoTimeDifference {
    RyTimeDifference(RyTimeDifference),
    Zoned(RyZoned),
    Time(RyTime),
    DateTime(RyDateTime),
    TimeDifferenceTuple(IntoTimeDifferenceTuple),
}

impl From<IntoTimeDifferenceTuple> for TimeDifference {
    fn from(val: IntoTimeDifferenceTuple) -> Self {
        match val {
            IntoTimeDifferenceTuple::UnitTime(unit, date) => TimeDifference::from((unit.0, date.0)),
            IntoTimeDifferenceTuple::UnitDateTime(unit, date_time) => {
                TimeDifference::from((unit.0, date_time.0))
            }
            IntoTimeDifferenceTuple::UnitZoned(unit, zoned) => {
                TimeDifference::from((unit.0, zoned.0))
            }
        }
    }
}
impl From<IntoTimeDifference> for TimeDifference {
    fn from(val: IntoTimeDifference) -> Self {
        match val {
            IntoTimeDifference::RyTimeDifference(d_diff) => d_diff.0,
            IntoTimeDifference::Zoned(zoned) => TimeDifference::from(zoned.0),
            IntoTimeDifference::Time(date) => TimeDifference::from(date.0),
            IntoTimeDifference::DateTime(date) => TimeDifference::from(date.0),
            IntoTimeDifference::TimeDifferenceTuple(tuple) => tuple.into(),
        }
    }
}

// ============================================================================
// Zoned/Time/DateTime
// ============================================================================

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum TimeDifferenceArg {
    Zoned(RyZoned),
    Time(RyTime),
    DateTime(RyDateTime),
}

impl TimeDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> TimeDifference {
        let mut diff = match self {
            TimeDifferenceArg::Time(other) => TimeDifference::from(other.0),
            TimeDifferenceArg::Zoned(other) => TimeDifference::from(other.0),
            TimeDifferenceArg::DateTime(other) => TimeDifference::from(other.0),
        };
        if let Some(smallest) = smallest {
            diff = diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            diff = diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            diff = diff.increment(increment);
        }
        diff
    }
}
