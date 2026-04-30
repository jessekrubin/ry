from __future__ import annotations

import re

import pytest

import ry


class TestJiffDeprecationsInTz:
    """Test deprecations for objs that have an `.intz(tz:str)->Self:` method"""

    def test_jiff_intz_deprecation_date(self) -> None:
        msg = re.escape(
            "`Date.intz` is deprecated; use `Date.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.Date.today().intz("UTC")  # type: ignore[deprecated]

    def test_jiff_intz_deprecation_datetime(self) -> None:
        msg = re.escape(
            "`DateTime.intz` is deprecated; use `DateTime.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.now().datetime().intz("UTC")  # type: ignore[deprecated]

    def test_jiff_intz_deprecation_timestamp(self) -> None:
        msg = re.escape(
            "`Timestamp.intz` is deprecated; use `Timestamp.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.now().timestamp().intz("UTC")  # type: ignore[deprecated]

    def test_jiff_intz_deprecation_zoned_datetime(self) -> None:
        msg = re.escape(
            "`ZonedDateTime.intz` is deprecated; use `ZonedDateTime.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.now().intz("UTC")  # type: ignore[deprecated]


@pytest.mark.skipif(
    ry.__version__ < "0.0.61",
    reason="These were removed in 0.0.61",
)
class TestJiffDeprecationsStringFunctions:
    """Test deprecations for objs that have a `.string()->str:` method"""

    def test_jiff_string_deprecation_date(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.Date.today().string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_datetime(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.now().datetime().string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_timestamp(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.now().timestamp().string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_time(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.Time.now().string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_offset(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.Offset(hours=5).string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_signed_duration(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.SignedDuration.MIN.string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_iso_week_date(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.ISOWeekDate.today().string()  # type: ignore[attr-defined]

    def test_jiff_string_deprecation_zoned_datetime(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.ZonedDateTime.now().string()  # type: ignore[attr-defined]
