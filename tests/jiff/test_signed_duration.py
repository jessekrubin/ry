from __future__ import annotations

import datetime as pydt

import pytest
from hypothesis import given

import ry

from ..strategies import st_i32, st_i64
from .strategies import st_signed_duration, st_signed_duration_args


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


def test_signed_duration_cmp_timedelta() -> None:
    left = ry.SignedDuration(1, 2)
    right = ry.SignedDuration(3, 4).to_pytimedelta()
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


class TestSignedDurationSaturatingArithmetic:
    @given(st_signed_duration(), st_signed_duration())
    def test_signed_duration_saturating_add(
        self, left: ry.SignedDuration, right: ry.SignedDuration
    ) -> None:
        left_plus_right = left.saturating_add(right)
        right_plus_left = right.saturating_add(left)
        assert left_plus_right == right_plus_left
        assert isinstance(left_plus_right, ry.SignedDuration)
        assert isinstance(right_plus_left, ry.SignedDuration)

    @given(st_signed_duration(), st_signed_duration())
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

    @given(st_signed_duration(), st_i32)
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


_NANOS_PER_SEC: int = 1_000_000_000
_NANOS_PER_MILLI: int = 1_000_000
_NANOS_PER_MICRO: int = 1_000
_MILLIS_PER_SEC: int = 1_000
_MICROS_PER_SEC: int = 1_000_000
_SECS_PER_MINUTE: int = 60
_MINS_PER_HOUR: int = 60


class TestSignedDurationAsXYZ:
    @given(st_signed_duration())
    def test_as_hours(self, dur: ry.SignedDuration) -> None:
        hours = dur.as_hours()
        assert isinstance(hours, int)
        expected = dur.signum() * int(
            abs(dur.secs) // 3600
            + abs(dur.nanos) // (_NANOS_PER_SEC * _SECS_PER_MINUTE * _MINS_PER_HOUR)
        )
        assert hours == expected

    @given(st_signed_duration())
    def test_as_mins(self, dur: ry.SignedDuration) -> None:
        minutes = dur.as_mins()
        assert isinstance(minutes, int)
        expected = dur.signum() * int(
            abs(dur.secs) // 60 + abs(dur.nanos) // 60_000_000_000
        )
        assert minutes == expected

    @given(st_signed_duration())
    def test_as_secs(self, dur: ry.SignedDuration) -> None:
        seconds = dur.as_secs()
        assert isinstance(seconds, int)
        expected = dur.signum() * (
            abs(dur.secs) + dur.signum() * int(abs(dur.nanos) // _NANOS_PER_SEC)
        )
        assert seconds == expected

    @given(st_signed_duration())
    def test_as_millis(self, dur: ry.SignedDuration) -> None:
        millis = dur.as_millis()
        assert isinstance(millis, int)
        expected = dur.signum() * int(
            abs(dur.secs) * _MILLIS_PER_SEC + abs(dur.nanos) // _NANOS_PER_MILLI
        )
        assert millis == expected

    @given(st_signed_duration())
    def test_as_micros(self, dur: ry.SignedDuration) -> None:
        micros = dur.as_micros()
        assert isinstance(micros, int)
        expected = dur.signum() * int(
            abs(dur.secs) * _MICROS_PER_SEC + abs(dur.nanos) // _NANOS_PER_MICRO
        )
        assert micros == expected

    @given(st_signed_duration())
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
