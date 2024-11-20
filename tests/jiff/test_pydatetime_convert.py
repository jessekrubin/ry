import datetime as _datetime

import ry


def test_from_date() -> None:
    py_date = _datetime.date(2020, 8, 26)
    ry_date = ry.date(2020, 8, 26)
    assert py_date.year == ry_date.year()
    assert py_date.month == ry_date.month()
    assert py_date.day == ry_date.day()
