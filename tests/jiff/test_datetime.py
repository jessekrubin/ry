from __future__ import annotations

import datetime as pydt
import itertools as it
import re

import pytest

import ry

_JIFF_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
]

_JIFF_ROUND_MODES = [
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half-ceil",
    "half-floor",
    "half-expand",
    "half-trunc",
    "half-even",
]


class TestDateTime:
    dt = ry.date(2020, 8, 26).at(6, 27, 0, 0)

    def test_from_parts(self) -> None:
        dt = ry.DateTime.from_parts(ry.date(2020, 8, 26), ry.time(6, 27, 0, 0))
        assert dt == self.dt

    def test_datetime_round_options(self) -> None:
        default = ry.DateTimeRound()
        expected_default_string = (
            'DateTimeRound(smallest="nanosecond", mode="half-expand", increment=1)'
        )
        assert str(default) == expected_default_string

        for unit, mode in it.product(_JIFF_UNITS, _JIFF_ROUND_MODES):
            options = ry.DateTimeRound(smallest=unit, mode=mode, increment=1)  # type: ignore[arg-type]

            options_chained = (
                ry.DateTimeRound()._smallest(unit)._mode(mode)._increment(1)  # type: ignore[arg-type]
            )
            expected_string = (
                f'DateTimeRound(smallest="{unit}", mode="{mode}", increment=1)'
            )
            assert str(options) == expected_string
            assert options == options_chained

    def test_tomorrow(self) -> None:
        tomorrow = self.dt.tomorrow()
        assert isinstance(tomorrow, ry.DateTime)
        assert tomorrow == ry.date(2020, 8, 27).at(6, 27, 0, 0)

    def test_yesterday(self) -> None:
        yesterday = self.dt.yesterday()
        assert isinstance(yesterday, ry.DateTime)
        assert yesterday == ry.date(2020, 8, 25).at(6, 27, 0, 0)

    def test_datetime_to_iso_week_date(self) -> None:
        iwd = self.dt.iso_week_date()
        assert isinstance(iwd, ry.ISOWeekDate)
        assert iwd == ry.ISOWeekDate(2020, 35, 3)

    def test_to_date(self) -> None:
        d = self.dt.date()
        assert isinstance(d, ry.Date)
        assert d == ry.date(2020, 8, 26)

    def test_to_time(self) -> None:
        t = self.dt.time()
        assert isinstance(t, ry.Time)
        assert t == ry.time(6, 27, 0, 0)

    def test_to_pydate(self) -> None:
        dt = self.dt.to_pydate()
        assert isinstance(dt, pydt.date)
        assert dt == pydt.date(2020, 8, 26)

    def test_to_pytime(self) -> None:
        t = self.dt.to_pytime()
        assert isinstance(t, pydt.time)
        assert t == pydt.time(6, 27, 0, 0)

    def test_start_of_day(self) -> None:
        sod = self.dt.start_of_day()
        assert isinstance(sod, ry.DateTime)
        assert sod == ry.date(2020, 8, 26).at(0, 0, 0, 0)

    def test_end_of_day(self) -> None:
        eod = self.dt.end_of_day()
        assert isinstance(eod, ry.DateTime)
        assert eod == ry.date(2020, 8, 26).at(23, 59, 59, 999_999_999)

    def test_weekday(self) -> None:
        weekday = self.dt.weekday
        assert isinstance(weekday, int)
        assert weekday == 3

        cur = self.dt
        days = [cur]
        for _ in range(7):
            cur = cur.tomorrow()
            days.append(cur)
        assert [d.weekday for d in days] == [3, 4, 5, 6, 7, 1, 2, 3]

    def test_strptime(self) -> None:
        """REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.strptime"""
        dt = ry.DateTime.strptime("2024-07-14 21:14", "%F %H:%M")
        assert dt.to_string() == "2024-07-14T21:14:00"

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
            (ry.date(2006, 8, 24).at(7, 30, 0, 0), 236),
            (ry.date(2023, 12, 31).at(7, 30, 0, 0), 365),
            (ry.date(2024, 12, 31).at(7, 30, 0, 0), 366),
        ],
    )
    def test_day_of_year(self, dt: ry.DateTime, expected: int) -> None:
        assert dt.day_of_year() == expected

    @pytest.mark.parametrize(
        ("dt", "expected"),
        [
            (ry.date(2006, 8, 24).at(7, 30, 0, 0), 236),
            (ry.date(2023, 12, 31).at(7, 30, 0, 0), 365),
            (ry.date(2024, 12, 31).at(7, 30, 0, 0), 365),
            (ry.date(2024, 2, 29).at(7, 30, 0, 0), None),
        ],
    )
    def test_day_of_year_no_leap(self, dt: ry.DateTime, expected: int | None) -> None:
        assert dt.day_of_year_no_leap() == expected

    @pytest.mark.parametrize(
        ("dt", "expected"),
        [
            (ry.date(2024, 2, 10).at(7, 30, 0, 0), 29),
            (ry.date(2023, 2, 10).at(7, 30, 0, 0), 28),
            (ry.date(2024, 8, 15).at(7, 30, 0, 0), 31),
        ],
    )
    def test_days_in_month(self, dt: ry.DateTime, expected: int) -> None:
        assert dt.days_in_month() == expected

    def test_days_in_year_in_leap_year(self) -> None:
        leap_dt = ry.date(2024, 1, 1).at(0, 0, 0, 0)
        assert leap_dt.days_in_year() == 366
        assert leap_dt.in_leap_year()
        non_leap_dt = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        assert not non_leap_dt.in_leap_year()
        assert non_leap_dt.days_in_year() == 365

    def test_first_of_year(self) -> None:
        dt = ry.date(2024, 2, 5).at(7, 30, 0, 0)
        assert dt.first_of_year() == ry.date(2024, 1, 1).at(7, 30, 0, 0)

    def test_last_of_year(self) -> None:
        dt = ry.date(2024, 2, 5).at(7, 30, 0, 0)
        assert dt.last_of_year() == ry.date(2024, 12, 31).at(7, 30, 0, 0)

    def test_first_of_month(self) -> None:
        dt = ry.date(2024, 2, 5).at(7, 30, 0, 0)
        assert dt.first_of_month() == ry.date(2024, 2, 1).at(7, 30, 0, 0)

    def test_last_of_month(self) -> None:
        dt = ry.date(2024, 2, 5).at(7, 30, 0, 0)
        assert dt.last_of_month() == ry.date(2024, 2, 29).at(7, 30, 0, 0)


