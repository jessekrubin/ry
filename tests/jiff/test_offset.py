import datetime as pydt
import re

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


def test_offset_utc() -> None:
    assert ry.Offset.UTC.seconds == 0
    assert (
        ry.Offset.UTC == ry.Offset(hours=0) == ry.Offset(seconds=0) == ry.Offset.utc()
    )


def test_offset_duration_until_since() -> None:
    offset = ry.Offset(hours=5)
    assert offset.duration_until(ry.Offset.UTC) == ry.SignedDuration(secs=-(5 * 3600))
    assert offset.duration_since(ry.Offset.UTC) == ry.SignedDuration(secs=(5 * 3600))


def test_offset_richcmp() -> None:
    o3 = ry.Offset(hours=3)
    o2 = ry.Offset(hours=5)
    o3_another = ry.Offset(hours=3)

    assert o3 < o2
    assert o2 > o3
    assert o3 <= o3_another
    assert o3 >= o3_another
    assert o3 == o3_another
    assert o3 != o2


def test_offset_to_timestamp_to_datetime_roundtrip() -> None:
    off = ry.Offset(seconds=30 + (2 * 3600))
    d = ry.datetime(2001, 9, 9, 1, 46, 40, 0)
    ts_with_offset = off.to_timestamp(d)
    assert isinstance(ts_with_offset, ry.Timestamp)
    expected_ts = ry.Timestamp(999992770, 0)
    assert ts_with_offset == expected_ts
    dt_from_ts = off.to_datetime(expected_ts)
    assert dt_from_ts == d


def test_offset_to_from_py() -> None:
    off = ry.Offset(seconds=30)
    py_td = off.to_py()
    assert isinstance(py_td, pydt.timedelta)
    off_from_py = ry.Offset.from_pytimedelta(py_td)
    assert off_from_py == off


def test_offset_to_pytzinfo() -> None:
    off = ry.Offset(hours=2)
    pytz_info = off.to_pytzinfo()
    assert isinstance(pytz_info, pydt.tzinfo)
    expected = pydt.timezone(pydt.timedelta(seconds=7200))
    assert pytz_info == expected
    roundtripped_off = ry.Offset.from_pytzinfo(pytz_info)
    assert roundtripped_off == off


