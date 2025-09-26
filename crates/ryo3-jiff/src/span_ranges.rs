//! Span Ranges
//!
//! | Unit           | Minimum Value                | Maximum Value               |
//! | -------------- | ---------------------------- | --------------------------- |
//! | `years`        | `-19_998`                    | `19_998`                    |
//! | `months`       | `-239_976`                   | `239_976`                   |
//! | `weeks`        | `-1_043_497`                 | `1_043_497`                 |
//! | `days`         | `-7_304_484`                 | `7_304_484`                 |
//! | `hours`        | `-175_307_616`               | `175_307_616`               |
//! | `minutes`      | `-10_518_456_960`            | `10_518_456_960`            |
//! | `seconds`      | `-631_107_417_600`           | `631_107_417_600`           |
//! | `milliseconds` | `-631_107_417_600_000`       | `631_107_417_600_000`       |
//! | `microseconds` | `-631_107_417_600_000_000`   | `631_107_417_600_000_000`   |
//! | `nanoseconds`  | `-9_223_372_036_854_775_807` | `9_223_372_036_854_775_807` |

use pyo3::prelude::*;
// i32::MIN is -2_147_483_648
// i32::MAX is  2_147_483_647

// goingt o macro this, but going to just do it manually for one unit first
// to make sure I get it right

// jiff errormsg looks like:  years: parameter 'years' with value -9223372036854775808 is not in the required range of -19998..=19998
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SpanYears(pub(crate) i64);

impl SpanYears {
    pub(crate) const MIN: i64 = -19_998;
    pub(crate) const MAX: i64 = 19_998;

    pub(crate) fn new(value: i64) -> PyResult<Self> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Self::py_error())
        }
    }

    pub(crate) fn py_error() -> PyErr {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "years: parameter 'years' must be an integer in the required range of -19_998..=19_998",
        )
    }
}

impl From<SpanYears> for i64 {
    fn from(value: SpanYears) -> Self {
        value.0
    }
}

impl FromPyObject<'_> for SpanYears {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract::<i64>().and_then(Self::new)
    }
}
