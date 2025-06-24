import datetime as pydt

from hypothesis import assume, given
from hypothesis import strategies as st

import ry


@given(st.datetimes(timezones=st.none()))
def test_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""

    py_isoformat = dt.isoformat()
    ry_dt = ry.DateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat, f"py: {py_isoformat}\nry: {ry_isoformat}"


@given(st.dates())
def test_date_isoformat(d: pydt.datetime) -> None:
    py_isoformat = d.isoformat()
    ry_dt = ry.Date.from_pydate(d)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat, f"py: {py_isoformat}\nry: {ry_isoformat}"


@given(st.times())
def test_time_isoformat(d: pydt.time) -> None:
    py_isoformat = d.isoformat()
    ry_dt = ry.Time.from_pytime(d)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat, f"py: {py_isoformat}\nry: {ry_isoformat}"


@given(st.datetimes(timezones=st.timezones()))
def test_zoned_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""

    assume(dt.tzinfo is not None)  # Ensure the datetime is timezone-aware

    try:
        if dt.tzname() != "build/etc/localtime":
            py_isoformat = dt.isoformat()
            ry_dt = ry.ZonedDateTime.from_pydatetime(dt)
            ry_isoformat = ry_dt.isoformat()
            assert ry_isoformat == py_isoformat or py_isoformat.startswith(
                ry_isoformat
            ), f"py: {py_isoformat}\nry: {ry_isoformat}"

    except ValueError as _e:
        ...
