"""Jiff examples (v1)

Translated jiff-examples from jiff-v1.0's docs (`Zoned` is `ZonedDateTime` now)

REF: https://docs.rs/jiff/1/jiff/#examples
DATE: 2026-11-16
"""

from __future__ import annotations

import json
from dataclasses import dataclass

import ry

_NY = "America/New_York"


def test_quick_example() -> None:
    """
    ```rust
    use jiff::{Timestamp, ToSpan};

    let time: Timestamp = "2024-07-11T01:14:00Z".parse()?;
    let zoned = time.in_tz("America/New_York")?.checked_add(1.month().hours(2))?;
    assert_eq!(zoned.to_string(), "2024-08-10T23:14:00-04:00[America/New_York]");
    // Or, if you want an RFC3339 formatted string:
    assert_eq!(zoned.timestamp().to_string(), "2024-08-11T03:14:00Z");
    ```
    """
    time = ry.Timestamp.parse("2024-07-11T01:14:00Z")
    zoned = time.in_tz(_NY).add(ry.TimeSpan(months=1, hours=2))
    assert str(zoned) == "2024-08-10T23:14:00-04:00[America/New_York]"
    assert str(zoned.timestamp()) == "2024-08-11T03:14:00Z"


def test_get_current_time_in_system_tz() -> None:
    """
    ```rust
    use jiff::ZonedDateTime;

    let now = ZonedDateTime::now();
    println!("{now}");
    // Output: 2024-07-10T17:09:28.168146054-04:00[America/New_York]
    ```
    """
    now = ry.ZonedDateTime.now()
    assert isinstance(now, ry.ZonedDateTime)
    assert now.tz.name


def test_print_current_time_rounded_to_second() -> None:
    """
    ```rust
    use jiff::{Unit, ZonedDateTime};

    let now = ZonedDateTime::now().round(Unit::Second)?;
    println!("{now}");
    // Output: 2024-07-10T17:09:28-04:00[America/New_York]
    ```
    """
    rounded = ry.ZonedDateTime.now().round("second")
    assert rounded.nanosecond == 0


class TestPrintTodaysDateAtSpecificTime:
    def test_with_fields(self) -> None:
        """
        ```rust
        use jiff::ZonedDateTime;

        let zdt = ZonedDateTime::now().with()
            .hour(14)
            .minute(0)
            .second(0)
            .subsec_nanosecond(0)
            .build()?;
        println!("{zdt}");
        // Output: 2024-07-12T14:00:00-04:00[America/New_York]
        ```
        """
        zdt = ry.ZonedDateTime.now().replace(hour=14, minute=0, second=0, nanosecond=0)
        assert (zdt.hour, zdt.minute, zdt.second, zdt.nanosecond) == (14, 0, 0, 0)

    def test_with_time(self) -> None:
        """
        ```rust
        use jiff::{civil::time, ZonedDateTime};

        let zdt = ZonedDateTime::now().with().time(time(14, 0, 0, 0)).build()?;
        println!("{zdt}");
        // Output: 2024-07-12T14:00:00-04:00[America/New_York]
        ```

        (the `const { time(14, 0, 0, 0) }` variant is rust-only)
        """
        now = ry.ZonedDateTime.now()
        zdt = now.replace(time=ry.time(14, 0, 0, 0))
        assert zdt.time() == ry.time(14, 0, 0, 0)
        assert zdt.date() == now.date()


def test_print_current_unix_timestamp() -> None:
    """
    ```rust
    use jiff::Timestamp;

    let now = Timestamp::now();
    println!("{}", now.as_second());
    // Output: 1720646365

    let now = Timestamp::now();
    println!("{}", now.as_nanosecond());
    // Output: 1720646414218901664
    ```
    """
    ts = ry.Timestamp.now()
    sec = ts.as_second()
    assert sec > 1_600_000_000
    assert ts.as_nanosecond() // 1_000_000_000 == sec


def test_print_datetime_for_a_timestamp() -> None:
    """
    ```rust
    use jiff::{tz::TimeZone, Timestamp};

    let ts = Timestamp::from_millisecond(1_720_646_365_567)?;
    let zdt = ts.to_zoned_datetime(TimeZone::system());
    println!("{zdt}");
    // Output: 2024-07-10T17:19:25.567-04:00[America/New_York]
    // Or if you just want the RFC 3339 time without bothering with time zones:
    assert_eq!(ts.to_string(), "2024-07-10T21:19:25.567Z");
    ```
    """
    ts = ry.Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned_datetime(ry.TimeZone(_NY))
    assert str(zdt) == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    assert str(ts) == "2024-07-10T21:19:25.567Z"


