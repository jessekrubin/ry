from __future__ import annotations

import datetime as pydt
import functools

import pytest

import ry.dev as ry


def test_span_fn_no_positionals_allowed() -> None:
    with pytest.raises(TypeError):
        ry.timespan(1)  # type: ignore


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


class TestTimeSpanStrings:
    def test_span_repr(self) -> None:
        s = ry.timespan(years=1)
        assert repr(s) == "TimeSpan(years=1)"
        _expected_repr_full = "TimeSpan(years=1, months=0, weeks=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0, microseconds=0, nanoseconds=0)"
        assert s.repr_full() == _expected_repr_full

    def test_span_str(self) -> None:
        s = ry.timespan(years=1)
        assert str(s) == "P1Y"

    def test_span_str_human(self) -> None:
        s = ry.TimeSpan.parse("P2M10DT2H30M")
        assert s.string(human=True) == "2mo 10d 2h 30m"
        assert s.string(True) == "2mo 10d 2h 30m"

    def test_span_str_alien_or_idk_but_not_human(self) -> None:
        s = ry.TimeSpan.parse("P2M10DT2H30M")
        assert s.string(human=False) == "P2M10DT2H30M"
        assert s.string() == "P2M10DT2H30M"


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
    assert span.string() == "-P5D"

    span = ry.TimeSpan()._days(5).negate()
    assert span.string() == "-P5D"

    span = ry.TimeSpan()._days(-5)
    assert span.string() == "-P5D"

    span = -ry.TimeSpan()._days(-5).negate()
    assert span.string() == "-P5D"

    span = ry.TimeSpan()._days(-5)
    assert span.string() == "-P5D"


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


class TestSpanCompare:
    def test_example(self) -> None:
        """
        ```rust
        use jiff::ToSpan;

        let span1 = 3.hours();
        let span2 = 180.minutes();
        assert_eq!(span1.compare(span2)?, std::cmp::Ordering::Equal);
        // But notice that the two spans are not equal via `Eq`:
        assert_ne!(span1.fieldwise(), span2.fieldwise());
        ```
        """
        span1 = ry.timespan(hours=3)
        span2 = ry.timespan(minutes=180)
        assert span1.compare(span2) == 0

    def test_example_negative_spans_are_less_than_zero(self) -> None:
        """
        ```rust
        use jiff::ToSpan;

        let span1 = -1.second();
        let span2 = 0.seconds();
        assert_eq!(span1.compare(span2)?, std::cmp::Ordering::Less);
        ```
        """
        span1 = -ry.timespan(seconds=1)
        span2 = ry.timespan(seconds=0)
        assert span1.compare(span2) == -1

    def test_example_comparisons_take_DST_into_account(self) -> None:  # noqa: N802
        """
        ```rust
        use jiff::{ToSpan, Zoned};

        let span1 = 79.hours().minutes(10);
        let span2 = 3.days().hours(7).seconds(630);
        let span3 = 3.days().hours(6).minutes(50);

        let relative: Zoned = "2020-11-01T00-07[America/Los_Angeles]".parse()?;
        let mut spans = [span1, span2, span3];
        spans.sort_by(|s1, s2| s1.compare((s2, &relative)).unwrap());
        assert_eq!(
            spans.map(|sp| sp.fieldwise()),
            [span1.fieldwise(), span3.fieldwise(), span2.fieldwise()],
        );

        // Compare with the result of sorting without taking DST into account.
        // We can do that here since days are considered 24 hours long in all
        // cases when no relative datetime is provided:
        spans.sort_by(|s1, s2| s1.compare(s2).unwrap());
        assert_eq!(
            spans.map(|sp| sp.fieldwise()),
            [span3.fieldwise(), span1.fieldwise(), span2.fieldwise()],
        );
        ```
        """

        span1 = ry.timespan(hours=79, minutes=10)
        span2 = ry.timespan(days=3, hours=7, seconds=630)
        span3 = ry.timespan(days=3, hours=6, minutes=50)
        relative = ry.ZonedDateTime.parse("2020-11-01T00-07[America/Los_Angeles]")

        def _compare_key_relative(a: ry.TimeSpan, b: ry.TimeSpan) -> int:
            return a.compare(b, relative=relative)

        spans = [span1, span2, span3]
        spans_sorted = sorted(spans, key=functools.cmp_to_key(_compare_key_relative))
        assert spans_sorted == [span1, span3, span2]

        # Compare with the result of sorting without taking DST into account.
        # We can do that here since days are considered 24 hours long in all
        # cases when no relative datetime is provided:
        def _compare_key_no_relative(a: ry.TimeSpan, b: ry.TimeSpan) -> int:
            return a.compare(b, days_are_24_hours=True)

        spans.sort(key=lambda s: s.compare(s, days_are_24_hours=True))
        spans_sorted_no_dst = sorted(
            spans, key=functools.cmp_to_key(_compare_key_no_relative)
        )
        assert spans_sorted_no_dst == [span3, span1, span2]


class TestSpanTotal:
    def test_example(self) -> None:
        """
        ```rust
        use jiff::{ToSpan, Unit};

        let span = 3.hours().minutes(10);
        assert_eq!(span.total(Unit::Second)?, 11_400.0);
        ```
        """
        span = ry.timespan(hours=3, minutes=10)
        assert span.total("second") == 11_400.0

    def test_example_24_hour_days(self) -> None:
        """
        ```rust
        use jiff::{ToSpan, Unit};

        let span = 123_456_789.seconds();
        assert_eq!(span.total(Unit::Day)?, 1428.8980208333332);
        ```
        """
        span = ry.timespan(seconds=123_456_789)
        assert span.total("day", days_are_24_hours=True) == 1428.8980208333332

    def test_example_DST_is_taken_into_account(self) -> None:  # noqa: N802
        """
        ```rust
        use jiff::{civil::date, ToSpan, Unit};

        let span = 744.hours();
        let relative = date(2024, 3, 1).in_tz("America/New_York")?;
        // Because of the short day, 744 hours is actually a little *more* than
        // 1 month starting from 2024-03-01.
        assert_eq!(span.total((Unit::Month, &relative))?, 1.0013888888888889);
        ```
        """
        span = ry.timespan(hours=744)
        relative = ry.Date(2024, 3, 1).in_tz("America/New_York")
        assert span.total("month", relative) == 1.0013888888888889

    def test_example_infallible_sorting(self) -> None:
        span1 = ry.timespan(hours=79, minutes=10)
        span2 = ry.timespan(days=3, hours=7, seconds=630)
        span3 = ry.timespan(days=3, hours=6, minutes=50)
        relative = ry.ZonedDateTime.parse("2020-11-01T00-07[America/Los_Angeles]")
        spans = [
            (span1, span1.total("day", relative)),
            (span2, span2.total("day", relative)),
            (span3, span3.total("day", relative)),
        ]
        spans_sorted = sorted(spans, key=lambda x: x[1])
        assert [x[0] for x in spans_sorted] == [span1, span3, span2]
        spans = [
            (span1, span1.total("day", days_are_24_hours=True)),
            (span2, span2.total("day", days_are_24_hours=True)),
            (span3, span3.total("day", days_are_24_hours=True)),
        ]
        spans_sorted_no_dst = sorted(spans, key=lambda x: x[1])
        assert [x[0] for x in spans_sorted_no_dst] == [span3, span1, span2]
