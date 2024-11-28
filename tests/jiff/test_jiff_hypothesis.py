from __future__ import annotations

import datetime as pydatetime

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry.dev as ry

# date_strategy = st.dates(min_value=date(1, 1, 1), max_value=date(9999, 12, 31)).map(
#     lambda dt: ry.date(dt.year, dt.month, dt.day)
# )
#
# time_strategy = st.times().map(
#     lambda t: ry.time(t.hour, t.minute, t.second, t.microsecond * 1000)
# )
#
# datetime_strategy = st.datetimes(
#     min_value=datetime(1, 1, 1, 0, 0, 0),
#     max_value=datetime(9999, 12, 31, 23, 59, 59, 999999),
# ).map(
#     lambda dt: ry.datetime(
#         dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second, dt.microsecond * 1000
#     )
# )

# Define strategies for generating test data
date_strategy = st.builds(
    # make build tuple
    lambda year, month, day: (year, month, day),
    st.integers(min_value=1, max_value=9999),  # Year
    st.integers(min_value=1, max_value=12),  # Month
    st.integers(min_value=1, max_value=31),
)  # Day

time_strategy = st.builds(
    lambda *args: tuple(map(int, args)),
    st.integers(min_value=0, max_value=23),  # Hour
    st.integers(min_value=0, max_value=59),  # Minute
    st.integers(min_value=0, max_value=59),  # Second
    st.integers(min_value=0, max_value=999_999_999),
)  # Nanosecond

datetime_strategy = st.builds(
    ry.datetime,
    st.integers(min_value=1, max_value=9999),  # Year
    st.integers(min_value=1, max_value=12),  # Month
    st.integers(min_value=1, max_value=31),  # Day
    st.integers(min_value=0, max_value=23),  # Hour
    st.integers(min_value=0, max_value=59),  # Minute
    st.integers(min_value=0, max_value=59),  # Second
    st.integers(min_value=0, max_value=999_999_999),
)  # Nanosecond

# timezone_strategy = st.sampled_from([
#     "UTC",
#     "America/New_York",
#     "Europe/London",
#     "Asia/Tokyo",
#     "Australia/Sydney",
#     "Europe/Berlin",
#     "Africa/Cairo",
#     "America/Los_Angeles",
#     # Add more timezones as needed
# ])

duration_strategy = st.builds(
    ry.SignedDuration,
    secs=st.integers(min_value=-(10**15), max_value=10**15),
    nanos=st.integers(min_value=-999_999_999, max_value=999_999_999),
)


# Test that creating a date and extracting its fields works correctly
@given(date_strategy)
def test_date_fields(date_tuple: tuple[int, int, int]) -> None:
    try:
        pydate = pydatetime.date(date_tuple[0], date_tuple[1], date_tuple[2])
        date = ry.date(date_tuple[0], date_tuple[1], date_tuple[2])
        assert date.year >= 1 and date.year <= 9999
        assert date.month >= 1 and date.month <= 12
        assert date.day >= 1 and date.day <= 31

        assert date.to_pydate() == pydate
    except ValueError:
        with pytest.raises(ValueError):
            ry.date(date_tuple[0], date_tuple[1], date_tuple[2])

    # pydate
    # date = ry.date(pydate.year, pydate.month, pydate.day)


# Test that creating a time and extracting its fields works correctly
@given(time_strategy)
def test_time_fields(time_tuple: tuple[int, int, int, int]) -> None:
    time = ry.time(time_tuple[0], time_tuple[1], time_tuple[2], time_tuple[3])
    assert time.hour >= 0 and time.hour <= 23
    assert time.minute >= 0 and time.minute <= 59
    assert time.second >= 0 and time.second <= 59
    assert time.nanosecond >= 0 and time.nanosecond <= 999_999_999


