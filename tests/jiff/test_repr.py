from __future__ import annotations

import typing as t

import pytest

import ry

JIFF_OBJECTS = [
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


@pytest.mark.parametrize("obj", JIFF_OBJECTS)
def test_reprs(obj: t.Any) -> None:
    repr_str = repr(obj)
    # eval the repr string
    assert eval("ry." + repr_str) == obj, f"Repr string: `{repr_str}`"
