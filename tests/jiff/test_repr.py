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

reprs = [
    "Date(year=2020, month=8, day=26)",
    "Time(hour=6, minute=27, second=0, nanosecond=0)",
    "DateTime(year=2020, month=8, day=26, hour=6, minute=27, second=0, subsec_nanosecond=0)",
    "TimeSpan(weeks=1)",
    "Timestamp(1598438400, 0)",
    'ZonedDateTime.parse("2020-08-26T06:27:00-04:00[America/New_York]")',
    "SignedDuration(secs=1, nanos=1)",
    "Offset(hours=1)",
    "ISOWeekDate(2020, 35, 'wednesday')",
]
for thing in JIFF_OBJECTS:
    print(repr(thing))


@pytest.mark.parametrize("obj", JIFF_OBJECTS)
def test_reprs(obj: t.Any) -> None:
    repr_str = repr(obj)
    # eval the repr string
    assert eval("ry." + repr_str) == obj, f"Repr string: `{repr_str}`"


def test_reprs_simple() -> None:
    d = ry.date(2020, 8, 26)
    assert repr(d) == "Date(year=2020, month=8, day=26)"

    t = ry.time(6, 27, 0, 0)
    assert repr(t) == "Time(hour=6, minute=27, second=0, nanosecond=0)"
