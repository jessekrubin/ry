use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use pyo3::FromPyObject;
use ryo3_std::PyDuration;

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyDeltaArithmeticSelf {
    Span(RySpan),
    SignedDuration(RySignedDuration),
    Duration(PyDuration),
}
