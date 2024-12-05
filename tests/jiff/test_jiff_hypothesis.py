from __future__ import annotations

import datetime as pydt

import pytest
from hypothesis import assume, given
from hypothesis import strategies as st

import ry.dev as ry

date_strategy = st.dates(
    min_value=pydt.date(1, 1, 1), max_value=pydt.date(9999, 12, 31)
).map(lambda dt: ry.date(dt.year, dt.month, dt.day))

time_strategy = st.times().map(
    lambda t: ry.time(t.hour, t.minute, t.second, t.microsecond * 1000)
)

datetime_strategy = st.datetimes(
    min_value=pydt.datetime(1, 1, 1, 0, 0, 0),
    max_value=pydt.datetime(9999, 12, 31, 23, 59, 59, 999999),
).map(
    lambda dt: ry.datetime(
        dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second, dt.microsecond * 1000
    )
)
timedelta_strategy = st.timedeltas()

# Define strategies for generating test data
date_tuple_strategy = st.builds(
    # make build tuple
    lambda year, month, day: (year, month, day),
    st.integers(min_value=1, max_value=9999),  # Year
    st.integers(min_value=1, max_value=12),  # Month
    st.integers(min_value=1, max_value=31),
)  # Day

time_tuple_strategy = st.builds(
    lambda *args: tuple(map(int, args)),
    st.integers(min_value=0, max_value=23),  # Hour
    st.integers(min_value=0, max_value=59),  # Minute
    st.integers(min_value=0, max_value=59),  # Second
    st.integers(min_value=0, max_value=999_999_999),
)  # Nanosecond

datetime_tuple_strategy = st.builds(
    lambda *args: tuple(map(int, args)),
    st.integers(min_value=1, max_value=9999),  # Year
    st.integers(min_value=1, max_value=12),  # Month
    st.integers(min_value=1, max_value=31),  # Day
    st.integers(min_value=0, max_value=23),  # Hour
    st.integers(min_value=0, max_value=59),  # Minute
    st.integers(min_value=0, max_value=59),  # Second
    st.integers(min_value=0, max_value=999_999_999),
)  # Nanosecond

timezone_strategy = st.sampled_from(
    [
        "UTC",
        "America/New_York",
        "Europe/London",
        "Asia/Tokyo",
        "Australia/Sydney",
        "Europe/Berlin",
        "Africa/Cairo",
        "America/Los_Angeles",
        # Add more timezones as needed
    ]
)

duration_strategy = st.builds(
    ry.SignedDuration,
    secs=st.integers(min_value=-(10**15), max_value=10**15),
    nanos=st.integers(min_value=-999_999_999, max_value=999_999_999),
)


# Test that creating a date and extracting its fields works correctly
@given(date_tuple_strategy)
def test_date_fields(date_tuple: tuple[int, int, int]) -> None:
    try:
        pydate = pydt.date(date_tuple[0], date_tuple[1], date_tuple[2])
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


@given(time_tuple_strategy)
def test_time_fields(time_tuple: tuple[int, int, int, int]) -> None:
    """Test that creating a time and extracting its fields works correctly"""
    time = ry.time(time_tuple[0], time_tuple[1], time_tuple[2], time_tuple[3])
    assert 0 <= time.hour <= 23
    assert 0 <= time.minute <= 59
    assert 0 <= time.second <= 59
    assert 0 <= time.nanosecond <= 999_999_999


@given(datetime_strategy, duration_strategy)
def test_datetime_add_subtract_signed_duration(
    dt: ry.DateTime, duration: ry.SignedDuration
) -> None:
    """Test that adding and subtracting durations works correctly"""
    print(dt, duration)
    try:
        dt_plus = dt + duration
        dt_minus = dt_plus - duration
        assert dt == dt_minus
    except OverflowError as _oe:
        with pytest.raises(OverflowError):
            dt_plus = dt + duration
            dt_minus = dt_plus - duration
            assert dt == dt_minus


@given(datetime_strategy, datetime_strategy)
def test_datetime_difference(dt1: ry.DateTime, dt2: ry.DateTime) -> None:
    """test that the difference between two datetimes is consistent"""
    duration = dt2 - dt1
    dt1_plus_duration = dt1 + duration
    assert dt1_plus_duration == dt2


