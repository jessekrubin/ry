import re

import pytest

import ry


def _deprecation_message(
    old: str, new: str, removal_version: str, *, escape: bool = True
) -> str:
    msg = f"`{old}` is deprecated; use `{new}` instead [removal: {removal_version}]"
    return re.escape(msg) if escape else msg


def test_deprecation_msg() -> None:
    assert (
        _deprecation_message("old_func", "new_func", "v0.1.0", escape=False)
        == "`old_func` is deprecated; use `new_func` instead [removal: v0.1.0]"
    )


class TestJiffDeprecations:
    def test_jiff_signed_duration_from_isoformat_deprecated(self) -> None:
        msg = _deprecation_message(
            "SignedDuration.from_isoformat", "SignedDuration.fromisoformat", "v0.0.96"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _sd = ry.SignedDuration.from_isoformat("PT48m")  # type: ignore[deprecated]

    def test_jiff_timespan_from_isoformat_deprecated(self) -> None:
        msg = _deprecation_message(
            "TimeSpan.from_isoformat", "TimeSpan.fromisoformat", "v0.0.96"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _s = ry.TimeSpan.from_isoformat("PT48m")  # type: ignore[deprecated]

    def test_jiff_span_parse_common_iso_deprecated(self) -> None:
        msg = _deprecation_message(
            "TimeSpan.parse_common_iso", "TimeSpan.fromisoformat", "v0.0.96"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _s = ry.TimeSpan.parse_common_iso("PT48m")  # type: ignore[deprecated]
