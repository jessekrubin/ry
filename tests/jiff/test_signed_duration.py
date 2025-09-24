from __future__ import annotations

import datetime as pydt
from math import isnan

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

from ..strategies import st_i32, st_i64
from .strategies import st_signed_duration_args, st_signed_durations

_NANOS_PER_SEC: int = 1_000_000_000
_NANOS_PER_MILLI: int = 1_000_000
_NANOS_PER_MICRO: int = 1_000
_MILLIS_PER_SEC: int = 1_000
_MICROS_PER_SEC: int = 1_000_000
_SECS_PER_MINUTE: int = 60
_MINS_PER_HOUR: int = 60


@given(st_signed_duration_args())
def test_signed_duration_new(duration_args: tuple[int, int]) -> None:
    secs, nanos = duration_args
    try:
        dur = ry.SignedDuration(secs, nanos)
        assert isinstance(dur, ry.SignedDuration)
    except OverflowError:
        ...


def test_signed_duration_min_max() -> None:
    assert ry.SignedDuration.MIN == ry.SignedDuration(-(1 << 63), -999_999_999)
    assert ry.SignedDuration.MAX == ry.SignedDuration((1 << 63) - 1, 999_999_999)


def test_signed_duration_cmp() -> None:
    left = ry.SignedDuration(1, 2)
    right = ry.SignedDuration(3, 4)
    assert left < right
    assert right > left
    assert left <= right
    assert right >= left
    assert left != right
    assert right != left
    assert left == left
    assert right == right


@given(st_signed_durations())
def test_cast_float(dur: ry.SignedDuration) -> None:
    f = float(dur)
    assert isinstance(f, float)
    if dur.is_zero:
        assert f == 0.0
    assert f == dur.as_secs() + dur.subsec_nanos / 1_000_000_000.0


@given(st_signed_durations())
def test_cast_int(dur: ry.SignedDuration) -> None:
    i = int(dur)
    assert isinstance(i, int)
    assert i == dur.as_nanos()


@given(st_signed_durations())
def test_cast_bool(dur: ry.SignedDuration) -> None:
    b = bool(dur)
    assert isinstance(b, bool)
    assert b == (not dur.is_zero)


def test_signed_duration_cmp_timedelta() -> None:
    left = ry.SignedDuration(1, 2)
    right = ry.SignedDuration(3, 4).to_py()
    assert left < right
    assert right > left
    assert left <= right
    assert right >= left
    assert left != right
    assert right != left
    assert left == left
    assert right == right


def test_duration_from_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    assert pydelta.days == 1
    assert pydelta.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert pydelta.microseconds == 5
    assert ryduration.days == 1
    assert ryduration.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert ryduration.microseconds == 5


def test_truediv() -> None:
    dur = ry.SignedDuration(10, 500_000_000)

    div_by_int = dur / 2
    assert isinstance(div_by_int, ry.SignedDuration)
    assert div_by_int == ry.SignedDuration(5, 250_000_000)

    div_by_float = dur / 2.0
    assert isinstance(div_by_float, ry.SignedDuration)
    assert div_by_float == ry.SignedDuration(5, 250_000_000)

    div_by_duration = dur / ry.SignedDuration(2, 0)
    assert isinstance(div_by_duration, float)
    assert div_by_duration == 5.25

    with pytest.raises(ZeroDivisionError):
        _ = dur / 0
    with pytest.raises(ZeroDivisionError):
        _ = dur / 0.0
    with pytest.raises(ZeroDivisionError):
        _ = dur / ry.SignedDuration(0, 0)

    with pytest.raises(TypeError):
        _ = dur / "string"  # type: ignore[operator]


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


