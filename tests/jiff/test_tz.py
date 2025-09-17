from __future__ import annotations

import datetime as pydt

import ry


def test_timezone_to_pytzinfo() -> None:
    zdt = ry.ZonedDateTime.now().in_tz("utc")
    rytz = zdt.timezone()
    py_tzinfo = rytz.to_pytzinfo()
    assert isinstance(py_tzinfo, pydt.tzinfo)


def test_timezone_from_pytzinfo() -> None:
    pydatetime = pydt.datetime(2020, 8, 26, 6, 27, 0, 0, pydt.UTC)
    tzinfo = pydatetime.tzinfo
    assert isinstance(tzinfo, pydt.tzinfo)
    assert tzinfo is not None
    ry_tz = ry.TimeZone.from_pytzinfo(tzinfo)
    assert isinstance(ry_tz, ry.TimeZone)
    assert ry_tz == ry.TimeZone.utc()


def test_timezone_from_str() -> None:
    timezones2test = [
        "utc",
        "America/New_York",
        "Europe/London",
        "Australia/Sydney",
    ]

    for tz in timezones2test:
        ry_tz = ry.TimeZone(tz)
        assert isinstance(ry_tz, ry.TimeZone)
        pytzinfo = ry_tz.to_pytzinfo()
        assert isinstance(pytzinfo, pydt.tzinfo)
        assert pytzinfo is not None

        assert ry_tz == ry.TimeZone.from_pytzinfo(pytzinfo)


class TestTimeZone:
    def test_fixed_offset_hours(self) -> None:
        tz = ry.TimeZone.fixed(ry.Offset.from_hours(-8))
        offset = tz.to_offset(ry.Timestamp(0, 0))
        assert offset == ry.Offset.from_hours(-8)
        assert repr(tz) == "TimeZone('-08')"
        assert str(tz) == "-08"

    def test_fixed_offset_seconds(self) -> None:
        tz = ry.TimeZone.fixed(ry.Offset.from_seconds(-61))
        offset: ry.Offset = tz.to_offset(ry.Timestamp(0, 0))
        assert isinstance(offset, ry.Offset)
        assert offset == ry.Offset.from_seconds(-61)
        assert repr(tz) == "TimeZone('-00:01:01')"
        assert str(tz) == "-00:01:01"
