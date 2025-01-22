from __future__ import annotations

import datetime as pydt

import ry


def test_time_to_pytime() -> None:
    py_time = pydt.time(10, 20, 30)
    ry_time = ry.time(10, 20, 30)
    from_ry = ry_time.to_pytime()
    assert py_time.hour == ry_time.hour
    assert py_time.minute == ry_time.minute
    assert py_time.second == ry_time.second
    assert from_ry == py_time


def test_time_from_pytime() -> None:
    py_time = pydt.time(10, 20, 30)
    ry_time = ry.Time.from_pytime(py_time)
    assert py_time.hour == ry_time.hour
    assert py_time.minute == ry_time.minute
    assert py_time.second == ry_time.second


def test_from_date() -> None:
    py_date = pydt.date(2020, 8, 26)
    ry_date = ry.date(2020, 8, 26)
    assert py_date.year == ry_date.year
    assert py_date.month == ry_date.month
    assert py_date.day == ry_date.day


def test_from_datetime() -> None:
    py_dt = pydt.datetime(2020, 8, 26, 6, 27, 0, 0)
    ry_dt = ry.DateTime.from_pydatetime(py_dt)
    assert py_dt.year == ry_dt.year
    assert py_dt.month == ry_dt.month
    assert py_dt.day == ry_dt.day
    assert py_dt.hour == ry_dt.hour
    assert py_dt.minute == ry_dt.minute
    assert py_dt.second == ry_dt.second


def test_from_zoned() -> None:
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    pdt = zdt.to_pydatetime()
    assert pdt.year == zdt.year
    assert pdt.month == zdt.month
    assert pdt.day == zdt.day
    assert pdt.hour == zdt.hour
    assert pdt.minute == zdt.minute
    assert pdt.second == zdt.second
    pdt_tzinfo = pdt.tzinfo

    assert pdt_tzinfo is not None
    assert pdt_tzinfo.utcoffset(pdt) is not None
    assert pdt_tzinfo.tzname(pdt) in ("EDT", "EST", "America/New_York")

    # round trip
    zdt_from_py = ry.ZonedDateTime.from_pydatetime(pdt)
    assert zdt == zdt_from_py
