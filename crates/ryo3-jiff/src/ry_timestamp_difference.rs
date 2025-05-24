use crate::difference_options::DifferenceOptions;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::{Timestamp, TimestampDifference};
use pyo3::types::PyTuple;
use pyo3::{prelude::*, IntoPyObjectExt};

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "TimestampDifference", module = "ry.ryo3", frozen)]
pub struct RyTimestampDifference {
    obj: Timestamp,
    options: DifferenceOptions,
}

// impl From<TimestampDifference> for RyTimestampDifference {
//     fn from(value: TimestampDifference) -> Self {
//         RyTimestampDifference {
//             obj: value,
//             options: DifferenceOptions::default(),
//         }
//     }
// }

impl From<(Timestamp, DifferenceOptions)> for RyTimestampDifference {
    fn from(value: (Timestamp, DifferenceOptions)) -> Self {
        RyTimestampDifference {
            obj: value.0,
            options: value.1,
        }
    }
}

#[pymethods]
impl RyTimestampDifference {
    #[new]
    #[pyo3(
       signature = (obj, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    #[must_use]
    fn py_new(
        obj: &RyTimestamp,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let mut d_diff = TimestampDifference::new(obj.0);
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
        RyTimestampDifference {
            obj: obj.0,
            options: DifferenceOptions::new(smallest, largest, mode, increment),
        }
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::new(py, vec![RyTimestamp::from(self.obj)])?.into_py_any(py)?;
        let kwargs = self.options.pydict(py)?.into_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        (self.obj, self.options.smallest(unit)).into()
    }

    fn largest(&self, unit: JiffUnit) -> Self {
        (self.obj, self.options.largest(unit)).into()
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        (self.obj, self.options.mode(mode)).into()
    }

    fn increment(&self, increment: i64) -> Self {
        (self.obj, self.options.increment(increment)).into()
    }

    fn __eq__(&self, other: &Self) -> PyResult<bool> {
        Ok(self.obj == other.obj && self.options == other.options)
    }
}

impl From<&RyTimestampDifference> for TimestampDifference {
    fn from(value: &RyTimestampDifference) -> Self {
        value.options.timestamp_diff(&RyTimestamp(value.obj))
    }
}

#[derive(Debug, Clone, FromPyObject)]
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
pub(crate) enum TimestampDifferenceArg {
    Zoned(RyZoned),
    Timestamp(RyTimestamp),
}

impl TimestampDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> TimestampDifference {
        let mut diff = match self {
            TimestampDifferenceArg::Zoned(zoned) => TimestampDifference::from(zoned.0),
            TimestampDifferenceArg::Timestamp(date) => TimestampDifference::from(date.0),
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
