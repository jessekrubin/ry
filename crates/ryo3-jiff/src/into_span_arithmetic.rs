use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_zoned::RyZoned;
use jiff::SpanArithmetic;
use pyo3::prelude::*;
use ryo3_std::time::PyDuration;

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum SpanArithmeticTupleIx0 {
    Span(RySpan),
    Duration(PyDuration),
    SignedDuration(RySignedDuration),
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum SpanArithmeticTupleIx1 {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum IntoSpanArithmetic {
    Uno(SpanArithmeticTupleIx0),
    Dos((SpanArithmeticTupleIx0, SpanArithmeticTupleIx1)),
}

impl<'a> From<&'a IntoSpanArithmetic> for SpanArithmetic<'a> {
    fn from(value: &'a IntoSpanArithmetic) -> Self {
        // HERE WE HAVE A TOTAL CLUSTER-FUCK OF MATCHING...
        // BUT I AM NOT SURE HOW TO GET THIS TO PLAY NICE WITH PYTHON + LIFETIMES
        match value {
            IntoSpanArithmetic::Uno(s) => match s {
                SpanArithmeticTupleIx0::Span(sp) => SpanArithmetic::from(sp.0).days_are_24_hours(),
                SpanArithmeticTupleIx0::Duration(dur) => {
                    SpanArithmetic::from(dur.0).days_are_24_hours()
                }
                SpanArithmeticTupleIx0::SignedDuration(dur) => {
                    SpanArithmetic::from(dur.0).days_are_24_hours()
                }
            },
            IntoSpanArithmetic::Dos((s, r)) => match s {
                SpanArithmeticTupleIx0::Span(sp) => match r {
                    SpanArithmeticTupleIx1::Zoned(z) => SpanArithmetic::from((&sp.0, &z.0)),
                    SpanArithmeticTupleIx1::Date(d) => SpanArithmetic::from((sp.0, d.0)),
                    SpanArithmeticTupleIx1::DateTime(dt) => SpanArithmetic::from((sp.0, dt.0)),
                },
                SpanArithmeticTupleIx0::Duration(dur) => match r {
                    SpanArithmeticTupleIx1::Zoned(z) => SpanArithmetic::from((dur.0, &z.0)),
                    SpanArithmeticTupleIx1::Date(d) => SpanArithmetic::from((dur.0, d.0)),
                    SpanArithmeticTupleIx1::DateTime(dt) => SpanArithmetic::from((dur.0, dt.0)),
                },
                SpanArithmeticTupleIx0::SignedDuration(dur) => match r {
                    SpanArithmeticTupleIx1::Zoned(z) => SpanArithmetic::from((dur.0, &z.0)),
                    SpanArithmeticTupleIx1::Date(d) => SpanArithmetic::from((dur.0, d.0)),
                    SpanArithmeticTupleIx1::DateTime(dt) => SpanArithmetic::from((dur.0, dt.0)),
                },
            },
        }
    }
}
