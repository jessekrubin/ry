from __future__ import annotations

import typing as t

import pytest

import ry

jiff_objects = [
    # date
    ry.date(2020, 8, 26),
    # time
    ry.time(6, 27, 0, 0),
    # datetime
    ry.datetime(2020, 8, 26, 6, 27, 0, 0),
    # span
    ry.timespan(weeks=1),
    # timestamp
    ry.Timestamp.from_millisecond(1598438400000),
    # Zoned
    ry.datetime(2020, 8, 26, 6, 27, 0, 0).in_tz("America/New_York"),
    # signed-duration
    ry.SignedDuration(1, 1),
    # offset
    ry.Offset(1),
    # iso-week-date
    ry.date(2020, 8, 26).iso_week_date(),
]


@pytest.mark.parametrize("obj", jiff_objects)
def test_reprs(obj: t.Any) -> None:
    repr_str = repr(obj)
    # eval the repr string
    assert eval("ry." + repr_str) == obj


def test_reprs_simple() -> None:
    d = ry.date(2020, 8, 26)
    assert repr(d) == "Date(year=2020, month=8, day=26)"

    t = ry.time(6, 27, 0, 0)
    assert repr(t) == "Time(hour=6, minute=27, second=0, nanosecond=0)"
