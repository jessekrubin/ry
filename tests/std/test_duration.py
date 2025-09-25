from __future__ import annotations

import datetime as pydt
from math import isnan
from typing import TYPE_CHECKING

import pytest
from hypothesis import assume, given
from hypothesis import strategies as st

import ry

from ..strategies import st_durations

if TYPE_CHECKING:
    from hypothesis.strategies import SearchStrategy

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
    return secs + (nanos // 1_000_000_000) > ry.U64_MAX


def st_duration_args() -> SearchStrategy[tuple[int, int]]:
    """Strategy for `ry.Duration` constructor arguments"""
    return st.tuples(
        st.integers(min_value=0, max_value=ry.U64_MAX),
        st.integers(min_value=0, max_value=ry.U32_MAX),
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


@given(st_duration_args())
def test_duration_constructor_safe(args: tuple[int, int]) -> None:
    secs, nanos = args
    carry = nanos // 1_000_000_000
    assume(secs + carry <= ry.U64_MAX)
    dur = ry.Duration(secs, nanos)
    assert isinstance(dur, ry.Duration)


class TestDurationArithmetic:
    # =========================================================================
    # ADDITION
    # =========================================================================

    @given(st_durations(), st_durations())
    def test_add(self, left: ry.Duration, right: ry.Duration) -> None:
        _expected_secs = left.secs + right.secs
        _expected_nanos = left.nanos + right.nanos
        if _duration_should_over_flow(_expected_secs, _expected_nanos):
            with pytest.raises(OverflowError):
                _ = left + right
            return
        result = left + right
        assert isinstance(result, ry.Duration)

    @given(
        st_durations(),
        st.timedeltas(
            min_value=pydt.timedelta(0),
        ),
    )
    def test_add_with_timedelta(self, left: ry.Duration, right: pydt.timedelta) -> None:
        _expected_secs = left.secs + int(right.total_seconds())
        _expected_nanos = left.nanos + (right.microseconds * 1000)
        if _duration_should_over_flow(_expected_secs, _expected_nanos):
            with pytest.raises(OverflowError):
                _ = left + right
            return

        result = left + right
        assert isinstance(result, ry.Duration)
        result_right = right + left
        assert isinstance(result_right, ry.Duration)
        assert result == result_right

    # =========================================================================
    # SUBTRACTION
    # =========================================================================
    @given(st_durations(), st_durations())
    def test_sub(self, left: ry.Duration, right: ry.Duration) -> None:
        if left < right:
            with pytest.raises(OverflowError):
                _ = left - right
            return
        result = left - right
        assert isinstance(result, ry.Duration)

    @given(
        st_durations(),
        st.timedeltas(
            min_value=pydt.timedelta(0),
        ),
    )
    def test_sub_with_timedelta(self, left: ry.Duration, right: pydt.timedelta) -> None:
        if left < ry.Duration.from_pytimedelta(right):
            with pytest.raises(OverflowError):
                _res = left - right
            return
        result = left - right
        assert isinstance(result, ry.Duration)

    @given(
        st_durations(),
        st.timedeltas(
            min_value=pydt.timedelta(0),
        ),
    )
    def test_sub_with_timedelta_rsub(
        self, left: ry.Duration, right: pydt.timedelta
    ) -> None:
        if left < ry.Duration.from_pytimedelta(right):
            with pytest.raises(OverflowError):
                _res = right - left
            return
        result = left - right
        assert isinstance(result, ry.Duration)

    # =========================================================================
    # DIVISION
    # =========================================================================

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
        assert isinstance(divided, float)
        assert divided == 4.0

    def test_duration_div_timedelta(self) -> None:
        dur = ry.Duration(16, 0)
        pydelta = pydt.timedelta(seconds=4)
        divided = dur / pydelta
        assert isinstance(divided, float)
        assert divided == 4.0

    def test_duration_div_zero_raises_zero_division_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(ZeroDivisionError):
            _r = dur / 0
        with pytest.raises(ZeroDivisionError):
            _r = dur / 0.0
        with pytest.raises(ZeroDivisionError):
            _f = dur / ry.Duration(0, 0)
        with pytest.raises(ZeroDivisionError):
            _f = dur / pydt.timedelta()
        with pytest.raises(ZeroDivisionError):
            dur.div_f32(0.0)
        with pytest.raises(ZeroDivisionError):
            dur.div_f64(0.0)

    def test_div_f32_nan_raises_value_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.div_f32(float("nan"))

    def test_div_f64_nan_raises_value_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.div_f64(float("nan"))

    def test_div_f32_inf_raises_overflow_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.div_f32(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.div_f32(float("-inf"))

    def test_div_f64_inf_raises_overflow_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.div_f64(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.div_f64(float("-inf"))

    def test_div_type_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(TypeError):
            _r = dur / "string"  # type: ignore[operator]
        with pytest.raises(TypeError):
            _r = dur / []  # type: ignore[operator]

    @given(st_durations(), st.floats())
    def test_duration_div_f32(
        self,
        dur: ry.Duration,
        divisor: float,
    ) -> None:
        if divisor == 0:
            with pytest.raises(ZeroDivisionError):
                _dur = dur.div_f32(divisor)
            return
        if isnan(divisor):
            with pytest.raises(ValueError):
                dur.div_f32(divisor)
            return
        if divisor == float("inf") or divisor == float("-inf"):
            with pytest.raises(OverflowError):
                _dur = dur.div_f32(divisor)
            return
        if divisor < 0:
            with pytest.raises((TypeError, OverflowError, ZeroDivisionError)):
                _dur = dur.div_f32(divisor)
            return

        try:
            divided = dur.div_f32(divisor)
            assert isinstance(divided, ry.Duration)
        except OverflowError:
            ...
        except ZeroDivisionError:
            ...

    @given(st_durations(), st.floats())
    def test_duration_div_f64(
        self,
        dur: ry.Duration,
        divisor: float,
    ) -> None:
        if divisor == 0:
            with pytest.raises(ZeroDivisionError):
                _dur = dur.div_f64(divisor)
            return
        if isnan(divisor):
            with pytest.raises(ValueError):
                dur.div_f64(divisor)
            return
        if divisor == float("inf") or divisor == float("-inf"):
            with pytest.raises(OverflowError):
                _dur = dur.div_f64(divisor)
            return
        if divisor < 0:
            with pytest.raises((TypeError, ZeroDivisionError)):
                _dur = dur.div_f64(divisor)
            return
        try:
            divided = dur.div_f64(divisor)
            assert isinstance(divided, ry.Duration)
        except OverflowError:
            ...
        except ZeroDivisionError:
            ...

    @given(st_durations(), st_durations())
    def test_div_duration_f32(
        self,
        left: ry.Duration,
        right: ry.Duration,
    ) -> None:
        if right.is_zero:
            with pytest.raises(ZeroDivisionError):
                _r = left.div_duration_f32(right)
            return
        try:
            result = left.div_duration_f32(right)
            assert isinstance(result, float)
        except OverflowError:
            ...

    @given(st_durations(), st_durations())
    def test_div_duration_f64(
        self,
        left: ry.Duration,
        right: ry.Duration,
    ) -> None:
        if right.is_zero:
            with pytest.raises(ZeroDivisionError):
                _r = left.div_duration_f64(right)
            return
        try:
            result = left.div_duration_f64(right)
            assert isinstance(result, float)
        except OverflowError:
            ...

    # =========================================================================
    # MULTIPLICATION
    # =========================================================================
    @given(st_durations(), st.integers(min_value=0, max_value=ry.U32_MAX))
    def test_duration_mul_int(
        self,
        dur: ry.Duration,
        factor: int,
    ) -> None:
        try:
            multiplied = dur * factor
            assert isinstance(multiplied, ry.Duration)
        except OverflowError:
            ...

    @given(st_durations(), st.integers(min_value=0, max_value=ry.U32_MAX))
    def test_duration_rmul_int(
        self,
        dur: ry.Duration,
        factor: int,
    ) -> None:
        try:
            multiplied = factor * dur
            assert isinstance(multiplied, ry.Duration)
        except OverflowError:
            ...

    @given(st_durations(), st.floats(width=32))
    def test_duration_mul_f32(
        self,
        dur: ry.Duration,
        factor: float,
    ) -> None:
        if factor == 0:
            assert dur.mul_f32(factor) == ry.Duration(0, 0)
            return

        if isnan(factor):
            with pytest.raises(ValueError):
                dur.mul_f32(factor)
            return
        if factor == float("inf") or factor == float("-inf"):
            with pytest.raises(OverflowError):
                _dur = dur.mul_f32(factor)
            return
        if factor < 0:
            with pytest.raises(TypeError):
                _dur = dur.mul_f32(factor)
            return
        try:
            divided = dur.mul_f32(factor)
            assert isinstance(divided, ry.Duration)
        except OverflowError:
            ...

    @given(st_durations(), st.floats())
    def test_duration_mul_f64(
        self,
        dur: ry.Duration,
        factor: float,
    ) -> None:
        if factor == 0:
            assert dur.mul_f64(factor) == ry.Duration(0, 0)
            return

        if isnan(factor):
            with pytest.raises(ValueError):
                dur.mul_f64(factor)
            return
        if factor == float("inf") or factor == float("-inf"):
            with pytest.raises(OverflowError):
                _dur = dur.mul_f64(factor)
            return
        if factor < 0:
            with pytest.raises(TypeError):
                _dur = dur.mul_f64(factor)
            return
        try:
            divided = dur.mul_f64(factor)
            assert isinstance(divided, ry.Duration)
        except OverflowError:
            ...

    def test_mul_f32_nan_raises_value_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.mul_f32(float("nan"))

    def test_mul_f64_nan_raises_value_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.mul_f64(float("nan"))

    def test_mul_f32_inf_raises_overflow_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.mul_f32(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.mul_f32(float("-inf"))

    def test_mul_f64_inf_raises_overflow_error(self) -> None:
        dur = ry.Duration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.mul_f64(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.mul_f64(float("-inf"))

    # =========================================================================
    # ABS_DIFF
    # =========================================================================
    @given(st_durations(), st_durations())
    def test_abs_diff(self, left: ry.Duration, right: ry.Duration) -> None:
        result = ry.Duration.abs_diff(left, right)
        assert isinstance(result, ry.Duration)

    @given(
        st_durations(),
        st.timedeltas(
            min_value=pydt.timedelta(0),
        ),
    )
    def test_abs_diff_with_timedelta(
        self, left: ry.Duration, right: pydt.timedelta
    ) -> None:
        result = ry.Duration.abs_diff(left, right)
        assert isinstance(result, ry.Duration)


class TestDurationOverflows:
    @given(st.integers())
    def test_duration_from_weeks(self, n: int) -> None:
        try:
            dur = ry.Duration.from_weeks(n)
            assert isinstance(dur, ry.Duration)
        except OverflowError:
            ...


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
        assert ry.Duration.ZERO.is_zero

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


def test_duration_properties() -> None:
    dur = ry.Duration(90061, 123456789)

    assert dur.days == 1
    assert dur.subsec_nanos == 123456789
    assert dur.subsec_micros == 123456
    assert dur.microseconds == 123456
    assert dur.is_zero is False
    assert dur.subsec_millis == 123
    assert dur.secs == 90061
    assert dur.nanos == 123456789


class TestDurationAs:
    def test_as_secs_f32(self) -> None:
        dur = ry.Duration(1, 500_000_000)
        assert dur.as_secs_f32() == 1.5
        assert dur.as_secs() == 1

    def test_as_secs_f64(self) -> None:
        dur = ry.Duration(1, 500_000_000)
        assert dur.as_secs_f64() == 1.5

    def test_as_millis(self) -> None:
        dur = ry.Duration(1, 500_000_000)
        assert dur.as_millis() == 1500

    def test_as_micros(self) -> None:
        dur = ry.Duration(1, 500_000_000)
        assert dur.as_micros() == 1_500_000

    def test_as_nanos(self) -> None:
        dur = ry.Duration(1, 500_000_000)
        assert dur.as_nanos() == 1_500_000_000


class TestDurationFromIntegers:
    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_secs(self, secs: int) -> None:
        dur = ry.Duration.from_secs(secs)
        assert isinstance(dur, ry.Duration)
        assert dur.secs == secs
        assert dur.nanos == 0

    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_millis(self, millis: int) -> None:
        dur = ry.Duration.from_millis(millis)
        assert isinstance(dur, ry.Duration)
        assert dur.secs == millis // 1000
        assert dur.nanos == (millis % 1000) * 1_000_000

    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_micros(self, micros: int) -> None:
        dur = ry.Duration.from_micros(micros)
        assert isinstance(dur, ry.Duration)
        assert dur.secs == micros // 1_000_000
        assert dur.nanos == (micros % 1_000_000) * 1_000

    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_nanos(self, nanos: int) -> None:
        dur = ry.Duration.from_nanos(nanos)
        assert isinstance(dur, ry.Duration)
        assert dur.secs == nanos // 1_000_000_000
        assert dur.nanos == nanos % 1_000_000_000

    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_hours(self, hours: int) -> None:
        if hours > ry.U64_MAX // 3600:
            with pytest.raises(OverflowError):
                _dur = ry.Duration.from_hours(hours)
        else:
            dur = ry.Duration.from_hours(hours)
            assert isinstance(dur, ry.Duration)

    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_mins(self, minutes: int) -> None:
        if minutes > ry.U64_MAX // 60:
            with pytest.raises(OverflowError):
                _dur = ry.Duration.from_mins(minutes)
        else:
            dur = ry.Duration.from_mins(minutes)
            assert isinstance(dur, ry.Duration)

    @given(st.integers(min_value=0, max_value=ry.U64_MAX))
    def test_from_days(self, days: int) -> None:
        if days > ry.U64_MAX // 86400:
            with pytest.raises(OverflowError):
                _dur = ry.Duration.from_days(days)
        else:
            dur = ry.Duration.from_days(days)
            assert isinstance(dur, ry.Duration)

    @given(st.floats(width=32))
    def test_from_secs_f32(self, secs: float) -> None:
        if isnan(secs):
            with pytest.raises(ValueError):
                _dur = ry.Duration.from_secs_f32(secs)
        elif secs < 0.0:
            with pytest.raises((TypeError, OverflowError)):
                _dur = ry.Duration.from_secs_f32(secs)
        elif secs == float("inf"):
            with pytest.raises(OverflowError):
                _dur = ry.Duration.from_secs_f32(secs)
        else:
            try:
                dur = ry.Duration.from_secs_f32(secs)
                assert isinstance(dur, ry.Duration)
            except OverflowError:
                ...

    @given(st.floats())
    def test_from_secs_f64(self, secs: float) -> None:
        if isnan(secs):
            with pytest.raises(ValueError):
                _dur = ry.Duration.from_secs_f64(secs)
        elif secs < 0.0:
            with pytest.raises((TypeError, OverflowError)):
                _dur = ry.Duration.from_secs_f64(secs)
        elif secs == float("inf"):
            with pytest.raises(OverflowError):
                _dur = ry.Duration.from_secs_f64(secs)
        else:
            try:
                dur = ry.Duration.from_secs_f64(secs)
                assert isinstance(dur, ry.Duration)
            except OverflowError:
                ...


class TestDurationCheckedArithmetic:
    @given(st_durations(), st_durations())
    def test_checked_add(self, left: ry.Duration, right: ry.Duration) -> None:
        result = left.checked_add(right)
        assert result is None or isinstance(result, ry.Duration)

    @given(st_durations(), st_durations())
    def test_checked_sub(self, left: ry.Duration, right: ry.Duration) -> None:
        result = left.checked_sub(right)
        assert result is None or isinstance(result, ry.Duration)

    @given(st_durations(), st.integers(min_value=0, max_value=ry.U32_MAX))
    def test_checked_mul(self, dur: ry.Duration, factor: int) -> None:
        result = dur.checked_mul(factor)
        assert result is None or isinstance(result, ry.Duration)

    @given(st_durations(), st.integers(min_value=1, max_value=ry.U32_MAX))
    def test_checked_div(self, dur: ry.Duration, divisor: int) -> None:
        result = dur.checked_div(divisor)
        assert result is None or isinstance(result, ry.Duration)


class TestDurationSaturatingArithmetic:
    @given(st_durations(), st_durations())
    def test_saturating_add(self, left: ry.Duration, right: ry.Duration) -> None:
        result = left.saturating_add(right)
        assert isinstance(result, ry.Duration)

    @given(st_durations(), st_durations())
    def test_saturating_sub(self, left: ry.Duration, right: ry.Duration) -> None:
        result = left.saturating_sub(right)
        assert isinstance(result, ry.Duration)

    @given(st_durations(), st.integers(min_value=0, max_value=ry.U32_MAX))
    def test_saturating_mul(self, dur: ry.Duration, factor: int) -> None:
        result = dur.saturating_mul(factor)
        assert isinstance(result, ry.Duration)
