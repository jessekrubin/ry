from __future__ import annotations

import pytest

import ry


class TestZonedRound:
    """
    REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.round
    """

    def test_zoned_difference_docs_example(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#example-44
        """
        # rounds up
        zdt = ry.date(2024, 6, 19).at(15, 0, 0, 0).in_tz("America/New_York")
        assert zdt.round("day") == ry.date(2024, 6, 20).at(0, 0, 0, 0).in_tz(
            "America/New_York"
        )
        # rounds down
        zdt = ry.date(2024, 6, 19).at(10, 0, 0, 0).in_tz("America/New_York")
        assert zdt.round("day") == ry.date(2024, 6, 19).at(0, 0, 0, 0).in_tz(
            "America/New_York"
        )

    def test_zoned_difference_docs_example_changing_the_rounding_mode(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#example-changing-the-rounding-mode
        """
        zdt = ry.date(2024, 6, 19).at(15, 0, 0, 0).in_tz("America/New_York")
        assert zdt.round("day") == ry.date(2024, 6, 20).at(0, 0, 0, 0).in_tz(
            "America/New_York"
        )

        assert zdt._round(
            ry.ZonedDateTimeRound()._smallest("day")._mode("trunc")
        ) == ry.date(2024, 6, 19).at(0, 0, 0, 0).in_tz("America/New_York")

    def test_zoned_difference_docs_example_rounding_to_the_nearest_5_minute_increment(
        self,
    ) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#example-rounding-to-the-nearest-5-minute-increment
        """

        zdt = ry.date(2024, 6, 19).at(15, 27, 29, 999_999_999).in_tz("America/New_York")
        assert zdt._round(
            ry.ZonedDateTimeRound()._smallest("minute")._increment(5)
        ) == ry.date(2024, 6, 19).at(15, 25, 0, 0).in_tz("America/New_York")

        zdt = ry.date(2024, 6, 19).at(15, 27, 30, 0).in_tz("America/New_York")
        assert zdt._round(
            ry.ZonedDateTimeRound()._smallest("minute")._increment(5)
        ) == ry.date(2024, 6, 19).at(15, 30, 0, 0).in_tz("America/New_York")

    def test_example_over_flow_error(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#example-overflow-error
        """
        zdt = ry.Timestamp.MAX.in_tz("America/New_York")
        with pytest.raises(
            ValueError
        ):  # TODO: figure out how to change to OverflowError
            zdt.round("day")


class TestOffsetRound:
    """
    REF: https://docs.rs/jiff/latest/jiff/tz/struct.Offset.html#method.round
    """

    def test_rounding_to_the_nearest_multiple_of_15_minutes(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/tz/struct.Offset.html#example-rounding-to-the-nearest-multiple-of-15-minutes
        """
        off = ry.Offset.from_seconds(-(44 * 60 + 30))
        rounded = off.round("minute", mode="half-expand", increment=15)
        assert rounded == ry.Offset.from_seconds(-45 * 60)
        # obj round
        rounded_via_obj = off._round(
            ry.OffsetRound("minute", mode="half-expand", increment=15)
        )
        assert rounded_via_obj == ry.Offset.from_seconds(-45 * 60)

    def test_rounding_can_fail_via_overflow(self) -> None:
        """
        REF: https://docs.rs/jiff/latest/jiff/tz/struct.Offset.html#example-rounding-can-fail-via-overflow
        """
        assert str(ry.Offset.MAX) == "+25:59:59"
        with pytest.raises(
            ValueError,
            match="rounding offset `\\+25:59:59` resulted in a duration of 26h, which overflows `Offset`",
        ):
            _r = ry.Offset.MAX.round("minute")


class TestSpanRound:
    """Translated (by hand) from the jiff::Span::round docs

    REF: https://docs.rs/jiff/latest/jiff/struct.Span.html#method.round
    """

    def test_example_balancing(self):
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Span.html#example-balancing
        """
        span = ry.TimeSpan(nanoseconds=123_456_789_123_456_789)
        rounded = span.round(largest="hour")
        assert (
            rounded.to_dict()
            == ry.TimeSpan(
                hours=34_293,
                minutes=33,
                seconds=9,
                milliseconds=123,
                microseconds=456,
                nanoseconds=789,
            ).to_dict()
        )
        # with days_are_24_hours
        rounded = span.round(largest="day", days_are_24_hours=True)
        assert (
            rounded.to_dict()
            == ry.TimeSpan(
                days=1_428,
                hours=21,
                minutes=33,
                seconds=9,
                milliseconds=123,
                microseconds=456,
                nanoseconds=789,
            ).to_dict()
        )

    def test_example_balancing_and_rounding(self):
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Span.html#example-balancing-and-rounding
        """
        span = ry.TimeSpan(nanoseconds=123_456_789_123_456_789)
        rounded = span.round(largest="hour", smallest="second")
        assert (
            rounded.to_dict()
            == ry.TimeSpan(hours=34_293, minutes=33, seconds=9).to_dict()
        )
        # just rounding to the nearest hour
        rounded = span.round("hour")  # smallest
        assert rounded.to_dict() == ry.TimeSpan(hours=34_294).to_dict()

    def test_example_balancing_with_a_relative_datetime(self):
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Span.html#example-balancing-with-a-relative-datetime
        """
        span = ry.TimeSpan(days=1_000)
        rounded = span.round(largest="year", relative=ry.date(2000, 1, 1))
        assert rounded.to_dict() == ry.TimeSpan(years=2, months=8, days=26).to_dict()

    def test_example_round_to_the_nearest_half_hour(self):
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Span.html#example-round-to-the-nearest-half-hour
        """
        span = ry.TimeSpan.from_str("PT23h50m3.123s")
        rounded = span.round("minute", increment=30)
        assert rounded.to_dict() == ry.TimeSpan(hours=24).to_dict()

    def test_example_yearly_quarters_in_a_span(self):
        """
        REF: https://docs.rs/jiff/latest/jiff/struct.Span.html#example-yearly-quarters-in-a-span
        """
        span1 = ry.TimeSpan(months=10, days=15)
        rounded = span1.round(
            smallest="month",
            increment=3,
            mode="trunc",
            relative=ry.date(2024, 1, 1),
        )
        assert rounded.months // 3 == 3

    def test_errors_with_reltative_n_days_are_24_hours(self) -> None:
        """NOT AN EXAMPLE IN THE DOCS"""
        span = ry.TimeSpan(days=10)
        with pytest.raises(
            ValueError,
            match="`relative` and `days_are_24_hours=True` are mutually exclusive",
        ):
            _r = span.round(
                largest="day",
                relative=ry.date(2024, 1, 1),
                days_are_24_hours=True,
            )
