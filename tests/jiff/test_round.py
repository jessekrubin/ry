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
