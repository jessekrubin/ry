from __future__ import annotations

import datetime as _datetime

import ry


def test_from_date() -> None:
    py_date = _datetime.date(2020, 8, 26)
    ry_date = ry.date(2020, 8, 26)
    assert py_date.year == ry_date.year
    assert py_date.month == ry_date.month
    assert py_date.day == ry_date.day


def test_from_zoned() -> None:
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    pdt = zdt.to_pydatetime()
    assert pdt.year == zdt.year
    assert pdt.month == zdt.month
    assert pdt.day == zdt.day
    assert pdt.hour == zdt.hour
    assert pdt.minute == zdt.minute
    assert pdt.second == zdt.second
