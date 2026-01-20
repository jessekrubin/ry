from __future__ import annotations

import datetime as pydt
import re

import pytest

import ry


class TestDate:
    d = ry.date(2020, 8, 26)

    def test_to_datetime(self) -> None:
        d = ry.date(2023, 3, 14)
        dt = d.to_datetime(ry.time(1, 2, 3))
        assert isinstance(dt, ry.DateTime)
        assert dt.year == 2023
        assert dt.month == 3
        assert dt.day == 14
        assert dt.hour == 1
        assert dt.minute == 2
        assert dt.second == 3

    def test_to_zoned(self) -> None:
        d = ry.date(2023, 3, 14)
        zdt = d.to_zoned(ry.TimeZone("America/New_York"))
        assert isinstance(zdt, ry.ZonedDateTime)
        assert zdt.year == 2023
        assert zdt.month == 3
        assert zdt.day == 14
        assert zdt.timezone == ry.TimeZone("America/New_York")

    def test_richcmp(self) -> None:
        d1 = ry.date(2020, 8, 26)
        d2 = ry.date(2020, 8, 27)
        assert d1 < d2
        assert d1 <= d2
        assert d2 > d1
        assert d2 >= d1
        assert d1 != d2
        assert d1 == ry.date(2020, 8, 26)

    def test_strptime(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.Date.html#method.strptime
        """
        date = ry.Date.strptime("7/14/24", "%m/%d/%y")
        assert date.to_string() == "2024-07-14"

    # COPIED FROM jiff/tests/jiff/test_datetime.py

    def test_tomorrow(self) -> None:
        tomorrow = self.d.tomorrow()
        assert isinstance(tomorrow, ry.Date)
        assert tomorrow == ry.date(2020, 8, 27)

    def test_yesterday(self) -> None:
        yesterday = self.d.yesterday()
        assert isinstance(yesterday, ry.Date)
        assert yesterday == ry.date(2020, 8, 25)

    def test_date_to_iso_week_date(self) -> None:
        iwd = self.d.iso_week_date()
        assert isinstance(iwd, ry.ISOWeekDate)
        assert iwd == ry.ISOWeekDate(2020, 35, 3)

    def test_to_pydate(self) -> None:
        dt = self.d.to_pydate()
        assert isinstance(dt, pydt.date)
        assert dt == pydt.date(2020, 8, 26)

    def test_weekday(self) -> None:
        weekday = self.d.weekday
        assert isinstance(weekday, int)
        assert weekday == 3

        cur = self.d
        days = [cur]
        for _ in range(7):
            cur = cur.tomorrow()
            days.append(cur)
        assert [d.weekday for d in days] == [3, 4, 5, 6, 7, 1, 2, 3]

    def test_today(self) -> None:
        today = ry.DateTime.today()
        assert isinstance(today, ry.DateTime)
        assert today.date() == ry.Date.today()

    def test_millisecond(self) -> None:
        dt = ry.DateTime(2020, 8, 26, 6, 27, 0, 123_456_789)
        assert dt.millisecond == 123

    def test_microsecond(self) -> None:
        dt = ry.DateTime(2020, 8, 26, 6, 27, 0, 123_456_789)
        assert dt.microsecond == 456

    @pytest.mark.parametrize(
        ("dt", "expected"),
        [
            (ry.date(2006, 8, 24), 236),
            (ry.date(2023, 12, 31), 365),
            (ry.date(2024, 12, 31), 366),
        ],
    )
    def test_day_of_year(self, dt: ry.Date, expected: int) -> None:
        assert dt.day_of_year() == expected

    @pytest.mark.parametrize(
        ("dt", "expected"),
        [
            (ry.date(2006, 8, 24), 236),
            (ry.date(2023, 12, 31), 365),
            (ry.date(2024, 12, 31), 365),
            (ry.date(2024, 2, 29), None),
        ],
    )
    def test_day_of_year_no_leap(self, dt: ry.Date, expected: int | None) -> None:
        assert dt.day_of_year_no_leap() == expected

    @pytest.mark.parametrize(
        ("dt", "expected"),
        [
            (ry.date(2024, 2, 10), 29),
            (ry.date(2023, 2, 10), 28),
            (ry.date(2024, 8, 15), 31),
        ],
    )
    def test_days_in_month(self, dt: ry.Date, expected: int) -> None:
        assert dt.days_in_month() == expected

    def test_days_in_year_in_leap_year(self) -> None:
        leap_d = ry.date(2024, 1, 1)
        assert leap_d.days_in_year() == 366
        assert leap_d.in_leap_year()
        non_leap_d = ry.date(2023, 1, 1)
        assert not non_leap_d.in_leap_year()
        assert non_leap_d.days_in_year() == 365

    def test_first_of_year(self) -> None:
        d = ry.date(2024, 2, 5)
        assert d.first_of_year() == ry.date(2024, 1, 1)

    def test_last_of_year(self) -> None:
        d = ry.date(2024, 2, 5)
        assert d.last_of_year() == ry.date(2024, 12, 31)

    def test_first_of_month(self) -> None:
        d = ry.date(2024, 2, 5)
        assert d.first_of_month() == ry.date(2024, 2, 1)

    def test_last_of_month(self) -> None:
        d = ry.date(2024, 2, 5)
        assert d.last_of_month() == ry.date(2024, 2, 29)


class TestDateAdd:
    @pytest.mark.parametrize(
        ("d", "delta", "expected"),
        [
            # days
            (ry.date(2023, 3, 14), ry.timespan(days=1), ry.date(2023, 3, 15)),
            (ry.date(2023, 3, 14), ry.timespan(days=-1), ry.date(2023, 3, 13)),
            # weeks
            (ry.date(2023, 3, 14), ry.timespan(weeks=1), ry.date(2023, 3, 21)),
            (ry.date(2023, 3, 14), ry.timespan(weeks=-1), ry.date(2023, 3, 7)),
            # months
            (ry.date(2023, 3, 14), ry.timespan(months=1), ry.date(2023, 4, 14)),
            (ry.date(2023, 3, 14), ry.timespan(months=-1), ry.date(2023, 2, 14)),
            # years
            (ry.date(2023, 3, 14), ry.timespan(years=1), ry.date(2024, 3, 14)),
            (ry.date(2023, 3, 14), ry.timespan(years=-1), ry.date(2022, 3, 14)),
        ],
    )
    def test_date_add_days(
        self, d: ry.Date, delta: ry.TimeSpan, expected: ry.Date
    ) -> None:
        via_operator = d + delta
        via_add_fn = d.add(delta)
        assert via_operator == expected
        assert via_add_fn == expected


class TestDateUntil:
    """
    ```
    use jiff::{civil::date, ToSpan};

    let earlier = date(2006, 8, 24);
    let later = date(2019, 1, 31);
    assert_eq!(earlier.until(later)?, 4543.days());

    // Flipping the dates is fine, but you'll get a negative span.
    let earlier = date(2006, 8, 24);
    let later = date(2019, 1, 31);
    assert_eq!(later.until(earlier)?, -4543.days());
    ```
    """

    def test_date_until_overflow(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert earlier.until(later) == ry.timespan(days=4543)

        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert later.until(earlier) == ry.timespan(days=-4543)


class TestDateTomorrowYesterday:
    def test_date_tomorrow(self) -> None:
        d = ry.date(2023, 3, 14)
        assert d.tomorrow() == ry.date(2023, 3, 15)

    def test_date_yesterday(self) -> None:
        d = ry.date(2023, 3, 14)
        assert d.yesterday() == ry.date(2023, 3, 13)


class TestDateReplace:
    def test_replace_ok(self) -> None:
        d = ry.date(2023, 1, 1)
        assert d.replace(day_of_year_no_leap=365) == ry.date(2023, 12, 31)
        # leap year
        d = ry.date(2024, 1, 1)
        assert d.replace(day_of_year_no_leap=365) == ry.date(2024, 12, 31)

    def test_replace_err(self) -> None:
        d = ry.date(2024, 11, 30)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' with value 31 is not in the required range of 1..=30"
            ),
        ):
            _r = d.replace(day=31)
        d = ry.date(2024, 2, 29)
        with pytest.raises(ValueError):
            _r = d.replace(year=2023)

    # ==== YEAR ====
    def test_replace_year_ok(self) -> None:
        d1 = ry.date(2005, 11, 5)
        assert d1.year == 2005
        d2 = d1.replace(year=2007)
        assert d2.year == 2007

    def test_replace_year_err(self) -> None:
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' with value 29 is not in the required range of 1..=28"
            ),
        ):
            d1 = ry.date(2024, 2, 29)
            d1.replace(year=2023)

    # ==== ERA YEAR ====
    def test_replace_era_year_ce(self) -> None:
        d1 = ry.date(2005, 11, 5)
        d2 = d1.replace(era_year=(2007, "CE"))
        assert d2.year == 2007
        assert d2.era_year() == (2007, "CE")

        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'CE year' with value -5 is not in the required range of 1..=9999"
            ),
        ):
            d1.replace(era_year=(-5, "CE"))
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'CE year' with value 10000 is not in the required range of 1..=9999"
            ),
        ):
            d1.replace(era_year=(10_000, "CE"))

    def test_replace_era_year_bce(self) -> None:
        d1 = ry.date(-27, 7, 1)
        d2 = d1.replace(era_year=(509, "BCE"))
        assert d2.year == -508
        d3 = d1.replace(era_year=(10_000, "BCE"))
        assert d3.year == -9_999

        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'BCE year' with value -5 is not in the required range of 1..=10000"
            ),
        ):
            d1.replace(era_year=(-5, "BCE"))
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'BCE year' with value 10001 is not in the required range of 1..=10000"
            ),
        ):
            d1.replace(era_year=(10_001, "BCE"))

    # ==== MONTH ====
    def test_replace_month(self) -> None:
        d1 = ry.date(2005, 11, 5)
        d2 = d1.replace(month=6)
        assert d2.month == 6

    def test_replace_month_err(self) -> None:
        d1 = ry.date(2024, 10, 31)
        with pytest.raises(ValueError):
            d1.replace(month=11)

    # ==== DAY ====
    def test_replace_day(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateWith.html#example-7
        """
        d1 = ry.date(2024, 2, 5)
        d2 = d1.replace(day=10)
        assert d2.day == 10
        d3 = d1.replace(day=29)
        assert d3.day == 29

    def test_replace_day_err(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateWith.html#example-changing-only-the-day-can-fail
        """
        d1 = ry.date(2023, 2, 5)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' with value 29 is not in the required range of 1..=28"
            ),
        ):
            d1.replace(day=29)
        d1 = ry.date(2023, 9, 5)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' with value 31 is not in the required range of 1..=30"
            ),
        ):
            d1.replace(day=31)

    # ==== DAY OF YEAR ====
    def test_replace_day_of_year(self) -> None:
        d1 = ry.date(2024, 1, 1)
        d2 = d1.replace(day_of_year=60)
        assert d2.month == 2
        assert d2.day == 29
        d1 = ry.date(2023, 1, 1)
        d2 = d1.replace(day_of_year=60)
        assert d2.month == 3
        assert d2.day == 1

    def test_replace_day_of_year_err(self) -> None:
        d1 = ry.date(2023, 1, 1)
        with pytest.raises(ValueError):
            d1.replace(day_of_year=366)
        d1 = ry.date(9999, 1, 1)
        with pytest.raises(ValueError):
            d1.replace(day_of_year=366)

    # ==== DAY OF YEAR NO LEAP ====
    def test_replace_day_of_year_no_leap(self) -> None:
        d1 = ry.date(2023, 1, 1)
        d2 = d1.replace(day_of_year_no_leap=60)
        assert d2.month == 3
        assert d2.day == 1
        d1 = ry.date(2024, 1, 1)
        d2 = d1.replace(day_of_year_no_leap=60)
        assert d2.month == 3
        assert d2.day == 1

    def test_replace_day_of_year_no_leap_err(self) -> None:
        d1 = ry.date(2023, 1, 1)

        err_msg = "number of days is invalid, must be in range `1..=365`"
        with pytest.raises(ValueError, match=re.escape(err_msg)):
            d1.replace(day_of_year_no_leap=366)
        d1 = ry.date(9999, 1, 1)
        with pytest.raises(ValueError, match=re.escape(err_msg)):
            d1.replace(day_of_year_no_leap=366)


class TestDateSinceUntil:
    def test_until(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert earlier.until(later) == ry.TimeSpan(days=4543)
        assert later.until(earlier) == -ry.TimeSpan(days=4543)

    def test_since(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert later.since(earlier) == ry.TimeSpan(days=4543)
        assert earlier.since(later) == -ry.TimeSpan(days=4543)

    def test_until_using_bigger_units(self) -> None:
        d1 = ry.date(1995, 12, 7)
        d2 = ry.date(2019, 1, 31)
        span = d1.until(d2)
        assert span.to_string() == "P8456D"
        span = d1.until(d2, largest="year")
        assert span.to_string() == "P23Y1M24D"

    def test_until_rounding_the_result(self) -> None:
        d1 = ry.date(1995, 12, 7)
        d2 = ry.date(2019, 1, 31)
        span = d1.until(d2, smallest="month")
        assert f"{span:#}" == "277mo"
        span = d1.until(d2, smallest="month", largest="year")
        assert span.to_string() == "P23Y1M"

    def test_until_bigger_than_days_inhibit_reversibility(self) -> None:
        d1 = ry.date(2024, 3, 2)
        d2 = ry.date(2024, 5, 1)
        span = d1.until(d2, largest="month")
        maybe_original = d2.sub(span)
        assert maybe_original != d1
        span = d1.until(d2)
        is_original = d2.sub(span)
        assert is_original == d1

    def test_since_example(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert later - earlier == ry.TimeSpan(days=4543)

    def test_duration_until(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert earlier.duration_until(later) == ry.SignedDuration.from_hours(4543 * 24)

    def test_duration_since(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert later.duration_since(earlier) == ry.SignedDuration.from_hours(4543 * 24)


class TestDateSaturatingArithmetic:
    def test_saturating_add(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.Date.html#method.saturating_add
        """
        dt = ry.date(2024, 3, 31)
        assert dt.saturating_add(ry.timespan(years=9000)) == ry.Date.MAX
        assert dt.saturating_add(ry.timespan(years=-19000)) == ry.Date.MIN
        assert dt.saturating_add(ry.SignedDuration.MAX) == ry.Date.MAX
        assert dt.saturating_add(ry.SignedDuration.MIN) == ry.Date.MIN
        assert dt.saturating_add(ry.Duration.MAX) == ry.Date.MAX

    def test_saturating_sub(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.Date.html#method.saturating_sub
        """
        dt = ry.date(2024, 3, 31)
        assert dt.saturating_sub(ry.timespan(years=19000)) == ry.Date.MIN
        assert dt.saturating_sub(ry.timespan(years=-9000)) == ry.Date.MAX
        assert dt.saturating_sub(ry.SignedDuration.MAX) == ry.Date.MIN
        assert dt.saturating_sub(ry.SignedDuration.MIN) == ry.Date.MAX
        assert dt.saturating_sub(ry.Duration.MAX) == ry.Date.MIN
