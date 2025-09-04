from __future__ import annotations

import pytest

import ry

hashers = [
    (ry.xxhash.xxh32, "xxh32"),
    (ry.xxhash.xxh64, "xxh64"),
    (ry.xxhash.xxh3, "xxh3"),
    (ry.xxhash.xxh3_128, "xxh3_128"),
]


@pytest.mark.parametrize("args", [pytest.param((h, n), id=n) for h, n in hashers])
def test_xxh_name(
    args: tuple[ry.xxhash.Xxh32 | ry.xxhash.Xxh64 | ry.xxhash.Xxh3, str],
) -> None:
    hasher, name = args
    assert hasher.name == name
