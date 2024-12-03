import datetime as pydt

import ry


def test_timezone_to_pytzinfo() -> None:
    zdt = ry.ZonedDateTime.now().intz("utc")
    rytz = zdt.timezone()
    py_tzinfo = rytz.to_pytzinfo()
    assert isinstance(py_tzinfo, pydt.tzinfo)


def test_timezone_from_pytzinfo() -> None:
    pydatetime = pydt.datetime(2020, 8, 26, 6, 27, 0, 0, pydt.timezone.utc)
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
        # py_tz = pydt.timezone(tz)
        # assert py_tz.tzname(None) == ry_tz.name

        ry_tz = ry.TimeZone(tz)

        assert isinstance(ry_tz, ry.TimeZone)
        pytzinfo = ry_tz.to_pytzinfo()
        assert isinstance(pytzinfo, pydt.tzinfo)
        assert pytzinfo is not None

        assert ry_tz == ry.TimeZone.from_pytzinfo(pytzinfo)
