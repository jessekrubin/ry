from __future__ import annotations

import pytest

import ry


def test_parse_with_params_dict() -> None:
    url = ry.URL.parse_with_params(
        "https://example.net?dont=clobberme",
        {
            "lang": "rust",
            "browser": "servo",
        },
    )
    assert str(url) == "https://example.net/?dont=clobberme&lang=rust&browser=servo"


def test_parse_new_params_kwarg() -> None:
    url = ry.URL(
        "https://example.net?dont=clobberme",
        params={
            "lang": "rust",
            "browser": "servo",
        },
    )
    assert str(url) == "https://example.net/?dont=clobberme&lang=rust&browser=servo"


def test_parse_with_params() -> None:
    url = ry.URL.parse_with_params(
        "https://example.net?dont=clobberme",
        params={
            "lang": "rust",
            "browser": "servo",
        },
    )
    assert str(url) == "https://example.net/?dont=clobberme&lang=rust&browser=servo"


def test_parse_with_params_none() -> None:
    url = ry.URL.parse(
        "https://example.net?dont=clobberme",
    )
    assert str(url) == "https://example.net/?dont=clobberme"


def test_parse_new_params_kwarg_only() -> None:
    with pytest.raises(TypeError):
        _url = ry.URL(  # type: ignore[misc]
            "https://example.net",
            {
                "lang": "rust",
                "browser": "servo",
            },
        )
