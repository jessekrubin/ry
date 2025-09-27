import typing as t

import pytest

import ry
from ry.protocols import Strftime


def test_strftime_timestamp() -> None:
    """Test strftime method of Timestamp.

    REF: https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strftime
    """
    ts = ry.Timestamp.from_second(86_400)
    string = ts.strftime("%a %b %e %I:%M:%S %p UTC %Y")
    assert string == "Fri Jan  2 12:00:00 AM UTC 1970"
    assert f"{ts:%a %b %e %I:%M:%S %p UTC %Y}" == "Fri Jan  2 12:00:00 AM UTC 1970"


def test_strftime_zoned_datetime() -> None:
    """Test strftime method of ZonedDateTime.

    REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strftime
    """
    zdt = ry.date(2024, 7, 15).at(16, 24, 59, 0).in_tz("America/New_York")
    string = zdt.strftime("%a %b %e %I:%M:%S %p %Z %Y")
    assert string == "Mon Jul 15 04:24:59 PM EDT 2024"
    assert f"{zdt:%a %b %e %I:%M:%S %p %Z %Y}" == "Mon Jul 15 04:24:59 PM EDT 2024"


def test_strftime_date() -> None:
    """Test strftime method of Date.

    REF: https://docs.rs/jiff/latest/jiff/civil/struct.Date.html#method.strftime
    """
    date = ry.date(2024, 7, 15)
    string = date.strftime("%Y-%m-%d is a %A")
    assert string == "2024-07-15 is a Monday"
    assert f"{date:%Y-%m-%d is a %A}" == "2024-07-15 is a Monday"


def test_strftime_time() -> None:
    """Test strftime method of Time.

    REF: https://docs.rs/jiff/latest/jiff/civil/struct.Time.html#method.strftime
    """
    t = ry.time(16, 30, 59, 0)
    string = t.strftime("%-I:%M%P")
    assert string == "4:30pm"
    assert f"{t:%-I:%M%P}" == "4:30pm"
    t_rounded = t.round("minute")
    string_rounded = t_rounded.strftime("%-I:%M%P")
    assert string_rounded == "4:31pm"
    assert f"{t_rounded:%-I:%M%P}" == "4:31pm"


def test_strftime_datetime() -> None:
    """Test strftime method of DateTime.

    REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.strftime
    """
    dt = ry.date(2024, 7, 15).at(16, 24, 59, 0)
    string = dt.strftime("%A, %B %e, %Y at %H:%M:%S")
    assert string == "Monday, July 15, 2024 at 16:24:59"
    assert f"{dt:%A, %B %e, %Y at %H:%M:%S}" == "Monday, July 15, 2024 at 16:24:59"


class _Specifier(t.TypedDict):
    specifier: str
    example: str
    description: str


class _FmtOb(t.TypedDict):
    dtype: str
    ob: Strftime


# painfully manually grep/sed/awk-ed specifiers from jiff docs.
# REF: https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html#conversion-specifications

