use crate::ry_timestamp::RyTimestamp;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit};
use jiff::TimestampDifference;
use pyo3::types::PyDict;
use pyo3::{intern, prelude::*};

#[derive(Clone, Copy, Debug, Default, PartialEq, Hash)]
pub(crate) struct DifferenceOptions {
    pub(crate) smallest: Option<JiffUnit>,
    pub(crate) largest: Option<JiffUnit>,
    pub(crate) mode: Option<JiffRoundMode>,
    pub(crate) increment: Option<i64>,
}

impl DifferenceOptions {
    pub(crate) fn new(
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        Self {
            smallest,
            largest,
            mode,
            increment,
        }
    }

    pub fn smallest(&self, unit: JiffUnit) -> Self {
        Self {
            smallest: Some(unit),
            ..*self
        }
    }

    pub fn largest(&self, unit: JiffUnit) -> Self {
        Self {
            largest: Some(unit),
            ..*self
        }
    }

    pub fn mode(&self, mode: JiffRoundMode) -> Self {
        Self {
            mode: Some(mode),
            ..*self
        }
    }

    pub fn increment(&self, increment: i64) -> Self {
        Self {
            increment: Some(increment),
            ..*self
        }
    }

    pub(crate) fn pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        if let Some(smallest) = self.smallest {
            dict.set_item(intern!(py, "smallest"), smallest)?;
        }
        if let Some(largest) = self.largest {
            dict.set_item(intern!(py, "largest"), largest)?;
        }
        if let Some(mode) = self.mode {
            dict.set_item(intern!(py, "mode"), mode)?;
        }
        if let Some(increment) = self.increment {
            dict.set_item(intern!(py, "increment"), increment)?;
        }
        Ok(dict)
    }

    pub(crate) fn timestamp_diff(&self, obj: &RyTimestamp) -> TimestampDifference {
        let mut d_diff = TimestampDifference::new(obj.0);
        if let Some(smallest) = self.smallest {
            d_diff = d_diff.smallest(smallest.0);
        }
        if let Some(largest) = self.largest {
            d_diff = d_diff.largest(largest.0);
        }
        if let Some(mode) = self.mode {
            d_diff = d_diff.mode(mode.0);
        }
        if let Some(increment) = self.increment {
            d_diff = d_diff.increment(increment);
        }
        d_diff
    }
}

// fn smallest(&self, unit: JiffUnit) -> Self {
//     RyTimestampDifference(self.0.smallest(unit.0))
// }

// fn largest(&self, unit: JiffUnit) -> Self {
//     RyTimestampDifference(self.0.largest(unit.0))
// }

// fn mode(&self, mode: JiffRoundMode) -> Self {
//     RyTimestampDifference(self.0.mode(mode.0))
// }

// fn increment(&self, increment: i64) -> Self {
//     RyTimestampDifference(self.0.increment(increment))
// }
