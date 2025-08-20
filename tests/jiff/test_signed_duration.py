from __future__ import annotations

import datetime as pydt

import pytest
from hypothesis import given
from hypothesis import strategies as st
from hypothesis.strategies import SearchStrategy

import ry

from ..strategies import st_i32, st_i64


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
        with pytest.raises(TypeError):
            assert sd.string(True) == "2h 30m"  # type: ignore[misc]
