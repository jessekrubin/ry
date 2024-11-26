import pytest

import ry.dev as ry


def test_span_fn_no_positionals_allowed() -> None:
    with pytest.raises(TypeError):
        s = ry.timespan(1)  # type: ignore


def test_span_repr() -> None:
    s = ry.timespan(years=1)
    assert repr(s) == "Span(years=1)"
    _expected_repr_full = "Span(years=1, months=0, weeks=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0, microseconds=0, nanoseconds=0)"
    assert s.repr_full() == _expected_repr_full
