use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_zoned::RyZoned;
use jiff::SpanArithmetic;
use pyo3::prelude::*;
use ryo3_std::PyDuration;

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

impl From<IntoSpanArithmetic> for SpanArithmetic<'_> {
    fn from<'b>(value: IntoSpanArithmetic) -> Self {
        // HERE WE HAVE A TOTAL CLUSTERFUCK OF MATCHING...
        // BUT I AM NOT SURE HOW TO GET THIS TO PLAY NICE WITH PYTHON + LIFETIMES
        match value {
            IntoSpanArithmetic::Uno(s) => match s {
                SpanArithmeticTupleIx0::Span(sp) => SpanArithmetic::from(sp.0),
                SpanArithmeticTupleIx0::Duration(dur) => SpanArithmetic::from(dur.0),
                SpanArithmeticTupleIx0::SignedDuration(dur) => SpanArithmetic::from(dur.0),
            },
            IntoSpanArithmetic::Dos((s, r)) => match s {
                SpanArithmeticTupleIx0::Span(sp) => match r {
                    // TODO: figure out if this is bad........
                    SpanArithmeticTupleIx1::Zoned(z) => {
                        SpanArithmetic::from((sp.0, z.0.datetime()))
                    }
                    SpanArithmeticTupleIx1::Date(d) => SpanArithmetic::from((sp.0, d.0)),
                    SpanArithmeticTupleIx1::DateTime(dt) => SpanArithmetic::from((sp.0, dt.0)),
                },
                SpanArithmeticTupleIx0::Duration(dur) => match r {
                    // TODO: figure out if this is bad........
                    SpanArithmeticTupleIx1::Zoned(z) => {
                        SpanArithmetic::from((dur.0, z.0.datetime()))
                    }
                    SpanArithmeticTupleIx1::Date(d) => SpanArithmetic::from((dur.0, d.0)),
                    SpanArithmeticTupleIx1::DateTime(dt) => SpanArithmetic::from((dur.0, dt.0)),
                },
                SpanArithmeticTupleIx0::SignedDuration(dur) => match r {
                    // TODO: figure out if this is bad........
                    SpanArithmeticTupleIx1::Zoned(z) => {
                        SpanArithmetic::from((dur.0, z.0.datetime()))
                    }
                    SpanArithmeticTupleIx1::Date(d) => SpanArithmetic::from((dur.0, d.0)),
                    SpanArithmeticTupleIx1::DateTime(dt) => SpanArithmetic::from((dur.0, dt.0)),
                },
            },
        }
    }
}