class TestOffset:
    def test_create_offset_with_hours(self) -> None:
        offset = ry.Offset(hours=-4)
        assert offset == ry.Offset(hours=-4)
        assert offset == ry.Offset.from_hours(-4)

    def test_offset_from_seconds(self) -> None:
        offset = ry.Offset.from_seconds(-4 * 60 * 60)
        assert offset == ry.Offset(hours=-4)

    @pytest.mark.skip(reason="legacy")
    def test_offset_errors_when_given_both_hours_and_seconds(self) -> None:
        with pytest.raises(TypeError):
            ry.Offset(hours=-4, seconds=-4 * 60 * 60)

    @pytest.mark.skip(reason="legacy")
    def test_offset_errors_when_given_neither_hours_nor_seconds(self) -> None:
        with pytest.raises(TypeError):
            ry.Offset()

    def test_const_max(self) -> None:
        assert ry.Offset.MAX == ry.Offset(seconds=93599)

    def test_const_min(self) -> None:
        assert ry.Offset.MIN == ry.Offset(seconds=-93599)

    def test_const_zero(self) -> None:
        assert ry.Offset.ZERO == ry.Offset(seconds=0)

    def test_const_utc(self) -> None:
        assert ry.Offset.UTC == ry.Offset(seconds=0)

    def test_seconds_property(self) -> None:
        offset = ry.Offset.from_seconds(61)
        assert offset.seconds == 61
        assert offset.is_positive
        assert not offset.is_negative
        offset_neg = -offset
        assert offset_neg.seconds == -61
        assert offset_neg.is_negative
        assert not offset_neg.is_positive

    def test_from_hours(self) -> None:
        offset = ry.Offset.from_hours(2)
        assert offset == ry.Offset(seconds=7200)

    def test_from_hours_error(self) -> None:
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'time zone offset total seconds' is not in the required range of -93599..=93599"
            ),
        ):
            _offset = ry.Offset.from_hours(26)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'time zone offset total seconds' is not in the required range of -93599..=93599"
            ),
        ):
            _offset = ry.Offset.from_hours(-26)

    def test_from_seconds(self) -> None:
        offset = ry.Offset.from_seconds(61)
        assert offset == ry.Offset(seconds=61)

    def test_from_seconds_error(self) -> None:
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'time zone offset total seconds' is not in the required range of -93599..=93599"
            ),
        ):
            _offset = ry.Offset.from_seconds(93600)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'time zone offset total seconds' is not in the required range of -93599..=93599"
            ),
        ):
            _offset = ry.Offset.from_seconds(-93600)

    def test_negate(self) -> None:
        offset = ry.Offset.from_seconds(61)
        assert -offset == ry.Offset.from_seconds(-61)

    def test_until(self) -> None:
        offset = ry.Offset.from_seconds(61)
        span_until = offset.until(ry.Offset.from_seconds(62))
        assert isinstance(span_until, ry.TimeSpan)
        assert span_until == ry.TimeSpan(seconds=1)
        assert offset.until(ry.Offset.from_seconds(61)) == ry.TimeSpan()

    def test_since(self) -> None:
        offset = ry.Offset.from_seconds(61)
        span_since = offset.since(ry.Offset.from_seconds(62))
        assert isinstance(span_since, ry.TimeSpan)

        assert span_since == ry.TimeSpan(seconds=-1)
        assert offset.since(ry.Offset.from_seconds(61)) == ry.TimeSpan()

    def test_to_timezone(self) -> None:
        offset = ry.Offset.from_seconds(61)
        tz = offset.to_timezone()
        assert isinstance(tz, ry.TimeZone)
        tz_offset = tz.to_offset(ry.Timestamp(0, 0))
        assert tz_offset == offset

    def test_checked_add(self) -> None:
        offset = ry.Offset.from_hours(-8)
        span = ry.timespan(hours=1)
        assert offset.add(span) == ry.Offset.from_hours(-7)
        signed_duration = span.to_signed_duration(
            ry.Date(
                year=2024,
                month=12,
                day=13,  # OOOOH friday the 13th
            )
        )
        assert offset.add(signed_duration) == ry.Offset.from_hours(-7)
        duration = ry.Duration(secs=3600)
        assert offset.add(duration) == ry.Offset.from_hours(-7)

    def test_checked_sub(self) -> None:
        offset = ry.Offset.from_hours(-8)
        span = ry.timespan(hours=1)
        assert offset.sub(span) == ry.Offset.from_hours(-9)
        signed_duration = span.to_signed_duration(
            ry.Date(year=2024, month=12, day=13)  # OOOOH friday the 13th (again)
        )
        assert offset.sub(signed_duration) == ry.Offset.from_hours(-9)
        duration = ry.Duration(secs=3600)
        assert offset.sub(duration) == ry.Offset.from_hours(-9)

    def test_saturating_add(self) -> None:
        offset = ry.Offset.from_hours(25)
        span = ry.TimeSpan(hours=2)
        assert offset.saturating_add(span) == ry.Offset.MAX
        signed_duration = span.to_signed_duration(ry.Date(year=2024, month=12, day=13))
        assert offset.saturating_add(signed_duration) == ry.Offset.MAX
        duration = ry.Duration(secs=7200)
        assert offset.saturating_add(duration) == ry.Offset.MAX

    def test_saturating_sub(self) -> None:
        offset = ry.Offset.from_hours(-25)
        span = ry.TimeSpan(hours=2)
        assert offset.saturating_sub(span) == ry.Offset.MIN
        signed_duration = span.to_signed_duration(ry.Date(year=2024, month=12, day=13))
        assert offset.saturating_sub(signed_duration) == ry.Offset.MIN
        duration = ry.Duration(secs=7200)
        assert offset.saturating_sub(duration) == ry.Offset.MIN


class TestOffsetHypothesis:
    _MAX_OFFSET_SECONDS = ry.Offset.MAX.seconds
    _MIN_OFFSET_SECONDS = ry.Offset.MIN.seconds

    @given(
        st.integers(min_value=ry.Offset.MIN.seconds, max_value=ry.Offset.MAX.seconds)
    )
    def test_offset_to_from_py_roundtrip(self, seconds: int) -> None:
        off = ry.Offset(seconds=seconds)
        assert isinstance(off, ry.Offset)
        assert off.seconds == seconds
        # check that the repr evals to the same thing
        off_repr = repr(off)
        off_from_repr = eval(off_repr, {"Offset": ry.Offset})
        assert off_from_repr == off

    @given(
        st.integers(min_value=ry.I8_MIN, max_value=ry.I8_MAX),
        st.integers(min_value=ry.I16_MIN, max_value=ry.I16_MAX),
        st.integers(min_value=ry.Offset.MIN.seconds, max_value=ry.Offset.MAX.seconds),
    )
    def test_offset_creation(self, hours: int, minutes: int, seconds: int) -> None:
        # total seconds
        total_seconds = (hours * 3600) + (minutes * 60) + seconds
        if (
            total_seconds > self._MAX_OFFSET_SECONDS
            or total_seconds < self._MIN_OFFSET_SECONDS
        ):
            match_str = f"parameter 'time zone offset total seconds' is not in the required range of {self._MIN_OFFSET_SECONDS}..={self._MAX_OFFSET_SECONDS}"
            with pytest.raises(ValueError, match=match_str):
                _off = ry.Offset(hours=hours, minutes=minutes, seconds=seconds)
        else:
            off = ry.Offset(seconds=seconds)
            assert isinstance(off, ry.Offset)
            assert off.seconds == seconds
            # check that the repr evals to the same thing
            off_repr = repr(off)
            off_from_repr = eval(off_repr, {"Offset": ry.Offset})
            assert off_from_repr == off
