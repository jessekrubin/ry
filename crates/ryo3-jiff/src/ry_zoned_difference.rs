use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::Unit;
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(name = "ZonedDateTimeDifference", module = "ryo3")]
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
    pub fn py_new(
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
//
// impl RyZonedDifference {
//     fn into_zoned_difference(self) -> ZonedDifference<'static> {
//         let z = self.zoned.0;
//         let mut diff = ZonedDifference::new(&z);
//         if let Some(smallest) = self.smallest {
//             diff = diff.smallest(smallest);
//         }
//         if let Some(largest) = self.largest {
//             diff = diff.largest(largest);
//         }
//         if let Some(mode) = self.mode {
//             diff = diff.mode(mode.0);
//         }
//         if let Some(increment) = self.increment {
//             diff = diff.increment(increment);
//         }
//         diff
//     }
// }
//
// #[derive(Debug, Clone, FromPyObject)]
// pub enum IntoZonedDifferenceTuple {
//     UnitZoned(JiffUnit, RyZoned),
// }
//
// #[derive(Debug, Clone, FromPyObject)]
// pub enum IntoZonedDifference {
//     RyDateTimeDifference(RyZonedDifference),
//     Zoned(RyZoned),
//     UnitZoned(JiffUnit, RyZoned),
// }
//
// impl Into<ZonedDifference<'static>> for IntoZonedDifference {
//     fn into(self) -> ZonedDifference<'static> {
//         match self {
//             IntoZonedDifference::RyDateTimeDifference(diff) => diff.into_zoned_difference(),
//             IntoZonedDifference::Zoned(zoned) => {
//                 todo!()
//                 // ZonedDifference::from(zoned.0)
//             }
//
//             IntoZonedDifference::UnitZoned(unit, zoned) => {
//                 todo!()
//
//                 // ZonedDifference::from((unit.0, zoned.0))
//             }
//         }
//     }
// }
