from __future__ import annotations

import datetime as pydt
from typing import TYPE_CHECKING

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

from ..strategies import st_i32, st_i64

if TYPE_CHECKING:
    from hypothesis.strategies import SearchStrategy


def st_signed_duration_args() -> SearchStrategy[tuple[int, int]]:
    """Strategy for `ry.Duration` constructor arguments"""
    return st.tuples(st_i64, st_i32)


@given(st_signed_duration_args())
def test_signed_duration_new(duration_args: tuple[int, int]) -> None:
    secs, nanos = duration_args
    try:
        dur = ry.SignedDuration(secs, nanos)
        assert isinstance(dur, ry.SignedDuration)
    except OverflowError:
        ...


def test_duration_from_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    assert pydelta.days == 1
    assert pydelta.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert pydelta.microseconds == 5
    assert ryduration.days == 1
    assert ryduration.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert ryduration.microseconds == 5


def test_duration_2_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    roundtrip = ryduration.to_pytimedelta()
    assert isinstance(roundtrip, pydt.timedelta)
    assert roundtrip == pydelta


def test_equality() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    pydelta2 = pydt.timedelta(days=0, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration2 = ry.SignedDuration.from_pytimedelta(pydelta2)
    assert ryduration != ryduration2
    assert ryduration == ryduration
    assert ryduration2 == ryduration2
    assert ryduration2 != ryduration

    assert ryduration == pydelta
    assert pydelta == ryduration
    assert ryduration2 == pydelta2
    assert pydelta2 == ryduration2

    assert ryduration != pydelta2
    assert pydelta2 != ryduration
    assert ryduration2 != pydelta
    assert pydelta != ryduration2


class TestSignedDurationAbs:
    def test_signed_duration_abs(self) -> None:
        sd = ry.SignedDuration(1, -1_999_999_999)

        assert abs(sd) == ry.SignedDuration(0, 999_999_999)
        assert sd.abs() == ry.SignedDuration(0, 999_999_999)

    def test_unsigned_abs(self) -> None:
        sd = ry.SignedDuration.MIN

        dur = sd.unsigned_abs()
        assert dur == ry.Duration(secs=9223372036854775808, nanos=999999999)


class TestSignedDurationStrings:
    def test_signed_duration_string(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert sd.string() == "PT2H30M"

    def test_signed_duration_parse(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert sd.string(friendly=True) == "2h 30m"
        assert sd.friendly() == "2h 30m"
        assert sd.string() == "PT2H30M"
        assert f"{sd}" == "PT2H30M"
        assert f"{sd:#}" == "2h 30m"
        with pytest.raises(TypeError):
            assert sd.string(True) == "2h 30m"  # type: ignore[misc]  # noqa: FBT003

    def test_invalid_format_specifier(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        with pytest.raises(TypeError):
            assert f"{sd:invalid}" == "PT2H30M"


class TestSignedDurationRound:
    def test_signed_duration_round(self) -> None:
        dur = ry.SignedDuration(4 * 60 * 60 + 17 * 60 + 1, 123_456_789)

        rounded = dur.round(smallest="second", mode="expand", increment=30)
        assert rounded == ry.SignedDuration.from_secs(4 * 60 * 60 + 17 * 60 + 30)

    def test_signed_duration_round_object(self) -> None:
        dur = ry.SignedDuration(4 * 60 * 60 + 17 * 60 + 1, 123_456_789)

        rounded = dur._round(
            ry.SignedDurationRound(smallest="second", mode="expand", increment=30)
        )

        assert rounded == ry.SignedDuration.from_secs(4 * 60 * 60 + 17 * 60 + 30)