class TestSignedDurationProperties:
    def test_positive(self) -> None:
        dur = ry.SignedDuration(90061, 123456789)

        assert dur.days == 1
        assert dur.subsec_nanos == 123456789
        assert dur.subsec_micros == 123456
        assert dur.microseconds == 123456
        assert dur.is_zero is False
        assert dur.subsec_millis == 123
        assert dur.secs == 90061
        assert dur.nanos == 123456789
        assert dur.is_positive is True
        assert dur.is_negative is False

    def test_negative(self) -> None:
        dur = -ry.SignedDuration(90061, 123456789)

        assert dur.days == -1
        assert dur.subsec_nanos == -123456789
        assert dur.subsec_micros == -123456
        assert dur.microseconds == -123456
        assert dur.is_zero is False
        assert dur.subsec_millis == -123
        assert dur.secs == -90061
        assert dur.nanos == -123456789
        assert dur.is_positive is False
        assert dur.is_negative is True

    def test_zero(self) -> None:
        dur = ry.SignedDuration(0, 0)

        assert dur.days == 0
        assert dur.subsec_nanos == 0
        assert dur.subsec_micros == 0
        assert dur.microseconds == 0
        assert dur.is_zero is True
        assert dur.subsec_millis == 0
        assert dur.secs == 0
        assert dur.nanos == 0
        assert dur.is_positive is False
        assert dur.is_negative is False


