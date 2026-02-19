from __future__ import annotations

import ry


class TestParse:
    d = ry.date(2020, 8, 26)
    dt = ry.date(2020, 8, 26).at(6, 27, 0, 0)
    t = ry.time(6, 27, 0, 0)
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")

    def test_parse_date(self) -> None:
        parsed_date = ry.Date.parse(str(self.d))
        assert parsed_date == self.d

    def test_parse_datetime(self) -> None:
        parsed_datetime = ry.DateTime.from_str(str(self.dt))
        assert parsed_datetime == self.dt

    def test_parse_time(self) -> None:
        parsed_time = ry.Time.parse(str(self.t))
        assert parsed_time == self.t

    def test_parse_zoned_datetime(self) -> None:
        parsed_zdt = ry.ZonedDateTime.parse(str(self.zdt))
        assert parsed_zdt == self.zdt
        assert parsed_zdt.timezone == self.zdt.timezone
        assert parsed_zdt.date() == self.zdt.date()
        assert parsed_zdt.time() == self.zdt.time()


class TestJiffFunctions:
    def test_jiff_date(self) -> None:
        d = ry.date(2020, 2, 29)
        assert d == ry.date(2020, 2, 29)

    def test_jiff_datetime(self) -> None:
        dt = ry.datetime(2020, 2, 29, 12, 30, 45)
        assert dt == ry.datetime(2020, 2, 29, 12, 30, 45)

    def test_jiff_zoned(self) -> None:
        zdt = ry.zoned(2020, 2, 29, 12, 30, 45, tz="America/Los_Angeles")
        assert isinstance(zdt, ry.ZonedDateTime)
        assert zdt.date() == ry.date(2020, 2, 29)
        assert zdt.time() == ry.time(12, 30, 45)
        assert str(zdt) == "2020-02-29T12:30:45-08:00[America/Los_Angeles]"
