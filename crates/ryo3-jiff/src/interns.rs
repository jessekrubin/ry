//! String interns that are obviously unpaid and over worked...
//!
//! classic millenials... nobody wants to be a string intern anymore...
//!
//! This keeps the string-interning in one place so we dont have multiple
//! pydantic based interns who are milling around, presumably not getting
//! me the coffee I asked for.
use pyo3::prelude::*;

macro_rules! unpaid_intern {
    ($name:ident, $lit:literal) => {
        pub(crate) fn $name(py: Python<'_>) -> &Bound<'_, pyo3::types::PyString> {
            pyo3::intern!(py, $lit)
        }
    };

    ($name:ident) => {
        pub(crate) fn $name(py: Python<'_>) -> &Bound<'_, pyo3::types::PyString> {
            pyo3::intern!(py, stringify!($name))
        }
    };
}

// singular duration(s)
unpaid_intern!(year);
unpaid_intern!(month);
unpaid_intern!(week);
unpaid_intern!(day);
unpaid_intern!(hour);
unpaid_intern!(minute);
unpaid_intern!(second);
unpaid_intern!(millisecond);
unpaid_intern!(microsecond);
unpaid_intern!(nanosecond);

// plural duration(s)
unpaid_intern!(years);
unpaid_intern!(months);
unpaid_intern!(weeks);
unpaid_intern!(days);
unpaid_intern!(hours);
unpaid_intern!(minutes);
unpaid_intern!(seconds);
unpaid_intern!(milliseconds);
unpaid_intern!(microseconds);
unpaid_intern!(nanoseconds);

// tz / offset
unpaid_intern!(tz);
unpaid_intern!(fmt);

// weekday
unpaid_intern!(weekday);

// signed duration
unpaid_intern!(secs);
unpaid_intern!(nanos);

// round / difference related
unpaid_intern!(smallest);
unpaid_intern!(largest);
unpaid_intern!(mode);
unpaid_intern!(increment);

// era
unpaid_intern!(bce, "BCE");
unpaid_intern!(ce, "CE");

// round-mode
unpaid_intern!(ceil);
unpaid_intern!(floor);
unpaid_intern!(expand);
unpaid_intern!(trunc);
unpaid_intern!(half_ceil, "half-ceil");
unpaid_intern!(half_floor, "half-floor");
unpaid_intern!(half_expand, "half-expand");
unpaid_intern!(half_trunc, "half-trunc");
unpaid_intern!(half_even, "half-even");
unpaid_intern!(unknown);