class TestSignedDurationStrings:
    def test_signed_duration_string(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert str(sd) == "PT2H30M"

    def test_signed_duration_isoformat(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert sd.isoformat() == "PT2H30M"
        assert ry.SignedDuration.from_isoformat(sd.isoformat()) == sd

    def test_signed_duration_parse(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert sd.to_string(friendly=True) == "2h 30m"
        assert sd.friendly() == "2h 30m"
        assert str(sd) == "PT2H30M"
        assert str(sd) == "PT2H30M"
        assert f"{sd}" == "PT2H30M"
        assert f"{sd:#}" == "2h 30m"
        with pytest.raises(TypeError):
            assert sd.to_string(True) == "2h 30m"  # type: ignore[misc]  # noqa: FBT003

    def test_invalid_format_specifier(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        with pytest.raises(TypeError):
            assert f"{sd:invalid}" == "PT2H30M"


class TestSignedDurationAbs:
    def test_signed_duration_abs(self) -> None:
        sd = ry.SignedDuration(1, -1_999_999_999)

        assert abs(sd) == ry.SignedDuration(0, 999_999_999)
        assert sd.abs() == ry.SignedDuration(0, 999_999_999)

    def test_unsigned_abs(self) -> None:
        sd = ry.SignedDuration.MIN

        dur = sd.unsigned_abs()
        assert dur == ry.Duration(secs=9223372036854775808, nanos=999999999)


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


class TestDurationArithmetic:
    # =========================================================================
    # ADDITION
    # =========================================================================

    @given(st_signed_durations(), st_signed_durations())
    def test_add(self, left: ry.SignedDuration, right: ry.SignedDuration) -> None:
        if left.checked_add(right) is None:
            with pytest.raises(OverflowError):
                _ = left + right
            return

        result = left + right
        assert isinstance(result, ry.SignedDuration)

    @given(
        st_signed_durations(),
        st.timedeltas(
            min_value=pydt.timedelta(0),
        ),
    )
    def test_add_with_timedelta(
        self, left: ry.SignedDuration, right: pydt.timedelta
    ) -> None:
        if left.checked_add(ry.SignedDuration.from_pytimedelta(right)) is None:
            with pytest.raises(OverflowError):
                _ = left + right
            return

        result = left + right
        assert isinstance(result, ry.SignedDuration)
        result_right = right + left
        assert isinstance(result_right, ry.SignedDuration)
        assert result == result_right

    # =========================================================================
    # SUBTRACTION
    # =========================================================================
    @given(st_signed_durations(), st_signed_durations())
    def test_sub(self, left: ry.SignedDuration, right: ry.SignedDuration) -> None:
        if left.checked_sub(right) is None:
            with pytest.raises(OverflowError):
                _res = left - right
            return
        result = left - right
        assert isinstance(result, ry.SignedDuration)

    @given(
        st_signed_durations(),
        st.timedeltas(),
    )
    def test_sub_with_timedelta(
        self, left: ry.SignedDuration, right: pydt.timedelta
    ) -> None:
        if left.checked_sub(ry.SignedDuration.from_pytimedelta(right)) is None:
            with pytest.raises(OverflowError):
                _res = left - right
            return
        result = left - right
        assert isinstance(result, ry.SignedDuration)

    @given(
        st_signed_durations(),
        st.timedeltas(),
    )
    def test_sub_with_timedelta_rsub(
        self, left: ry.SignedDuration, right: pydt.timedelta
    ) -> None:
        if left.checked_sub(ry.SignedDuration.from_pytimedelta(right)) is None:
            with pytest.raises(OverflowError):
                _res = right - left
            return

        result = left - right
        assert isinstance(result, ry.SignedDuration)

    # =========================================================================
    # DIVISION
    # =========================================================================

    def test_div_number(self) -> None:
        dur = ry.SignedDuration(16, 0)
        divided = dur / 2
        assert isinstance(divided, ry.SignedDuration)
        assert divided == ry.SignedDuration(8, 0)

        divided = dur / 2.0
        assert isinstance(divided, ry.SignedDuration)
        assert divided == ry.SignedDuration(8, 0)

    def test_div_self(self) -> None:
        dur1 = ry.SignedDuration(16, 0)
        dur2 = ry.SignedDuration(4, 0)
        divided = dur1 / dur2
        assert isinstance(divided, float)
        assert divided == 4.0

    def test_div_timedelta(self) -> None:
        dur = ry.SignedDuration(16, 0)
        pydelta = pydt.timedelta(seconds=4)
        divided = dur / pydelta
        assert isinstance(divided, float)
        assert divided == 4.0

    def test_div_zero_raises_zero_division_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(ZeroDivisionError):
            _r = dur / 0
        with pytest.raises(ZeroDivisionError):
            _r = dur / 0.0
        with pytest.raises(ZeroDivisionError):
            _f = dur / ry.SignedDuration.ZERO
        with pytest.raises(ZeroDivisionError):
            _f = dur / pydt.timedelta()
        with pytest.raises(ZeroDivisionError):
            dur.div_f32(0.0)
        with pytest.raises(ZeroDivisionError):
            dur.div_f64(0.0)

    def test_div_f32_nan_raises_value_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.div_f32(float("nan"))

    def test_div_f64_nan_raises_value_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.div_f64(float("nan"))

    def test_div_f32_inf_raises_overflow_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.div_f32(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.div_f32(float("-inf"))

    def test_div_f64_inf_raises_overflow_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.div_f64(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.div_f64(float("-inf"))

    def test_div_type_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(TypeError):
            _r = dur / "string"  # type: ignore[operator]
        with pytest.raises(TypeError):
            _r = dur / []  # type: ignore[operator]

    @given(st_signed_durations(), st.floats(width=32))
    def test_div_f32(
        self,
        dur: ry.SignedDuration,
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
            assert isinstance(divided, ry.SignedDuration)
        except OverflowError:
            ...
        except ZeroDivisionError:
            ...

    @given(st_signed_durations(), st.floats())
    def test_div_f64(
        self,
        dur: ry.SignedDuration,
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
            assert isinstance(divided, ry.SignedDuration)
        except OverflowError:
            ...
        except ZeroDivisionError:
            ...

    @given(st_signed_durations(), st_signed_durations())
    def test_div_duration_f32(
        self,
        left: ry.SignedDuration,
        right: ry.SignedDuration,
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

    @given(st_signed_durations(), st_signed_durations())
    def test_div_duration_f64(
        self,
        left: ry.SignedDuration,
        right: ry.SignedDuration,
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
    def test_mul_f32_nan_raises_value_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.mul_f32(float("nan"))

    def test_mul_f64_nan_raises_value_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(ValueError):
            _r = dur.mul_f64(float("nan"))

    def test_mul_f32_inf_raises_overflow_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.mul_f32(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.mul_f32(float("-inf"))

    def test_mul_f64_inf_raises_overflow_error(self) -> None:
        dur = ry.SignedDuration(1, 0)
        with pytest.raises(OverflowError):
            _r = dur.mul_f64(float("inf"))
        with pytest.raises(OverflowError):
            _r = dur.mul_f64(float("-inf"))

    @given(st_signed_durations(), st.integers(min_value=0, max_value=ry.U32_MAX))
    def test_mul_int(
        self,
        dur: ry.SignedDuration,
        factor: int,
    ) -> None:
        try:
            multiplied = dur * factor
            assert isinstance(multiplied, ry.SignedDuration)
        except OverflowError:
            ...

    @given(st_signed_durations(), st.integers(min_value=0, max_value=ry.U32_MAX))
    def test_rmul_int(
        self,
        dur: ry.SignedDuration,
        factor: int,
    ) -> None:
        try:
            multiplied = factor * dur
            assert isinstance(multiplied, ry.SignedDuration)
        except OverflowError:
            ...

    @given(st_signed_durations(), st.floats(width=32))
    def test_duration_mul_f32(
        self,
        dur: ry.SignedDuration,
        factor: float,
    ) -> None:
        if factor == 0:
            assert dur.mul_f32(factor) == ry.SignedDuration(0, 0)
            return

        if isnan(factor):
            with pytest.raises(ValueError):
                dur.mul_f32(factor)
            return
        if factor == float("inf") or factor == float("-inf"):
            with pytest.raises(OverflowError):
                _dur = dur.mul_f32(factor)
            return
        try:
            divided = dur.mul_f32(factor)
            assert isinstance(divided, ry.SignedDuration)
        except OverflowError:
            ...

    @given(st_signed_durations(), st.floats(width=64))
    def test_mul_f64(
        self,
        dur: ry.SignedDuration,
        factor: float,
    ) -> None:
        if factor == 0:
            assert dur.mul_f64(factor) == ry.SignedDuration(0, 0)
            return

        if isnan(factor):
            with pytest.raises(ValueError):
                dur.mul_f64(factor)
            return
        if factor == float("inf") or factor == float("-inf"):
            with pytest.raises(OverflowError):
                _dur = dur.mul_f64(factor)
            return
        try:
            divided = dur.mul_f64(factor)
            assert isinstance(divided, ry.SignedDuration)
        except OverflowError:
            ...


class TestSignedDurationCheckedArithmetic:
    @given(st_signed_durations(), st_signed_durations())
    def test_checked_add(
        self, left: ry.SignedDuration, right: ry.SignedDuration
    ) -> None:
        assert isinstance(left, ry.SignedDuration)
        assert isinstance(right, ry.SignedDuration)
        result = left.checked_add(right)
        assert result is None or isinstance(result, ry.SignedDuration)

    @given(st_signed_durations(), st_signed_durations())
    def test_checked_sub(
        self, left: ry.SignedDuration, right: ry.SignedDuration
    ) -> None:
        assert isinstance(left, ry.SignedDuration)
        assert isinstance(right, ry.SignedDuration)
        result = left.checked_sub(right)
        assert result is None or isinstance(result, ry.SignedDuration)

    @given(
        st_signed_durations(), st.integers(min_value=ry.I32_MIN, max_value=ry.I32_MAX)
    )
    def test_checked_mul(self, dur: ry.SignedDuration, factor: int) -> None:
        assert isinstance(dur, ry.SignedDuration)
        result = dur.checked_mul(factor)
        assert result is None or isinstance(result, ry.SignedDuration)

    @given(st_signed_durations())
    def test_checked_neg(self, dur: ry.SignedDuration) -> None:
        """Returns `None` if the negation does not exist.

        Occurs in precisely the cases when [`SignedDuration::as_secs`] is equal
        to `i64::MIN` (ry.I64_MIN).
        """
        assert isinstance(dur, ry.SignedDuration)
        if dur.as_secs() == ry.I64_MIN:
            assert dur.checked_neg() is None
        else:
            assert dur.checked_neg() == -dur

    @given(
        st_signed_durations(), st.integers(min_value=ry.I32_MIN, max_value=ry.I32_MAX)
    )
    def test_checked_div(self, dur: ry.SignedDuration, divisor: int) -> None:
        assert isinstance(dur, ry.SignedDuration)
        result = dur.checked_div(divisor)
        assert result is None or isinstance(result, ry.SignedDuration)


class TestSignedDurationSaturatingArithmetic:
    @given(st_signed_durations(), st_signed_durations())
    def test_signed_duration_saturating_add(
        self, left: ry.SignedDuration, right: ry.SignedDuration
    ) -> None:
        left_plus_right = left.saturating_add(right)
        right_plus_left = right.saturating_add(left)
        assert left_plus_right == right_plus_left
        assert isinstance(left_plus_right, ry.SignedDuration)
        assert isinstance(right_plus_left, ry.SignedDuration)

    @given(st_signed_durations(), st_signed_durations())
    def test_signed_duration_saturating_sub(
        self, left: ry.SignedDuration, right: ry.SignedDuration
    ) -> None:
        left_minus_right = left.saturating_sub(right)
        right_minus_left = right.saturating_sub(left)
        assert isinstance(left_minus_right, ry.SignedDuration)
        assert isinstance(right_minus_left, ry.SignedDuration)

        if left == right:
            assert left_minus_right == right_minus_left
        if left.is_zero and right.is_zero:
            assert left_minus_right == ry.SignedDuration(0, 0)
            assert right_minus_left == ry.SignedDuration(0, 0)

    @given(st_signed_durations(), st_i32)
    def test_signed_duration_saturating_mul(
        self, dur: ry.SignedDuration, factor: int
    ) -> None:
        dur_times_factor = dur.saturating_mul(factor)
        assert isinstance(dur_times_factor, ry.SignedDuration)
        if factor == 0:
            assert dur_times_factor == ry.SignedDuration(0, 0)
        elif factor == 1:
            assert dur_times_factor == dur
        elif factor == -1:
            assert dur_times_factor == -dur


class TestSignedDurationAsXYZ:
    def test_as_secs_f32(self) -> None:
        dur = ry.SignedDuration(1, 500_000_000)
        assert dur.as_secs_f32() == 1.5
        assert dur.as_secs() == 1
        assert isinstance(dur.as_secs(), int)
        ndur = -dur
        assert ndur.as_secs_f32() == -1.5
        assert ndur.as_secs() == -1
        assert isinstance(ndur.as_secs(), int)

    def test_as_secs_f64(self) -> None:
        dur = ry.SignedDuration(1, 500_000_000)
        assert dur.as_secs_f64() == 1.5
        assert dur.as_secs() == 1
        assert isinstance(dur.as_secs(), int)
        ndur = -dur
        assert ndur.as_secs_f64() == -1.5
        assert ndur.as_secs() == -1
        assert isinstance(ndur.as_secs(), int)

    def test_as_millis_f32(self) -> None:
        dur = ry.SignedDuration(1, 123_456_789)
        assert dur.as_millis_f32() - 1123.456787109375 < 1e-5
        assert dur.as_millis() == 1123
        ndur = -dur
        assert abs(dur.as_millis_f32()) - 1123.456787109375 < 1e-5
        assert ndur.as_millis() == -1123

    def test_as_millis_f64(self) -> None:
        dur = ry.SignedDuration(1, 123_456_789)

        assert dur.as_millis_f64() - 1123.456787109375 < 1e-5
        assert dur.as_millis() == 1123
        ndur = -dur
        assert abs(dur.as_millis_f64()) - 1123.456787109375 < 1e-5
        assert ndur.as_millis() == -1123

    @given(st_signed_durations())
    def test_as_hours(self, dur: ry.SignedDuration) -> None:
        hours = dur.as_hours()
        assert isinstance(hours, int)
        expected = dur.signum() * int(
            abs(dur.secs) // 3600
            + abs(dur.nanos) // (_NANOS_PER_SEC * _SECS_PER_MINUTE * _MINS_PER_HOUR)
        )
        assert hours == expected

    @given(st_signed_durations())
    def test_as_mins(self, dur: ry.SignedDuration) -> None:
        minutes = dur.as_mins()
        assert isinstance(minutes, int)
        expected = dur.signum() * int(
            abs(dur.secs) // 60 + abs(dur.nanos) // 60_000_000_000
        )
        assert minutes == expected

    @given(st_signed_durations())
    def test_as_secs(self, dur: ry.SignedDuration) -> None:
        seconds = dur.as_secs()
        assert isinstance(seconds, int)
        expected = dur.signum() * (
            abs(dur.secs) + dur.signum() * int(abs(dur.nanos) // _NANOS_PER_SEC)
        )
        assert seconds == expected

    @given(st_signed_durations())
    def test_as_millis(self, dur: ry.SignedDuration) -> None:
        millis = dur.as_millis()
        assert isinstance(millis, int)
        expected = dur.signum() * int(
            abs(dur.secs) * _MILLIS_PER_SEC + abs(dur.nanos) // _NANOS_PER_MILLI
        )
        assert millis == expected

    @given(st_signed_durations())
    def test_as_micros(self, dur: ry.SignedDuration) -> None:
        micros = dur.as_micros()
        assert isinstance(micros, int)
        expected = dur.signum() * int(
            abs(dur.secs) * _MICROS_PER_SEC + abs(dur.nanos) // _NANOS_PER_MICRO
        )
        assert micros == expected

    @given(st_signed_durations())
    def test_as_nanos(self, dur: ry.SignedDuration) -> None:
        nanos = dur.as_nanos()
        assert isinstance(nanos, int)
        expected = dur.signum() * (abs(dur.secs) * _NANOS_PER_SEC + abs(dur.nanos))
        assert nanos == expected


class TestSignedDurationFromXYZ:
    @given(st_i64)
    def test_from_hours(self, hours: int) -> None:
        if -2_562_047_788_015_215 <= hours <= 2_562_047_788_015_215:
            dur = ry.SignedDuration.from_hours(hours)
            assert isinstance(dur, ry.SignedDuration)
            expected_secs = hours * _SECS_PER_MINUTE * _MINS_PER_HOUR
            assert dur.secs == expected_secs
            assert dur.nanos == 0
        else:
            with pytest.raises(OverflowError):
                ry.SignedDuration.from_hours(hours)

    @given(st_i64)
    def test_from_mins(self, mins: int) -> None:
        if mins < -153_722_867_280_912_930 or mins > 153_722_867_280_912_930:
            with pytest.raises(OverflowError):
                ry.SignedDuration.from_mins(mins)
        else:
            dur = ry.SignedDuration.from_mins(mins)
            assert isinstance(dur, ry.SignedDuration)
            expected_secs = mins * _SECS_PER_MINUTE
            assert dur.secs == expected_secs
            assert dur.nanos == 0

    @given(st_i64)
    def test_from_secs(self, secs: int) -> None:
        dur = ry.SignedDuration.from_secs(secs)
        assert isinstance(dur, ry.SignedDuration)
        assert dur.secs == secs
        assert dur.nanos == 0

    @given(st_i64)
    def test_from_millis(self, millis: int) -> None:
        dur = ry.SignedDuration.from_millis(millis)
        if millis >= 0:
            assert isinstance(dur, ry.SignedDuration)
            expected_secs = millis // _MILLIS_PER_SEC
            expected_nanos = (millis % _MILLIS_PER_SEC) * _NANOS_PER_MILLI
            assert dur.secs == expected_secs
            assert dur.nanos == expected_nanos
        else:
            expected_secs = abs(millis) // _MILLIS_PER_SEC
            expected_nanos = (abs(millis) % _MILLIS_PER_SEC) * _NANOS_PER_MILLI
            assert dur.secs == -expected_secs
            assert dur.nanos == -expected_nanos
            assert isinstance(dur, ry.SignedDuration)

    @given(st_i64)
    def test_from_micros(self, micros: int) -> None:
        dur = ry.SignedDuration.from_micros(micros)
        if micros >= 0:
            assert isinstance(dur, ry.SignedDuration)
            expected_secs = micros // _MICROS_PER_SEC
            expected_nanos = (micros % _MICROS_PER_SEC) * _NANOS_PER_MICRO
            assert dur.secs == expected_secs
            assert dur.nanos == expected_nanos
        else:
            expected_secs = abs(micros) // _MICROS_PER_SEC
            expected_nanos = (abs(micros) % _MICROS_PER_SEC) * _NANOS_PER_MICRO
            assert dur.secs == -expected_secs
            assert dur.nanos == -expected_nanos
            assert isinstance(dur, ry.SignedDuration)


class TestSignedDurationFromIntegers:
    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_secs(self, secs: int) -> None:
        dur = ry.SignedDuration.from_secs(secs)
        assert isinstance(dur, ry.SignedDuration)
        assert dur.secs == secs
        assert dur.nanos == 0

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_millis(self, millis: int) -> None:
        dur = ry.SignedDuration.from_millis(millis)
        assert isinstance(dur, ry.SignedDuration)
        assert dur.secs == abs(millis) // 1000 * (1 if millis >= 0 else -1)
        assert dur.nanos == (abs(millis) % 1000) * 1_000_000 * (
            1 if millis >= 0 else -1
        )

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_micros(self, micros: int) -> None:
        dur = ry.SignedDuration.from_micros(micros)
        assert isinstance(dur, ry.SignedDuration)
        assert dur.secs == abs(micros) // 1_000_000 * (1 if micros >= 0 else -1)
        assert dur.nanos == (abs(micros) % 1_000_000) * 1_000 * (
            1 if micros >= 0 else -1
        )

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_nanos(self, nanos: int) -> None:
        dur = ry.SignedDuration.from_nanos(nanos)
        assert isinstance(dur, ry.SignedDuration)
        assert dur.secs == abs(nanos) // 1_000_000_000 * (1 if nanos >= 0 else -1)
        assert dur.nanos == (abs(nanos) % 1_000_000_000) * (1 if nanos >= 0 else -1)

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_hours(self, hours: int) -> None:
        if abs(hours) > ry.I64_MAX // 3600:
            with pytest.raises(OverflowError):
                _dur = ry.SignedDuration.from_hours(hours)
        else:
            dur = ry.SignedDuration.from_hours(hours)
            assert isinstance(dur, ry.SignedDuration)

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_mins(self, minutes: int) -> None:
        if abs(minutes) > ry.I64_MAX // 60:
            with pytest.raises(OverflowError):
                _dur = ry.SignedDuration.from_mins(minutes)
        else:
            dur = ry.SignedDuration.from_mins(minutes)
            assert isinstance(dur, ry.SignedDuration)

    def test_from_days(self) -> None:
        with pytest.raises(AttributeError):
            _ = ry.SignedDuration.from_days(1)  # type: ignore[attr-defined]

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_secs_f32(self, secs: float) -> None:
        if isnan(secs):
            with pytest.raises(ValueError):
                _dur = ry.SignedDuration.from_secs_f32(secs)
        elif secs == float("inf"):
            with pytest.raises(OverflowError):
                _dur = ry.SignedDuration.from_secs_f32(secs)
        else:
            try:
                dur = ry.SignedDuration.from_secs_f32(secs)
                assert isinstance(dur, ry.SignedDuration)
            except OverflowError:
                ...

    @given(st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX))
    def test_from_secs_f64(self, secs: float) -> None:
        if isnan(secs):
            with pytest.raises(ValueError):
                _dur = ry.SignedDuration.from_secs_f64(secs)
        elif secs == float("inf"):
            with pytest.raises(OverflowError):
                _dur = ry.SignedDuration.from_secs_f64(secs)
        else:
            try:
                dur = ry.SignedDuration.from_secs_f64(secs)
                assert isinstance(dur, ry.SignedDuration)
            except OverflowError:
                ...
