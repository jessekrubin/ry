from __future__ import annotations

import datetime as pydt

import ry


def test_date() -> None:
    ry_date = ry.date(2020, 8, 26)
    py_date = ry_date.to_pydate()
    assert pydt.date(2020, 8, 26) == py_date
    rydate_from_pydate = ry.Date.from_pydate(pydt.date(2020, 8, 26))
    assert ry_date == rydate_from_pydate


def test_date_tuple() -> None:
    rdt = ry.date(2020, 8, 26)
    assert (2020, 8, 26) == rdt.astuple()


def test_date_to_dict() -> None:
    rdt = ry.date(2020, 8, 26)
    d = rdt.to_dict()
    assert isinstance(d, dict)
    assert d == {"year": 2020, "month": 8, "day": 26}


def test_rytime2pytime() -> None:
    pydt.time(10, 20, 30)
    rytime = ry.time(10, 20, 30)
    pytime = rytime.to_pytime()
    assert pydt.time(10, 20, 30) == pytime


def test_time_tuple() -> None:
    rytime = ry.time(10, 20, 30, 0)
    assert (10, 20, 30, 0) == rytime.astuple()


def test_time_to_dict() -> None:
    rytime = ry.time(10, 20, 30)
    assert {
        "hour": 10,
        "minute": 20,
        "second": 30,
        "nanosecond": 0,
    } == rytime.to_dict()


def test_datetime_to_dict() -> None:
    rytime = ry.datetime(2020, 8, 26, 10, 20, 30)
    assert {
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 10,
        "minute": 20,
        "second": 30,
        "nanosecond": 0,
    } == rytime.to_dict()


def test_zoned_to_dict() -> None:
    rytime = ry.datetime(2020, 8, 26, 10, 20, 30)
    assert {
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 10,
        "minute": 20,
        "second": 30,
        "nanosecond": 0,
    } == rytime.to_dict()
