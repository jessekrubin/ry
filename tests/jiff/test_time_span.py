from __future__ import annotations

import datetime as pydt

import pytest

import ry.dev as ry


def test_span_fn_no_positionals_allowed() -> None:
    with pytest.raises(TypeError):
        ry.timespan(1)  # type: ignore


def test_span_repr() -> None:
    s = ry.timespan(years=1)
    assert repr(s) == "TimeSpan(years=1)"
    _expected_repr_full = "TimeSpan(years=1, months=0, weeks=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0, microseconds=0, nanoseconds=0)"
    assert s.repr_full() == _expected_repr_full


def test_span_dict() -> None:
    s = ry.timespan(years=1)
    assert s.asdict() == {
        "years": 1,
        "months": 0,
        "weeks": 0,
        "days": 0,
        "hours": 0,
        "minutes": 0,
        "seconds": 0,
        "milliseconds": 0,
        "microseconds": 0,
        "nanoseconds": 0,
    }


def test_span_to_py_timedelta() -> None:
    s = ry.timespan(hours=1)
    py_timedelta = s.to_pytimedelta()
    assert isinstance(py_timedelta, pydt.timedelta)
    assert py_timedelta == pydt.timedelta(hours=1)


def test_negative_spans() -> None:
    """
    use jiff::{Span, ToSpan};

    let span = -Span::new().days(5);
    assert_eq!(span.to_string(), "-P5d");

    let span = Span::new().days(5).negate();
    assert_eq!(span.to_string(), "-P5d");

    let span = Span::new().days(-5);
    assert_eq!(span.to_string(), "-P5d");

    let span = -Span::new().days(-5).negate();
    assert_eq!(span.to_string(), "-P5d");

    let span = -5.days();
    assert_eq!(span.to_string(), "-P5d");

    let span = (-5).days();
    assert_eq!(span.to_string(), "-P5d");

    let span = -(5.days());
    assert_eq!(span.to_string(), "-P5d");
    """
    span = -ry.TimeSpan()._days(5)
    assert span.string() == "-P5d"

    span = ry.TimeSpan()._days(5).negate()
    assert span.string() == "-P5d"

    span = ry.TimeSpan()._days(-5)
    assert span.string() == "-P5d"

    span = -ry.TimeSpan()._days(-5).negate()
    assert span.string() == "-P5d"

    span = ry.TimeSpan()._days(-5)
    assert span.string() == "-P5d"


class TestSpanCheckedAdd:
    """From the checked_add doctests"""

    def test_checked_add_root(self) -> None:
        span1 = ry.timespan(days=2, hours=23)
        span2 = ry.timespan(hours=2)
        assert span1.checked_add(span2) == ry.timespan(days=3, hours=1)

    def test_example_rebalancing(self) -> None:
        span1 = ry.timespan(days=2, hours=23)
        span2 = ry.timespan(hours=2)
        assert span1.checked_add(span2) == ry.timespan(days=3, hours=1)

    def test_checked_add_with_relative_datetime(self) -> None:
        span1 = ry.timespan(months=1, days=15)
        span2 = ry.timespan(days=15)
        assert span1.checked_add((span2, ry.Date(2008, 3, 1))) == ry.timespan(months=2)

        span1 = ry.timespan(months=1, days=15)
        span2 = ry.timespan(days=15)
        assert span1.checked_add((span2, ry.Date(2008, 4, 1))) == ry.timespan(
            months=1, days=30
        )

    def test_adding_spans_with_calendar_units(self) -> None:
        span1 = ry.timespan(months=1, days=15)
        span2 = ry.timespan(days=15)
        with pytest.raises(OverflowError):
            span1.checked_add(span2)

    def test_adding_spans_with_calendar_units_with_relative_datetime(
        self,
    ) -> None:
        # with relative datetime
        span1 = ry.timespan(months=1, days=15)
        span2 = ry.timespan(days=15)
        assert span1.checked_add((span2, ry.Date(2008, 3, 1))) == ry.timespan(months=2)

        # but 1 month from April 1 is 30 days!
        span1 = ry.timespan(months=1, days=15)
        span2 = ry.timespan(days=15)
        assert span1.checked_add((span2, ry.Date(2008, 4, 1))) == ry.timespan(
            months=1, days=30
        )

    def test_error_on_overflow(self) -> None:
        with pytest.raises(OverflowError):
            ry.timespan(years=19_998).checked_add(ry.timespan(years=1))
