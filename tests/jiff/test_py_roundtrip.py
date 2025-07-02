from __future__ import annotations

import datetime as pydt
import zoneinfo

from hypothesis import assume, given
from hypothesis import strategies as st

import ry

from ..strategies import st_timezones


@given(st.dates())
def test_date_roundtrip(dt: pydt.date) -> None:
    """Test that datetime.isoformat() produces the expected string."""
    ry_dt = ry.Date.from_pydate(dt)
    roundtrip_date = ry_dt.to_py()
    assert roundtrip_date == dt, f"Expected {dt}, got {roundtrip_date}"


@given(st.times())
def test_time_roundtrip(t: pydt.time) -> None:
    """Test that datetime.isoformat() produces the expected string."""

    ry_t = ry.Time.from_pytime(t)
    roundtrip_date = ry_t.to_py()
    assert roundtrip_date == t, f"Expected {t}, got {roundtrip_date}"


@given(st_timezones())
def test_timezone_roundtrip(tz: pydt.tzinfo | zoneinfo.ZoneInfo) -> None:
    """Test that datetime.isoformat() produces the expected string."""
    assume(tz is not None)
    ry_tz = ry.TimeZone.from_pytzinfo(tz)
    roundtrip_tz = ry_tz.to_py()

    assert isinstance(roundtrip_tz, pydt.tzinfo), (
        f"Expected a tzinfo instance, got {type(roundtrip_tz)}"
    )
    # the input tz info may be a zoneinfo.ZoneInfo or a pytz timezone,
    # so we need to handle both cases
    tz_utcoffset = tz.utcoffset(pydt.datetime.now(tz=pydt.timezone.utc))
    roundtrip_tz_utcoffset = roundtrip_tz.utcoffset(
        pydt.datetime.now(tz=pydt.timezone.utc)
    )
    assert roundtrip_tz_utcoffset == tz_utcoffset, (
        f"Expected timezone offset {tz_utcoffset}, got {roundtrip_tz_utcoffset}"
    )


@given(st.datetimes(timezones=st.none()))
def test_datetime_roundtrip(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""
    ry_dt = ry.DateTime.from_pydatetime(dt)
    roundtrip_date = ry_dt.to_py()
    assert roundtrip_date == dt, f"Expected {dt}, got {roundtrip_date}"


@given(st.datetimes(timezones=st_timezones()))
def test_zoned_datetime_roundtrip(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""
    assume(dt.tzinfo is not None)
    ry_zdt = ry.ZonedDateTime.from_pydatetime(dt)
    roundtrip_date = ry_zdt.to_py()
    # get the input tz offset
    assert dt.tzinfo is not None, "Expected datetime to have a timezone"
    tz_utcoffset = dt.tzinfo.utcoffset(dt)
    assert roundtrip_date.tzinfo is not None
    roundtrip_tz_utcoffset = roundtrip_date.tzinfo.utcoffset(roundtrip_date)
    assert roundtrip_tz_utcoffset == tz_utcoffset, (
        f"Expected timezone offset {tz_utcoffset}, got {roundtrip_tz_utcoffset}"
    )
