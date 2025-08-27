from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from ry._types import FromStr

JIFF_OBJECTS: list[FromStr] = [
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
]


@pytest.mark.parametrize("obj", JIFF_OBJECTS)
def test_from_string(obj: t.Any) -> None:
    string = obj.string()
    cls = obj.__class__
    roundtrip_from_str = cls.from_str(string)
    roundtrip_parse = cls.parse(string)
    assert roundtrip_from_str == obj, f"Expected {obj} but got {roundtrip_from_str}"
    assert roundtrip_parse == obj, f"Expected {obj} but got {roundtrip_parse}"
