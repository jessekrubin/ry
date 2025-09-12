from __future__ import annotations

import pickle
from typing import TYPE_CHECKING, TypeAlias

import pytest

import ry

if TYPE_CHECKING:
    from ry.ryo3 import JiffRoundMode, JiffUnit


_ROUND_CLASSES = (
    ry.DateTimeRound,
    ry.OffsetRound,
    ry.SignedDurationRound,
    ry.TimeRound,
    ry.TimestampRound,
    ry.ZonedDateTimeRound,
)
RoundType: TypeAlias = (
    ry.DateTimeRound
    | ry.OffsetRound
    | ry.SignedDurationRound
    | ry.TimeRound
    | ry.TimestampRound
    | ry.ZonedDateTimeRound
)


@pytest.mark.parametrize("cls", _ROUND_CLASSES)
def test_round_getters(
    cls: type[RoundType],
    jiff_unit: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    round_obj = cls(smallest=jiff_unit, mode=jiff_round_mode, increment=2)  # type: ignore[arg-type]
    assert round_obj.smallest == jiff_unit
    assert round_obj.mode == jiff_round_mode
    assert round_obj.increment == 2


@pytest.mark.parametrize("cls", _ROUND_CLASSES)
def test_round_obj_to_dict(
    cls: type[RoundType],
    jiff_unit: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    round_obj = cls(smallest=jiff_unit, mode=jiff_round_mode, increment=2)  # type: ignore[arg-type]
    round_dict = round_obj.to_dict()
    assert round_dict == {
        "smallest": jiff_unit,
        "mode": jiff_round_mode,
        "increment": 2,
    }


@pytest.mark.parametrize("cls", _ROUND_CLASSES)
def test_round_pickling(
    cls: type[RoundType],
    jiff_unit: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    round_obj = cls(smallest=jiff_unit, mode=jiff_round_mode, increment=2)  # type: ignore[arg-type]
    pickled = pickle.dumps(round_obj)
    unpickled = pickle.loads(pickled)
    assert round_obj == unpickled


@pytest.mark.parametrize("cls", _ROUND_CLASSES)
def test_round_replace(
    cls: type[RoundType],
    jiff_unit: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    round_obj = cls()

    replace_smallest = round_obj._smallest(jiff_unit)  # type: ignore[arg-type]
    assert replace_smallest.smallest == jiff_unit

    replace_mode = round_obj._mode(jiff_round_mode)
    assert replace_mode.mode == jiff_round_mode

    replace_increment = round_obj._increment(2)
    assert replace_increment.increment == 2

    replace_all = round_obj.replace(
        smallest=jiff_unit,  # type: ignore[arg-type]
        mode=jiff_round_mode,
        increment=2,
    )

    assert replace_all == cls(smallest=jiff_unit, mode=jiff_round_mode, increment=2)  # type: ignore[arg-type]


@pytest.mark.parametrize("cls", _ROUND_CLASSES)
def test_round_obj_defaults(
    cls: type[RoundType],
) -> None:
    round_obj = cls()
    round_dict = round_obj.to_dict()
    if cls is ry.OffsetRound:  # only OffsetRound defaults to "second"
        assert round_dict == {
            "smallest": "second",
            "mode": "half-expand",
            "increment": 1,
        }
    else:
        assert round_dict == {
            "smallest": "nanosecond",
            "mode": "half-expand",
            "increment": 1,
        }
