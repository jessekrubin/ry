import datetime as pydt
import zoneinfo

import pytest
from hypothesis import assume, given
from hypothesis import strategies as st

import ry


@given(st.datetimes(timezones=st.none()))
def test_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""

    py_isoformat = dt.isoformat()
    ry_dt = ry.DateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat or py_isoformat.startswith(ry_isoformat), (
        f"py: {py_isoformat}\nry: {ry_isoformat}"
    )


def test_zoned_datetime_isoformat_build_etc_localtime() -> None:
    """Test that datetime.isoformat() produces the expected string for naive datetime."""

    dt = pydt.datetime(
        2000, 1, 1, 0, 0, tzinfo=zoneinfo.ZoneInfo(key="build/etc/localtime")
    )
    # TODO: remove when build/etc/localtime is handled removed
    with pytest.raises(ValueError):
        ry.ZonedDateTime.from_pydatetime(dt)


@given(st.datetimes(timezones=st.timezones()))
def test_zoned_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""

    assume(dt.tzinfo is not None)  # Ensure the datetime is timezone-aware
    # TODO: handle 'build/etc/localtime' case
    if dt.tzname() != "build/etc/localtime":
        py_isoformat = dt.isoformat()
        ry_dt = ry.ZonedDateTime.from_pydatetime(dt)
        ry_isoformat = ry_dt.isoformat()
        assert ry_isoformat == py_isoformat or py_isoformat.startswith(ry_isoformat), (
            f"py: {py_isoformat}\nry: {ry_isoformat}"
        )
