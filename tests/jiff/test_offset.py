import datetime as pydt

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
            # msg will look like
            # parameter 'offset-seconds' with value 93600 is not in the required range of -93599..=93599
            match_str = f"parameter 'offset-seconds' with value {total_seconds} is not in the required range of {self._MIN_OFFSET_SECONDS}..={self._MAX_OFFSET_SECONDS}"
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
