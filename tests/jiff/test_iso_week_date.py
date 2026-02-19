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
