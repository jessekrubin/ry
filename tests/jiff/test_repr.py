from __future__ import annotations

import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from ry.ryo3 import JiffRoundMode, JiffUnit

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
def test_reprs(obj: t.Any) -> None:
    repr_str = repr(obj)
    # eval the repr string
    assert eval("ry." + repr_str) == obj, f"Repr string: `{repr_str}`"


def test_repr_span() -> None:
    s = ry.timespan(
        years=1,
        months=1,
        weeks=1,
        days=1,
        hours=1,
        minutes=1,
        seconds=1,
        milliseconds=1,
        microseconds=1,
        nanoseconds=1,
    )
    repr_str = repr(s)
    # eval the repr string
    assert eval("ry." + repr_str) == s, f"Repr string: `{repr_str}`"


@pytest.mark.parametrize(
    "cls",
    [
        ry.DateTimeRound,
        ry.TimestampRound,
        ry.ZonedDateTimeRound,
        ry.TimeRound,
        ry.SignedDurationRound,
    ],
)
def test_round_reprs(
    cls: type[
        ry.DateTimeRound
        | ry.TimestampRound
        | ry.ZonedDateTimeRound
        | ry.TimeRound
        | ry.SignedDurationRound
    ],
    jiff_unit: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    # TODO: fix tests so that these aren't possible...
    round_obj = cls(smallest=jiff_unit, mode=jiff_round_mode, increment=1)  # type: ignore[arg-type]
    repr_str = repr(round_obj)
    assert repr_str == str(round_obj)
    evaled = eval("ry." + repr_str)
    assert evaled == round_obj


@pytest.mark.parametrize("obj", _JIFF_OBJECTS)
def test_hash(obj: t.Any) -> None:
    hash_n = hash(obj)
    assert hash_n == hash(eval("ry." + repr(obj))), f"Hash mismatch for: {obj}"
