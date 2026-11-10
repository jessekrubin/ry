import pytest

import ry


class TestISOWeekDate:
    def test_iso_week_date(self) -> None:
        d = ry.date(2024, 3, 10)
        iso_week = d.iso_week_date()
        assert iso_week == ry.ISOWeekDate(2024, 10, 7)

    def test_iso_week_date_properties(self) -> None:
        iso_week = ry.ISOWeekDate(2024, 10, 7)
        assert iso_week.year == 2024
        assert iso_week.week == 10
        assert iso_week.weekday == 7

    def test_iso_week_date_from_date(self) -> None:
        d = ry.date(2024, 3, 10)
        iso_week = ry.ISOWeekDate.from_date(d)
        assert iso_week == ry.ISOWeekDate(2024, 10, 7)
        assert iso_week.date() == d

    def test_iwd_equality(self) -> None:
        iwd1 = ry.ISOWeekDate(2024, 10, 7)
        iwd2 = ry.ISOWeekDate(2024, 10, 7)
        assert iwd1 == iwd2

    def test_iwd_inequality(self) -> None:
        iwd1 = ry.ISOWeekDate(2024, 10, 7)
        iwd2 = ry.ISOWeekDate(2024, 10, 6)
        assert iwd1 != iwd2

    def test_iwd_hash(self) -> None:
        iwd1 = ry.ISOWeekDate(2024, 10, 7)
        iwd2 = ry.ISOWeekDate(2024, 10, 7)
        assert hash(iwd1) == hash(iwd2)

    def test_iwd_hash_inequality(self) -> None:
        iwd1 = ry.ISOWeekDate(2024, 10, 7)
        iwd2 = ry.ISOWeekDate(2024, 10, 6)
        assert hash(iwd1) != hash(iwd2)


class TestISOWeekDateMethods:
    @pytest.mark.parametrize(
        ("year", "long", "weeks", "days"),
        [(2020, True, 53, 371), (2024, False, 52, 364), (2026, True, 53, 371)],
    )
    def test_year_info(self, year: int, long: bool, weeks: int, days: int) -> None:  # noqa: FBT001
        iwd = ry.ISOWeekDate(year, 10, 3)
        assert iwd.in_long_year() is long
        assert iwd.weeks_in_year() == weeks
        assert iwd.days_in_year() == days

    def test_first_last_of_week(self) -> None:
        iwd = ry.ISOWeekDate(2024, 10, 3)
        assert iwd.first_of_week() == ry.ISOWeekDate(2024, 10, 1)
        assert iwd.last_of_week() == ry.ISOWeekDate(2024, 10, 7)

    def test_first_last_of_year(self) -> None:
        assert ry.ISOWeekDate(2024, 10, 3).first_of_year() == ry.ISOWeekDate(2024, 1, 1)
        assert ry.ISOWeekDate(2024, 10, 3).last_of_year() == ry.ISOWeekDate(2024, 52, 7)
        assert ry.ISOWeekDate(2020, 10, 3).last_of_year() == ry.ISOWeekDate(2020, 53, 7)

    def test_tomorrow_yesterday(self) -> None:
        iwd = ry.ISOWeekDate(2024, 52, 7)
        assert iwd.tomorrow() == ry.ISOWeekDate(2025, 1, 1)
        assert iwd.tomorrow().yesterday() == iwd
        assert iwd.tomorrow().date() == iwd.date().tomorrow()

    @pytest.mark.parametrize(
        ("iwd", "method"),
        [
            (ry.ISOWeekDate.MAX, "tomorrow"),
            (ry.ISOWeekDate.MAX, "last_of_week"),
            (ry.ISOWeekDate.MAX, "last_of_year"),
            (ry.ISOWeekDate.MIN, "yesterday"),
        ],
    )
    def test_edges_raise(self, iwd: ry.ISOWeekDate, method: str) -> None:
        with pytest.raises(ValueError, match="parameter"):
            getattr(iwd, method)()

    def test_edges_ok(self) -> None:
        assert ry.ISOWeekDate.MIN.first_of_week() == ry.ISOWeekDate.MIN
        assert ry.ISOWeekDate.MIN.first_of_year() == ry.ISOWeekDate.MIN
        assert ry.ISOWeekDate.MAX.first_of_week() == ry.ISOWeekDate(9999, 52, 1)
