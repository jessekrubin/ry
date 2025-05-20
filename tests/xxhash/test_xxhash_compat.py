from __future__ import annotations

import ast
import sys

import pytest

import ry

from ._xxhash_fixtures import (
    XX32_SEEDS,
    XX64_SEEDS,
    XX128_SEEDS,
    XXHASH_TEST_DATA,
    XXHashDataRecord,
)

try:
    # test against python-xxhash if importable...
    import xxhash
except ImportError:
    ...

pytest_skip_xxhash = pytest.mark.skipif(
    "xxhash" not in sys.modules, reason="xxhash is not installed"
)


def _bytes_from_record(rec: XXHashDataRecord) -> bytes:
    """Eval the bytes from the rec"""
    b = ast.literal_eval(rec["buf"])
    assert isinstance(b, bytes)
    return b


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh32() -> None:
    for seed in XX32_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.Xxh32(data, seed).digest()
                == xxhash.xxh32(data, seed).digest()
            )
            assert (
                ry.xxhash.Xxh32(data, seed).intdigest()
                == xxhash.xxh32(data, seed).intdigest()
            )
            assert (
                ry.xxhash.Xxh32(data, seed).hexdigest()
                == xxhash.xxh32(data, seed).hexdigest()
            )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh64() -> None:
    for seed in XX64_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.Xxh64(data, seed).digest()
                == xxhash.xxh64(data, seed).digest()
            )
            assert (
                ry.xxhash.Xxh64(data, seed).intdigest()
                == xxhash.xxh64(data, seed).intdigest()
            )
            assert (
                ry.xxhash.Xxh64(data, seed).hexdigest()
                == xxhash.xxh64(data, seed).hexdigest()
            )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh128() -> None:
    for seed in XX128_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.Xxh3(data, seed).digest128()
                == xxhash.xxh128(data, seed).digest()
            )
            assert (
                ry.xxhash.Xxh3(data, seed).intdigest128()
                == xxhash.xxh128(data, seed).intdigest()
            )
            assert (
                ry.xxhash.Xxh3(data, seed).hexdigest128()
                == xxhash.xxh128(data, seed).hexdigest()
            )
