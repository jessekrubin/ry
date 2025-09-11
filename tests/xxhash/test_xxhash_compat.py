from __future__ import annotations

import ast
import sys
from typing import TYPE_CHECKING, Protocol, TypeAlias

import pytest

import ry

from ._xxhash_fixtures import (
    XX32_SEEDS,
    XX64_SEEDS,
    XX128_SEEDS,
    XXHASH_TEST_DATA,
    XXHashDataRecord,
)

if TYPE_CHECKING:
    from collections.abc import Sequence

    import xxhash

    _PyHasher: TypeAlias = type[
        xxhash.xxh32 | xxhash.xxh64 | xxhash.xxh128 | xxhash.xxh3_64 | xxhash.xxh3_128
    ]

    _RyHasher: TypeAlias = type[
        ry.xxhash.xxh32
        | ry.xxhash.xxh64
        | ry.xxhash.xxh128
        | ry.xxhash.xxh3_64
        | ry.xxhash.xxh3_128
    ]

try:
    # test against python-xxhash if importable...
    import xxhash
except ImportError:

    class _xxhash:  # noqa: N801
        def __getattr__(self, name: str) -> None:
            return None

    xxhash = _xxhash()

pytest_skip_xxhash = pytest.mark.skipif(
    "xxhash" not in sys.modules, reason="xxhash is not installed"
)


class _RyXxHashOneShotFn(Protocol):
    def __call__(self, data: bytes, *, seed: int = 0) -> bytes | str | int: ...


class _PyXxHashOneShotFn(Protocol):
    def __call__(self, data: bytes, seed: int = 0) -> bytes | str | int: ...


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
    ry_exports = set(filter(lambda n: not n.startswith("_"), dir(ry.xxhash)))
    py_xxhash_exports = set(_python_xxhash_exports)
    assert py_xxhash_exports.issubset(ry_exports)


@pytest_skip_xxhash
@pytest.mark.parametrize(
    "ryxx,pyxx",
    [
        (ry.xxhash.xxh32_digest, xxhash.xxh32_digest),
        (ry.xxhash.xxh32_hexdigest, xxhash.xxh32_hexdigest),
        (ry.xxhash.xxh32_intdigest, xxhash.xxh32_intdigest),
    ],
)
def test_xxhash_matches_ry_xxh32(
    ryxx: _RyXxHashOneShotFn, pyxx: _PyXxHashOneShotFn
) -> None:
    for seed in XX32_SEEDS:
        for rec in XXHASH_TEST_DATA:
            ry_res = ryxx(_bytes_from_record(rec), seed=seed)
            py_res = pyxx(_bytes_from_record(rec), seed)
            assert ry_res == py_res
            assert isinstance(ry_res, type(py_res))


@pytest_skip_xxhash
@pytest.mark.parametrize(
    "ryxx,pyxx",
    [
        (ry.xxhash.xxh64_digest, xxhash.xxh64_digest),
        (ry.xxhash.xxh64_hexdigest, xxhash.xxh64_hexdigest),
        (ry.xxhash.xxh64_intdigest, xxhash.xxh64_intdigest),
    ],
)
def test_xxhash_matches_ry_xxh64(
    ryxx: _RyXxHashOneShotFn, pyxx: _PyXxHashOneShotFn
) -> None:
    for seed in XX64_SEEDS:
        for rec in XXHASH_TEST_DATA:
            ry_res = ryxx(_bytes_from_record(rec), seed=seed)
            py_res = pyxx(_bytes_from_record(rec), seed)
            assert ry_res == py_res
            assert isinstance(ry_res, type(py_res))


@pytest_skip_xxhash
@pytest.mark.parametrize(
    "ryxx,pyxx",
    [
        (ry.xxhash.xxh3_64_digest, xxhash.xxh3_64_digest),
        (ry.xxhash.xxh3_64_hexdigest, xxhash.xxh3_64_hexdigest),
        (ry.xxhash.xxh3_64_intdigest, xxhash.xxh3_64_intdigest),
    ],
)
def test_xxhash_matches_ry_xxh3_64(
    ryxx: _RyXxHashOneShotFn, pyxx: _PyXxHashOneShotFn
) -> None:
    for seed in XX64_SEEDS:
        for rec in XXHASH_TEST_DATA:
            ry_res = ryxx(_bytes_from_record(rec), seed=seed)
            py_res = pyxx(_bytes_from_record(rec), seed)
            assert ry_res == py_res
            assert isinstance(ry_res, type(py_res))


@pytest_skip_xxhash
@pytest.mark.parametrize(
    "ryxx,pyxx",
    [
        (ry.xxhash.xxh3_128_digest, xxhash.xxh3_128_digest),
        (ry.xxhash.xxh3_128_hexdigest, xxhash.xxh3_128_hexdigest),
        (ry.xxhash.xxh3_128_intdigest, xxhash.xxh3_128_intdigest),
    ],
)
def test_xxhash_matches_ry_xxh3_128(
    ryxx: _RyXxHashOneShotFn, pyxx: _PyXxHashOneShotFn
) -> None:
    for seed in XX128_SEEDS:
        for rec in XXHASH_TEST_DATA:
            ry_res = ryxx(_bytes_from_record(rec), seed=seed)
            py_res = pyxx(_bytes_from_record(rec), seed)
            assert ry_res == py_res
            assert isinstance(ry_res, type(py_res))


@pytest_skip_xxhash
@pytest.mark.parametrize(
    "ryhasher,pyhasher,seeds",
    [
        (ry.xxhash.xxh32, xxhash.xxh32, XX32_SEEDS),
        (ry.xxhash.xxh64, xxhash.xxh64, XX64_SEEDS),
        (ry.xxhash.xxh128, xxhash.xxh128, XX128_SEEDS),
    ],
)
def test_xxhash_hashers_starting_data(
    ryhasher: _RyHasher,
    pyhasher: _PyHasher,
    seeds: Sequence[int],
) -> None:
    for seed in seeds:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)

            assert ryhasher(data, seed=seed).digest() == pyhasher(data, seed).digest()
            assert (
                ryhasher(data, seed=seed).intdigest()
                == pyhasher(data, seed).intdigest()
            )
            assert (
                ryhasher(data, seed=seed).hexdigest()
                == pyhasher(data, seed).hexdigest()
            )


@pytest_skip_xxhash
@pytest.mark.parametrize(
    "ryhasher,pyhasher,seeds",
    [
        (ry.xxhash.xxh32, xxhash.xxh32, XX32_SEEDS),
        (ry.xxhash.xxh64, xxhash.xxh64, XX64_SEEDS),
        (ry.xxhash.xxh128, xxhash.xxh128, XX128_SEEDS),
    ],
)
def test_xxhash_hashers_starting_update(
    ryhasher: _RyHasher,
    pyhasher: _PyHasher,
    seeds: Sequence[int],
) -> None:
    for seed in seeds:
        for rec in XXHASH_TEST_DATA:
            data = _bytes_from_record(rec)

            _ry = ryhasher(seed=seed)
            _py = pyhasher(seed=seed)
            _ry.update(data)
            _py.update(data)
            assert _ry.digest() == _py.digest()
            assert _ry.intdigest() == _py.intdigest()
            assert _ry.hexdigest() == _py.hexdigest()