# @given(datetime_strategy,
#        st.sampled_from(["second", "minute", "hour", "day", "month", "year"]),
#        st.sampled_from(["floor", "ceil", "round"]),
#        st.integers(min_value=1, max_value=1000))
# def test_datetime_rounding(dt: ry.DateTime, unit: str, mode: str, increment: int) -> None:
#     """Test that rounding a datetime with various options works correctly"""
#     options = ry.DateTimeRound(smallest=unit, mode=mode, increment=increment)
#     rounded_dt = dt.round(options)
#     # Since rounding may not produce the original datetime, test that the rounded datetime is valid
#     assert isinstance(rounded_dt, ry.DateTime)


@given(datetime_strategy, timezone_strategy)
def test_zoned_datetime_creation(dt: ry.DateTime, tz: str) -> None:
    """Test that tz are handled correctly"""
    try:
        zdt = dt.intz(tz)
        assert zdt.timezone() == tz
        assert zdt.datetime() == dt
    except ValueError:
        # Some combinations might raise exceptions due to invalid dates or timezones
        assume(False)


@given(datetime_strategy)
def test_datetime_serialization(dt: ry.DateTime) -> None:
    """Test serialization and deserialization"""
    dt_string = dt.string()
    dt_parsed = ry.DateTime.parse(dt_string)
    assert dt == dt_parsed


@given(duration_strategy)
def test_duration_negation(duration: ry.SignedDuration) -> None:
    """Test duration negation"""
    negated_duration = -duration
    if duration.is_zero():
        assert duration.secs == 0
        assert negated_duration.secs == 0
        assert duration.nanos == 0
        assert negated_duration.nanos == 0
    else:
        assert negated_duration != duration
    double_negated_duration = -negated_duration
    assert duration == double_negated_duration


@given(datetime_strategy)
def test_datetime_repr(dt: ry.DateTime) -> None:
    """Test that repr produces a string that can be evaluated back to the object"""
    dt_repr = repr(dt)
    dt_evaluated = eval(f"ry.{dt_repr}")
    assert dt == dt_evaluated


@given(datetime_strategy)
def test_datetime_add_zero_duration(dt: ry.DateTime) -> None:
    """Test that adding zero duration does not change the datetime"""
    zero_duration = ry.SignedDuration(secs=0, nanos=0)
    dt_plus_zero = dt + zero_duration
    assert dt == dt_plus_zero


@given(duration_strategy)
def test_duration_addition_cancellation(duration: ry.SignedDuration) -> None:
    """Test that adding a duration and its negation results in zero"""
    neg_duration = -duration
    zero_duration = duration + neg_duration
    assert zero_duration.secs == 0 and zero_duration.nanos == 0


@given(st.integers(), st.integers(), st.integers())
def test_invalid_date_creation(year: int, month: int, day: int) -> None:
    assume(not (-9999 <= year <= 9999 and 1 <= month <= 12 and 1 <= day <= 31))
    try:
        # pydt.date(year, month, day)
        ry.date(year, month, day)
    # TODO: figure out if should be OverflowError or ValueError
    except (ValueError, OverflowError):
        pass
    else:
        assert False, "Expected ValueError for invalid date"


@given(datetime_strategy)
def test_datetime_string_format(dt: ry.DateTime) -> None:
    """Test that the string representation matches expected format"""
    dt_string = dt.string()
    assert isinstance(dt_string, str)
    assert "T" in dt_string


@given(time_strategy, st.integers(min_value=1, max_value=10000))
def test_time_addition_overflow(time: ry.Time, seconds_to_add: int) -> None:
    """Test that adding seconds to a time wraps correctly"""
    tspan = ry.timespan(seconds=seconds_to_add)
    try:
        new_time = time + tspan
        assert isinstance(new_time, ry.Time)
    except OverflowError:
        with pytest.raises(OverflowError):
            time + tspan


@given(datetime_strategy, timezone_strategy, duration_strategy)
def test_zoned_datetime_add_duration(
    dt: ry.DateTime, tz: str, duration: ry.SignedDuration
) -> None:
    """Test that adding a duration to a zoned datetime works correctly"""
    try:
        zdt = dt.intz(tz)
        new_zdt = zdt + duration
        assert isinstance(new_zdt, ry.ZonedDateTime)
    except Exception:
        # Handle invalid combinations
        assume(False)


