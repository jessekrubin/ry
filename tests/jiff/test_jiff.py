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


class TestOffset:
    def test_create_offset_with_hours(self) -> None:
        offset = ry.Offset(hours=-4)
        assert offset == ry.Offset(hours=-4)
        assert offset == ry.Offset.from_hours(-4)

    def test_offset_from_seconds(self) -> None:
        offset = ry.Offset.from_seconds((-4 * 60 * 60))
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


class TestSpan:
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

        with pytest.raises(Exception):
            ry.timespan(years=100, days=max_i64)