def test_create_zoned_datetime_from_civil_time() -> None:
    """
    ```rust
    use jiff::civil::date;

    let zdt = date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")?;
    assert_eq!(zdt.to_string(), "2023-12-31T18:30:00-05:00[America/New_York]");
    ```
    """
    zdt = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz(_NY)
    assert str(zdt) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_change_an_instant_from_one_timezone_to_another() -> None:
    """
    ```rust
    use jiff::civil::date;

    let zdt1 = date(1918, 11, 11).at(11, 0, 0, 0).in_tz("Europe/Paris")?;
    let zdt2 = zdt1.in_tz("America/New_York")?;
    assert_eq!(
        zdt2.to_string(),
        "1918-11-11T06:00:00-05:00[America/New_York]",
    );
    ```
    """
    zdt1 = ry.date(1918, 11, 11).at(11, 0, 0, 0).in_tz("Europe/Paris")
    zdt2 = zdt1.in_tz(_NY)
    assert str(zdt2) == "1918-11-11T06:00:00-05:00[America/New_York]"


class TestFindDurationBetweenTwoZonedDatetimes:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz(_NY)
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz(_NY)

    def test_sub(self) -> None:
        """
        ```rust
        use jiff::civil::date;

        let zdt1 = date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")?;
        let zdt2 = date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")?;
        let span = zdt2 - zdt1;
        assert_eq!(format!("{span:#}"), "29341h 3m");
        ```
        """
        span = self.zdt2 - self.zdt1
        assert f"{span:#}" == "29341h 3m"

    def test_until_largest_year(self) -> None:
        """
        ```rust
        use jiff::{civil::date, Unit};

        let zdt1 = date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")?;
        let zdt2 = date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")?;
        let span = zdt1.until((Unit::Year, &zdt2))?;
        assert_eq!(format!("{span:#}"), "3y 4mo 5d 12h 3m");
        ```
        """
        span = self.zdt1.until(self.zdt2, largest="year")
        assert f"{span:#}" == "3y 4mo 5d 12h 3m"


def test_add_duration_to_a_zoned_datetime() -> None:
    """
    ```rust
    use jiff::{civil::date, ToSpan};

    let zdt1 = date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")?;
    let span = 3.years().months(4).days(5).hours(12).minutes(3);
    let zdt2 = zdt1.checked_add(span)?;
    assert_eq!(zdt2.to_string(), "2023-12-31T18:30:00-05:00[America/New_York]");
    ```
    """
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz(_NY)
    span = ry.TimeSpan(years=3, months=4, days=5, hours=12, minutes=3)
    zdt2 = zdt1.add(span)  # python ~ everything is `checked`
    assert str(zdt2) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity() -> None:
    """
    ```rust
    use jiff::civil::date;

    // 2:30 on 2024-03-10 in New York didn't exist. It's a "gap."
    // The compatible strategy selects the datetime after the gap.
    let zdt = date(2024, 3, 10).at(2, 30, 0, 0).in_tz("America/New_York")?;
    assert_eq!(zdt.to_string(), "2024-03-10T03:30:00-04:00[America/New_York]");

    // 1:30 on 2024-11-03 in New York appeared twice. It's a "fold."
    // The compatible strategy selects the datetime before the fold.
    let zdt = date(2024, 11, 3).at(1, 30, 0, 0).in_tz("America/New_York")?;
    assert_eq!(zdt.to_string(), "2024-11-03T01:30:00-04:00[America/New_York]");
    ```
    """
    gap = ry.date(2024, 3, 10).at(2, 30, 0, 0).in_tz(_NY)
    assert str(gap) == "2024-03-10T03:30:00-04:00[America/New_York]"

    fold = ry.date(2024, 11, 3).at(1, 30, 0, 0).in_tz(_NY)
    assert str(fold) == "2024-11-03T01:30:00-04:00[America/New_York]"


class TestParsingASpan:
    expected = ry.TimeSpan(years=5, weeks=1, days=10, hours=5, minutes=59)

    def test_iso8601(self) -> None:
        """
        ```rust
        use jiff::Span;

        let span: Span = "P5y1w10dT5h59m".parse()?;
        let expected = Span::new().years(5).weeks(1).days(10).hours(5).minutes(59);
        assert_eq!(span, expected.fieldwise());
        ```
        """
        span = ry.TimeSpan.parse("P5y1w10dT5h59m")
        assert span == self.expected
        assert str(span) == "P5Y1W10DT5H59M"

    def test_friendly(self) -> None:
        """
        ```rust
        use jiff::Span;

        let expected = Span::new().years(5).weeks(1).days(10).hours(5).minutes(59);
        let span: Span = "5 years, 1 week, 10 days, 5 hours, 59 minutes".parse()?;
        assert_eq!(span, expected.fieldwise());
        let span: Span = "5yrs 1wk 10d 5hrs 59mins".parse()?;
        assert_eq!(span, expected.fieldwise());
        let span: Span = "5y 1w 10d 5h 59m".parse()?;
        assert_eq!(span, expected.fieldwise());
        ```
        """
        for s in (
            "5 years, 1 week, 10 days, 5 hours, 59 minutes",
            "5yrs 1wk 10d 5hrs 59mins",
            "5y 1w 10d 5h 59m",
        ):
            assert ry.TimeSpan.parse(s) == self.expected, s
        assert f"{self.expected:#}" == "5y 1w 10d 5h 59m"


