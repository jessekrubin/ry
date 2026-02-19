from __future__ import annotations

import datetime as pydt

import pytest

import ry


def test_zoned_eg() -> None:
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    assert str(zdt) == "2020-08-26T06:27:00-04:00[America/New_York]"

    zdt_fields = {
        "tz": str(zdt.timezone),
        "year": zdt.year,
        "month": zdt.month,
        "day": zdt.day,
        "hour": zdt.hour,
        "minute": zdt.minute,
        "second": zdt.second,
        "nanosecond": zdt.nanosecond,
        "subsec_nanosecond": zdt.subsec_nanosecond,
    }

    assert zdt_fields == {
        "tz": "America/New_York",
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 6,
        "minute": 27,
        "second": 0,
        "nanosecond": 0,
        "subsec_nanosecond": 0,
    }

    ry_datetime = zdt.datetime()
    assert ry_datetime == ry.datetime(2020, 8, 26, 6, 27, 0, 0)

    dt_fields = {
        "year": ry_datetime.year,
        "month": ry_datetime.month,
        "day": ry_datetime.day,
        "hour": ry_datetime.hour,
        "minute": ry_datetime.minute,
        "second": ry_datetime.second,
        "nanosecond": ry_datetime.nanosecond,
    }
    dt_dictionary = {
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 6,
        "minute": 27,
        "second": 0,
        "nanosecond": 0,
    }
    assert dt_fields == dt_dictionary
    assert ry_datetime.to_dict() == {
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 6,
        "minute": 27,
        "second": 0,
        "nanosecond": 0,
    }

    ry_time = zdt.time()
    assert ry_time == ry.time(6, 27, 0, 0)
    t_fields = {
        "hour": ry_time.hour,
        "minute": ry_time.minute,
        "second": ry_time.second,
        "microsecond": ry_time.microsecond,
    }
    assert t_fields == {
        "hour": 6,
        "minute": 27,
        "second": 0,
        "microsecond": 0,
    }

    expected_time_dict = {"hour": 6, "minute": 27, "second": 0, "nanosecond": 0}
    assert ry_time.to_dict() == expected_time_dict


