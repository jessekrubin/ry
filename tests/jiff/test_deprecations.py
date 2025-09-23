from __future__ import annotations

import pytest

import ry


class TestJiffDeprecationsInTz:
    """Test deprecations for objs that have an `.intz(tz:str)->Self:` method"""

    def test_jiff_intz_deprecation_date(self) -> None:
        with pytest.warns(DeprecationWarning):
            _d = ry.Date.today().intz("UTC")  # type: ignore[deprecated]

    def test_jiff_intz_deprecation_datetime(self) -> None:
        with pytest.warns(DeprecationWarning):
            _d = ry.now().datetime().intz("UTC")  # type: ignore[deprecated]

    def test_jiff_intz_deprecation_timestamp(self) -> None:
        with pytest.warns(DeprecationWarning):
            _d = ry.now().timestamp().intz("UTC")  # type: ignore[deprecated]

    def test_jiff_intz_deprecation_zoned_datetime(self) -> None:
        with pytest.warns(DeprecationWarning):
            _d = ry.now().intz("UTC")  # type: ignore[deprecated]


class TestJiffDeprecationsString:
    """Test deprecations for objs that have a `.string()->str:` method"""

    def test_jiff_string_deprecation_date(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.Date.today().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_datetime(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.now().datetime().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_timestamp(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.now().timestamp().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_time(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.Time.now().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_offset(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.Offset(hours=5).string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_signed_duration(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.SignedDuration.MIN.string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_iso_week_date(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.ISOWeekDate.today().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_zoned_datetime(self) -> None:
        with pytest.warns(DeprecationWarning):
            _s = ry.ZonedDateTime.now().string()  # type: ignore[deprecated]


@pytest.mark.skipif(
    ry.__version__ < "0.0.60",
    reason="These were removed in 0.0.60",
)
class TestJiffDeprecationsStringRemovedByVersion000060:
    """Test deprecations for objs that have a `.string()->str:` method"""

    def test_jiff_string_deprecation_date(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.Date.today().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_datetime(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.now().datetime().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_timestamp(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.now().timestamp().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_time(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.Time.now().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_offset(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.Offset(hours=5).string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_signed_duration(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.SignedDuration.MIN.string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_iso_week_date(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.ISOWeekDate.today().string()  # type: ignore[deprecated]

    def test_jiff_string_deprecation_zoned_datetime(self) -> None:
        with pytest.raises(AttributeError):
            _s = ry.ZonedDateTime.now().string()  # type: ignore[deprecated]