def test_parsing_an_rfc2822_datetime_string() -> None:
    """
    ```rust
    use jiff::fmt::rfc2822;

    let zdt1 = rfc2822::parse("Thu, 29 Feb 2024 05:34 -0500")?;
    let zdt2 = zdt1.in_tz("Australia/Hobart")?;
    assert_eq!(rfc2822::to_string(&zdt2)?, "Thu, 29 Feb 2024 21:34:00 +1100");
    let zdt3 = zdt1.in_tz("Asia/Kolkata")?;
    assert_eq!(rfc2822::to_string(&zdt3)?, "Thu, 29 Feb 2024 16:04:00 +0530");
    ```
    """
    zdt1 = ry.ZonedDateTime.parse_rfc2822("Thu, 29 Feb 2024 05:34 -0500")
    zdt2 = zdt1.in_tz("Australia/Hobart")
    assert zdt2.format_rfc2822() == "Thu, 29 Feb 2024 21:34:00 +1100"
    zdt3 = zdt1.in_tz("Asia/Kolkata")
    assert zdt3.format_rfc2822() == "Thu, 29 Feb 2024 16:04:00 +0530"


class TestStrftimeAndStrptime:
    hobart = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Hobart")

    def test_strptime(self) -> None:
        """
        ```rust
        use jiff::ZonedDateTime;

        let zdt = ZonedDateTime::strptime(
            "%A, %B %d, %Y at %I:%M%p %Q",
            "Monday, July 15, 2024 at 5:30pm America/New_York",
        )?;
        assert_eq!(zdt.to_string(), "2024-07-15T17:30:00-04:00[America/New_York]");
        ```
        """
        # NOTE: ry's arg order is (string, format) like python's `strptime`
        zdt = ry.ZonedDateTime.strptime(
            "Monday, July 15, 2024 at 5:30pm America/New_York",
            "%A, %B %d, %Y at %I:%M%p %Q",
        )
        assert str(zdt) == "2024-07-15T17:30:00-04:00[America/New_York]"

    def test_strftime_tz_abbreviation(self) -> None:
        """
        ```rust
        use jiff::civil::date;

        let zdt = date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Hobart")?;
        // %-I instead of %I means no padding.
        let string = zdt.strftime("%A, %B %d, %Y at %-I:%M%P %Z").to_string();
        assert_eq!(string, "Monday, July 15, 2024 at 5:30pm AEST");
        ```
        """
        formatted = self.hobart.strftime("%A, %B %d, %Y at %-I:%M%P %Z")
        assert formatted == "Monday, July 15, 2024 at 5:30pm AEST"

    def test_strftime_iana_name(self) -> None:
        """
        ```rust
        use jiff::civil::date;

        let zdt = date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Hobart")?;
        // %-I instead of %I means no padding.
        let string = zdt.strftime("%A, %B %d, %Y at %-I:%M%P %Q").to_string();
        assert_eq!(string, "Monday, July 15, 2024 at 5:30pm Australia/Hobart");
        ```
        """
        formatted = self.hobart.strftime("%A, %B %d, %Y at %-I:%M%P %Q")
        assert formatted == "Monday, July 15, 2024 at 5:30pm Australia/Hobart"


@dataclass
class _Record:
    timestamp: ry.Timestamp

    def to_json(self) -> str:
        return json.dumps(
            {"timestamp": self.timestamp.as_second()}, separators=(",", ":")
        )

    @classmethod
    def from_json(cls, raw: str) -> _Record:
        return cls(timestamp=ry.Timestamp.from_second(json.loads(raw)["timestamp"]))


def test_serializing_and_deserializing_integer_timestamps() -> None:
    """
    ```rust
    use jiff::Timestamp;

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Record {
        #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
        timestamp: Timestamp,
    }

    let json = r#"{"timestamp":1517644800}"#;
    let got: Record = serde_json::from_str(&json)?;
    assert_eq!(got.timestamp, Timestamp::from_second(1517644800)?);
    assert_eq!(serde_json::to_string(&got)?, json);
    ```
    """
    raw = '{"timestamp":1517644800}'
    got = _Record.from_json(raw)
    assert got.timestamp == ry.Timestamp.from_second(1_517_644_800)
    assert got.to_json() == raw
