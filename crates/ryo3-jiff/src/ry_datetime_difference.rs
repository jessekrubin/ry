use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::DateTimeDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "DateTimeDifference", module = "ryo3", frozen)]
pub struct RyDateTimeDifference(pub(crate) DateTimeDifference);

impl From<DateTimeDifference> for RyDateTimeDifference {
    fn from(value: DateTimeDifference) -> Self {
        RyDateTimeDifference(value)
    }
}

#[pymethods]
impl RyDateTimeDifference {
    #[new]
    #[pyo3(
       signature = (datetime, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    pub fn py_new(
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
        RyDateTimeDifference(diff)
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        RyDateTimeDifference(self.0.smallest(unit.0))
    }

    fn largest(&self, unit: JiffUnit) -> Self {
        RyDateTimeDifference(self.0.largest(unit.0))
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        RyDateTimeDifference(self.0.mode(mode.0))
    }

    fn increment(&self, increment: i64) -> Self {
        RyDateTimeDifference(self.0.increment(increment))
    }
}
#[derive(Debug, Clone, FromPyObject)]
#[expect(clippy::enum_variant_names)]
pub(crate) enum IntoDateTimeDifferenceTuple {
    UnitDate(JiffUnit, RyDate),
    UnitDateTime(JiffUnit, RyDateTime),
    UnitZoned(JiffUnit, RyZoned),
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum IntoDateTimeDifference {
    RyDateTimeDifference(RyDateTimeDifference),
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
    DateTimeDifferenceTuple(IntoDateTimeDifferenceTuple),
}

impl From<IntoDateTimeDifferenceTuple> for DateTimeDifference {
    fn from(val: IntoDateTimeDifferenceTuple) -> Self {
        match val {
            IntoDateTimeDifferenceTuple::UnitDate(unit, date) => {
                DateTimeDifference::from((unit.0, date.0))
            }
            IntoDateTimeDifferenceTuple::UnitDateTime(unit, date_time) => {
                DateTimeDifference::from((unit.0, date_time.0))
            }
            IntoDateTimeDifferenceTuple::UnitZoned(unit, zoned) => {
                DateTimeDifference::from((unit.0, zoned.0))
            }
        }
    }
}
impl From<IntoDateTimeDifference> for DateTimeDifference {
    fn from(val: IntoDateTimeDifference) -> Self {
        match val {
            IntoDateTimeDifference::RyDateTimeDifference(d_diff) => d_diff.0,
            IntoDateTimeDifference::Zoned(zoned) => DateTimeDifference::from(zoned.0),
            IntoDateTimeDifference::Date(date) => DateTimeDifference::from(date.0),
            IntoDateTimeDifference::DateTime(date) => DateTimeDifference::from(date.0),
            IntoDateTimeDifference::DateTimeDifferenceTuple(tuple) => tuple.into(),
        }
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
            DateTimeDifferenceArg::Zoned(other) => DateTimeDifference::from(other.0),
            DateTimeDifferenceArg::DateTime(other) => DateTimeDifference::from(other.0),
            DateTimeDifferenceArg::Date(other) => DateTimeDifference::from(other.0),
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
