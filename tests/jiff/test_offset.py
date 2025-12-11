import datetime as pydt

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
