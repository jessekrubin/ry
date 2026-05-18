import re

import pytest

import ry


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
