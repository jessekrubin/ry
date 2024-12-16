use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::DateTimeDifference;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "DateTimeDifference", module = "ryo3")]
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
    pub fn py_new(
        datetime: RyDateTime,
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
pub enum IntoDateTimeDifferenceTuple {
    UnitDate(JiffUnit, RyDate),
    UnitDateTime(JiffUnit, RyDateTime),
    UnitZoned(JiffUnit, RyZoned),
}

#[derive(Debug, Clone, FromPyObject)]
pub enum IntoDateTimeDifference {
    RyDateTimeDifference(RyDateTimeDifference),
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
    IntoDateTimeDifferenceTuple(IntoDateTimeDifferenceTuple),
}

impl Into<DateTimeDifference> for IntoDateTimeDifferenceTuple {
    fn into(self) -> DateTimeDifference {
        match self {
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
impl Into<DateTimeDifference> for IntoDateTimeDifference {
    fn into(self) -> DateTimeDifference {
        match self {
            IntoDateTimeDifference::RyDateTimeDifference(d_diff) => d_diff.0,
            IntoDateTimeDifference::Zoned(zoned) => DateTimeDifference::from(zoned.0),
            IntoDateTimeDifference::Date(date) => DateTimeDifference::from(date.0),
            IntoDateTimeDifference::DateTime(date) => DateTimeDifference::from(date.0),
            IntoDateTimeDifference::IntoDateTimeDifferenceTuple(tuple) => tuple.into(),
        }
    }
}
