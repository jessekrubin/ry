use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::DateTimeDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "DateTimeDifference", module = "ry.ryo3", frozen)]
pub struct RyDateTimeDifference(pub(crate) DateTimeDifference);

impl From<DateTimeDifference> for RyDateTimeDifference {
    fn from(value: DateTimeDifference) -> Self {
        Self(value)
    }
}

#[pymethods]
impl RyDateTimeDifference {
    #[new]
    #[pyo3(
       signature = (datetime, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    fn py_new(
        datetime: &RyDateTime,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let mut diff = DateTimeDifference::new(datetime.0);
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
// Date/DateTime/Zoned
// ============================================================================

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum DateTimeDifferenceArg {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}

impl DateTimeDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> DateTimeDifference {
        let mut diff = match self {
            Self::Zoned(other) => DateTimeDifference::from(other.0),
            Self::DateTime(other) => DateTimeDifference::from(other.0),
            Self::Date(other) => DateTimeDifference::from(other.0),
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