# fmt: off
_SPECIFIERS: list[_Specifier] = [
    { "specifier": "%%",    "example": "%%",                        "description": "A literal %."},
    # broken down entry
    { "specifier": "%A",    "example": "Sunday",                    "description": "The full weekday name." },
    { "specifier": "%a",    "example": "Sun",                       "description": "The abbreviated weekday name." },
    # broken down entry
    { "specifier": "%B",    "example": "June",                      "description": "The full month name." },
    { "specifier": "%b",    "example": "Jun",                       "description": "The abbreviated month name." },
    { "specifier": "%h",    "example": "Jun",                       "description": "The abbreviated month name." },
    { "specifier": "%C",    "example": "20",                        "description": "The century of the year. No padding." },
    { "specifier": "%c",    "example": "2024 M07 14, Sun 17:31:59", "description": "The date and clock time via [Custom]. Supported when formatting only." },
    { "specifier": "%D",    "example": "7/14/24",                   "description": "Equivalent to %m/%d/%y." },
    # broken down entry
    { "specifier": "%d",    "example": "25",                        "description": "The day of the month. %d is zero-padded, %e is space padded." },
    { "specifier": "%e",    "example": "5",                         "description": "The day of the month. %d is zero-padded, %e is space padded." },
    { "specifier": "%F",    "example": "2024-07-14",                "description": "Equivalent to %Y-%m-%d." },
    { "specifier": "%f",    "example": "000456",                    "description": "Fractional seconds, up to nanosecond precision." },
    { "specifier": "%.f",   "example": ".000456",                   "description": "Optional fractional seconds, with dot, up to nanosecond precision." },
    { "specifier": "%G",    "example": "2024",                      "description": "An ISO 8601 week-based year. Zero padded to 4 digits." },
    { "specifier": "%g",    "example": "24",                        "description": "A two-digit ISO 8601 week-based year. Represents only 1969-2068. Zero padded." },
    { "specifier": "%H",    "example": "23",                        "description": "The hour in a 24 hour clock. Zero padded." },
    { "specifier": "%I",    "example": "11",                        "description": "The hour in a 12 hour clock. Zero padded." },
    { "specifier": "%j",    "example": "060",                       "description": "The day of the year. Range is 1..=366. Zero padded to 3 digits." },
    { "specifier": "%k",    "example": "15",                        "description": "The hour in a 24 hour clock. Space padded." },
    { "specifier": "%l",    "example": " 3",                        "description": "The hour in a 12 hour clock. Space padded." },
    { "specifier": "%M",    "example": "04",                        "description": "The minute. Zero padded." },
    { "specifier": "%m",    "example": "01",                        "description": "The month. Zero padded." },
    { "specifier": "%N",    "example": "123456000",                 "description": "Fractional seconds, up to nanosecond precision. Alias for %9f." },
    { "specifier": "%n",    "example": "\\n",                       "description": "Formats as a newline character. Parses arbitrary whitespace." },
    { "specifier": "%P",    "example": "am",                        "description": "Whether the time is in the AM or PM, lowercase." },
    { "specifier": "%p",    "example": "PM",                        "description": "Whether the time is in the AM or PM, uppercase." },
    { "specifier": "%Q",    "example": "America/New_York, +0530",   "description": "An IANA time zone identifier, or %z if one doesn't exist." },
    { "specifier": "%:Q",   "example": "America/New_York, +05:30",  "description": "An IANA time zone identifier, or %:z if one doesn't exist." },
    { "specifier": "%q",    "example": "4",                         "description": "The quarter of the year. Supported when formatting only." },
    { "specifier": "%R",    "example": "23:30",                     "description": "Equivalent to %H:%M." },
    { "specifier": "%r",    "example": "8:30:00 AM",                "description": "The 12-hour clock time via [Custom]. Supported when formatting only." },
    { "specifier": "%S",    "example": "59",                        "description": "The second. Zero padded." },
    { "specifier": "%s",    "example": "1737396540",                "description": "A Unix timestamp, in seconds." },
    { "specifier": "%T",    "example": "23:30:59",                  "description": "Equivalent to %H:%M:%S." },
    { "specifier": "%t",    "example": "\\t",                       "description": "Formats as a tab character. Parses arbitrary whitespace." },
    { "specifier": "%U",    "example": "03",                        "description": "Week number. Week 1 is the first week starting with a Sunday. Zero padded." },
    { "specifier": "%u",    "example": "7",                         "description": "The day of the week beginning with Monday at 1." },
    { "specifier": "%V",    "example": "05",                        "description": "Week number in the ISO 8601 week-based calendar. Zero padded." },
    { "specifier": "%W",    "example": "03",                        "description": "Week number. Week 1 is the first week starting with a Monday. Zero padded." },
    { "specifier": "%w",    "example": "0",                         "description": "The day of the week beginning with Sunday at 0." },
    { "specifier": "%X",    "example": "17:31:59",                  "description": "The clock time via [Custom]. Supported when formatting only." },
    { "specifier": "%x",    "example": "2024 M07 14",               "description": "The date via [Custom]. Supported when formatting only." },
    { "specifier": "%Y",    "example": "2024",                      "description": "A full year, including century. Zero padded to 4 digits." },
    { "specifier": "%y",    "example": "24",                        "description": "A two-digit year. Represents only 1969-2068. Zero padded." },
    { "specifier": "%Z",    "example": "EDT",                       "description": "A time zone abbreviation. Supported when formatting only." },
    { "specifier": "%z",    "example": "+0530",                     "description": "A time zone offset in the format [+-]HHMM[SS]." },
    { "specifier": "%:z",   "example": "+05:30",                    "description": "A time zone offset in the format [+-]HH:MM[:SS]." },
    { "specifier": "%::z",  "example": "+05:30:00",                 "description": "A time zone offset in the format [+-]HH:MM:SS." },
    { "specifier": "%:::z", "example": "-04, +05:30",               "description": "A time zone offset in the format [+-]HH:[MM[:SS]]." }
]
# fmt: on

