"""test jiff examples section

tests based on the examples in the jiff documentation (as of 2024-11-18)
"""

from __future__ import annotations

import ry


def test_print_datetime_for_a_timestamp() -> None:
    ts = ry.Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned(ry.TimeZone("America/New_York"))
    assert str(zdt) == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    assert str(ts) == "2024-07-10T21:19:25.567Z"
    assert ts.as_second() == 1720646365
    assert ts.as_nanosecond() == 1720646365567000000


def test_create_zoned_datetime_from_civil_time() -> None:
    zdt = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")
    assert str(zdt) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_change_an_instant_from_one_time_zone_to_another() -> None:
    zdt1 = ry.date(1918, 11, 11).at(11, 0, 0, 0).in_tz("Europe/Paris")
    zdt2 = zdt1.in_tz("America/New_York")
    assert str(zdt2) == "1918-11-11T06:00:00-05:00[America/New_York]"


def test_find_duration_between_datetimes() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")
    span = zdt2 - zdt1
    assert isinstance(span, ry.TimeSpan)
    assert str(span) == "PT29341H3M"


def test_add_duration_to_a_zoned_datetime() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    span = ry.TimeSpan()._years(3)._months(4)._days(5)._hours(12)._minutes(3)
    zdt2 = zdt1.add(span)
    assert str(zdt2) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity() -> None:
    zdt = ry.date(2024, 3, 10).at(2, 30, 0, 0).in_tz("America/New_York")
    assert str(zdt) == "2024-03-10T03:30:00-04:00[America/New_York]"
    zdt = ry.date(2024, 11, 3).at(1, 30, 0, 0).in_tz("America/New_York")
    assert str(zdt) == "2024-11-03T01:30:00-04:00[America/New_York]"


def test_parsing_a_span() -> None:
    span: ry.TimeSpan = ry.TimeSpan.parse("P5y1w10dT5h59m")
    expected = ry.TimeSpan()._years(5)._weeks(1)._days(10)._hours(5)._minutes(59)
    assert span == expected
    assert str(span) == "P5Y1W10DT5H59M"


def test_using_strftime_and_strptime_for_formatting_and_parsing() -> None:
    zdt = ry.ZonedDateTime.strptime(
        "Monday, July 15, 2024 at 5:30pm US/Eastern",
        "%A, %B %d, %Y at %I:%M%p %Q",
    )
    assert str(zdt) == "2024-07-15T17:30:00-04:00[US/Eastern]"
    zdt = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Tasmania")
    string = zdt.strftime("%A, %B %d, %Y at %-I:%M%P %Z")
    assert string == "Monday, July 15, 2024 at 5:30pm AEST"

    zdt = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Tasmania")
    string = zdt.strftime("%A, %B %d, %Y at %-I:%M%P %Q")
    assert string == "Monday, July 15, 2024 at 5:30pm Australia/Tasmania"


def test_rounding_a_span() -> None:
    """
    ```rust
    use jiff::{RoundMode, SpanRound, ToSpan, Unit};

    // The default rounds like how you were taught in school:
    assert_eq!(1.hour().minutes(59).round(Unit::Hour)?, 2.hours());
    // But we can change the mode, e.g., truncation:
    let options = SpanRound::new().smallest(Unit::Hour).mode(RoundMode::Trunc);
    assert_eq!(1.hour().minutes(59).round(options)?, 1.hour());
    ```
    """

    span = ry.TimeSpan(
        hours=1,
        minutes=59,
    )

    assert span.round("hour") == ry.TimeSpan(hours=2)


def test_rounding_a_zoned_datetime() -> None:
    """
    ref: https://docs.rs/jiff/latest/jiff/enum.Unit.html

    ```rust
    use jiff::{Unit, Zoned};

    let zdt: Zoned = "2024-07-06 17:44:22.158-04[America/New_York]".parse()?;
    let nearest_minute = zdt.round(Unit::Minute)?;
    assert_eq!(
        nearest_minute.__str__(),
        "2024-07-06T17:44:00-04:00[America/New_York]",
    );
    ```
    """

    zdt = ry.ZonedDateTime.parse("2024-07-06 17:44:22.158-04[America/New_York]")
    nearest_minute = zdt.round("minute")
    assert str(nearest_minute) == "2024-07-06T17:44:00-04:00[America/New_York]"
