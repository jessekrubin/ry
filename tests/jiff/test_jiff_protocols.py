from __future__ import annotations

import typing as t

import pytest

import ry

_JIFF_OBJECTS = [
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


@pytest.mark.parametrize("obj", _JIFF_OBJECTS)
def test_from_str(obj: t.Any) -> None:
    string = obj.to_string()
    cls = obj.__class__
    roundtrip_from_str = cls.from_str(string)
    roundtrip_parse = cls.parse(string)
    assert roundtrip_from_str == obj, f"Expected {obj} but got {roundtrip_from_str}"
    assert roundtrip_parse == obj, f"Expected {obj} but got {roundtrip_parse}"


@pytest.mark.parametrize("obj", _JIFF_OBJECTS)
def test_to_dict(obj: t.Any) -> None:
    d = obj.to_dict()
    assert isinstance(d, dict)
    assert len(d) > 0
