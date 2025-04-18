from __future__ import annotations

import pytest

import ry

hashers = [
    (ry.xxhash.xxh32, "xxh32"),
    (ry.xxhash.xxh64, "xxh64"),
    (ry.xxhash.xxh3, "xxh3"),
]


@pytest.mark.parametrize("args", [pytest.param((h, n), id=n) for h, n in hashers])
def test_xxh_name(
    args: tuple[ry.xxhash.xxh32 | ry.xxhash.xxh64 | ry.xxhash.xxh3, str],
) -> None:
    hasher, name = args
    assert hasher.name == name
