import re

import pytest

import ry


class TestDateTimeNthWeekdayOfMonth:
    def test_nth_weekday_of_month(self) -> None:
        dt = ry.date(2024, 3, 1).at(7, 30, 0, 0)
        assert dt.nth_weekday_of_month(2, "friday") == ry.date(2024, 3, 8).at(
            7, 30, 0, 0
        )
        assert dt.nth_weekday_of_month(-1, "thursday") == ry.date(2024, 3, 28).at(
            7, 30, 0, 0
        )
        assert dt.nth_weekday_of_month(-2, "thursday") == ry.date(2024, 3, 21).at(
            7, 30, 0, 0
        )

    def test_nth_weekday_of_month_err(self) -> None:

        dt = ry.date(2024, 3, 25).at(7, 30, 0, 0)
        fourth_monday = dt.nth_weekday_of_month(4, "monday")
        assert fourth_monday == ry.date(2024, 3, 25).at(7, 30, 0, 0)
        # There is no 5th Monday.
        with pytest.raises(
            ValueError,
            match=re.escape(
                "number of days for `2024-03` is invalid, must be in range `1..=31`"
            ),
        ):
            _r = dt.nth_weekday_of_month(5, "monday")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "number of days for `2024-03` is invalid, must be in range `1..=31`"
            ),
        ):
            _r = dt.nth_weekday_of_month(-5, "monday")


class TestDateTimeNthWeekday:
    """
    REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.nth_weekday
    """

    def test_nth_weekday(self) -> None:
        dt = ry.date(2024, 3, 10).at(7, 30, 0, 0)
        assert dt.weekday == 7
        next_monday = dt.nth_weekday(1, "monday")
        assert next_monday == ry.date(2024, 3, 11).at(7, 30, 0, 0)
        next_sunday = dt.nth_weekday(1, "sunday")
        assert next_sunday == ry.date(2024, 3, 17).at(7, 30, 0, 0)
        next_next_thursday = dt.nth_weekday(2, "thursday")
        assert next_next_thursday == ry.date(2024, 3, 21).at(7, 30, 0, 0)

    def test_nth_weekday_reverse(self) -> None:
        dt = ry.date(2024, 3, 10).at(7, 30, 0, 0)
        assert dt.weekday == 7
        last_saturday = dt.nth_weekday(-1, "saturday")
        assert last_saturday == ry.date(2024, 3, 9).at(7, 30, 0, 0)
        last_sunday = dt.nth_weekday(-1, "sunday")
        assert last_sunday == ry.date(2024, 3, 3).at(7, 30, 0, 0)
        prev_prev_thursday = dt.nth_weekday(-2, "thursday")
        assert prev_prev_thursday == ry.date(2024, 2, 29).at(7, 30, 0, 0)

    def test_overflow_results_in_error(self) -> None:
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'year' with value 1 is not in the required range of -9999..=9999"
            ),
        ):
            _r = ry.DateTime.MAX.nth_weekday(1, "saturday")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'year' with value 1 is not in the required range of -9999..=9999"
            ),
        ):
            _r = ry.DateTime.MIN.nth_weekday(-1, "monday")


class TestDateNthWeekdayOfMonth:
    def test_nth_weekday_of_month(self) -> None:
        d = ry.date(2024, 3, 1)
        assert d.nth_weekday_of_month(2, "friday") == ry.date(2024, 3, 8)
        assert d.nth_weekday_of_month(-1, "thursday") == ry.date(2024, 3, 28)
        assert d.nth_weekday_of_month(-2, "thursday") == ry.date(2024, 3, 21)

    def test_nth_weekday_of_month_err(self) -> None:
        dt = ry.date(2024, 3, 25)
        fourth_monday = dt.nth_weekday_of_month(4, "monday")
        assert fourth_monday == ry.date(2024, 3, 25)
        # There is no 5th Monday.
        with pytest.raises(
            ValueError,
            match=re.escape(
                "number of days for `2024-03` is invalid, must be in range `1..=31`"
            ),
        ):
            _r = dt.nth_weekday_of_month(5, "monday")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "number of days for `2024-03` is invalid, must be in range `1..=31`"
            ),
        ):
            _r = dt.nth_weekday_of_month(-5, "monday")


