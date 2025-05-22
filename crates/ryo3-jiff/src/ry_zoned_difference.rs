use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::Unit;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "ZonedDateTimeDifference", module = "ry.ryo3", frozen)]
pub struct RyZonedDifference {
    zoned: RyZoned,
    smallest: Option<Unit>,
    largest: Option<Unit>,
    mode: Option<JiffRoundMode>,
    increment: Option<i64>,
}

#[pymethods]
impl RyZonedDifference {
    #[new]
    #[pyo3(
       signature = (zoned_datetime, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    fn py_new(
        zoned_datetime: RyZoned,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        Self {
            zoned: zoned_datetime,
            smallest: smallest.map(|unit| unit.0),
            largest: largest.map(|unit| unit.0),
            mode,
            increment,
        }
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        Self {
            zoned: self.zoned.clone(),
            smallest: Some(unit.0),
            largest: self.largest,
            mode: self.mode,
            increment: self.increment,
        }
    }

    fn largest(&self, unit: JiffUnit) -> Self {
        Self {
            zoned: self.zoned.clone(),
            smallest: self.smallest,
            largest: Some(unit.0),
            mode: self.mode,
            increment: self.increment,
        }
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        Self {
            zoned: self.zoned.clone(),
            smallest: self.smallest,
            largest: self.largest,
            mode: Some(mode),
            increment: self.increment,
        }
    }

    fn increment(&self, increment: i64) -> Self {
        Self {
            zoned: self.zoned.clone(),
            smallest: self.smallest,
            largest: self.largest,
            mode: self.mode,
            increment: Some(increment),
        }
    }
}
