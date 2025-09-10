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


_python_xxhash_exports: tuple[str, ...] = (
    "xxh128",
    "xxh128_digest",
    "xxh128_hexdigest",
    "xxh128_intdigest",
    "xxh32",
    "xxh32_digest",
    "xxh32_hexdigest",
    "xxh32_intdigest",
    "xxh3_128",
    "xxh3_128_digest",
    "xxh3_128_hexdigest",
    "xxh3_128_intdigest",
    "xxh3_64",
    "xxh3_64_digest",
    "xxh3_64_hexdigest",
    "xxh3_64_intdigest",
    "xxh64",
    "xxh64_digest",
    "xxh64_hexdigest",
    "xxh64_intdigest",
)


@pytest.mark.parametrize(
    "attr_name",
    (
        "xxh128",
        "xxh128_digest",
        "xxh128_hexdigest",
        "xxh128_intdigest",
        "xxh32",
        "xxh32_digest",
        "xxh32_hexdigest",
        "xxh32_intdigest",
        "xxh3_128",
        "xxh3_128_digest",
        "xxh3_128_hexdigest",
        "xxh3_128_intdigest",
        "xxh3_64",
        "xxh3_64_digest",
        "xxh3_64_hexdigest",
        "xxh3_64_intdigest",
        "xxh64",
        "xxh64_digest",
        "xxh64_hexdigest",
        "xxh64_intdigest",
    ),
)
def test_all_xxhash_attributes(attr_name: str) -> None:
    """Test that all expected attributes exist in ry.xxhash"""
    assert hasattr(ry.xxhash, attr_name)
    assert callable(getattr(ry.xxhash, attr_name))


def test_attributes_set() -> None:
    assert set(filter(lambda n: not n.startswith("_"), dir(ry.xxhash))) == set(
        _python_xxhash_exports
    )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh32() -> None:
    for seed in XX32_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.xxh32(data, seed=seed).digest()
                == xxhash.xxh32(data, seed).digest()
            )
            assert (
                ry.xxhash.xxh32(data, seed=seed).intdigest()
                == xxhash.xxh32(data, seed).intdigest()
            )
            assert (
                ry.xxhash.xxh32(data, seed=seed).hexdigest()
                == xxhash.xxh32(data, seed).hexdigest()
            )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh64() -> None:
    for seed in XX64_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.xxh64(data, seed=seed).digest()
                == xxhash.xxh64(data, seed).digest()
            )
            assert (
                ry.xxhash.xxh64(data, seed=seed).intdigest()
                == xxhash.xxh64(data, seed).intdigest()
            )
            assert (
                ry.xxhash.xxh64(data, seed=seed).hexdigest()
                == xxhash.xxh64(data, seed).hexdigest()
            )


@pytest_skip_xxhash
def test_xxhash_matches_ry_xxh128() -> None:
    for seed in XX128_SEEDS:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)
            assert (
                ry.xxhash.xxh128(data, seed=seed).digest()
                == xxhash.xxh128(data, seed).digest()
            )
            assert (
                ry.xxhash.xxh128(data, seed=seed).intdigest()
                == xxhash.xxh128(data, seed).intdigest()
            )
            assert (
                ry.xxhash.xxh128(data, seed=seed).hexdigest()
                == xxhash.xxh128(data, seed).hexdigest()
            )