#
# # Test that adding and subtracting durations works correctly
# @given(datetime_strategy, duration_strategy)
# def test_datetime_add_subtract_duration(dt: ry.DateTime, duration: ry.SignedDuration) -> None:
#     dt_plus = dt + duration
#     dt_minus = dt_plus - duration
#     assert dt == dt_minus
#
#
# # Test that the difference between two datetimes is consistent
# @given(datetime_strategy, datetime_strategy)
# def test_datetime_difference(dt1: ry.DateTime, dt2: ry.DateTime) -> None:
#     duration = dt2 - dt1
#     dt1_plus_duration = dt1 + duration
#     assert dt1_plus_duration == dt2
#
#
# # Test that rounding a datetime with various options works correctly
# @given(datetime_strategy,
#        st.sampled_from(ry.JIFF_UNITS),
#        st.sampled_from(ry.JIFF_ROUND_MODES),
#        st.integers(min_value=1, max_value=1000))
# def test_datetime_rounding(dt: ry.DateTime, unit: str, mode: str, increment: int) -> None:
#     options = ry.DateTimeRound(smallest=unit, mode=mode, increment=increment)
#     rounded_dt = dt.round(options)
#     # Since rounding may not produce the original datetime, test that the rounded datetime is valid
#     assert isinstance(rounded_dt, ry.DateTime)
#
#
# # Test that timezones are handled correctly
# @given(datetime_strategy, timezone_strategy)
# def test_zoned_datetime_creation(dt: ry.DateTime, tz: str) -> None:
#     try:
#         zdt = dt.intz(tz)
#         assert zdt.timezone() == tz
#         assert zdt.datetime() == dt
#     except Exception:
#         # Some combinations might raise exceptions due to invalid dates or timezones
#         assume(False)
#
#
# # Test serialization and deserialization
# @given(datetime_strategy)
# def test_datetime_serialization(dt: ry.DateTime) -> None:
#     dt_string = dt.string()
#     dt_parsed = ry.parse_datetime(dt_string)
#     assert dt == dt_parsed
#
#
# # Test duration negation
# @given(duration_strategy)
# def test_duration_negation(duration: ry.SignedDuration) -> None:
#     negated_duration = -duration
#     double_negated_duration = -negated_duration
#     assert duration == double_negated_duration
#
#
# # Test that repr produces a string that can be evaluated back to the object
# @given(datetime_strategy)
# def test_datetime_repr(dt: ry.DateTime) -> None:
#     dt_repr = repr(dt)
#     dt_evaluated = eval(dt_repr)
#     assert dt == dt_evaluated
#
#
# # Test that adding zero duration does not change the datetime
# @given(datetime_strategy)
# def test_datetime_add_zero_duration(dt: ry.DateTime) -> None:
#     zero_duration = ry.SignedDuration(secs=0, nanos=0)
#     dt_plus_zero = dt + zero_duration
#     assert dt == dt_plus_zero
#
#
# # Test that durations with opposite signs cancel out
# @given(duration_strategy)
# def test_duration_addition_cancellation(duration: ry.SignedDuration) -> None:
#     zero_duration = duration + (-duration)
#     assert zero_duration.secs == 0 and zero_duration.nanos == 0
#
#
# # Test that creating a date with invalid values raises an error
# @given(st.integers(), st.integers(), st.integers())
# def test_invalid_date_creation(year: int, month: int, day: int) -> None:
#     assume(not (1 <= year <= 9999 and 1 <= month <= 12 and 1 <= day <= 31))
#     try:
#         ry.date(year, month, day)
#     except ValueError:
#         pass
#     else:
#         assert False, "Expected ValueError for invalid date"
#
#
# # Test that the string representation matches expected format
# @given(datetime_strategy)
# def test_datetime_string_format(dt: ry.DateTime) -> None:
#     dt_string = dt.string()
#     # Simple check for ISO 8601 format
#     assert isinstance(dt_string, str)
#     assert "T" in dt_string
#
#
# # Test that time arithmetic wraps correctly (e.g., adding seconds over 59)
# @given(time_strategy, st.integers(min_value=1, max_value=10000))
# def test_time_addition_overflow(time: ry.Time, seconds_to_add: int) -> None:
#     new_time = time.add_seconds(seconds_to_add)
#     assert isinstance(new_time, ry.Time)
#     # Cannot assert exact value without implementation details
#
#
# # Test that adding durations to zoned datetimes works correctly
# @given(datetime_strategy, timezone_strategy, duration_strategy)
# def test_zoned_datetime_add_duration(dt: ry.DateTime, tz: str, duration: ry.SignedDuration) -> None:
#     try:
#         zdt = dt.intz(tz)
#         new_zdt = zdt + duration
#         assert isinstance(new_zdt, ry.ZonedDateTime)
#     except Exception:
#         # Handle invalid combinations
#         assume(False)
#
#
# # Test that the difference between two times is consistent
# @given(time_strategy, time_strategy)
# def test_time_difference(t1: ry.Time, t2: ry.Time) -> None:
#     duration = t2 - t1
#     t1_plus_duration = t1 + duration
#     assert t1_plus_duration == t2
#
#
# # Test that durations are correctly converted to strings
# @given(duration_strategy)
# def test_duration_string(duration: ry.SignedDuration) -> None:
#     duration_string = duration.string()
#     assert isinstance(duration_string, str)
#     # Optionally, parse back the duration and compare
#
#
# # Test that rounding options are correctly applied
# @given(datetime_strategy, st.integers(min_value=1, max_value=1000))
# def test_datetime_round_increment(dt: ry.DateTime, increment: int) -> None:
#     options = ry.DateTimeRound(smallest="second", mode="floor", increment=increment)
#     rounded_dt = dt.round(options)
#     assert isinstance(rounded_dt, ry.DateTime)
#     # Cannot assert exact value without implementation details
#
#
# # Test that span between two datetimes is the inverse when order is reversed
# @given(datetime_strategy, datetime_strategy)
# def test_span_inverse(dt1: ry.DateTime, dt2: ry.DateTime) -> None:
#     span1 = dt2 - dt1
#     span2 = dt1 - dt2
#     assert span1 == -span2
#
#
# # Test that the addition of durations is associative
# @given(duration_strategy, duration_strategy, duration_strategy)
# def test_duration_associativity(d1: ry.SignedDuration, d2: ry.SignedDuration, d3: ry.SignedDuration) -> None:
#     result1 = (d1 + d2) + d3
#     result2 = d1 + (d2 + d3)
#     assert result1 == result2
#
#
# # Test that the subtraction of a duration is equivalent to adding its negation
# @given(datetime_strategy, duration_strategy)
# def test_duration_subtraction(dt: ry.DateTime, duration: ry.SignedDuration) -> None:
#     result_subtract = dt - duration
#     result_add_negation = dt + (-duration)
#     assert result_subtract == result_add_negation
