from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from ry.ryo3 import JIFF_ROUND_MODE, JIFF_UNIT


@pytest.mark.parametrize(
    "cls",
    [
        ry.DateTimeRound,
        ry.SignedDurationRound,
        ry.TimestampRound,
        ry.ZonedDateTimeRound,
    ],
)
def test_round_getters(
    cls: type[
        ry.DateTimeRound
        | ry.SignedDurationRound
        | ry.TimestampRound
        | ry.ZonedDateTimeRound
    ],
    jiff_unit: JIFF_UNIT,
    jiff_round_mode: JIFF_ROUND_MODE,
) -> None:
    round_obj = cls(smallest=jiff_unit, mode=jiff_round_mode, increment=2)
    assert round_obj._smallest() == jiff_unit
    assert round_obj._mode() == jiff_round_mode
    assert round_obj._increment() == 2


@pytest.mark.parametrize(
    "cls", [ry.DateTimeRound, ry.TimestampRound, ry.ZonedDateTimeRound]
)
def test_round_replace(
    cls: type[
        ry.DateTimeRound
        | ry.SignedDurationRound
        | ry.TimestampRound
        | ry.ZonedDateTimeRound
    ],
    jiff_unit: JIFF_UNIT,
    jiff_round_mode: JIFF_ROUND_MODE,
) -> None:
    round_obj = cls()

    replace_smallest = round_obj.smallest(jiff_unit)
    assert replace_smallest._smallest() == jiff_unit

    replace_mode = round_obj.mode(jiff_round_mode)
    assert replace_mode._mode() == jiff_round_mode

    replace_increment = round_obj.increment(2)
    assert replace_increment._increment() == 2

    replace_all = round_obj.replace(
        smallest=jiff_unit, mode=jiff_round_mode, increment=2
    )

    assert replace_all == cls(smallest=jiff_unit, mode=jiff_round_mode, increment=2)