class TestDateNthWeekday:
    """
    REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.nth_weekday
    """

    def test_nth_weekday(self) -> None:
        d = ry.date(2024, 3, 10)
        assert d.weekday == 7
        next_monday = d.nth_weekday(1, "monday")
        assert next_monday == ry.date(2024, 3, 11)
        next_sunday = d.nth_weekday(1, "sunday")
        assert next_sunday == ry.date(2024, 3, 17)
        next_next_thursday = d.nth_weekday(2, "thursday")
        assert next_next_thursday == ry.date(2024, 3, 21)

    def test_nth_weekday_reverse(self) -> None:
        d = ry.date(2024, 3, 10)
        assert d.weekday == 7
        last_saturday = d.nth_weekday(-1, "saturday")
        assert last_saturday == ry.date(2024, 3, 9)
        last_sunday = d.nth_weekday(-1, "sunday")
        assert last_sunday == ry.date(2024, 3, 3)
        prev_prev_thursday = d.nth_weekday(-2, "thursday")
        assert prev_prev_thursday == ry.date(2024, 2, 29)

    def test_overflow_results_in_error(self) -> None:
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'year' with value 1 is not in the required range of -9999..=9999",
            ),
        ):
            _r = ry.Date.MAX.nth_weekday(1, "saturday")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "parameter 'year' with value 1 is not in the required range of -9999..=9999"
            ),
        ):
            _r = ry.Date.MIN.nth_weekday(-1, "monday")


class TestZonedDateTimeNthWeekdayOfMonth:
    def test_nth_weekday_of_month(self) -> None:
        zdt = ry.date(2024, 3, 1).at(7, 30, 0, 0).in_tz("America/New_York")
        assert zdt.nth_weekday_of_month(2, "friday") == ry.date(2024, 3, 8).at(
            7, 30, 0, 0
        ).in_tz("America/New_York")
        assert zdt.nth_weekday_of_month(-1, "thursday") == ry.date(2024, 3, 28).at(
            7, 30, 0, 0
        ).in_tz("America/New_York")
        assert zdt.nth_weekday_of_month(-2, "thursday") == ry.date(2024, 3, 21).at(
            7, 30, 0, 0
        ).in_tz("America/New_York")

    def test_nth_weekday_of_month_err(self) -> None:
        zdt = ry.date(2024, 3, 25).at(7, 30, 0, 0).in_tz("America/New_York")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "number of days for `2024-03` is invalid, must be in range `1..=31`"
            ),
        ):
            _r = zdt.nth_weekday_of_month(5, "monday")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "number of days for `2024-03` is invalid, must be in range `1..=31`"
            ),
        ):
            _r = zdt.nth_weekday_of_month(-5, "monday")


class TestZonedDateTimeNthWeekday:
    """
    REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.nth_weekday
    """

    def test_nth_weekday(self) -> None:
        zdt = ry.date(2024, 3, 10).at(7, 30, 0, 0).in_tz("America/New_York")
        assert zdt.weekday == 7
        next_monday = zdt.nth_weekday(1, "monday")
        assert next_monday == ry.date(2024, 3, 11).at(7, 30, 0, 0).in_tz(
            "America/New_York"
        )
        next_sunday = zdt.nth_weekday(1, "sunday")
        assert next_sunday == ry.date(2024, 3, 17).at(7, 30, 0, 0).in_tz(
            "America/New_York"
        )
        next_next_thursday = zdt.nth_weekday(2, "thursday")
        assert next_next_thursday == ry.date(2024, 3, 21).at(7, 30, 0, 0).in_tz(
            "America/New_York"
        )

    def test_nth_weekday_reverse(self) -> None:
        zdt = ry.date(2024, 3, 10).at(7, 30, 0, 0).in_tz("America/New_York")
        assert zdt.weekday == 7
        last_saturday = zdt.nth_weekday(-1, "saturday")
        assert last_saturday == ry.date(2024, 3, 9).at(7, 30, 0, 0).in_tz(
            "America/New_York"
        )
        last_sunday = zdt.nth_weekday(-1, "sunday")
        assert last_sunday == ry.date(2024, 3, 3).at(7, 30, 0, 0).in_tz(
            "America/New_York"
        )
        prev_prev_thursday = zdt.nth_weekday(-2, "thursday")
        assert prev_prev_thursday == ry.date(2024, 2, 29).at(7, 30, 0, 0).in_tz(
            "America/New_York"
        )

    def test_overflow_results_in_error(self) -> None:
        with pytest.raises(
            ValueError,
            match=re.escape(
                "converting datetime with time zone offset `-05` to timestamp overflowed: parameter 'unix-seconds' with value 253402318799 is not in the required range of -377705023201..=253402207200"
            ),
        ):
            _r = ry.DateTime.MAX.in_tz("America/New_York").nth_weekday(1, "saturday")
        with pytest.raises(
            ValueError,
            match=re.escape(
                "converting datetime with time zone offset `-04:56:02` to timestamp overflowed: parameter 'unix-seconds' with value -377705099038 is not in the required range of -377705023201..=253402207200"
            ),
        ):
            _r = ry.DateTime.MIN.in_tz("America/New_York").nth_weekday(-1, "monday")
