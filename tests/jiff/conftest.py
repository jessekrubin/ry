from __future__ import annotations

from typing import TYPE_CHECKING, cast

import pytest

if TYPE_CHECKING:
    from ry.ryo3._jiff import _RoundMode
    from ry.ryo3._jiff import _Unit as _JiffUnit

_JIFF_UNITS: tuple[_JiffUnit, ...] = (
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

_JIFF_ROUND_MODES: tuple[_RoundMode, ...] = (
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
def jiff_unit(request: pytest.FixtureRequest) -> _JiffUnit:
    return cast("_JiffUnit", request.param)


@pytest.fixture(params=_JIFF_ROUND_MODES)
def jiff_round_mode(request: pytest.FixtureRequest) -> _RoundMode:
    return cast("_RoundMode", request.param)


@pytest.fixture
def jiff_units() -> tuple[_JiffUnit, ...]:
    return _JIFF_UNITS


@pytest.fixture
def jiff_round_modes() -> tuple[_RoundMode, ...]:
    return _JIFF_ROUND_MODES
