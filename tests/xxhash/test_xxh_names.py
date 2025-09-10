from __future__ import annotations

import ry


def test_xxh32_name() -> None:
    assert ry.xxhash.xxh32.name == "xxh32"


def test_xxh64_name() -> None:
    assert ry.xxhash.xxh64.name == "xxh64"


def test_xxh3_64_name() -> None:
    assert ry.xxhash.xxh3_64.name == "xxh3_64"


def test_xxh3_128_name() -> None:
    assert ry.xxhash.xxh3_128.name == "xxh3_128"
