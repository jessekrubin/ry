"""Jiff examples (v3)

Translated jiff-examples from jiff-v1.0's docs

REF: https://docs.rs/jiff/1/jiff/#examples
DATE: 2026-11-16
"""

from __future__ import annotations

import json
from dataclasses import dataclass

import ry


def test_quick_example() -> None:
    time = ry.Timestamp.parse("2024-07-11T01:14:00Z")
    zoned = time.in_tz("America/New_York").add(ry.TimeSpan(months=1, hours=2))
    assert str(zoned) == "2024-08-10T23:14:00-04:00[America/New_York]"
    # Or, if you want an RFC3339 formatted string:
    assert str(zoned.timestamp()) == "2024-08-11T03:14:00Z"


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
    assert (zdt.hour, zdt.minute, zdt.second, zdt.nanosecond) == (14, 0, 0, 0)

    # or w/ a `Time`
    zdt = ry.ZonedDateTime.now().replace(time=ry.time(14, 0, 0, 0))
    assert zdt.time() == ry.time(14, 0, 0, 0)


def test_print_current_unix_timestamp() -> None:
    ts = ry.Timestamp.now()
    sec = ts.as_second()
    assert isinstance(sec, int) and sec > 1_600_000_000
    # nanosecond count divided by 1e9 should equal seconds
    assert ts.as_nanosecond() // 1_000_000_000 == sec


def test_print_datetime_for_a_timestamp() -> None:
    ts = ry.Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned_datetime(ry.TimeZone("America/New_York"))
    assert str(zdt) == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    # Or if you just want the RFC 3339 time without bothering with time zones:
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
    assert f"{span:#}" == "29341h 3m"
    # bigger units (not reversible) via `until` w/ `largest`
    span2 = a.until(b, largest="year")
    assert f"{span2:#}" == "3y 4mo 5d 12h 3m"


def test_add_duration_to_a_zoned_datetime() -> None:
    start = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    span = ry.TimeSpan(years=3, months=4, days=5, hours=12, minutes=3)
    finish = start.add(span)  # python ~ everything is `checked`
    assert str(finish) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity() -> None:
    # 2:30 on 2024-03-10 in New York didn't exist. It's a "gap."
    # The compatible strategy selects the datetime after the gap.
    gap = ry.date(2024, 3, 10).at(2, 30, 0, 0).in_tz("America/New_York")
    assert str(gap) == "2024-03-10T03:30:00-04:00[America/New_York]"

    # 1:30 on 2024-11-03 in New York appeared twice. It's a "fold."
    # The compatible strategy selects the datetime before the fold.
    fold = ry.date(2024, 11, 3).at(1, 30, 0, 0).in_tz("America/New_York")
    assert str(fold) == "2024-11-03T01:30:00-04:00[America/New_York]"


def test_parsing_a_span() -> None:
    expected = ry.TimeSpan(years=5, weeks=1, days=10, hours=5, minutes=59)

    # ISO 8601
    iso = ry.TimeSpan.parse("P5y1w10dT5h59m")
    assert iso == expected
    assert str(iso) == "P5Y1W10DT5H59M"

    # "friendly" format(s)
    for friendly in (
        "5 years, 1 week, 10 days, 5 hours, 59 minutes",
        "5yrs 1wk 10d 5hrs 59mins",
        "5y 1w 10d 5h 59m",
    ):
        assert ry.TimeSpan.parse(friendly) == expected
    assert f"{expected:#}" == "5y 1w 10d 5h 59m"
    assert expected.friendly() == "5y 1w 10d 5h 59m"


def test_parsing_an_rfc2822_datetime_string() -> None:
    base = ry.ZonedDateTime.parse_rfc2822("Thu, 29 Feb 2024 05:34 -0500")
    hobart = base.in_tz("Australia/Hobart")
    kolkata = base.in_tz("Asia/Kolkata")
    assert hobart.format_rfc2822() == "Thu, 29 Feb 2024 21:34:00 +1100"
    assert kolkata.format_rfc2822() == "Thu, 29 Feb 2024 16:04:00 +0530"


def test_using_strftime_and_strptime() -> None:
    # NOTE: ry's arg order is (string, format) like python's `strptime`
    zdt = ry.ZonedDateTime.strptime(
        "Monday, July 15, 2024 at 5:30pm America/New_York",
        "%A, %B %d, %Y at %I:%M%p %Q",
    )
    assert str(zdt) == "2024-07-15T17:30:00-04:00[America/New_York]"

    hobart = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Hobart")
    # %-I instead of %I means no padding; %Z is the tz abbreviation (not
    # parsable since abbreviations are ambiguous)
    formatted = hobart.strftime("%A, %B %d, %Y at %-I:%M%P %Z")
    assert formatted == "Monday, July 15, 2024 at 5:30pm AEST"
    # %Q is the IANA tz name (parsable)
    formatted = hobart.strftime("%A, %B %d, %Y at %-I:%M%P %Q")
    assert formatted == "Monday, July 15, 2024 at 5:30pm Australia/Hobart"


@dataclass
class Record:
    timestamp: ry.Timestamp

    def to_json(self) -> str:
        return json.dumps(
            {"timestamp": self.timestamp.as_second()}, separators=(",", ":")
        )

    @classmethod
    def from_json(cls, raw: str) -> Record:
        data = json.loads(raw)
        return cls(timestamp=ry.Timestamp.from_second(data["timestamp"]))


def test_serializing_and_deserializing_integer_timestamps() -> None:
    raw = '{"timestamp":1517644800}'
    got = Record.from_json(raw)
    assert got.timestamp == ry.Timestamp.from_second(1_517_644_800)
    assert got.to_json() == raw


def main() -> None:
    test_quick_example()
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
