from __future__ import annotations

import datetime as pydt

import pytest
from hypothesis import assume, given, settings
from hypothesis import strategies as st

import ry

from ..strategies import st_timezones

settings.register_profile("slow", max_examples=10_000, deadline=30_000)


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


@given(st.datetimes(timezones=st_timezones()))
@pytest.mark.skip("TODO: revisit something is screwy")
def test_zoned_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that ZondedDateTime.isoformat() produces the expected string."""

    assume(dt.tzinfo is not None)  # Ensure the datetime is timezone-aware
    py_isoformat = dt.isoformat()
    ry_dt = ry.ZonedDateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    is_eq = ry_isoformat == py_isoformat
    ry_prefix_ok = py_isoformat.startswith(ry_isoformat)

    assert ry_isoformat == py_isoformat or py_isoformat.startswith(ry_isoformat), (
        f"py: {py_isoformat}\nry: {ry_isoformat}\nis_eq: {is_eq}\nry_prefix_ok: {ry_prefix_ok}\n"
    )


@given(st.datetimes(timezones=st_timezones()))
def test_zoned_datetime_iso_format_works_with_py_datetime_from(
    dt: pydt.datetime,
) -> None:
    """Test that ZondedDateTime.isoformat() produces the expected string."""

    assume(dt.tzinfo is not None)  # Ensure the datetime is timezone-aware
    ry_dt = ry.ZonedDateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    py_from_zdt_isoformat = pydt.datetime.fromisoformat(ry_isoformat)
    assert isinstance(py_from_zdt_isoformat, pydt.datetime), (
        f"Expected a datetime instance, got {type(py_from_zdt_isoformat)}"
    )
