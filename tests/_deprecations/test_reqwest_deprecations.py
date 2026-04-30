import re

import pytest

import ry


def test_http_client_deprecation_warning() -> None:
    with pytest.deprecated_call(
        match=re.escape(
            "`HttpClient` is deprecated; use `Client` instead [removal: v0.0.93]"
        )
    ):
        _c = ry.HttpClient()  # type: ignore[deprecated]
