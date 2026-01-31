use crate::ry_span::SpanKwargs;
use crate::spanish::Spanish;
use jiff::Span;
use pyo3::PyResult;
use ryo3_macro_rules::py_type_err;

pub(crate) fn add_or_kw<T, FOther, FSpan, FNone>(
    other: Option<Spanish>,
    kwargs: SpanKwargs,
    apply_other: FOther,
    apply_span: FSpan,
    apply_none: FNone,
) -> PyResult<T>
where
    FOther: FnOnce(Spanish) -> PyResult<T>,
    FSpan: FnOnce(Span) -> PyResult<T>,
    FNone: FnOnce() -> PyResult<T>,
{
    let any_kw = kwargs.is_zero();
    match (other, any_kw) {
        (Some(o), false) => apply_other(o),
        (None, true) => {
            let span = kwargs.build()?;
            apply_span(span)
        }
        (Some(_), true) => {
            py_type_err!("add accepts either a span-like object or keyword units, not both")
        }
        (None, false) => apply_none(),
    }
}
