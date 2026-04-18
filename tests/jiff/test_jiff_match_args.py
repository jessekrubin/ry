from __future__ import annotations

import ry

_JIFF_OBJECTS_2_MATCH_ARGS = [
    # date
    (ry.Date, ("year", "month", "day")),
    # time
    (ry.Time, ("hour", "minute", "second", "subsec_nanosecond")),
    # datetime
    (
        ry.DateTime,
        ("year", "month", "day", "hour", "minute", "second", "subsec_nanosecond"),
    ),
    # zoned
    (
        ry.ZonedDateTime,
        ("year", "month", "day", "hour", "minute", "second", "subsec_nanosecond"),
    ),
    # signed-duration
    (ry.SignedDuration, ("secs", "nanos")),
    # iso-week-date
    (ry.ISOWeekDate, ("year", "week", "weekday")),
    # offset; TBD
]


def test_match_args_exist() -> None:
    """Test that __match_args__ is defined for all jiff types."""
    for jiff_type, expected_args in _JIFF_OBJECTS_2_MATCH_ARGS:
        assert hasattr(jiff_type, "__match_args__"), (
            f"{jiff_type.__name__} missing __match_args__"
        )
        assert jiff_type.__match_args__ == expected_args, (
            f"{jiff_type.__name__}.__match_args__ = {jiff_type.__match_args__}, "
            f"expected {expected_args}"
        )


def test_date_positional_pattern() -> None:
    """Test positional pattern matching on Date."""
    d = ry.Date(2025, 3, 15)
    match d:
        case ry.Date(year, month, day):
            assert year == 2025
            assert month == 3
            assert day == 15
        case _:
            ry.unreachable()


def test_time_positional_pattern() -> None:
    """Test positional pattern matching on Time."""
    t = ry.Time(14, 30, 45, 123456789)
    match t:
        case ry.Time(hour, minute, second, subsec_nanosecond):
            assert hour == 14
            assert minute == 30
            assert second == 45
            assert subsec_nanosecond == 123456789
        case _:
            ry.unreachable()


def test_datetime_positional_pattern() -> None:
    """Test positional pattern matching on DateTime."""
    dt = ry.DateTime(2025, 3, 15, 14, 30, 45, 987654321)
    match dt:
        case ry.DateTime(year, month, day, hour, minute, second, subsec_nanosecond):
            assert year == 2025
            assert month == 3
            assert day == 15
            assert hour == 14
            assert minute == 30
            assert second == 45
            assert subsec_nanosecond == 987654321
        case _:
            ry.unreachable()


def test_signed_duration_positional_pattern() -> None:
    """Test positional pattern matching on SignedDuration."""
    dur = ry.SignedDuration(3600, 500000000)
    match dur:
        case ry.SignedDuration(s, n):
            assert s == 3600
            assert n == 500000000
        case _:
            ry.unreachable()


def test_iso_week_date_positional_pattern() -> None:
    """Test positional pattern matching on ISOWeekDate."""
    iwd = ry.ISOWeekDate(2025, 11, 1)
    match iwd:
        case ry.ISOWeekDate(y, w, weekday):
            assert y == 2025
            assert w == 11
            assert weekday == 1
        case _:
            ry.unreachable()
