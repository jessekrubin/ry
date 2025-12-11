from __future__ import annotations

import pickle
import typing as t

import pytest

import ry
from ry import Date, DateTime, Time, Timestamp, ZonedDateTime

if t.TYPE_CHECKING:
    from ry.ryo3 import JiffRoundMode, JiffUnit

DiffType: t.TypeAlias = (
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


class _DifferenceClasses(t.TypedDict):
    diff_type: type[DiffType]
    obj: Date | DateTime | Time | Timestamp | ZonedDateTime
    cls_name: str
    dict_key: str
    smallest_default: JiffUnit


_DIFFERENCE_CLASSES: list[_DifferenceClasses] = [
    {
        "diff_type": ry.DateDifference,
        "obj": Date.today(),
        "cls_name": "DateDifference",
        "dict_key": "date",
        "smallest_default": "day",
    },
    {
        "diff_type": ry.DateTimeDifference,
        "obj": DateTime.now(),
        "cls_name": "DateTimeDifference",
        "dict_key": "datetime",
        "smallest_default": "nanosecond",
    },
    {
        "diff_type": ry.TimeDifference,
        "obj": Time(hour=1, minute=30, second=15),
        "cls_name": "TimeDifference",
        "dict_key": "time",
        "smallest_default": "nanosecond",
    },
    {
        "diff_type": ry.TimestampDifference,
        "obj": Timestamp.now(),
        "cls_name": "TimestampDifference",
        "dict_key": "timestamp",
        "smallest_default": "nanosecond",
    },
    {
        "diff_type": ry.ZonedDateTimeDifference,
        "obj": ZonedDateTime.now(),
        "cls_name": "ZonedDateTimeDifference",
        "dict_key": "zoned",
        "smallest_default": "nanosecond",
    },
]


class _DifferenceOptions(t.TypedDict):
    largest: JiffUnit | None
    smallest: JiffUnit | None
    mode: JiffRoundMode


@pytest.fixture(params=[None, *_JIFF_UNITS])
def jiff_unit_smallest(request: pytest.FixtureRequest) -> JiffUnit | None:
    return t.cast("JiffUnit | None", request.param)


@pytest.fixture(params=[None, *_JIFF_UNITS])
def jiff_unit_largest(request: pytest.FixtureRequest) -> JiffUnit | None:
    return t.cast("JiffUnit | None", request.param)


@pytest.fixture()
def diff_opts(
    jiff_unit_smallest: JiffUnit | None,
    jiff_unit_largest: JiffUnit | None,
    jiff_round_mode: JiffRoundMode,
) -> _DifferenceOptions:
    return _DifferenceOptions(
        smallest=jiff_unit_smallest,
        largest=jiff_unit_largest,
        mode=jiff_round_mode,
    )


@pytest.mark.parametrize(
    "diff_cls",
    _DIFFERENCE_CLASSES,
)
def test_difference_obj(
    diff_cls: _DifferenceClasses,
    diff_opts: _DifferenceOptions,
) -> None:
    obj = diff_cls["obj"]
    _kwargs_no_none = {k: v for k, v in diff_opts.items() if v is not None}
    diff_ob = diff_cls["diff_type"](obj, increment=2, **_kwargs_no_none)  # type: ignore[arg-type]
    if diff_opts["smallest"] is None:
        assert diff_ob.smallest == diff_cls["smallest_default"]
    else:
        assert diff_ob.smallest == diff_opts["smallest"]

    if diff_opts["largest"] is None:
        assert diff_ob.largest is None
    else:
        assert diff_ob.largest == diff_opts["largest"]

    assert diff_ob.mode == diff_opts["mode"]
    assert diff_ob.increment == 2

    expected_dict = {
        diff_cls["dict_key"]: obj,
        "smallest": diff_opts["smallest"]
        if diff_opts["smallest"] is not None
        else diff_cls["smallest_default"],
        "largest": diff_opts["largest"] if diff_opts["largest"] is not None else None,
        "mode": diff_opts["mode"],
        "increment": 2,
    }

    diff_dict = diff_ob.to_dict()
    assert diff_dict == expected_dict

    diff_ob_repr = repr(diff_ob)
    assert isinstance(diff_ob_repr, str)

    expected_repr_str = (
        (
            f'{diff_cls["cls_name"]}({obj!r}, smallest="{diff_ob.smallest}", '
            f'largest="{diff_opts["largest"]}", mode="{diff_ob.mode}", increment={diff_ob.increment})'
        )
        if diff_ob.largest is not None
        else (
            f'{diff_cls["cls_name"]}({obj!r}, smallest="{diff_ob.smallest}", '
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
