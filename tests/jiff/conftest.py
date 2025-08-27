from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from ry.ryo3 import JIFF_ROUND_MODE, JIFF_UNIT

_JIFF_UNITS = (
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

_JIFF_ROUND_MODES = (
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
def jiff_unit(request: pytest.FixtureRequest) -> JIFF_UNIT:
    return request.param


@pytest.fixture(params=_JIFF_ROUND_MODES)
def jiff_round_mode(request: pytest.FixtureRequest) -> JIFF_ROUND_MODE:
    return request.param


@pytest.fixture()
def jiff_units() -> tuple[JIFF_UNIT, ...]:
    return _JIFF_UNITS


@pytest.fixture()
def jiff_round_modes() -> tuple[JIFF_ROUND_MODE, ...]:
    return _JIFF_ROUND_MODES
