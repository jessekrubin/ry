"""Jiff examples (v2)

Translated jiff-examples from jiff-v2's docs

REF: https://docs.rs/jiff/latest/jiff/#examples
DATE: 2025-05-23
"""

from __future__ import annotations

import json
from dataclasses import dataclass

import ry


def test_get_current_time_in_system_tz() -> None:
    now = ry.ZonedDateTime.now()
    assert isinstance(now, ry.ZonedDateTime)
    assert now.tz.name


def test_print_current_time_rounded_to_second() -> None:
    rounded = ry.ZonedDateTime.now().round("second")
    # nanoseconds should be zero after rounding to second
    assert rounded.nanosecond == 0


def test_print_todays_date_at_specific_time() -> None:
    zdt = ry.ZonedDateTime.now().replace(
        hour=14, minute=0, second=0, nanosecond=0
    )
    assert zdt.hour == 14 and zdt.minute == 0 and zdt.second == 0
    assert zdt.nanosecond == 0


def test_print_current_unix_timestamp() -> None:
    ts = ry.Timestamp.now()
    sec = ts.as_second()
    ns = ts.as_nanosecond()
    assert isinstance(sec, int) and sec > 1_600_000_000
    # nanosecond count divided by 1e9 should equal seconds
    assert ns // 1_000_000_000 == sec


def test_print_datetime_for_a_timestamp() -> None:
    ts = ry.Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned(ry.TimeZone("America/New_York"))
    assert str(zdt) == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    assert str(ts) == "2024-07-10T21:19:25.567Z"


def test_create_zoned_datetime_from_civil_time() -> None:
    zdt = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")
    assert str(zdt) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_change_an_instant_from_one_timezone_to_another() -> None:
    paris = ry.date(1918, 11, 11).at(11, 0, 0, 0).in_tz("Europe/Paris")
    nyc = paris.in_tz("America/New_York")
    assert str(nyc) == "1918-11-11T06:00:00-05:00[America/New_York]"


def test_find_duration_between_two_zoned_datetimes() -> None:
    a = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    b = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")
    span = b - a
    assert str(span) == "PT29341H3M"
    # until: specify largest unit via helper
    span2 = a.until(b, largest="year")
    assert str(span2) == "P3Y4M5DT12H3M"


def test_add_duration_to_a_zoned_datetime() -> None:
    start = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    span = ry.TimeSpan()._years(3)._months(4)._days(5)._hours(12)._minutes(3)
    finish = start.add(span)  # previously `checked_add`
    assert str(finish) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity() -> None:
    gap = ry.date(2024, 3, 10).at(2, 30, 0, 0).in_tz("America/New_York")
    assert str(gap) == "2024-03-10T03:30:00-04:00[America/New_York]"

    fold = ry.date(2024, 11, 3).at(1, 30, 0, 0).in_tz("America/New_York")
    assert str(fold) == "2024-11-03T01:30:00-04:00[America/New_York]"


def test_parsing_a_span() -> None:
    iso = ry.TimeSpan.parse("P5y1w10dT5h59m")
    expected = (
        ry.TimeSpan()._years(5)._weeks(1)._days(10)._hours(5)._minutes(59)
    )
    assert iso == expected
    assert str(iso) == "P5Y1W10DT5H59M"

    from_friendly = ry.TimeSpan.parse(
        "5 years, 1 week, 10 days, 5 hours, 59 minutes"
    )
    assert iso == from_friendly
    assert from_friendly.to_string(friendly=True) == "5y 1w 10d 5h 59m"
    assert from_friendly.friendly() == "5y 1w 10d 5h 59m"
    assert str(from_friendly) == "P5Y1W10DT5H59M"


def test_parsing_an_rfc2822_datetime_string() -> None:
    base = ry.ZonedDateTime.parse_rfc2822("Thu, 29 Feb 2024 05:34 -0500")
    tas = base.in_tz("Australia/Tasmania")
    kol = base.in_tz("Asia/Kolkata")
    assert tas.format_rfc2822() == "Thu, 29 Feb 2024 21:34:00 +1100"
    assert kol.format_rfc2822() == "Thu, 29 Feb 2024 16:04:00 +0530"


def test_using_strftime_and_strptime() -> None:
    zdt = ry.ZonedDateTime.strptime(
        "Monday, July 15, 2024 at 5:30pm US/Eastern",
        "%A, %B %d, %Y at %I:%M%p %Q",
    )
    assert str(zdt) == "2024-07-15T17:30:00-04:00[US/Eastern]"

    tas = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Tasmania")
    formatted = tas.strftime("%A, %B %d, %Y at %-I:%M%P %Q")
    assert formatted == "Monday, July 15, 2024 at 5:30pm Australia/Tasmania"


@dataclass
class Record:
    timestamp: ry.Timestamp

    def to_json(self) -> str:
        return json.dumps({"timestamp": self.timestamp.as_second()})

    @classmethod
    def from_json(cls, raw: str) -> Record:
        data = json.loads(raw)
        return cls(timestamp=ry.Timestamp.from_second(data["timestamp"]))


def test_serializing_and_deserializing_integer_timestamps() -> None:
    src = Record(timestamp=ry.Timestamp.from_second(1_517_644_800))
    wire = src.to_json()
    got = Record.from_json(wire)
    assert got.timestamp == src.timestamp
    assert wire == '{"timestamp": 1517644800}'


def main() -> None:
    test_get_current_time_in_system_tz()
    test_print_current_time_rounded_to_second()
    test_print_todays_date_at_specific_time()
    test_print_current_unix_timestamp()
    test_print_datetime_for_a_timestamp()
    test_create_zoned_datetime_from_civil_time()
    test_change_an_instant_from_one_timezone_to_another()
    test_find_duration_between_two_zoned_datetimes()
    test_add_duration_to_a_zoned_datetime()
    test_dealing_with_ambiguity()
    test_parsing_a_span()
    test_parsing_an_rfc2822_datetime_string()
    test_using_strftime_and_strptime()
    test_serializing_and_deserializing_integer_timestamps()


if __name__ == "__main__":
    main()
