from __future__ import annotations

import pickle
from typing import TYPE_CHECKING, TypeAlias, cast

import pytest

import ry
from ry import Date, DateTime, Time, Timestamp, ZonedDateTime

if TYPE_CHECKING:
    from ry.ryo3 import JiffRoundMode, JiffUnit

RoundType: TypeAlias = (
    ry.TimeDifference
    | ry.TimestampDifference
    | ry.DateDifference
    | ry.DateTimeDifference
    | ry.ZonedDateTimeDifference
)

_JIFF_UNITS: tuple[JiffUnit, ...] = (
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
)


@pytest.fixture(params=[None, *_JIFF_UNITS])
def jiff_unit_smallest(request: pytest.FixtureRequest) -> JiffUnit | None:
    return cast("JiffUnit | None", request.param)


@pytest.fixture(params=[None, *_JIFF_UNITS])
def jiff_unit_largest(request: pytest.FixtureRequest) -> JiffUnit | None:
    return cast("JiffUnit | None", request.param)


def test_date_difference_obj(
    jiff_unit_smallest: JiffUnit,
    jiff_unit_largest: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    d = Date.today()
    diff_ob = ry.DateDifference(
        d,
        smallest=jiff_unit_smallest,
        largest=jiff_unit_largest,
        mode=jiff_round_mode,
        increment=2,
    )

    if jiff_unit_smallest is None:
        assert diff_ob.smallest == "nanosecond"
    else:
        assert diff_ob.smallest == jiff_unit_smallest

    if jiff_unit_largest is None:
        assert diff_ob.largest is None
    else:
        assert diff_ob.largest == jiff_unit_largest

    assert diff_ob.mode == jiff_round_mode
    assert diff_ob.increment == 2

    expected_dict = {
        "date": d,
        "smallest": jiff_unit_smallest
        if jiff_unit_smallest is not None
        else "nanosecond",
        "largest": jiff_unit_largest if jiff_unit_largest is not None else None,
        "mode": jiff_round_mode,
        "increment": 2,
    }

    diff_dict = diff_ob.to_dict()
    assert diff_dict == expected_dict

    diff_ob_repr = repr(diff_ob)
    assert isinstance(diff_ob_repr, str)

    expected_repr_str = (
        (
            f'DateDifference({d!r}, smallest="{diff_ob.smallest}", '
            f'largest="{jiff_unit_largest}", mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
        if diff_ob.largest is not None
        else (
            f'DateDifference({d!r}, smallest="{diff_ob.smallest}", '
            f'largest=None, mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
    )

    # test repr and eval of repr
    assert repr(diff_ob) == expected_repr_str
    evaled = eval("ry." + diff_ob_repr)
    assert evaled == diff_ob

    # test pickling
    pickled = pickle.dumps(diff_ob)
    unpickled = pickle.loads(pickled)
    assert diff_ob == unpickled


def test_datetime_difference_obj(
    jiff_unit_smallest: JiffUnit,
    jiff_unit_largest: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    d = DateTime.now()
    diff_ob = ry.DateTimeDifference(
        d,
        smallest=jiff_unit_smallest,
        largest=jiff_unit_largest,
        mode=jiff_round_mode,
        increment=2,
    )

    if jiff_unit_smallest is None:
        assert diff_ob.smallest == "nanosecond"
    else:
        assert diff_ob.smallest == jiff_unit_smallest

    if jiff_unit_largest is None:
        assert diff_ob.largest is None
    else:
        assert diff_ob.largest == jiff_unit_largest

    assert diff_ob.mode == jiff_round_mode
    assert diff_ob.increment == 2

    expected_dict = {
        "datetime": d,
        "smallest": jiff_unit_smallest
        if jiff_unit_smallest is not None
        else "nanosecond",
        "largest": jiff_unit_largest if jiff_unit_largest is not None else None,
        "mode": jiff_round_mode,
        "increment": 2,
    }

    diff_dict = diff_ob.to_dict()
    assert diff_dict == expected_dict

    diff_ob_repr = repr(diff_ob)
    assert isinstance(diff_ob_repr, str)

    expected_repr_str = (
        (
            f'DateTimeDifference({d!r}, smallest="{diff_ob.smallest}", '
            f'largest="{jiff_unit_largest}", mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
        if diff_ob.largest is not None
        else (
            f'DateTimeDifference({d!r}, smallest="{diff_ob.smallest}", '
            f'largest=None, mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
    )

    # test repr and eval of repr
    assert repr(diff_ob) == expected_repr_str
    evaled = eval("ry." + diff_ob_repr)
    assert evaled == diff_ob

    # test pickling
    pickled = pickle.dumps(diff_ob)
    unpickled = pickle.loads(pickled)
    assert diff_ob == unpickled


def test_time_difference_obj(
    jiff_unit_smallest: JiffUnit,
    jiff_unit_largest: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    t = Time(hour=1, minute=30, second=15)
    diff_ob = ry.TimeDifference(
        t,
        smallest=jiff_unit_smallest,
        largest=jiff_unit_largest,
        mode=jiff_round_mode,
        increment=2,
    )

    if jiff_unit_smallest is None:
        assert diff_ob.smallest == "nanosecond"
    else:
        assert diff_ob.smallest == jiff_unit_smallest

    if jiff_unit_largest is None:
        assert diff_ob.largest is None
    else:
        assert diff_ob.largest == jiff_unit_largest

    assert diff_ob.mode == jiff_round_mode
    assert diff_ob.increment == 2

    expected_dict = {
        "time": ry.Time(hour=1, minute=30, second=15),
        "smallest": jiff_unit_smallest
        if jiff_unit_smallest is not None
        else "nanosecond",
        "largest": jiff_unit_largest if jiff_unit_largest is not None else None,
        "mode": jiff_round_mode,
        "increment": 2,
    }

    diff_dict = diff_ob.to_dict()
    assert diff_dict == expected_dict

    diff_ob_repr = repr(diff_ob)
    assert isinstance(diff_ob_repr, str)

    expected_repr_str = (
        (
            f'TimeDifference({t!r}, smallest="{diff_ob.smallest}", '
            f'largest="{jiff_unit_largest}", mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
        if diff_ob.largest is not None
        else (
            f'TimeDifference({t!r}, smallest="{diff_ob.smallest}", '
            f'largest=None, mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
    )

    # test repr and eval of repr
    assert repr(diff_ob) == expected_repr_str
    evaled = eval("ry." + diff_ob_repr)
    assert evaled == diff_ob

    # test pickling
    pickled = pickle.dumps(diff_ob)
    unpickled = pickle.loads(pickled)
    assert diff_ob == unpickled


def test_timestamp_difference_obj(
    jiff_unit_smallest: JiffUnit,
    jiff_unit_largest: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    t = Timestamp.now()
    diff_ob = ry.TimestampDifference(
        t,
        smallest=jiff_unit_smallest,
        largest=jiff_unit_largest,
        mode=jiff_round_mode,
        increment=2,
    )

    if jiff_unit_smallest is None:
        assert diff_ob.smallest == "nanosecond"
    else:
        assert diff_ob.smallest == jiff_unit_smallest

    if jiff_unit_largest is None:
        assert diff_ob.largest is None
    else:
        assert diff_ob.largest == jiff_unit_largest

    assert diff_ob.mode == jiff_round_mode
    assert diff_ob.increment == 2

    expected_dict = {
        "timestamp": t,
        "smallest": jiff_unit_smallest
        if jiff_unit_smallest is not None
        else "nanosecond",
        "largest": jiff_unit_largest if jiff_unit_largest is not None else None,
        "mode": jiff_round_mode,
        "increment": 2,
    }

    diff_dict = diff_ob.to_dict()
    assert diff_dict == expected_dict

    diff_ob_repr = repr(diff_ob)
    assert isinstance(diff_ob_repr, str)

    expected_repr_str = (
        (
            f'TimestampDifference({t!r}, smallest="{diff_ob.smallest}", '
            f'largest="{jiff_unit_largest}", mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
        if diff_ob.largest is not None
        else (
            f'TimestampDifference({t!r}, smallest="{diff_ob.smallest}", '
            f'largest=None, mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
    )

    # test repr and eval of repr
    assert repr(diff_ob) == expected_repr_str
    evaled = eval("ry." + diff_ob_repr)
    assert evaled == diff_ob

    # test pickling
    pickled = pickle.dumps(diff_ob)
    unpickled = pickle.loads(pickled)
    assert diff_ob == unpickled


def test_zoned_difference_obj(
    jiff_unit_smallest: JiffUnit,
    jiff_unit_largest: JiffUnit,
    jiff_round_mode: JiffRoundMode,
) -> None:
    t = ZonedDateTime.now()
    diff_ob = ry.ZonedDateTimeDifference(
        t,
        smallest=jiff_unit_smallest,
        largest=jiff_unit_largest,
        mode=jiff_round_mode,
        increment=2,
    )

    if jiff_unit_smallest is None:
        assert diff_ob.smallest == "nanosecond"
    else:
        assert diff_ob.smallest == jiff_unit_smallest

    if jiff_unit_largest is None:
        assert diff_ob.largest is None
    else:
        assert diff_ob.largest == jiff_unit_largest

    assert diff_ob.mode == jiff_round_mode
    assert diff_ob.increment == 2

    expected_dict = {
        "zoned": t,
        "smallest": jiff_unit_smallest
        if jiff_unit_smallest is not None
        else "nanosecond",
        "largest": jiff_unit_largest if jiff_unit_largest is not None else None,
        "mode": jiff_round_mode,
        "increment": 2,
    }

    diff_dict = diff_ob.to_dict()
    assert diff_dict == expected_dict

    diff_ob_repr = repr(diff_ob)
    assert isinstance(diff_ob_repr, str)

    expected_repr_str = (
        (
            f'ZonedDateTimeDifference({t!r}, smallest="{diff_ob.smallest}", '
            f'largest="{jiff_unit_largest}", mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
        if diff_ob.largest is not None
        else (
            f'ZonedDateTimeDifference({t!r}, smallest="{diff_ob.smallest}", '
            f'largest=None, mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
    )

    # test repr and eval of repr
    assert repr(diff_ob) == expected_repr_str
    evaled = eval("ry." + diff_ob_repr)
    assert evaled == diff_ob

    # test pickling
    pickled = pickle.dumps(diff_ob)
    unpickled = pickle.loads(pickled)
    assert diff_ob == unpickled
