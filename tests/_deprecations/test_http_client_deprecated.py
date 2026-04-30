import re

import pytest

import ry


def test_http_client_deprecation_warning() -> None:
    with pytest.deprecated_call(
        match=re.escape(
            "HttpClient is deprecated use Client instead (slated for removal in 0.0.100)"
        )
    ):
        _c = ry.HttpClient()  # type: ignore[deprecated]
