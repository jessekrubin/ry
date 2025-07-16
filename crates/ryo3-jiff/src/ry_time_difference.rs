use crate::ry_datetime::RyDateTime;
use crate::ry_time::RyTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::TimeDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "TimeDifference", module = "ry.ryo3", frozen)]
pub struct RyTimeDifference(pub(crate) TimeDifference);

impl From<TimeDifference> for RyTimeDifference {
    fn from(value: TimeDifference) -> Self {
        Self(value)
    }
}

#[pymethods]
impl RyTimeDifference {
    #[new]
    #[pyo3(
       signature = (time, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    fn py_new(
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
        Self(diff)
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        Self(self.0.smallest(unit.0))
    }

    fn largest(&self, unit: JiffUnit) -> Self {
        Self(self.0.largest(unit.0))
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        Self(self.0.mode(mode.0))
    }

    fn increment(&self, increment: i64) -> Self {
        Self(self.0.increment(increment))
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
            Self::Time(other) => TimeDifference::from(other.0),
            Self::Zoned(other) => TimeDifference::from(other.0),
            Self::DateTime(other) => TimeDifference::from(other.0),
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