class TestZonedDateTime:
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/Los_Angeles")

    def test_new_with_no_tz(self) -> None:
        zdt = ry.ZonedDateTime(2020, 8, 26, 6, 27, 0)
        tz = zdt.timezone
        assert isinstance(tz, ry.TimeZone)
        s = ry.TimeZone.system()
        assert tz == s

    def test_from_parts(self) -> None:
        ts = ry.Timestamp(second=1598448420, nanosecond=0)
        tz = ry.TimeZone("America/Los_Angeles")
        zdt = ry.ZonedDateTime.from_parts(ts, tz)
        assert zdt == self.zdt

    def test_tomorrow(self) -> None:
        tomorrow = self.zdt.tomorrow()
        assert isinstance(tomorrow, ry.ZonedDateTime)
        assert tomorrow == ry.date(2020, 8, 27).at(6, 27, 0, 0).in_tz(
            "America/Los_Angeles"
        )

    def test_yesterday(self) -> None:
        yesterday = self.zdt.yesterday()
        assert isinstance(yesterday, ry.ZonedDateTime)
        assert yesterday == ry.date(2020, 8, 25).at(6, 27, 0, 0).in_tz(
            "America/Los_Angeles"
        )

    def test_datetime_to_iso_week_date(self) -> None:
        iwd = self.zdt.iso_week_date()
        assert isinstance(iwd, ry.ISOWeekDate)
        assert iwd == ry.ISOWeekDate(2020, 35, 3)

    def test_to_date(self) -> None:
        d = self.zdt.date()
        assert isinstance(d, ry.Date)
        assert d == ry.date(2020, 8, 26)

    def test_to_time(self) -> None:
        t = self.zdt.time()
        assert isinstance(t, ry.Time)
        assert t == ry.time(6, 27, 0, 0)

    def test_to_pydate(self) -> None:
        py_d = self.zdt.to_pydate()
        assert isinstance(py_d, pydt.date)
        assert py_d == pydt.date(2020, 8, 26)

    def test_to_pytime(self) -> None:
        py_t = self.zdt.to_pytime()
        assert isinstance(py_t, pydt.time)
        assert py_t == pydt.time(6, 27, 0, 0)

    def test_start_of_day(self) -> None:
        sod = self.zdt.start_of_day()
        assert isinstance(sod, ry.ZonedDateTime)
        assert sod == ry.date(2020, 8, 26).at(0, 0, 0, 0).in_tz("America/Los_Angeles")

    def test_end_of_day(self) -> None:
        eod = self.zdt.end_of_day()
        assert isinstance(eod, ry.ZonedDateTime)
        assert eod == ry.date(2020, 8, 26).at(23, 59, 59, 999_999_999).in_tz(
            "America/Los_Angeles"
        )

    def test_iso_week_date(self) -> None:
        iwd = self.zdt.iso_week_date()
        assert iwd == ry.ISOWeekDate(2020, 35, 3)

    def test_offset_from_zdt(self) -> None:
        zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
        offset = zdt.offset()
        assert isinstance(offset, ry.Offset)
        assert offset == ry.Offset(hours=-4)

    def test_weekday(self) -> None:
        weekday = self.zdt.weekday
        assert isinstance(weekday, int)
        assert weekday == 3

        cur = self.zdt
        days = [cur]
        for _ in range(7):
            cur = cur.tomorrow()
            days.append(cur)
        assert [d.weekday for d in days] == [3, 4, 5, 6, 7, 1, 2, 3]

    def test_era_year(self) -> None:
        zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
        era_year = zdt.era_year()
        assert era_year == (2020, "CE")

    def test_millisecond(self) -> None:
        zdt = ry.ZonedDateTime(
            2020, 8, 26, 6, 27, 0, 123_456_789, "America/Los_Angeles"
        )
        assert zdt.millisecond == 123

    def test_microsecond(self) -> None:
        zdt = ry.ZonedDateTime(
            2020, 8, 26, 6, 27, 0, 123_456_789, "America/Los_Angeles"
        )
        assert zdt.microsecond == 456

    @pytest.mark.parametrize(
        ("zdt", "expected"),
        [
            (ry.date(2006, 8, 24).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 236),
            (ry.date(2023, 12, 31).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 365),
            (ry.date(2024, 12, 31).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 366),
        ],
    )
    def test_day_of_year(self, zdt: ry.ZonedDateTime, expected: int) -> None:
        assert zdt.day_of_year() == expected

    @pytest.mark.parametrize(
        ("zdt", "expected"),
        [
            (ry.date(2006, 8, 24).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 236),
            (ry.date(2023, 12, 31).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 365),
            (ry.date(2024, 12, 31).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 365),
            (ry.date(2024, 2, 29).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), None),
        ],
    )
    def test_day_of_year_no_leap(
        self, zdt: ry.ZonedDateTime, expected: int | None
    ) -> None:
        assert zdt.day_of_year_no_leap() == expected

    @pytest.mark.parametrize(
        ("zdt", "expected"),
        [
            (ry.date(2024, 2, 10).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 29),
            (ry.date(2023, 2, 10).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 28),
            (ry.date(2024, 8, 15).at(7, 30, 0, 0).in_tz("America/Los_Angeles"), 31),
        ],
    )
    def test_days_in_month(self, zdt: ry.ZonedDateTime, expected: int) -> None:
        assert zdt.days_in_month() == expected

    def test_days_in_year_in_leap_year(self) -> None:
        leap_zdt = ry.date(2024, 1, 1).at(0, 0, 0, 0).in_tz("America/Los_Angeles")
        assert leap_zdt.days_in_year() == 366
        assert leap_zdt.in_leap_year()
        non_leap_zdt = ry.date(2023, 1, 1).at(0, 0, 0, 0).in_tz("America/Los_Angeles")
        assert not non_leap_zdt.in_leap_year()
        assert non_leap_zdt.days_in_year() == 365

    def test_first_of_year(self) -> None:
        zdt = ry.date(2024, 2, 5).at(7, 30, 0, 0).in_tz("America/Los_Angeles")
        assert zdt.first_of_year() == ry.date(2024, 1, 1).at(7, 30, 0, 0).in_tz(
            "America/Los_Angeles"
        )

    def test_last_of_year(self) -> None:
        zdt = ry.date(2024, 2, 5).at(7, 30, 0, 0).in_tz("America/Los_Angeles")
        assert zdt.last_of_year() == ry.date(2024, 12, 31).at(7, 30, 0, 0).in_tz(
            "America/Los_Angeles"
        )

    def test_first_of_month(self) -> None:
        zdt = ry.date(2024, 2, 5).at(7, 30, 0, 0).in_tz("America/Los_Angeles")
        assert zdt.first_of_month() == ry.date(2024, 2, 1).at(7, 30, 0, 0).in_tz(
            "America/Los_Angeles"
        )

    def test_last_of_month(self) -> None:
        zdt = ry.date(2024, 2, 5).at(7, 30, 0, 0).in_tz("America/Los_Angeles")
        assert zdt.last_of_month() == ry.date(2024, 2, 29).at(7, 30, 0, 0).in_tz(
            "America/Los_Angeles"
        )


class TestZonedDateTimeProperties:
    """Test all the properties of the ZonedDateTime class

    properties:
        - year
        - month
        - day
        - weekday
        - hour
        - minute
        - second
        - microsecond
        - millisecond
        - nanosecond
        - subsec_nanosecond
    """

    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")

    def test_year(self) -> None:
        assert self.zdt.year == 2020

    def test_month(self) -> None:
        assert self.zdt.month == 8

    def test_day(self) -> None:
        assert self.zdt.day == 26

    def test_weekday(self) -> None:
        assert self.zdt.weekday == 3

    def test_hour(self) -> None:
        assert self.zdt.hour == 6

    def test_minute(self) -> None:
        assert self.zdt.minute == 27

    def test_second(self) -> None:
        assert self.zdt.second == 0

    def test_microsecond(self) -> None:
        assert self.zdt.microsecond == 0

    def test_millisecond(self) -> None:
        assert self.zdt.millisecond == 0

    def test_nanosecond(self) -> None:
        assert self.zdt.nanosecond == 0

    def test_subsec_nanosecond(self) -> None:
        assert self.zdt.subsec_nanosecond == 0
