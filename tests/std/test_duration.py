from __future__ import annotations

import datetime as pydt

import pytest
from hypothesis import assume, given
from hypothesis import strategies as st
from hypothesis.strategies import DrawFn, SearchStrategy

import ry
from ry import Duration

from ..strategies import MAX_U32, MAX_U64

timedelta_positive_strategy = st.timedeltas(
    min_value=pydt.timedelta(0), max_value=pydt.timedelta(days=365 * 100)
)
timedelta_negative_strategy = st.timedeltas(
    min_value=pydt.timedelta(days=-365 * 100), max_value=pydt.timedelta(0)
)


def _duration_should_over_flow(
    secs: int,
    nanos: int,
) -> bool:
    return secs + (nanos // 1_000_000_000) > MAX_U64


@st.composite
def st_duration(draw: DrawFn) -> Duration:
    secs = draw(st.integers(min_value=0, max_value=MAX_U64))
    nanos = draw(st.integers(min_value=0, max_value=MAX_U32))
    carry = nanos // 1_000_000_000
    assume(secs + carry <= MAX_U64)
    return Duration(secs, nanos)


def st_duration_args() -> SearchStrategy[tuple[int, int]]:
    """Strategy for `ry.Duration` constructor arguments"""
    return st.tuples(
        st.integers(min_value=0, max_value=MAX_U64),
        st.integers(min_value=0, max_value=MAX_U32),
    )


@given(st_duration_args())
def test_duration_new(duration_args: tuple[int, int]) -> None:
    secs, nanos = duration_args
    if _duration_should_over_flow(secs, nanos):
        with pytest.raises(OverflowError):
            ry.Duration(secs, nanos)
    else:
        dur = ry.Duration(secs, nanos)
        assert isinstance(dur, ry.Duration)


class TestDurationArithmetic:
    def test_duration_div_number(self) -> None:
        dur = ry.Duration(16, 0)
        divided = dur / 2
        assert isinstance(divided, ry.Duration)
        assert divided == ry.Duration(8, 0)

        divided = dur / 2.0
        assert isinstance(divided, ry.Duration)
        assert divided == ry.Duration(8, 0)

    def test_duration_div_duration(self) -> None:
        dur1 = ry.Duration(16, 0)
        dur2 = ry.Duration(4, 0)
        divided = dur1 / dur2
        assert isinstance(divided, ry.Duration)
        assert divided == ry.Duration(4, 0)

    def test_duration_div_timedelta(self) -> None:
        dur = ry.Duration(16, 0)
        pydelta = pydt.timedelta(seconds=4)
        divided = dur / pydelta
        assert isinstance(divided, ry.Duration)
        assert divided == ry.Duration(4, 0)

    def test_duration_div_zero_raises_zero_division_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(ZeroDivisionError):
            _r = dur / 0
        with pytest.raises(ZeroDivisionError):
            _r = dur / 0.0
        with pytest.raises(ZeroDivisionError):
            dur.div_f32(0.0)
        with pytest.raises(ZeroDivisionError):
            dur.div_f64(0.0)

    @given(st_duration(), st.floats())
    def test_duration_div_f32(
        self,
        dur: Duration,
        divisor: float,
    ) -> None:
        if divisor == 0:
            with pytest.raises(ZeroDivisionError):
                dur.div_f32(divisor)
            return
        if divisor < 0:
            with pytest.raises((ValueError, ZeroDivisionError)):
                dur.div_f32(divisor)
            return
        if (
            divisor == float("nan")
            or divisor == float("inf")
            or divisor == float("-inf")
        ):
            with pytest.raises(ValueError):
                dur.div_f32(divisor)
            return

        try:
            divided = dur.div_f32(divisor)
            assert isinstance(divided, ry.Duration)
        except ValueError:
            pass
        except ZeroDivisionError:
            pass

    @given(st_duration(), st.floats())
    def test_duration_div_f64(
        self,
        dur: Duration,
        divisor: float,
    ) -> None:
        if divisor == 0:
            with pytest.raises(ZeroDivisionError):
                _dur = dur.div_f64(divisor)
            return
        if divisor < 0:
            with pytest.raises((ValueError, ZeroDivisionError)):
                _dur = dur.div_f64(divisor)
            return
        if (
            divisor == float("nan")
            or divisor == float("inf")
            or divisor == float("-inf")
        ):
            with pytest.raises(ValueError):
                dur.div_f64(divisor)
            return
        try:
            divided = dur.div_f64(divisor)
            assert isinstance(divided, ry.Duration)
        except ValueError:
            pass
        except ZeroDivisionError:
            pass


@given(st_duration_args())
def test_duration_constructor_safe(args: tuple[int, int]) -> None:
    secs, nanos = args
    carry = nanos // 1_000_000_000
    assume(secs + carry <= MAX_U64)
    dur = Duration(secs, nanos)
    assert isinstance(dur, Duration)


class TestDurationOverflows:
    @given(st.integers())
    def test_duration_from_weeks(self, n: int) -> None:
        try:
            dur = ry.Duration.from_weeks(n)
            assert isinstance(dur, ry.Duration)
        except OverflowError:
            pass


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
        roundtrip_via_to_py = ryduration.to_py()
        assert isinstance(roundtrip, pydt.timedelta)
        assert roundtrip == pydelta
        assert isinstance(roundtrip_via_to_py, pydt.timedelta)
        assert roundtrip_via_to_py == pydelta

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


def test_duration_equality_w_timedelta() -> None:
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


class TestDurationDunders:
    def test_boolean(self) -> None:
        assert ry.Duration(0, 0) == ry.Duration.ZERO
        assert ry.Duration(1, 0)
        assert ry.Duration(0, 1)
        assert ry.Duration(1, 1)
        assert ry.Duration(0, 1000000000)
        assert ry.Duration(1000000000, 0)
        assert not bool(ry.Duration(0, 0))
        assert bool(ry.Duration(1, 0))

    def test_hash(self) -> None:
        assert ry.Duration(0, 0) == ry.Duration.ZERO
        assert ry.Duration(1, 0)
        assert ry.Duration(0, 1)
        assert ry.Duration(1, 1)
        assert ry.Duration(0, 1000000000)
        assert ry.Duration(1000000000, 0)

        d = {ry.Duration(1, 2): "test"}
        assert d[ry.Duration(1, 2)] == "test"
        assert d[ry.Duration(1, 2)] != "not test"

    def test_float(self) -> None:
        assert float(ry.Duration(0, 0)) == 0.0
        assert float(ry.Duration(1, 0)) == 1.0
        assert float(ry.Duration(0, 1)) == 0.000_000_001
        assert float(ry.Duration(1, 500_000_000)) == 1.5
        assert float(ry.Duration(0, 1_000_000_000)) == 1.0
        assert float(ry.Duration(1_000_000_000, 0)) == 1_000_000_000.0
        assert float(ry.Duration(1, 1000000000)) == 2

    def test_int(self) -> None:
        assert int(ry.Duration(0, 0)) == 0
        assert int(ry.Duration(1, 0)) == 1_000_000_000
        assert int(ry.Duration(0, 1)) == 1
        assert int(ry.Duration(1, 1)) == 1_000_000_001
        assert int(ry.Duration(1, 500_000_000)) == 1_500_000_000
