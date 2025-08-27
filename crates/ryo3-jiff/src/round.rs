use crate::{JiffRoundMode, JiffUnit, RySignedDuration};
use jiff::civil::DateTimeRound;
use jiff::{RoundMode, SignedDurationRound, TimestampRound, Unit, ZonedRound};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObjectExt, intern};
use ryo3_macro_rules::py_value_error;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RoundOptions {
    pub(crate) smallest: JiffUnit,
    pub(crate) mode: JiffRoundMode,
    pub(crate) increment: i64,
}

impl RoundOptions {
    pub(crate) fn new(smallest: JiffUnit, mode: JiffRoundMode, increment: i64) -> Self {
        Self {
            smallest,
            mode,
            increment,
        }
    }

    pub(crate) fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item(intern!(py, "smallest"), self.smallest)?;
        d.set_item(intern!(py, "mode"), self.mode)?;
        d.set_item(intern!(py, "increment"), self.increment)?;
        Ok(d)
    }

    pub(crate) fn datetime_round(&self) -> DateTimeRound {
        DateTimeRound::new()
            .smallest(self.smallest.0)
            .mode(self.mode.0)
            .increment(self.increment)
    }

    pub(crate) fn signed_duration_round(&self) -> SignedDurationRound {
        SignedDurationRound::new()
            .smallest(self.smallest.0)
            .mode(self.mode.0)
            .increment(self.increment)
    }

    pub(crate) fn timestamp_round(&self) -> TimestampRound {
        TimestampRound::new()
            .smallest(self.smallest.0)
            .mode(self.mode.0)
            .increment(self.increment)
    }

    pub(crate) fn zoned_round(&self) -> ZonedRound {
        ZonedRound::new()
            .smallest(self.smallest.0)
            .mode(self.mode.0)
            .increment(self.increment)
    }
}

#[derive(Clone, Copy, Debug)]
#[pyclass(name = "SignedDurationRound", module = "ry.ryo3", frozen)]
pub struct RySignedDurationRound {
    options: RoundOptions,
    pub(crate) jiff_round: SignedDurationRound,
}

#[pymethods]
impl RySignedDurationRound {
    #[new]
    #[pyo3(signature = (smallest=None, *, mode=None, increment=1))]
    fn py_new(smallest: Option<JiffUnit>, mode: Option<JiffRoundMode>, increment: i64) -> Self {
        let options = RoundOptions::new(
            smallest.unwrap_or(JiffUnit(Unit::Nanosecond)),
            mode.unwrap_or(JiffRoundMode(RoundMode::HalfExpand)),
            increment,
        );
        let jiff_round = options.signed_duration_round();
        Self {
            options,
            jiff_round,
        }
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.options == other.options
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.options.as_pydict(py)
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    #[pyo3(signature = (smallest=None, mode=None, increment=None))]
    fn replace(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let options = RoundOptions::new(
            smallest.unwrap_or(self.options.smallest),
            mode.unwrap_or(self.options.mode),
            increment.unwrap_or(self.options.increment),
        );
        let jiff_round = options.signed_duration_round();
        Self {
            options,
            jiff_round,
        }
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    fn _smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    fn _mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    fn _increment(&self) -> i64 {
        self.options.increment
    }

    fn round(&self, ob: &RySignedDuration) -> PyResult<RySignedDuration> {
        ob.0.round(self.jiff_round)
            .map(RySignedDuration::from)
            .map_err(|e| py_value_error!("Error rounding SignedDuration: {}", e))
    }
}

impl Display for RySignedDurationRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SignedDurationRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.options.smallest, self.options.mode, self.options.increment
        )
    }
}
