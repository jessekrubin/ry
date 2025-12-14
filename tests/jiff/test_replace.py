import pytest

import ry


class TestTimeReplace:
    def test_time_replace_noop(self) -> None:
        midnight = ry.Time.midnight()
        no_replacement = midnight.replace()
        assert no_replacement == midnight

    @pytest.mark.parametrize(
        "kw",
        [
            {
                "millisecond": 12,
            },
            {
                "microsecond": 12,
            },
            {
                "nanosecond": 12,
            },
        ],
    )
    def test_replace_subsec_nanosecond_conflict(self, kw: dict[str, int]) -> None:
        midnight = ry.Time.midnight()
        with pytest.raises(
            TypeError,
            match="Cannot specify both subsec_nanosecond and millisecond/microsecond/nanosecond",
        ):
            midnight.replace(subsec_nanosecond=123456789, **kw)

    @pytest.mark.parametrize(
        "kw",
        [
            {
                "subsec_nanosecond": 123456789,
            },
            # equiv broken down
            {
                "millisecond": 123,
                "microsecond": 456,
                "nanosecond": 789,
            },
        ],
    )
    def test_replace_time_simple(self, kw: dict[str, int]) -> None:
        midnight = ry.Time.midnight()
        t = midnight.replace(hour=1, minute=2, second=3, **kw)
        assert t.hour == 1
        assert t.minute == 2
        assert t.second == 3
        assert t.subsec_nanosecond == 123456789
        assert t.millisecond == 123
        assert t.microsecond == 456
        assert t.nanosecond == 789


class TestZonedDateTimeReplace:
    def test_zoned_datetime_replace_noop(self) -> None:
        now = ry.ZonedDateTime.now()
        no_replacement = now.replace()
        assert no_replacement == now

    def test_replace_simple_values(self) -> None:
        zdt = ry.ZonedDateTime(2022, 1, 1, 10, 30, 0, tz="UTC")
        replaced = zdt.replace(year=2023, month=2, day=2, hour=12, minute=0, second=15)
        assert replaced.year == 2023
        assert replaced.month == 2
        assert replaced.day == 2
        assert replaced.hour == 12
        assert replaced.minute == 0
        assert replaced.second == 15
        assert replaced.timezone == ry.TimeZone("UTC")

    def test_replace_with_date_object(self) -> None:
        zdt = ry.ZonedDateTime(2022, 1, 1, 10, 30, 0, tz="UTC")
        new_date = ry.Date(2025, 5, 5)
        replaced = zdt.replace(date=new_date)
        assert replaced.year == 2025
        assert replaced.month == 5
        assert replaced.day == 5
        assert replaced.hour == 10
        assert replaced.minute == 30
        assert replaced.second == 0

    def test_replace_with_time_object(self) -> None:
        zdt = ry.ZonedDateTime(2022, 1, 1, 10, 30, 0, tz="UTC")
        new_time = ry.Time(20, 15, 5)
        replaced = zdt.replace(time=new_time)
        assert replaced.year == 2022
        assert replaced.month == 1
        assert replaced.day == 1
        assert replaced.hour == 20
        assert replaced.minute == 15
        assert replaced.second == 5

    def test_replace_with_datetime_object(self) -> None:
        zdt = ry.ZonedDateTime(2022, 1, 1, 10, 30, 0, tz="UTC")
        new_dt = ry.DateTime(2024, 1, 1, 1, 1, 1)
        replaced = zdt.replace(obj=new_dt)
        assert replaced.year == 2024
        assert replaced.month == 1
        assert replaced.day == 1
        assert replaced.hour == 1
        assert replaced.minute == 1
        assert replaced.second == 1

    def test_replace_with_offset(self) -> None:
        """Manually translated from the jiff eg code"""
        zdt = ry.ZonedDateTime.parse("2024-11-03T01:30:00-04:00[America/New_York]")
        replaced = zdt.replace(offset=ry.Offset.from_hours(-5))
        assert str(replaced) == "2024-11-03T01:30:00-05:00[America/New_York]"
        replaced_invalid = zdt.replace(offset=ry.Offset.from_hours(-12))
        assert str(replaced_invalid) == "2024-11-03T01:30:00-04:00[America/New_York]"
        with pytest.raises(ValueError):
            zdt.replace(
                offset=ry.Offset.from_hours(-12),
                disambiguation="reject",
            )

    def test_replace_day_of_year(self) -> None:
        zdt = ry.ZonedDateTime(2023, 1, 1, tz="UTC")
        replaced = zdt.replace(day_of_year=365)
        assert replaced.year == 2023
        assert replaced.month == 12
        assert replaced.day == 31

    def test_replace_subsec_nanosecond_conflict(self) -> None:
        zdt = ry.ZonedDateTime.now()
        with pytest.raises(
            ValueError,
        ):
            zdt.replace(subsec_nanosecond=1, millisecond=1)
