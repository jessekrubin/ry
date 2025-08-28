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
