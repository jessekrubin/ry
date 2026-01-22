from __future__ import annotations

import datetime as pydt
from typing import TYPE_CHECKING

from hypothesis import strategies as st

import ry

if TYPE_CHECKING:
    from hypothesis.strategies import SearchStrategy

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
        dt.year,
        dt.month,
        dt.day,
        dt.hour,
        dt.minute,
        dt.second,
        dt.microsecond * 1000,
    )
)
timedelta_minmax_strategy = st.timedeltas(
    min_value=pydt.timedelta(days=-7304484), max_value=pydt.timedelta(days=7304484)
)
timedelta_positive_strategy = st.timedeltas(
    min_value=pydt.timedelta(0), max_value=pydt.timedelta(days=365 * 100)
)
timedelta_negative_strategy = st.timedeltas(
    min_value=pydt.timedelta(days=-365 * 100), max_value=pydt.timedelta(0)
)

# out of range timedelta composite strategy of 2 timedelta strategies
timedelta_out_of_range_strategy = st.one_of(
    # below min
    st.timedeltas(max_value=pydt.timedelta(days=-7304484)),
    # above max
    st.timedeltas(min_value=pydt.timedelta(days=7304484)),
)

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

timezone_strategy = st.sampled_from([
    "UTC",
    "America/New_York",
    "Europe/London",
    "Asia/Tokyo",
    "Australia/Sydney",
    "Europe/Berlin",
    "Africa/Cairo",
    "America/Los_Angeles",
])

duration_strategy = st.builds(
    ry.SignedDuration,
    secs=st.integers(min_value=-(10**15), max_value=10**15),
    nanos=st.integers(min_value=-999_999_999, max_value=999_999_999),
)

# unsigned ────────────────────────────────────────────────────────────


def st_signed_durations(
    *,
    min_value: ry.SignedDuration = ry.SignedDuration.MIN,
    max_value: ry.SignedDuration = ry.SignedDuration.MAX,
) -> SearchStrategy[ry.SignedDuration]:
    """Strategy for `ry.Duration` instances"""
    if not isinstance(min_value, ry.SignedDuration):
        msg = f"min_value must be a ry.SignedDuration, got {type(min_value)}"
        raise TypeError(msg)
    if not isinstance(max_value, ry.SignedDuration):
        msg = f"max_value must be a ry.SignedDuration, got {type(max_value)}"
        raise TypeError(msg)
    if min_value > max_value:
        emsg = f"min_value {min_value} must be <= max_value {max_value}"
        raise ValueError(emsg)
    if min_value == max_value:
        return st.just(min_value)
    if min_value == ry.SignedDuration.MIN and max_value == ry.SignedDuration.MAX:
        return st.builds(
            ry.SignedDuration,
            st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX),
            st.integers(min_value=-999_999_999, max_value=999_999_999),
        )
    return st.builds(
        ry.SignedDuration,
        st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX),
        st.integers(min_value=-999_999_999, max_value=999_999_999),
    ).filter(lambda d: min_value <= d <= max_value)


def st_offsets(
    min_value: ry.Offset = ry.Offset.MIN, max_value: ry.Offset = ry.Offset.MAX
) -> SearchStrategy[ry.Offset]:
    """Strategy for `ry.Offset` instances"""
    if not isinstance(min_value, ry.Offset):
        msg = f"min_value must be a ry.Offset, got {type(min_value)}"
        raise TypeError(msg)
    if not isinstance(max_value, ry.Offset):
        msg = f"max_value must be a ry.Offset, got {type(max_value)}"
        raise TypeError(msg)
    if min_value > max_value:
        emsg = f"min_value {min_value} must be <= max_value {max_value}"
        raise ValueError(emsg)

    return st.builds(
        ry.Offset.from_seconds,
        st.integers(min_value=min_value.seconds, max_value=max_value.seconds),
    )


def st_timestamps(
    *,
    min_value: ry.Timestamp = ry.Timestamp.MIN,
    max_value: ry.Timestamp = ry.Timestamp.MAX,
) -> SearchStrategy[ry.Timestamp]:
    """Strategy for `ry.Timestamp` instances"""
    if not isinstance(min_value, ry.Timestamp):
        msg = f"min_value must be a ry.Timestamp, got {type(min_value)}"
        raise TypeError(msg)
    if not isinstance(max_value, ry.Timestamp):
        msg = f"max_value must be a ry.Timestamp, got {type(max_value)}"
        raise TypeError(msg)
    if min_value > max_value:
        emsg = f"min_value {min_value} must be <= max_value {max_value}"
        raise ValueError(emsg)
    if min_value == max_value:
        return st.just(min_value)

    return st.builds(
        ry.Timestamp,
        st.integers(min_value=min_value.second, max_value=max_value.second),
        st.integers(min_value=0, max_value=999_999_999),
    )
