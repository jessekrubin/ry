from __future__ import annotations

from typing import TYPE_CHECKING, cast

import pytest

if TYPE_CHECKING:
    from ry.ryo3 import JiffRoundMode, JiffUnit

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

_JIFF_ROUND_MODES: tuple[JiffRoundMode, ...] = (
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half-ceil",
    "half-floor",
    "half-expand",
    "half-trunc",
    "half-even",
)


@pytest.fixture(params=_JIFF_UNITS)
def jiff_unit(request: pytest.FixtureRequest) -> JiffUnit:
    return cast("JiffUnit", request.param)


@pytest.fixture(params=_JIFF_ROUND_MODES)
def jiff_round_mode(request: pytest.FixtureRequest) -> JiffRoundMode:
    return cast("JiffRoundMode", request.param)


@pytest.fixture()
def jiff_units() -> tuple[JiffUnit, ...]:
    return _JIFF_UNITS


@pytest.fixture()
def jiff_round_modes() -> tuple[JiffRoundMode, ...]:
    return _JIFF_ROUND_MODES
