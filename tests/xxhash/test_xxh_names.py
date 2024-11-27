from __future__ import annotations

import pytest

import ry

hashers = [
    (ry.Xxh32, "xxh32"),
    (ry.Xxh64, "xxh64"),
    (ry.Xxh3, "xxh3"),
]


@pytest.mark.parametrize("args", [pytest.param((h, n), id=n) for h, n in hashers])
def test_xxh_name(args: tuple[ry.Xxh32 | ry.Xxh64 | ry.Xxh3, str]) -> None:
    hasher, name = args
    assert hasher.name == name
