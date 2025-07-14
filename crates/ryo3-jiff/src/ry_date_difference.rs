use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::DateDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "DateDifference", module = "ry.ryo3", frozen)]
pub struct RyDateDifference(pub(crate) DateDifference);

impl From<DateDifference> for RyDateDifference {
    fn from(value: DateDifference) -> Self {
        Self(value)
    }
}

#[pymethods]
impl RyDateDifference {
    #[new]
    #[pyo3(
       signature = (date, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    fn py_new(
        date: &RyDate,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let mut diff = DateDifference::new(date.0);
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
#[derive(Debug, Clone, FromPyObject)]
#[expect(clippy::enum_variant_names)]
pub(crate) enum IntoDateDifferenceTuple {
    UnitDate(JiffUnit, RyDate),
    UnitDateTime(JiffUnit, RyDateTime),
    UnitZoned(JiffUnit, RyZoned),
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum IntoDateDifference {
    RyDateDifference(RyDateDifference),
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
    DateDifferenceTuple(IntoDateDifferenceTuple),
}

impl From<IntoDateDifferenceTuple> for DateDifference {
    fn from(val: IntoDateDifferenceTuple) -> Self {
        match val {
            IntoDateDifferenceTuple::UnitDate(unit, date) => Self::from((unit.0, date.0)),
            IntoDateDifferenceTuple::UnitDateTime(unit, date_time) => {
                Self::from((unit.0, date_time.0))
            }
            IntoDateDifferenceTuple::UnitZoned(unit, zoned) => Self::from((unit.0, zoned.0)),
        }
    }
}
impl From<IntoDateDifference> for DateDifference {
    fn from(val: IntoDateDifference) -> Self {
        match val {
            IntoDateDifference::RyDateDifference(d_diff) => d_diff.0,
            IntoDateDifference::Zoned(zoned) => Self::from(zoned.0),
            IntoDateDifference::Date(date) => Self::from(date.0),
            IntoDateDifference::DateTime(date) => Self::from(date.0),
            IntoDateDifference::DateDifferenceTuple(tuple) => tuple.into(),
        }
    }
}

// ============================================================================
// Date/DateTime/Zoned
// ============================================================================

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum DateDifferenceArg {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}

impl DateDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> DateDifference {
        let mut diff = match self {
            Self::Zoned(zoned) => DateDifference::from(zoned.0),
            Self::Date(date) => DateDifference::from(date.0),
            Self::DateTime(date_time) => DateDifference::from(date_time.0),
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
