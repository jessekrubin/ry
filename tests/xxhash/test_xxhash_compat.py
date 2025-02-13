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

# from ._xxhash_test_data import (
#     XX32_SEEDS,
#     XX32_TEST_DATA,
#     XX64_SEEDS,
#     XX64_TEST_DATA,
#     XX128_SEEDS,
#     XX128_TEST_DATA,
# )

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
    return ast.literal_eval(rec["buf"])


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh32() -> None:
    for seed in XX32_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.xxh32(data, seed).digest()
                == xxhash.xxh32(data, seed).digest()
            )
            assert (
                ry.xxhash.xxh32(data, seed).intdigest()
                == xxhash.xxh32(data, seed).intdigest()
            )
            assert (
                ry.xxhash.xxh32(data, seed).hexdigest()
                == xxhash.xxh32(data, seed).hexdigest()
            )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh64() -> None:
    for seed in XX64_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.xxh64(data, seed).digest()
                == xxhash.xxh64(data, seed).digest()
            )
            assert (
                ry.xxhash.xxh64(data, seed).intdigest()
                == xxhash.xxh64(data, seed).intdigest()
            )
            assert (
                ry.xxhash.xxh64(data, seed).hexdigest()
                == xxhash.xxh64(data, seed).hexdigest()
            )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh128() -> None:
    for seed in XX128_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.xxh3(data, seed).digest128()
                == xxhash.xxh128(data, seed).digest()
            )
            assert (
                ry.xxhash.xxh3(data, seed).intdigest128()
                == xxhash.xxh128(data, seed).intdigest()
            )
            assert (
                ry.xxhash.xxh3(data, seed).hexdigest128()
                == xxhash.xxh128(data, seed).hexdigest()
            )