class TestDateTimeSinceUntil:
    def test_until(self) -> None:
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0)
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0)
        assert earlier.until(later) == ry.TimeSpan(days=4542, hours=22, minutes=30)
        assert later.until(earlier) == -ry.TimeSpan(days=4542, hours=22, minutes=30)

    def test_since(self) -> None:
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0)
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0)
        assert later.since(earlier) == ry.TimeSpan(days=4542, hours=22, minutes=30)
        assert earlier.since(later) == -ry.TimeSpan(days=4542, hours=22, minutes=30)

    def test_until_using_bigger_units(self) -> None:
        dt1 = ry.date(1995, 12, 7).at(3, 24, 30, 3500)
        dt2 = ry.date(2019, 1, 31).at(15, 30, 0, 0)
        span = dt1.until(dt2)
        assert span.to_string() == "P8456DT12H5M29.9999965S"
        span = dt1.until(dt2, largest="year")
        assert span.to_string() == "P23Y1M24DT12H5M29.9999965S"

    def test_until_rounding_the_result(self) -> None:
        dt1 = ry.date(1995, 12, 7).at(3, 24, 30, 3500)
        dt2 = ry.date(2019, 1, 31).at(15, 30, 0, 0)
        span = dt1.until(dt2, smallest="second")
        assert f"{span:#}" == "8456d 12h 5m 29s"
        span = dt1.until(dt2, smallest="second", largest="year")
        assert span.to_string() == "P23Y1M24DT12H5M29S"

    def test_until_bigger_than_days_inhibit_reversibility(self) -> None:
        dt1 = ry.date(2024, 3, 2).at(0, 0, 0, 0)
        dt2 = ry.date(2024, 5, 1).at(0, 0, 0, 0)
        span = dt1.until(dt2, largest="month")
        maybe_original = dt2.sub(span)
        assert maybe_original != dt1
        span = dt1.until(dt2)
        is_original = dt2.sub(span)
        assert is_original == dt1

    def test_since_example(self) -> None:
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0)
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0)
        assert later - earlier == ry.timespan(days=4542, hours=22, minutes=30)

    def test_duration_until(self) -> None:
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0)
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0)
        assert earlier.duration_until(later) == ry.SignedDuration.from_hours(
            4542 * 24 + 22
        ) + ry.SignedDuration.from_mins(30)

    def test_duration_since(self) -> None:
        earlier = ry.date(2006, 8, 24).at(22, 30, 0, 0)
        later = ry.date(2019, 1, 31).at(21, 0, 0, 0)
        assert later.duration_since(earlier) == ry.SignedDuration.from_hours(
            4542 * 24 + 22
        ) + ry.SignedDuration.from_mins(30)


