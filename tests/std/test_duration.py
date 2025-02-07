from __future__ import annotations

import datetime as pydt

import pytest
from hypothesis import assume, given
from hypothesis import strategies as st

import ry

timedelta_positive_strategy = st.timedeltas(
    min_value=pydt.timedelta(0), max_value=pydt.timedelta(days=365 * 100)
)
timedelta_negative_strategy = st.timedeltas(
    min_value=pydt.timedelta(days=-365 * 100), max_value=pydt.timedelta(0)
)


class TestDurationPydeltaConversion:
    def test_duration_from_pydelta(self) -> None:
        pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
        ryduration = ry.Duration.from_pytimedelta(pydelta)
        assert pydelta.days == 1
        assert pydelta.seconds == (2 * 60 * 60) + (3 * 60) + 4
        assert pydelta.microseconds == 5
        assert ryduration.days == 1
        assert ryduration.seconds == (2 * 60 * 60) + (3 * 60) + 4
        assert ryduration.microseconds == 5

    def test_duration_2_pydelta(self) -> None:
        pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
        ryduration = ry.Duration.from_pytimedelta(pydelta)
        roundtrip = ryduration.to_pytimedelta()
        assert isinstance(roundtrip, pydt.timedelta)
        assert roundtrip == pydelta

    @given(timedelta_positive_strategy)
    def test_positive_signed_duration_round_trip(self, tdelta: pydt.timedelta) -> None:
        # assume the duration is positive
        assume(tdelta.days >= 0)
        ry_signed_duration = ry.Duration.from_pytimedelta(tdelta)
        assert isinstance(ry_signed_duration, ry.Duration)
        round_trip_tdelta = ry_signed_duration.to_pytimedelta()
        assert isinstance(round_trip_tdelta, pydt.timedelta)
        assert round_trip_tdelta == tdelta

    @given(timedelta_negative_strategy)
    def test_negative_signed_duration_round_trip(self, tdelta: pydt.timedelta) -> None:
        # assume the duration is negative
        assume(tdelta.days < 0)
        with pytest.raises(ValueError):
            _ry_signed_duration = ry.Duration.from_pytimedelta(tdelta)


def test_equality() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.Duration.from_pytimedelta(pydelta)

    pydelta2 = pydt.timedelta(days=0, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration2 = ry.Duration.from_pytimedelta(pydelta2)
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


class TestDurationConstants:
    def test_zero(self) -> None:
        assert ry.Duration.ZERO == ry.Duration(0, 0)

    def test_min(self) -> None:
        assert ry.Duration.MIN == ry.Duration(0, 0)

    def test_max(self) -> None:
        assert ry.Duration.MAX == ry.Duration(
            secs=18446744073709551615, nanos=999999999
        )

    def test_nanosecond(self) -> None:
        assert ry.Duration.NANOSECOND == ry.Duration(0, 1)

    def test_microsecond(self) -> None:
        assert ry.Duration.MICROSECOND == ry.Duration(0, 1000)

    def test_millisecond(self) -> None:
        assert ry.Duration.MILLISECOND == ry.Duration(0, 1000000)

    def test_second(self) -> None:
        assert ry.Duration.SECOND == ry.Duration(1, 0)
