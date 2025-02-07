from __future__ import annotations

import datetime as pydt

from hypothesis import strategies as st

import ry

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
