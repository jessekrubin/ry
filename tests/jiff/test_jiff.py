from __future__ import annotations

import itertools as it

import pytest

import ry

# ====================
# Zoned
# ====================


class TestZonedDateTime:
    def test_era_year(self) -> None:
        zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
        era_year = zdt.era_year()
        assert era_year == (2020, "CE")

    def test_offset_from_zdt(self) -> None:
        zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
        offset = zdt.offset()
        assert isinstance(offset, ry.Offset)
        assert offset == ry.Offset(hours=-4)

    def test_start_of_day(self) -> None:
        zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
        start_of_day = zdt.start_of_day()
        assert isinstance(start_of_day, ry.ZonedDateTime)
        assert start_of_day == ry.date(2020, 8, 26).at(0, 0, 0, 0).intz(
            "America/New_York"
        )

    def test_end_of_day(self) -> None:
        zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
        end_of_day = zdt.end_of_day()
        assert isinstance(end_of_day, ry.ZonedDateTime)
        assert end_of_day == ry.date(2020, 8, 26).at(23, 59, 59, 999_999_999).intz(
            "America/New_York"
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

    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")

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


class TestOffset:
    def test_create_offset_with_hours(self) -> None:
        offset = ry.Offset(hours=-4)
        assert offset == ry.Offset(hours=-4)
        assert offset == ry.Offset.from_hours(-4)

    def test_offset_from_seconds(self) -> None:
        offset = ry.Offset.from_seconds(-4 * 60 * 60)
        assert offset == ry.Offset(hours=-4)

    def test_offset_errors_when_given_both_hours_and_seconds(self) -> None:
        with pytest.raises(TypeError):
            ry.Offset(hours=-4, seconds=-4 * 60 * 60)

    def test_offset_errors_when_given_neither_hours_nor_seconds(self) -> None:
        with pytest.raises(TypeError):
            ry.Offset()


def test_zoned() -> None:
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    assert zdt.string() == "2020-08-26T06:27:00-04:00[America/New_York]"

    zdt_fields = {
        "tz": str(zdt.timezone()),
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
        "subsec_nanosecond": ry_datetime.subsec_nanosecond,
    }
    dt_dictionary = {
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 6,
        "minute": 27,
        "second": 0,
        "nanosecond": 0,
        "subsec_nanosecond": 0,
    }
    assert dt_fields == dt_dictionary
    assert ry_datetime.asdict() == {
        "year": 2020,
        "month": 8,
        "day": 26,
        "hour": 6,
        "minute": 27,
        "second": 0,
        "subsec_nanosecond": 0,
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
    assert ry_time.asdict() == expected_time_dict


# ====================
# SPAN
# ====================


class TestTimeSpan:
    def test_span_negate(self) -> None:
        zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
        zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
        span = zdt2 - zdt1
        assert span.string() == "PT29341h3m"
        span_negated = -span
        assert span_negated.string() == "-PT29341h3m"

        span_inverted = ~span
        assert span_inverted.string() == "-PT29341h3m"

    def test_span_2_duration(self) -> None:
        zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
        zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
        span = zdt2 - zdt1
        duration = span.to_signed_duration(zdt2)
        assert duration == ry.SignedDuration(secs=105627780, nanos=0)


class TestTimeSpanProperties:
    """Test all the properties of the TimeSpan class

    properties:
        - is_positive
        - is_negative
        - is_zero
        - years
        - months
        - weeks
        - days
        - hours
        - minutes
        - seconds
        - milliseconds
        - microseconds
        - nanoseconds
    """

    ts = ry.TimeSpan(
        days=1,
        hours=2,
        minutes=3,
        seconds=4,
        milliseconds=5,
        microseconds=5_000,
        nanoseconds=5_000_000,
    )

    def test_is_positive(self) -> None:
        assert self.ts.is_positive

    def test_is_negative(self) -> None:
        assert not self.ts.is_negative

    def test_is_zero(self) -> None:
        assert not self.ts.is_zero

    def test_years(self) -> None:
        assert self.ts.years == 0

    def test_months(self) -> None:
        assert self.ts.months == 0

    def test_weeks(self) -> None:
        assert self.ts.weeks == 0

    def test_days(self) -> None:
        assert self.ts.days == 1

    def test_hours(self) -> None:
        assert self.ts.hours == 2

    def test_minutes(self) -> None:
        assert self.ts.minutes == 3

    def test_seconds(self) -> None:
        assert self.ts.seconds == 4

    def test_milliseconds(self) -> None:
        assert self.ts.milliseconds == 5

    def test_microseconds(self) -> None:
        assert self.ts.microseconds == 5_000

    def test_nanoseconds(self) -> None:
        assert self.ts.nanoseconds == 5_000_000


# ====================
# round mode
# ====================

JIFF_UNITS = [
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

JIFF_ROUND_MODES = [
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half_ceil",
    "half_floor",
    "half_expand",
    "half_trunc",
    "half_even",
]


class TestDateTime:
    def test_datetime_round_options(self) -> None:
        default = ry.DateTimeRound()
        expected_default_string = (
            'DateTimeRound(smallest="nanosecond", mode="half_expand", increment=1)'
        )
        assert str(default) == expected_default_string

        for unit, mode in it.product(JIFF_UNITS, JIFF_ROUND_MODES):
            options = ry.DateTimeRound(smallest=unit, mode=mode, increment=1)  # type: ignore[arg-type]

            options_chained = ry.DateTimeRound().smallest(unit).mode(mode).increment(1)  # type: ignore[arg-type]
            expected_string = (
                f'DateTimeRound(smallest="{unit}", mode="{mode}", increment=1)'
            )
            assert str(options) == expected_string
            assert options == options_chained


# repr


class TestTimespanFunction:
    def test_timespan_fn(self) -> None:
        ts = ry.timespan(weeks=1)
        assert ts.string() == "P1w"

    def test_timespan_overflow(self) -> None:
        max_i64 = 9_223_372_036_854_775_807
        with pytest.raises(OverflowError):
            ry.timespan(years=100, days=max_i64)

    def test_timespan_overflow_unchecked(self) -> None:
        max_i64 = 9_223_372_036_854_775_807

        with pytest.raises(BaseException):  # noqa: B017
            ry.timespan(years=100, days=max_i64, unchecked=True)


class TestTzOffset:
    def test_const_max(self) -> None:
        assert ry.Offset.MAX == ry.Offset(seconds=93599)

    def test_const_min(self) -> None:
        assert ry.Offset.MIN == ry.Offset(seconds=-93599)

    def test_const_zero(self) -> None:
        assert ry.Offset.ZERO == ry.Offset(seconds=0)

    def test_const_utc(self) -> None:
        assert ry.Offset.UTC == ry.Offset(seconds=0)

    def test_seconds_property(self) -> None:
        offset = ry.Offset.from_seconds(61)
        assert offset.seconds == 61
        assert offset.is_positive
        assert not offset.is_negative
        offset_neg = -offset
        assert offset_neg.seconds == -61
        assert offset_neg.is_negative
        assert not offset_neg.is_positive

    def test_from_hours(self) -> None:
        offset = ry.Offset.from_hours(2)
        assert offset == ry.Offset(seconds=7200)

    def test_from_hours_error(self) -> None:
        with pytest.raises(ValueError):
            _offset = ry.Offset.from_hours(26)
        with pytest.raises(ValueError):
            _offset = ry.Offset.from_hours(-26)

    def test_from_seconds(self) -> None:
        offset = ry.Offset.from_seconds(61)
        assert offset == ry.Offset(seconds=61)

    def test_from_seconds_error(self) -> None:
        with pytest.raises(ValueError):
            _offset = ry.Offset.from_seconds(93600)
        with pytest.raises(ValueError):
            _offset = ry.Offset.from_seconds(-93600)

    def test_negate(self) -> None:
        offset = ry.Offset.from_seconds(61)
        assert -offset == ry.Offset.from_seconds(-61)

    def test_until(self) -> None:
        offset = ry.Offset.from_seconds(61)
        span_until = offset.until(ry.Offset.from_seconds(62))
        assert isinstance(span_until, ry.TimeSpan)
        assert span_until == ry.TimeSpan(seconds=1)
        assert offset.until(ry.Offset.from_seconds(61)) == ry.TimeSpan()

    def test_since(self) -> None:
        offset = ry.Offset.from_seconds(61)
        span_since = offset.since(ry.Offset.from_seconds(62))
        assert isinstance(span_since, ry.TimeSpan)

        assert span_since == ry.TimeSpan(seconds=-1)
        assert offset.since(ry.Offset.from_seconds(61)) == ry.TimeSpan()

    def test_to_timezone(self) -> None:
        offset = ry.Offset.from_seconds(61)
        tz = offset.to_timezone()
        assert isinstance(tz, ry.TimeZone)
        tz_offset, dst, tzname = tz.to_offset(ry.Timestamp(0, 0))
        assert tz_offset == offset
        assert dst is False
        assert tzname == "+00:01:01"

    def test_checked_add(self) -> None:
        offset = ry.Offset.from_hours(-8)
        span = ry.timespan(hours=1)
        assert offset.checked_add(span) == ry.Offset.from_hours(-7)
        signed_duration = span.to_signed_duration(
            ry.Date(
                year=2024,
                month=12,
                day=13,  # OOOOH friday the 13th
            )
        )
        assert offset.checked_add(signed_duration) == ry.Offset.from_hours(-7)
        duration = ry.Duration(secs=3600)
        assert offset.checked_add(duration) == ry.Offset.from_hours(-7)

    def test_checked_sub(self) -> None:
        offset = ry.Offset.from_hours(-8)
        span = ry.timespan(hours=1)
        assert offset.checked_sub(span) == ry.Offset.from_hours(-9)
        signed_duration = span.to_signed_duration(
            ry.Date(year=2024, month=12, day=13)  # OOOOH friday the 13th (again)
        )
        assert offset.checked_sub(signed_duration) == ry.Offset.from_hours(-9)
        duration = ry.Duration(secs=3600)
        assert offset.checked_sub(duration) == ry.Offset.from_hours(-9)

    def test_saturating_add(self) -> None:
        offset = ry.Offset.from_hours(25)
        span = ry.TimeSpan(hours=2)
        assert offset.saturating_add(span) == ry.Offset.MAX
        signed_duration = span.to_signed_duration(ry.Date(year=2024, month=12, day=13))
        assert offset.saturating_add(signed_duration) == ry.Offset.MAX
        duration = ry.Duration(secs=7200)
        assert offset.saturating_add(duration) == ry.Offset.MAX

    def test_saturating_sub(self) -> None:
        offset = ry.Offset.from_hours(-25)
        span = ry.TimeSpan(hours=2)
        assert offset.saturating_sub(span) == ry.Offset.MIN
        signed_duration = span.to_signed_duration(ry.Date(year=2024, month=12, day=13))
        assert offset.saturating_sub(signed_duration) == ry.Offset.MIN
        duration = ry.Duration(secs=7200)
        assert offset.saturating_sub(duration) == ry.Offset.MIN
