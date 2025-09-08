use crate::{JiffRoundMode, JiffUnit, RyDateTime, RySignedDuration, RyTime, RyTimestamp, RyZoned};
use jiff::civil::{DateTimeRound, TimeRound};
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
}

macro_rules! impl_from_round_options_for {
    ($type:ty) => {
        impl From<&RoundOptions> for $type {
            fn from(value: &RoundOptions) -> Self {
                Self::new()
                    .smallest(value.smallest.0)
                    .mode(value.mode.0)
                    .increment(value.increment)
            }
        }
    };
}

impl_from_round_options_for!(DateTimeRound);
impl_from_round_options_for!(SignedDurationRound);
impl_from_round_options_for!(TimeRound);
impl_from_round_options_for!(TimestampRound);
impl_from_round_options_for!(ZonedRound);

// ----------------------------------------------------------------------------
// DateTimeRound
// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
#[pyclass(name = "DateTimeRound", module = "ry.ryo3", frozen)]
pub struct RyDateTimeRound {
    pub(crate) options: RoundOptions,
    pub(crate) jiff_round: DateTimeRound,
}

impl From<RoundOptions> for RyDateTimeRound {
    fn from(options: RoundOptions) -> Self {
        Self {
            options,
            jiff_round: (&options).into(),
        }
    }
}

#[pymethods]
impl RyDateTimeRound {
    #[new]
    #[pyo3(signature = (smallest=None, *, mode=None, increment=1))]
    fn py_new(smallest: Option<JiffUnit>, mode: Option<JiffRoundMode>, increment: i64) -> Self {
        Self::from(RoundOptions::new(
            smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond)),
            mode.unwrap_or(JiffRoundMode(jiff::RoundMode::HalfExpand)),
            increment,
        ))
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
        Self::from(options)
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn _increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    pub(crate) fn round(&self, ob: &RyDateTime) -> PyResult<RyDateTime> {
        ob.0.round(self.jiff_round)
            .map(RyDateTime::from)
            .map_err(|e| py_value_error!("Error rounding DateTime: {}", e))
    }
}

impl Display for RyDateTimeRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DateTimeRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.options.smallest, self.options.mode, self.options.increment
        )
    }
}

// ----------------------------------------------------------------------------
// SignedDurationRound
// ----------------------------------------------------------------------------
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
        Self {
            options,
            jiff_round: (&options).into(),
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
        Self {
            options,
            jiff_round: (&options).into(),
        }
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn _increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
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

// ----------------------------------------------------------------------------
// TimeRound
// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
#[pyclass(name = "TimeRound", module = "ry.ryo3", frozen)]
pub struct RyTimeRound {
    options: RoundOptions,
    pub(crate) jiff_round: TimeRound,
}

#[pymethods]
impl RyTimeRound {
    #[new]
    #[pyo3(signature = (smallest=None, *, mode=None, increment=1))]
    fn py_new(smallest: Option<JiffUnit>, mode: Option<JiffRoundMode>, increment: i64) -> Self {
        let options = RoundOptions::new(
            smallest.unwrap_or(JiffUnit(Unit::Nanosecond)),
            mode.unwrap_or(JiffRoundMode(RoundMode::HalfExpand)),
            increment,
        );
        Self {
            options,
            jiff_round: (&options).into(),
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
        Self {
            options,
            jiff_round: (&options).into(),
        }
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn _increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    pub(crate) fn round(&self, ob: &RyTime) -> PyResult<RyTime> {
        ob.0.round(self.jiff_round)
            .map(RyTime::from)
            .map_err(|e| py_value_error!("Error rounding Time: {}", e))
    }
}

impl Display for RyTimeRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimeRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.options.smallest, self.options.mode, self.options.increment
        )
    }
}

// ----------------------------------------------------------------------------
// TimestampRound
// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
#[pyclass(name = "TimestampRound", module = "ry.ryo3", frozen)]
pub struct RyTimestampRound {
    options: RoundOptions,
    pub(crate) jiff_round: TimestampRound,
}

impl From<RoundOptions> for RyTimestampRound {
    fn from(options: RoundOptions) -> Self {
        Self {
            options,
            jiff_round: (&options).into(),
        }
    }
}

#[pymethods]
impl RyTimestampRound {
    #[new]
    #[pyo3(signature = (smallest=None, *, mode=None, increment=1))]
    fn py_new(smallest: Option<JiffUnit>, mode: Option<JiffRoundMode>, increment: i64) -> Self {
        Self::from(RoundOptions::new(
            smallest.unwrap_or(JiffUnit(Unit::Nanosecond)),
            mode.unwrap_or(JiffRoundMode(RoundMode::HalfExpand)),
            increment,
        ))
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
        Self::from(options)
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn _increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    pub(crate) fn round(&self, ob: &RyTimestamp) -> PyResult<RyTimestamp> {
        ob.0.round(self.jiff_round)
            .map(RyTimestamp::from)
            .map_err(|e| py_value_error!("Error rounding Timestamp: {}", e))
    }
}

impl Display for RyTimestampRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimestampRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.options.smallest, self.options.mode, self.options.increment
        )
    }
}

// ---------------------------------------------------------------------------
// ZonedDateTimeRound
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
#[pyclass(name = "ZonedDateTimeRound", module = "ry.ryo3", frozen)]
pub struct RyZonedDateTimeRound {
    options: RoundOptions,
    pub(crate) jiff_round: ZonedRound,
}

impl From<RoundOptions> for RyZonedDateTimeRound {
    fn from(options: RoundOptions) -> Self {
        Self {
            options,
            jiff_round: (&options).into(),
        }
    }
}

#[pymethods]
impl RyZonedDateTimeRound {
    #[new]
    #[pyo3(signature = (smallest=None, *, mode=None, increment=1))]
    fn py_new(smallest: Option<JiffUnit>, mode: Option<JiffRoundMode>, increment: i64) -> Self {
        Self::from(RoundOptions::new(
            smallest.unwrap_or(JiffUnit(Unit::Nanosecond)),
            mode.unwrap_or(JiffRoundMode(RoundMode::HalfExpand)),
            increment,
        ))
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
        Self::from(options)
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn _increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    pub(crate) fn round(&self, ob: &RyZoned) -> PyResult<RyZoned> {
        ob.0.round(self.jiff_round)
            .map(RyZoned::from)
            .map_err(|e| py_value_error!("Error rounding ZonedDateTime: {}", e))
    }
}

impl Display for RyZonedDateTimeRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ZonedDateTimeRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.options.smallest, self.options.mode, self.options.increment
        )
    }
}
