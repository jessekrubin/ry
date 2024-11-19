import datetime as pydt

import ry.dev as ry


def test_date() -> None:
    ry_date = ry.date(2020, 8, 26)
    py_date = ry_date.to_pydate()
    assert pydt.date(2020, 8, 26) == py_date
    rydate_from_pydate = ry.Date.from_pydate(pydt.date(2020, 8, 26))
    assert ry_date == rydate_from_pydate


def test_date_tuple() -> None:
    rdt = ry.date(2020, 8, 26)
    assert (2020, 8, 26) == rdt.astuple()


def test_date_asdict() -> None:
    rdt = ry.date(2020, 8, 26)
    assert {"year": 2020, "month": 8, "day": 26} == rdt.asdict()