@given(time_strategy, time_strategy)
def test_time_difference(t1: ry.Time, t2: ry.Time) -> None:
    """Test that the difference between two times is consistent"""
    duration = t2 - t1
    t1_plus_duration = t1 + duration
    assert t1_plus_duration == t2


@given(duration_strategy)
def test_duration_string(duration: ry.SignedDuration) -> None:
    """Test that the string representation of a duration is valid"""
    duration_string = duration.string()
    assert isinstance(duration_string, str)


@given(datetime_strategy, st.integers(min_value=1, max_value=1000))
def test_datetime_round_increment(dt: ry.DateTime, increment: int) -> None:
    options = ry.DateTimeRound(smallest="second", mode="floor", increment=increment)
    try:
        rounded_dt = dt.round(options)
        assert isinstance(rounded_dt, ry.DateTime)
    except ValueError as _ve:
        with pytest.raises(ValueError):
            dt.round(options)


@given(datetime_strategy, datetime_strategy)
def test_span_inverse(dt1: ry.DateTime, dt2: ry.DateTime) -> None:
    """Test that the span between two datetimes is the inverse when order is reversed"""
    span1 = dt2 - dt1
    span2 = dt1 - dt2
    assert span1 == -span2


@given(duration_strategy, duration_strategy, duration_strategy)
def test_duration_associativity(
    d1: ry.SignedDuration, d2: ry.SignedDuration, d3: ry.SignedDuration
) -> None:
    """Test that the addition of durations is associative"""
    result1 = (d1 + d2) + d3
    result2 = d1 + (d2 + d3)
    assert result1 == result2


@given(datetime_strategy, duration_strategy)
def test_duration_subtraction(dt: ry.DateTime, duration: ry.SignedDuration) -> None:
    """Test that the subtraction of a duration is equivalent to adding its negation"""
    try:
        result_subtract = dt - duration
        result_add_negation = dt + (-duration)
        assert result_subtract == result_add_negation
    except OverflowError:
        with pytest.raises(OverflowError):
            dt - duration


class TestSignedDurationConversion:
    @given(timedelta_strategy)
    def test_span_from_timedelta_min_max(self, tdelta: pydt.timedelta) -> None:
        assume(-7304484 <= tdelta.days <= 7304484)
        ry_signed_dur = ry.TimeSpan.from_pytimedelta(tdelta)
        assert isinstance(ry_signed_dur, ry.TimeSpan)

    @given(timedelta_strategy)
    def test_positive_signed_duration_round_trip(self, tdelta: pydt.timedelta) -> None:
        # assume the duration is positive
        assume(tdelta.days >= 0)
        ry_signed_duration = ry.SignedDuration.from_pytimedelta(tdelta)
        assert isinstance(ry_signed_duration, ry.SignedDuration)
        round_trip_tdelta = ry_signed_duration.to_pytimedelta()
        assert isinstance(round_trip_tdelta, pydt.timedelta)
        assert round_trip_tdelta == tdelta

    @given(timedelta_strategy)
    def test_negative_signed_duration_round_trip(self, tdelta: pydt.timedelta) -> None:
        # assume the duration is negative
        assume(tdelta.days < 0)
        ry_signed_duration = ry.SignedDuration.from_pytimedelta(tdelta)
        assert isinstance(ry_signed_duration, ry.SignedDuration)
        round_trip_tdelta = ry_signed_duration.to_pytimedelta()
        assert isinstance(round_trip_tdelta, pydt.timedelta)
        assert round_trip_tdelta == tdelta


class TestTimeSpanConversion:
    @given(timedelta_strategy)
    def test_span_from_timedelta_round_trip(self, tdelta: pydt.timedelta) -> None:
        assume(-7304484 <= tdelta.days <= 7304484)
        ry_span = ry.TimeSpan.from_pytimedelta(tdelta)

        assert isinstance(ry_span, ry.TimeSpan)
        round_trip_tdelta = ry_span.to_pytimedelta()
        assert isinstance(round_trip_tdelta, pydt.timedelta)
        assert round_trip_tdelta == tdelta

    @given(timedelta_strategy)
    def test_span_from_timedelta_to_many_days(self, tdelta: pydt.timedelta) -> None:
        # to span
        print("===========")
        print("tdelta", tdelta, tdelta.__repr__())
        assume(-7304484 > tdelta.days or tdelta.days > 7304484)
        with pytest.raises(ValueError):
            ry.TimeSpan.from_pytimedelta(tdelta)