_FMT_OBJECTS: list[_FmtOb] = [
    {"dtype": "Timestamp", "ob": ry.Timestamp.from_second(86_400)},
    {"dtype": "Time", "ob": ry.time(16, 30, 59, 0)},
    {"dtype": "Date", "ob": ry.date(2024, 7, 15)},
    {
        "dtype": "ZonedDateTime",
        "ob": ry.date(2024, 7, 15).at(16, 24, 59, 0).in_tz("America/New_York"),
    },
    {"dtype": "DateTime", "ob": ry.date(2024, 7, 15).at(16, 24, 59, 0)},
]
_FMT_FLAGS = [
    None,  # none...
    "_",  # Pad a numeric result to the left with spaces.
    "-",  # Do not pad a numeric result.
    "0",  # Pad a numeric result to the left with zeros.
    "^",  # Use alphabetic uppercase for all relevant strings.
    "#",  # Swap the case of the result string. This is typically only useful with %p or %Z, since they are the only conversion specifiers that emit strings entirely in uppercase by default.
]

# fmt: off
_PROBLEM_COMBOS = {
    "Timestamp": [
      "%Z"
    ],
    "Time": [
        "%A", "%a", "%B", "%b", "%h", "%C", "%c", "%D", "%d", "%e", "%F", "%G",
        "%g", "%j", "%m", "%Q", "%:Q", "%q", "%s", "%U", "%u", "%V", "%W", "%w",
        "%x", "%Y", "%y", "%Z", "%z", "%:z", "%::z", "%:::z"
    ],
    "Date": [
        "%c", "%f", "%H", "%I", "%k", "%l", "%M", "%N", "%P", "%p", "%Q",
        "%:Q", "%R", "%r", "%S", "%s", "%T", "%X", "%Z", "%z", "%:z", "%::z",
        "%:::z"
    ],
    "DateTime": ["%Q", "%:Q", "%s", "%Z", "%z", "%:z", "%::z", "%:::z"],
    "ZonedDateTime": []
}
# fmt: on

_PROBLEM_COMBOS_LUT = {k: set(v) for k, v in _PROBLEM_COMBOS.items()}


@pytest.mark.parametrize("obj", _FMT_OBJECTS)
@pytest.mark.parametrize("spec", _SPECIFIERS)
@pytest.mark.parametrize("flag", _FMT_FLAGS)
def test_strftime(obj: _FmtOb, spec: _Specifier, flag: str | None) -> None:
    should_error = spec["specifier"] in _PROBLEM_COMBOS_LUT.get(obj["dtype"], set())
    specifier = spec["specifier"] if flag is None else f"%{flag}{spec['specifier'][1:]}"

    if should_error:
        with pytest.raises(ValueError):
            _ = obj["ob"].strftime(specifier)
        with pytest.raises(ValueError):
            _ = f"{obj['ob']:{specifier}}"
    else:
        fmt_res = obj["ob"].strftime(specifier)
        assert isinstance(fmt_res, str)
        via_fstring = f"{obj['ob']:{specifier}}"
        assert fmt_res == via_fstring
