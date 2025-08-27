use std::fmt::Display;

use crate::round::RoundOptions;
use crate::{JiffRoundMode, JiffUnit, RyDateTime};
use jiff::civil::DateTimeRound;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use ryo3_macro_rules::py_value_error;

#[derive(Clone, Copy, Debug)]
#[pyclass(name = "DateTimeRound", module = "ry.ryo3", frozen)]
pub struct RyDateTimeRound {
    pub(crate) options: RoundOptions,
    pub(crate) jiff_round: DateTimeRound,
}
impl From<RoundOptions> for RyDateTimeRound {
    fn from(options: RoundOptions) -> Self {
        let jiff_round = options.datetime_round();
        Self {
            options,
            jiff_round,
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

// ============================================================================
// #[derive(Debug, Clone)]
// #[pyclass(name = "DateTimeRound", module = "ry.ryo3", frozen)]
// pub struct RyDateTimeRound {
//     pub smallest: JiffUnit,
//     pub mode: JiffRoundMode,
//     pub increment: i64,
//     // internal
//     pub round: DateTimeRound,
// }
//
// #[pymethods]
// impl RyDateTimeRound {
//     #[new]
//     #[pyo3(signature = (smallest=None, *, mode=None, increment=1))]
//     fn py_new(smallest: Option<JiffUnit>, mode: Option<JiffRoundMode>, increment: i64) -> Self {
//         let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
//         let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::HalfExpand));
//         let round = DateTimeRound::new()
//             .smallest(smallest.0)
//             .mode(mode.0)
//             .increment(increment);
//         Self {
//             smallest,
//             mode,
//             increment,
//             round,
//         }
//     }
//
//     fn __str__(&self) -> String {
//         self.__repr__()
//     }
//
//     fn __repr__(&self) -> String {
//         format!("{self}")
//     }
//
//     fn __eq__(&self, other: &Self) -> bool {
//         self.mode == other.mode
//             && self.smallest == other.smallest
//             && self.increment == other.increment
//     }
//
//     #[pyo3(signature = (smallest=None, mode=None, increment=None))]
//     fn replace(
//         &self,
//         smallest: Option<JiffUnit>,
//         mode: Option<JiffRoundMode>,
//         increment: Option<i64>,
//     ) -> Self {
//         let smallest = smallest.unwrap_or(self.smallest);
//         let mode = mode.unwrap_or(self.mode);
//         let increment = increment.unwrap_or(self.increment);
//         let round = DateTimeRound::new()
//             .smallest(smallest.0)
//             .mode(mode.0)
//             .increment(increment);
//         Self {
//             smallest,
//             mode,
//             increment,
//             round,
//         }
//     }
//
//     fn smallest(&self, unit: JiffUnit) -> Self {
//         self.replace(Some(unit), None, None)
//     }
//
//     fn mode(&self, mode: JiffRoundMode) -> Self {
//         self.replace(None, Some(mode), None)
//     }
//
//     fn increment(&self, increment: i64) -> Self {
//         self.replace(None, None, Some(increment))
//     }
//
//     fn _smallest(&self) -> JiffUnit {
//         self.smallest
//     }
//
//     fn _mode(&self) -> JiffRoundMode {
//         self.mode
//     }
//
//     fn _increment(&self) -> i64 {
//         self.increment
//     }
// }
//
// impl Display for RyDateTimeRound {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "DateTimeRound(smallest=\"{}\", mode=\"{}\", increment={})",
//             self.smallest, self.mode, self.increment
//         )
//     }
// }
