from __future__ import annotations

import datetime as pydt
from typing import TYPE_CHECKING, Final

from hypothesis import strategies as st

import ry

if TYPE_CHECKING:
    from hypothesis.strategies import SearchStrategy
# unsigned ──────────────────────────────────────────────────────────
_MIN_U8: Final = 0
_MAX_U8: Final = (1 << 8) - 1  # 255

_MIN_U16: Final = 0
_MAX_U16: Final = (1 << 16) - 1  # 65_535

_MIN_U32: Final = 0
_MAX_U32: Final = (1 << 32) - 1  # 4_294_967_295

_MIN_U64: Final = 0
_MAX_U64: Final = (1 << 64) - 1  # 18_446_744_073_709_551_615

_MIN_U128: Final = 0
_MAX_U128: Final = (
    1 << 128
) - 1  # 340_282_366_841_710_656_408_393_487_639_999_999_999_999_999_999_999_999_999_999

# signed ────────────────────────────────────────────────────────────
_MIN_I8: Final = -(1 << 7)  # -128
_MAX_I8: Final = (1 << 7) - 1  # 127

_MIN_I16: Final = -(1 << 15)  # -32_768
_MAX_I16: Final = (1 << 15) - 1  # 32_767

_MIN_I32: Final = -(1 << 31)  # -2_147_483_648
_MAX_I32: Final = (1 << 31) - 1  # 2_147_483_647

_MIN_I64: Final = -(1 << 63)  # -9_223_372_036_854_775_808
_MAX_I64: Final = (1 << 63) - 1  # 9_223_372_036_854_775_807

_MIN_I128: Final = -(1 << 127)  # -170_141_183_460_469_231_731_687_303_715_884_105_728
_MAX_I128: Final = (1 << 127) - 1  # 170_141_183_460_469_231_731_687_303_715_884_105_727

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
    # Add more timezones as needed
])

duration_strategy = st.builds(
    ry.SignedDuration,
    secs=st.integers(min_value=-(10**15), max_value=10**15),
    nanos=st.integers(min_value=-999_999_999, max_value=999_999_999),
)


# unsigned ────────────────────────────────────────────────────────────
def st_signed_duration_args() -> SearchStrategy[tuple[int, int]]:
    """Strategy for `ry.Duration` constructor arguments"""
    return st.tuples(
        st.integers(min_value=-(1 << 63), max_value=(1 << 63) - 1),
        st.integers(min_value=-999_999_999, max_value=999_999_999),
    )


def st_signed_duration(
    *,
    min_value: ry.SignedDuration | None = None,
    max_value: ry.SignedDuration | None = None,
) -> SearchStrategy[ry.SignedDuration]:
    """Strategy for `ry.SignedDuration` instances"""
    st_nanos = st.integers(min_value=-999_999_999, max_value=999_999_999)
    st_seconds = st.integers(min_value=-(1 << 63), max_value=(1 << 63) - 1)
    if min_value is None and max_value is None:
        return st.builds(ry.SignedDuration, st_seconds, st_nanos)

    _min_val = min_value if min_value is not None else ry.SignedDuration.MIN
    _max_val = max_value if max_value is not None else ry.SignedDuration.MAX
    assert isinstance(_min_val, ry.SignedDuration), (
        "min_value must be ry.SignedDuration or None"
    )
    assert isinstance(_max_val, ry.SignedDuration), (
        "max_value must be ry.SignedDuration or None"
    )

    return st.builds(
        ry.SignedDuration,
        st.integers(min_value=_min_val.secs, max_value=_max_val.secs),
        st.integers(min_value=-999_999_999, max_value=999_999_999),
    ).filter(lambda d: (_min_val <= d <= _max_val))
