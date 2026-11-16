from __future__ import annotations

import datetime as pydt

import pydantic
import pytest
from typing_extensions import TypedDict

import ry


class _OffsetDict(TypedDict):
    seconds: int
    fmt: str


class _OffsetInfoDict(TypedDict):
    offset: _OffsetDict
    dst: bool
    abbreviation: str


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


def test_timezone_equivalence() -> None:
    tz_utc = ry.TimeZone("utc")
    tz_utc2 = ry.TimeZone("utc")
    assert tz_utc.equiv(tz_utc2)
    assert tz_utc.equiv("UTC")
    py_tzinfo = tz_utc.to_pytzinfo()
    assert tz_utc.equiv(py_tzinfo)
    with pytest.raises(TypeError):
        tz_utc.equiv(123)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


def test_tz_to_dict() -> None:
    tz = ry.TimeZone("America/Los_Angeles")
    tz_dict = tz.to_dict()
    assert isinstance(tz_dict, dict)
    assert len(tz_dict) == 1
    assert "tz" in tz_dict
    assert tz_dict["tz"] == "America/Los_Angeles"


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

    @pytest.mark.parametrize("tzname", ["", "unknown"])
    def test_unknown_args(self, tzname: str) -> None:
        tz = ry.TimeZone(tzname)
        offset = tz.to_offset(ry.Timestamp(0, 0))
        assert offset == ry.Offset(0)
        assert tz.is_unknown

        assert repr(tz) == 'TimeZone("unknown")'


_TS_NOW = ry.Timestamp.now()


@pytest.mark.parametrize("tzname", ry.TimeZoneDatabase().available())
def test_offset_infos(tzname: str) -> None:
    tadapt = pydantic.TypeAdapter(_OffsetInfoDict)

    tz = ry.TimeZone(tzname)

    info = tz.to_offset_info(_TS_NOW)
    assert isinstance(info, dict)
    val = tadapt.validate_python(info, strict=True)
    assert isinstance(val, dict)
    assert set(val.keys()) == {"offset", "dst", "abbreviation"}


# NOT IMPLEMENTED YET
def test_to_ambiguous_timestamp() -> None:
    tz = ry.TimeZone("America/New_York")
    with pytest.raises(NotImplementedError):
        tz.to_ambiguous_timestamp()


def test_to_ambiguous_zoned_datetime() -> None:
    tz = ry.TimeZone("America/New_York")
    with pytest.raises(NotImplementedError):
        tz.to_ambiguous_zoned_datetime()


class TestTimeZonePrimaryAndLocation:
    """jiff 0.2.45+ ~ ECMAScript primary tz ids + tz locations"""

    @pytest.mark.parametrize(
        ("name", "primary", "is_primary"),
        [
            ("Asia/Calcutta", "Asia/Kolkata", False),
            ("Asia/Kolkata", "Asia/Kolkata", True),
            ("Atlantic/Jan_Mayen", "Arctic/Longyearbyen", False),
            ("America/New_York", "America/New_York", True),
            ("UTC", "UTC", True),
        ],
    )
    def test_primary_iana_name(
        self, name: str, primary: str, *, is_primary: bool
    ) -> None:
        tz = ry.TimeZone(name)
        assert tz.primary_iana_name() == primary
        assert tz.is_primary() is is_primary

    def test_primary_iana_name_fixed_offset(self) -> None:
        tz = ry.TimeZone.fixed(ry.Offset(hours=-5))
        assert tz.primary_iana_name() is None
        assert tz.is_primary() is False

    def test_location(self) -> None:
        loc = ry.TimeZone("Europe/Copenhagen").location()
        assert loc is not None
        assert loc["country"] == "DK"
        assert isinstance(loc["latitude"], float)
        assert isinstance(loc["longitude"], float)
        assert set(loc) == {"country", "latitude", "longitude"}

    def test_location_alias_uses_primary(self) -> None:
        # alias + primary resolve to the same location
        assert (
            ry.TimeZone("Asia/Calcutta").location()
            == ry.TimeZone("Asia/Kolkata").location()
        )

    @pytest.mark.parametrize(
        "tz",
        [
            ry.TimeZone.UTC,
            ry.TimeZone.fixed(ry.Offset(hours=-5)),
            ry.TimeZone("EST5EDT"),
        ],
    )
    def test_location_none(self, tz: ry.TimeZone) -> None:
        assert tz.location() is None
