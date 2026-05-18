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


class TestJiffDeprecations:
    def test_jiff_signed_duration_from_isoformat_deprecated(self) -> None:
        msg = re.escape(
            "`SignedDuration.from_isoformat` is deprecated; use `SignedDuration.fromisoformat` instead [removal: v0.0.96]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _sd = ry.SignedDuration.from_isoformat(  # type: ignore[deprecated]
                "PT48m"
            )

    def test_jiff_timespan_from_isoformat_deprecated(self) -> None:
        msg = re.escape(
            "`TimeSpan.from_isoformat` is deprecated; use `TimeSpan.fromisoformat` instead [removal: v0.0.96]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _s = ry.TimeSpan.from_isoformat("PT48m")  # type: ignore[deprecated]

    def test_jiff_span_parse_common_iso_deprecated(self) -> None:
        msg = re.escape(
            "`TimeSpan.parse_common_iso` is deprecated; use `TimeSpan.fromisoformat` instead [removal: v0.0.96]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _s = ry.TimeSpan.parse_common_iso("PT48m")  # type: ignore[deprecated]
