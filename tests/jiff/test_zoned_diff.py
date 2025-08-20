from __future__ import annotations

import ry


class TestZonedUntil:
    def test_zoned_until(self) -> None:
        """
        ```rust
        use jiff::{civil::date, ToSpan};

        let earlier = date(2006, 8, 24).at(22, 30, 0, 0).in_tz("America/New_York")?;
        let later = date(2019, 1, 31).at(21, 0, 0, 0).in_tz("America/New_York")?;
        assert_eq!(earlier.until(&later)?, 109_031.hours().minutes(30));

        // Flipping the dates is fine, but you'll get a negative span.
        assert_eq!(later.until(&earlier)?, -109_031.hours().minutes(30));
        ```
        """
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0).in_tz("America/New_York")
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0).in_tz("America/New_York")
        assert earlier.until(later) == ry.timespan(hours=109_031, minutes=30)

        assert later.until(earlier) == ry.timespan(hours=-109_031, minutes=30)

    def test_zoned_until_using_bigger_units(self) -> None:
        """
        ```rust
        use jiff::{civil::date, Unit, ToSpan};

        let zdt1 = date(1995, 12, 07).at(3, 24, 30, 3500).in_tz("America/New_York")?;
        let zdt2 = date(2019, 01, 31).at(15, 30, 0, 0).in_tz("America/New_York")?;

        // The default limits durations to using "hours" as the biggest unit.
        let span = zdt1.until(&zdt2)?;
        assert_eq!(span.to_string(), "PT202956h5m29.9999965s");

        // But we can ask for units all the way up to years.
        let span = zdt1.until((Unit::Year, &zdt2))?;
        assert_eq!(span.to_string(), "P23y1m24dT12h5m29.9999965s");
        ```
        """
        zdt1 = ry.date(1995, 12, 7).at(3, 24, 30, 3500).in_tz("America/New_York")
        zdt2 = ry.date(2019, 1, 31).at(15, 30, 0, 0).in_tz("America/New_York")

        span = zdt1.until(zdt2)
        assert str(span) == "PT202956H5M29.9999965S"

        span = zdt1.until(zdt2, largest="year")
        assert str(span) == "P23Y1M24DT12H5M29.9999965S"

    def test_zoned_until_rounding_the_result(self) -> None:
        """
        ```rust
        use jiff::{civil::date, Unit, ToSpan, ZonedDifference};

        let zdt1 = date(1995, 12, 07).at(3, 24, 30, 3500).in_tz("America/New_York")?;
        let zdt2 = date(2019, 01, 31).at(15, 30, 0, 0).in_tz("America/New_York")?;

        let span = zdt1.until(
            ZonedDifference::from(&zdt2).smallest(Unit::Second),
        )?;
        assert_eq!(span, 202_956.hours().minutes(5).seconds(29));

        // We can combine smallest and largest units too!
        let span = zdt1.until(
            ZonedDifference::from(&zdt2)
                .smallest(Unit::Second)
                .largest(Unit::Year),
        )?;
        assert_eq!(span, 23.years().months(1).days(24).hours(12).minutes(5).seconds(29));
        ```
        """
        zdt1 = ry.date(1995, 12, 7).at(3, 24, 30, 3500).in_tz("America/New_York")
        zdt2 = ry.date(2019, 1, 31).at(15, 30, 0, 0).in_tz("America/New_York")

        span = zdt1.until(zdt2, smallest="second")
        assert span == ry.timespan(hours=202_956, minutes=5, seconds=29)

        span = zdt1.until(zdt2, smallest="second", largest="year")
        assert span == ry.timespan(
            years=23, months=1, days=24, hours=12, minutes=5, seconds=29
        )

    def test_units_biggers_than_days_inhibit_reversibility(self) -> None:
        """
        ```rust
        use jiff::{civil::date, Unit, ToSpan};

        let zdt1 = date(2024, 3, 2).at(0, 0, 0, 0).in_tz("America/New_York")?;
        let zdt2 = date(2024, 5, 1).at(0, 0, 0, 0).in_tz("America/New_York")?;

        let span = zdt1.until((Unit::Month, &zdt2))?;
        assert_eq!(span, 1.month().days(29));
        let maybe_original = zdt2.checked_sub(span)?;
        // Not the same as the original datetime!
        assert_eq!(
            maybe_original,
            date(2024, 3, 3).at(0, 0, 0, 0).in_tz("America/New_York")?,
        );

        // But in the default configuration, hours are always the biggest unit
        // and reversibility is guaranteed.
        let span = zdt1.until(&zdt2)?;
        assert_eq!(span, 1439.hours());
        let is_original = zdt2.checked_sub(span)?;
        assert_eq!(is_original, zdt1);
        ```
        """
        zdt1 = ry.date(2024, 3, 2).at(0, 0, 0, 0).in_tz("America/New_York")
        zdt2 = ry.date(2024, 5, 1).at(0, 0, 0, 0).in_tz("America/New_York")

        span = zdt1.until(zdt2, largest="month")
        assert span == ry.timespan(months=1, days=29)
        maybe_original = zdt2 - span
        assert maybe_original == ry.date(2024, 3, 3).at(0, 0, 0, 0).in_tz(
            "America/New_York"
        )

        span = zdt1.until(zdt2)
        assert span == ry.timespan(hours=1439)
        is_original = zdt2.sub(span)
        assert is_original == zdt1


class TestZonedSince:
    def test_zoned_since(self) -> None:
        """
        ```rust
        use jiff::{civil::date, ToSpan};

        let earlier = date(2006, 8, 24).at(22, 30, 0, 0).in_tz("America/New_York")?;
        let later = date(2019, 1, 31).at(21, 0, 0, 0).in_tz("America/New_York")?;
        assert_eq!(&later - &earlier, 109_031.hours().minutes(30));
        ```
        """
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0).in_tz("America/New_York")
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0).in_tz("America/New_York")
        assert later - earlier == ry.timespan(hours=109_031, minutes=30)