class TestDateTimeSaturatingArithmetic:
    def test_saturating_add(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.saturating_add
        """
        dt = ry.date(2024, 3, 31).at(13, 13, 13, 13)
        assert dt.saturating_add(ry.timespan(years=9000)) == ry.DateTime.MAX
        assert dt.saturating_add(ry.timespan(years=-19000)) == ry.DateTime.MIN
        assert dt.saturating_add(ry.SignedDuration.MAX) == ry.DateTime.MAX
        assert dt.saturating_add(ry.SignedDuration.MIN) == ry.DateTime.MIN
        assert dt.saturating_add(ry.Duration.MAX) == ry.DateTime.MAX

    def test_saturating_sub(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.saturating_sub
        """
        dt = ry.date(2024, 3, 31).at(13, 13, 13, 13)
        assert dt.saturating_sub(ry.timespan(years=19000)) == ry.DateTime.MIN
        assert dt.saturating_sub(ry.timespan(years=-9000)) == ry.DateTime.MAX
        assert dt.saturating_sub(ry.SignedDuration.MAX) == ry.DateTime.MIN
        assert dt.saturating_sub(ry.SignedDuration.MIN) == ry.DateTime.MAX
        assert dt.saturating_sub(ry.Duration.MAX) == ry.DateTime.MIN


class TestDateTimeReplace:
    """Tests for `ry.DateTime.replace`

    Based on the docs for `jiff::civil::DateTimeWith`

    Ref: https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeWith.html
    """

    def test_invalid_positional_arg(self) -> None:
        dt1 = ry.date(2024, 10, 31).at(0, 0, 0, 0)
        with pytest.raises(
            TypeError, match=re.escape("obj must be a Date or Time; given: (1+2j)")
        ):
            dt1.replace(complex(1, 2))  # type: ignore[arg-type]

    def test_replace_example(self) -> None:

        dt1 = ry.date(2024, 10, 31).at(0, 0, 0, 0)
        dt2 = dt1.replace(month=11, day=30)
        assert dt2 == ry.date(2024, 11, 30).at(0, 0, 0, 0)

        dt1 = ry.date(2024, 4, 30).at(0, 0, 0, 0)
        dt2 = dt1.replace(day=31, month=7)
        assert dt2 == ry.date(2024, 7, 31).at(0, 0, 0, 0)

    def test_replace_invalid_datetime(self) -> None:
        dt1 = ry.date(2024, 2, 29).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' for `2024-02` is invalid, must be in range `1..=29`"
            ),
        ):
            dt1.replace(day=31)

        dt1 = ry.date(2024, 2, 29).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' for `2023-02` is invalid, must be in range `1..=28`"
            ),
        ):
            dt1.replace(year=2023)

    # ==== DATE ====
    def test_replace_date(self) -> None:
        dt1 = ry.date(2005, 11, 5).at(15, 30, 0, 0)
        # via kwarg
        dt2 = dt1.replace(date=ry.date(2017, 10, 31))
        assert dt2 == ry.date(2017, 10, 31).at(15, 30, 0, 0)
        # via first positional
        dt2 = dt1.replace(ry.date(2017, 10, 31))
        assert dt2 == ry.date(2017, 10, 31).at(15, 30, 0, 0)

    # ==== TIME ====
    def test_replace_time(self) -> None:
        dt1 = ry.date(2005, 11, 5).at(15, 30, 0, 0)
        # via kwarg
        dt2 = dt1.replace(time=ry.time(23, 59, 59, 123_456_789))
        assert dt2 == ry.date(2005, 11, 5).at(23, 59, 59, 123_456_789)
        # via first positional
        dt2 = dt1.replace(ry.time(23, 59, 59, 123_456_789))
        assert dt2 == ry.date(2005, 11, 5).at(23, 59, 59, 123_456_789)

    # ==== YEAR ====
    def test_replace_year_ok(self) -> None:
        dt1 = ry.date(2005, 11, 5).at(15, 30, 0, 0)
        assert dt1.year == 2005
        dt2 = dt1.replace(year=2007)
        assert dt2.year == 2007

    def test_replace_year_err(self) -> None:
        dt1 = ry.date(2024, 2, 29).at(1, 30, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' for `2023-02` is invalid, must be in range `1..=28`"
            ),
        ):
            dt1.replace(year=2023)

    # ==== ERA YEAR ====
    def test_replace_era_year_ce(self) -> None:
        dt1 = ry.date(2005, 11, 5).at(8, 0, 0, 0)
        dt2 = dt1.replace(era_year=(2007, "CE"))
        assert dt2.year == 2007
        assert dt2.era_year() == (2007, "CE")

        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'CE year' is not in the required range of 1..=9999"
            ),
        ):
            dt1.replace(era_year=(-5, "CE"))
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'CE year' is not in the required range of 1..=9999"
            ),
        ):
            dt1.replace(era_year=(10_000, "CE"))

    def test_replace_era_year_bce(self) -> None:
        dt1 = ry.date(-27, 7, 1).at(8, 22, 30, 0)
        dt2 = dt1.replace(era_year=(509, "BCE"))
        assert dt2.year == -508
        dt3 = dt1.replace(era_year=(10_000, "BCE"))
        assert dt3.year == -9_999

        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'BCE year' is not in the required range of 1..=10000"
            ),
        ):
            dt1.replace(era_year=(-5, "BCE"))
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'BCE year' is not in the required range of 1..=10000"
            ),
        ):
            dt1.replace(era_year=(10_001, "BCE"))

    # ==== MONTH ====
    def test_replace_month(self) -> None:
        dt1 = ry.date(2005, 11, 5).at(18, 3, 59, 123_456_789)
        dt2 = dt1.replace(month=6)
        assert dt2.month == 6

    def test_replace_month_err(self) -> None:
        dt1 = ry.date(2024, 10, 31).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' for `2024-11` is invalid, must be in range `1..=30`"
            ),
        ):
            dt1.replace(month=11)

    # ==== DAY ====
    def test_replace_day(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeWith.html#example-7
        """
        dt1 = ry.date(2024, 2, 5).at(21, 59, 1, 999)
        dt2 = dt1.replace(day=10)
        assert dt2.day == 10
        dt3 = dt1.replace(day=29)
        assert dt3.day == 29

    def test_replace_day_err(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeWith.html#example-changing-only-the-day-can-fail
        """
        dt1 = ry.date(2023, 2, 5).at(22, 58, 58, 9_999)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' for `2023-02` is invalid, must be in range `1..=28`"
            ),
        ):
            dt1.replace(day=29)
        dt1 = ry.date(2023, 9, 5).at(22, 58, 58, 9_999)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'day' for `2023-09` is invalid, must be in range `1..=30`"
            ),
        ):
            dt1.replace(day=31)

    # ==== DAY OF YEAR ====
    def test_replace_day_of_year(self) -> None:
        dt1 = ry.date(2024, 1, 1).at(23, 59, 59, 999_999_999)
        dt2 = dt1.replace(day_of_year=60)
        assert dt2.month == 2
        assert dt2.day == 29
        dt1 = ry.date(2023, 1, 1).at(23, 59, 59, 999_999_999)
        dt2 = dt1.replace(day_of_year=60)
        assert dt2.month == 3
        assert dt2.day == 1

    def test_replace_day_of_year_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match="number of days for `2023` is invalid, must be in range `1\\.\\.=365`",
        ):
            dt1.replace(day_of_year=366)
        dt1 = ry.date(9999, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(ValueError, match="day of year is invalid"):
            dt1.replace(day_of_year=366)

    # ==== DAY OF YEAR NO LEAP ====
    def test_replace_day_of_year_no_leap(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(23, 59, 59, 999_999_999)
        dt2 = dt1.replace(day_of_year_no_leap=60)
        assert dt2.month == 3
        assert dt2.day == 1
        dt1 = ry.date(2024, 1, 1).at(23, 59, 59, 999_999_999)
        dt2 = dt1.replace(day_of_year_no_leap=60)
        assert dt2.month == 3
        assert dt2.day == 1

    def test_replace_day_of_year_no_leap_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)

        err_msg = "number of days is invalid, must be in range `1..=365`"
        with pytest.raises(ValueError, match=re.escape(err_msg)):
            dt1.replace(day_of_year_no_leap=366)
        dt1 = ry.date(9999, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(ValueError, match=re.escape(err_msg)):
            dt1.replace(day_of_year_no_leap=366)

    # ==== HOUR ====
    def test_replace_hour(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        dt2 = dt1.replace(hour=12)
        assert dt2.hour == 12
        assert dt2.minute == 0
        assert dt2.second == 0
        assert dt2.nanosecond == 0

    def test_replace_hour_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape("parameter 'hour' is not in the required range of 0..=23"),
        ):
            dt1.replace(hour=24)

    # ==== MINUTE ====
    def test_replace_minute(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        dt2 = dt1.replace(minute=30)
        assert dt2.hour == 0
        assert dt2.minute == 30
        assert dt2.second == 0
        assert dt2.nanosecond == 0

    def test_replace_minute_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'minute' is not in the required range of 0..=59"
            ),
        ):
            dt1.replace(minute=60)

    # ==== SECOND ====
    def test_replace_second(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        dt2 = dt1.replace(second=30)
        assert dt2.hour == 0
        assert dt2.minute == 0
        assert dt2.second == 30
        assert dt2.nanosecond == 0

    def test_replace_second_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'second' is not in the required range of 0..=59"
            ),
        ):
            dt1.replace(second=60)

    # ==== MILLISECOND ====
    def test_replace_millisecond(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        dt2 = dt1.replace(millisecond=123)
        assert dt2.hour == 0
        assert dt2.minute == 0
        assert dt2.second == 0
        assert dt2.nanosecond == 0
        assert dt2.millisecond == 123
        assert dt2.subsec_nanosecond == 123_000_000

    def test_replace_millisecond_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'millisecond' is not in the required range of 0..=999"
            ),
        ):
            dt1.replace(millisecond=1000)

    # ==== MICROSECOND====
    def test_replace_microsecond(self) -> None:

        dt1 = ry.date(2010, 6, 1).at(15, 21, 35, 0)
        dt2 = dt1.replace(microsecond=123)
        assert dt2.subsec_nanosecond == 123_000

    # ==== NANOSECOND ====
    def test_replace_nanosecond(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        dt2 = dt1.replace(nanosecond=123)
        assert dt2.hour == 0
        assert dt2.minute == 0
        assert dt2.second == 0
        assert dt2.nanosecond == 123
        assert dt2.millisecond == 0

    def test_replace_nanosecond_err(self) -> None:
        dt1 = ry.date(2023, 1, 1).at(0, 0, 0, 0)
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'nanosecond' is not in the required range of 0..=999"
            ),
        ):
            dt1.replace(nanosecond=1_000)

    # ==== SUBSECOND NANOSECOND ====
    def test_replace_subsec_nanosecond(self) -> None:
        dt1 = ry.date(2010, 6, 1).at(15, 21, 35, 0)
        dt2 = dt1.replace(subsec_nanosecond=123_456_789)
        assert dt2.millisecond == 123
        assert dt2.microsecond == 456
        assert dt2.nanosecond == 789
