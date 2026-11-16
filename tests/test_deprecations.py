import re
import warnings

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


class TestDeprecationWarningEg:
    def _deprecated_fn(self) -> None:
        msg = _deprecation_message(
            "ry.deprecated_fn", "ry.new_fn", "v0.1.0", escape=False
        )
        warnings.warn(msg, DeprecationWarning, stacklevel=1)

    def test_deprecated_fn_warning(self) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message("ry.deprecated_fn", "ry.new_fn", "v0.1.0"),
        ):
            self._deprecated_fn()


# =============================================================================
# jiff 1.0 renames ~ `to_zoned` -> `to_zoned_datetime` (remove in v0.0.112)
# =============================================================================
class TestJiffToZonedDeprecation:
    tz = ry.TimeZone("America/New_York")

    def test_date_to_zoned(self) -> None:
        d = ry.date(2023, 3, 14)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "Date.to_zoned", "Date.to_zoned_datetime", "v0.0.112"
            ),
        ):
            zdt = d.to_zoned(self.tz)  # type: ignore[deprecated]  # ty: ignore[deprecated]  # pyrefly: ignore[deprecated]
        assert zdt == d.to_zoned_datetime(self.tz)

    def test_datetime_to_zoned(self) -> None:
        dt = ry.datetime(2023, 3, 14, 12, 30)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "DateTime.to_zoned", "DateTime.to_zoned_datetime", "v0.0.112"
            ),
        ):
            zdt = dt.to_zoned(self.tz)  # type: ignore[deprecated]  # ty: ignore[deprecated]  # pyrefly: ignore[deprecated]
        assert zdt == dt.to_zoned_datetime(self.tz)

    def test_timezone_to_zoned(self) -> None:
        dt = ry.datetime(2023, 3, 14, 12, 30)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "TimeZone.to_zoned", "TimeZone.to_zoned_datetime", "v0.0.112"
            ),
        ):
            zdt = self.tz.to_zoned(dt)  # type: ignore[deprecated]  # ty: ignore[deprecated]  # pyrefly: ignore[deprecated]
        assert zdt == self.tz.to_zoned_datetime(dt)

    def test_timestamp_to_zoned(self) -> None:
        ts = ry.Timestamp(1_700_000_000)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "Timestamp.to_zoned", "Timestamp.to_zoned_datetime", "v0.0.112"
            ),
        ):
            zdt = ts.to_zoned(self.tz)  # type: ignore[deprecated]  # ty: ignore[deprecated]  # pyrefly: ignore[deprecated]
        assert zdt == ts.to_zoned_datetime(self.tz)
