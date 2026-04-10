"""Tests for the `ry.xxhash` module.

This test module was previously very very very large and handwritten. I did a
bit of refactoring and as a part of that process I handed my refactored version
to Chad-Gippity and asked it to re-organize it...
"""

from __future__ import annotations

import dataclasses
import typing as t

import pytest

import ry
from ry import xxhash as ry_xxh

from ._xxhash_fixtures import XXHASH_TEST_DATA, XXHashDataRecord, _bytes_from_record

_TXxHasher = ry_xxh.xxh32 | ry_xxh.xxh64 | ry_xxh.xxh3_64 | ry_xxh.xxh3_128


@dataclasses.dataclass
class _XxHasher:
    name: str
    hasher: type[_TXxHasher]
    digest_size: int
    block_size: int


_XXHASHERS = (
    _XxHasher(
        name="xxh32",
        hasher=ry_xxh.xxh32,
        digest_size=4,
        block_size=16,
    ),
    _XxHasher(
        name="xxh64",
        hasher=ry_xxh.xxh64,
        digest_size=8,
        block_size=32,
    ),
    _XxHasher(
        name="xxh3_64",
        hasher=ry_xxh.xxh3_64,
        digest_size=8,
        block_size=32,
    ),
    _XxHasher(
        name="xxh3_128",
        hasher=ry_xxh.xxh3_128,
        digest_size=16,
        block_size=64,
    ),
)


@pytest.mark.parametrize("xxhasher", _XXHASHERS)
def test_attributes(xxhasher: _XxHasher) -> None:
    assert xxhasher.hasher.name == xxhasher.name
    assert xxhasher.hasher.digest_size == xxhasher.digest_size
    assert xxhasher.hasher.block_size == xxhasher.block_size


@pytest.mark.parametrize("xxhasher", _XXHASHERS)
def test_repr(xxhasher: _XxHasher) -> None:
    hasher = xxhasher.hasher(b"dingo")
    hex_digest = hasher.hexdigest()
    expected = f"{xxhasher.name}<{hex_digest}>"
    repr_str = repr(hasher)
    assert repr_str == expected


class TestXxHash3Secret:
    _OK_SECRET = b"dingo" * 100  # 128 bytes, which is >= 136
    _HASH_DATA = b"dingo" * 100
    _XXH3_64_EXPECTED = 2021458189288565107
    _XXH3_128_EXPECTED = 203810581866337968790239271433929320819

    @pytest.mark.parametrize(
        "xxh3_fn",
        [
            lambda data, secret: ry_xxh.xxh3_64(
                data, seed=0, secret=secret
            ).intdigest(),
            lambda data, secret: ry_xxh.xxh3_128(
                data, seed=0, secret=secret
            ).intdigest(),
            lambda data, secret: ry_xxh.xxh3_64.oneshot(data, seed=0, secret=secret),
            lambda data, secret: ry_xxh.xxh3_128.oneshot(data, seed=0, secret=secret),
        ],
    )
    @pytest.mark.parametrize(
        "secret_type",
        [bytes, memoryview, bytearray, ry.Bytes],
    )
    def test_xxhash3_secret_ok(
        self,
        xxh3_fn: t.Callable[[bytes, bytes], _TXxHasher],
        secret_type: type[bytes | memoryview | bytearray | ry.Bytes],
    ) -> None:
        result = xxh3_fn(self._HASH_DATA, secret_type(self._OK_SECRET))
        assert isinstance(result, int)
        assert result in (self._XXH3_64_EXPECTED, self._XXH3_128_EXPECTED)

    @pytest.mark.parametrize(
        "xxh3_fn",
        [
            lambda data, secret: ry_xxh.xxh3_64(data, seed=0, secret=secret),
            lambda data, secret: ry_xxh.xxh3_128(data, seed=0, secret=secret),
        ],
    )
    def test_xxh3_secret_too_short(
        self, xxh3_fn: t.Callable[[bytes, bytes], _TXxHasher]
    ) -> None:
        with pytest.raises(ValueError, match="secret must be at least 136 bytes long"):
            xxh3_fn(b"hello", b"short secret")

    @pytest.mark.parametrize(
        "xxh3_fn",
        [
            lambda data, secret: ry_xxh.xxh3_64(data, seed=0, secret=secret),
            lambda data, secret: ry_xxh.xxh3_128(data, seed=0, secret=secret),
        ],
    )
    def test_xxh3_secret_type_error(
        self, xxh3_fn: t.Callable[[bytes, bytes], _TXxHasher]
    ) -> None:
        with pytest.raises(
            TypeError,
            match="xxhash3-secret must be readable-buffer with of at least 136 bytes",
        ):
            xxh3_fn(b"hello", complex(1, 2))  # type: ignore[arg-type]


class TestXxh32Hasher:
    def test_xxh32_hasher_digest(self) -> None:
        assert ry_xxh.xxh32(b"a").digest() == (1426945110).to_bytes(4, "big")
        assert ry_xxh.xxh32(b"a", seed=0).digest() == (1426945110).to_bytes(4, "big")
        assert ry_xxh.xxh32(b"a", seed=1).digest() == (4111757423).to_bytes(4, "big")
        assert ry_xxh.xxh32(b"a", seed=2**32 - 1).digest() == (3443684653).to_bytes(
            4, "big"
        )

    def test_xxh32_hasher_intdigest(self) -> None:
        assert ry_xxh.xxh32(b"a").intdigest() == 1426945110
        assert ry_xxh.xxh32(b"a", seed=0).intdigest() == 1426945110
        assert ry_xxh.xxh32(b"a", seed=1).intdigest() == 4111757423
        assert ry_xxh.xxh32(b"a", seed=2**32 - 1).intdigest() == 3443684653

    def test_xxh32_hasher_oneshot_int_digest(self) -> None:
        assert ry_xxh.xxh32.oneshot(b"a") == 1426945110
        assert ry_xxh.xxh32.oneshot(b"a", seed=0) == 1426945110
        assert ry_xxh.xxh32.oneshot(b"a", seed=1) == 4111757423
        assert ry_xxh.xxh32.oneshot(b"a", seed=2**32 - 1) == 3443684653

    def test_xxh32_hasher_hexdigest(self) -> None:
        assert ry_xxh.xxh32(b"a").hexdigest() == (1426945110).to_bytes(4, "big").hex()
        assert (
            ry_xxh.xxh32(b"a", seed=0).hexdigest()
            == (1426945110).to_bytes(4, "big").hex()
        )
        assert (
            ry_xxh.xxh32(b"a", seed=1).hexdigest()
            == (4111757423).to_bytes(4, "big").hex()
        )
        assert (
            ry_xxh.xxh32(b"a", seed=2**32 - 1).hexdigest()
            == (3443684653).to_bytes(4, "big").hex()
        )

    def test_xxh32_hasher_copy(self) -> None:
        h = ry_xxh.xxh32()
        h.update(b"hello")
        h2 = h.copy()
        assert h.digest() == h2.digest()
        assert h.intdigest() == h2.intdigest()
        assert h.hexdigest() == h2.hexdigest()
        h2.update(b"world")
        assert h.digest() != h2.digest()
        assert h.intdigest() != h2.intdigest()
        assert h.hexdigest() != h2.hexdigest()

        assert h2.digest() == ry_xxh.xxh32(b"helloworld").digest()
        assert h2.intdigest() == ry_xxh.xxh32(b"helloworld").intdigest()
        assert h2.hexdigest() == ry_xxh.xxh32(b"helloworld").hexdigest()


@pytest.mark.parametrize(
    "hasher",
    [
        ry_xxh.xxh32(),
        ry_xxh.xxh32(seed=123),
        ry_xxh.xxh64(),
        ry_xxh.xxh64(seed=123),
        ry_xxh.xxh3_64(),
        ry_xxh.xxh3_64(seed=123),
        ry_xxh.xxh3_128(),
        ry_xxh.xxh3_128(seed=123),
    ],
)
def test_hashers_reset(hasher: _TXxHasher) -> None:
    initial_digest = hasher.digest()
    initial_seed = hasher.seed
    hasher.update(b"hello")
    if isinstance(hasher, (ry_xxh.xxh32, ry_xxh.xxh64)):
        assert hasher.length == 5
    hexdig = hasher.hexdigest()
    assert hexdig in str(hexdig)
    assert hexdig in repr(hexdig)
    hasher.reset()
    if isinstance(hasher, (ry_xxh.xxh32, ry_xxh.xxh64)):
        assert hasher.length == 0
    assert hasher.seed == initial_seed
    assert hasher.digest() == initial_digest


@pytest.mark.parametrize(
    "hasher",
    [
        ry_xxh.xxh32,
        ry_xxh.xxh64,
        ry_xxh.xxh3_64,
        ry_xxh.xxh3_128,
    ],
)
def test_hashers_copy(
    hasher: type[_TXxHasher],
) -> None:
    og = hasher(b"hello", seed=123)
    if isinstance(og, (ry_xxh.xxh32, ry_xxh.xxh64)):
        assert og.length == 5
    copycat = og.copy()
    assert og.digest() == copycat.digest()
    assert og.intdigest() == copycat.intdigest()
    assert og.hexdigest() == copycat.hexdigest()
    copycat.update(b"world")
    if isinstance(og, (ry_xxh.xxh32, ry_xxh.xxh64)):
        assert og.length == 5
        assert copycat.length == 10  # type: ignore[union-attr]
    assert og.digest() != copycat.digest()
    assert og.intdigest() != copycat.intdigest()
    assert og.hexdigest() != copycat.hexdigest()


def test_xxh32_digest() -> None:
    assert ry_xxh.xxh32_digest(b"a") == (1426945110).to_bytes(4, "big")
    assert ry_xxh.xxh32_digest(b"a", seed=0) == (1426945110).to_bytes(4, "big")
    assert ry_xxh.xxh32_digest(b"a", seed=1) == (4111757423).to_bytes(4, "big")
    assert ry_xxh.xxh32_digest(b"a", seed=2**32 - 1) == (3443684653).to_bytes(4, "big")


def test_xxh32_intdigest() -> None:
    assert ry_xxh.xxh32_intdigest(b"a") == 1426945110
    assert ry_xxh.xxh32_intdigest(b"a", seed=0) == 1426945110
    assert ry_xxh.xxh32_intdigest(b"a", seed=1) == 4111757423
    assert ry_xxh.xxh32_intdigest(b"a", seed=2**32 - 1) == 3443684653


def test_xxh32_hexdigest() -> None:
    assert ry_xxh.xxh32_hexdigest(b"a") == (1426945110).to_bytes(4, "big").hex()
    assert ry_xxh.xxh32_hexdigest(b"a", seed=0) == (1426945110).to_bytes(4, "big").hex()
    assert ry_xxh.xxh32_hexdigest(b"a", seed=1) == (4111757423).to_bytes(4, "big").hex()
    assert (
        ry_xxh.xxh32_hexdigest(b"a", seed=2**32 - 1)
        == (3443684653).to_bytes(4, "big").hex()
    )


# ===========================================================================
# PARAM TESTS
# ===========================================================================
# -----------------------------------------------------------------------------
# UTILS
# -----------------------------------------------------------------------------


def _assert_xxh32_all_forms(
    data: bytes, seeds: list[int], expected_hexes: list[str]
) -> None:
    """Tests xxh32_{intdigest,hexdigest,digest} for each seed"""
    expected_ints = [int(h, 16) for h in expected_hexes]

    # intdigest
    actual_ints = [ry_xxh.xxh32_intdigest(data, seed=s) for s in seeds]
    assert actual_ints == expected_ints, (
        f"xxh32_intdigest mismatch: got {actual_ints}, expected {expected_ints}"
    )

    # hexdigest
    actual_hexes = [ry_xxh.xxh32_hexdigest(data, seed=s) for s in seeds]
    assert [int(h, 16) for h in actual_hexes] == expected_ints, (
        "xxh32_hexdigest mismatch"
    )

    # digest (raw bytes)
    actual_digests = [ry_xxh.xxh32_digest(data, seed=s) for s in seeds]
    actual_from_bytes = [int.from_bytes(d, "big") for d in actual_digests]
    assert actual_from_bytes == expected_ints, "xxh32_digest mismatch"


def _assert_xxh64_all_forms(
    data: bytes, seeds: list[int], expected_hexes: list[str]
) -> None:
    """Tests xxh64_{intdigest,hexdigest,digest} for each seed"""
    expected_ints = [int(h, 16) for h in expected_hexes]

    # intdigest
    actual_ints = [ry_xxh.xxh64_intdigest(data, seed=s) for s in seeds]
    assert actual_ints == expected_ints

    # oneshot
    actual_ints_oneshot = [ry_xxh.xxh64.oneshot(data, seed=s) for s in seeds]
    assert actual_ints_oneshot == expected_ints

    # hexdigest
    actual_hexes = [ry_xxh.xxh64_hexdigest(data, seed=s) for s in seeds]
    assert [int(h, 16) for h in actual_hexes] == expected_ints

    # digest
    actual_digests = [ry_xxh.xxh64_digest(data, seed=s) for s in seeds]
    assert [int.from_bytes(d, "big") for d in actual_digests] == expected_ints


def _assert_xxh3_64_all_forms(
    data: bytes, seeds: list[int], expected_hexes: list[str]
) -> None:
    """Tests xxh3_64_{intdigest,hexdigest,digest} for each seed"""
    expected_ints = [int(h, 16) for h in expected_hexes]

    # intdigest
    actual_ints = [ry_xxh.xxh3_64_intdigest(data, seed=s) for s in seeds]
    assert actual_ints == expected_ints

    # hexdigest
    actual_hexes = [ry_xxh.xxh3_64_hexdigest(data, seed=s) for s in seeds]
    assert [int(h, 16) for h in actual_hexes] == expected_ints

    # digest
    actual_digests = [ry_xxh.xxh3_64_digest(data, seed=s) for s in seeds]
    assert [int.from_bytes(d, "big") for d in actual_digests] == expected_ints

    # oneshot on hasher
    actual_ints_oneshot = [ry_xxh.xxh3_64.oneshot(data, seed=s) for s in seeds]
    assert actual_ints_oneshot == expected_ints

    # hasher init
    actual_ints_hasher = [ry_xxh.xxh3_64(data, seed=s).intdigest() for s in seeds]
    assert actual_ints_hasher == expected_ints


def _assert_xxh3_128_all_forms(
    data: bytes, seeds: list[int], expected_hexes: list[str]
) -> None:
    """Tests xxh3_128_{intdigest,hexdigest,digest} for each seed"""
    expected_ints = [int(h, 16) for h in expected_hexes]

    # intdigest
    actual_ints = [ry_xxh.xxh3_128_intdigest(data, seed=s) for s in seeds]
    assert actual_ints == expected_ints

    # hexdigest
    actual_hexes = [ry_xxh.xxh3_128_hexdigest(data, seed=s) for s in seeds]
    assert [int(h, 16) for h in actual_hexes] == expected_ints

    # digest
    actual_digests = [ry_xxh.xxh3_128_digest(data, seed=s) for s in seeds]
    assert [int.from_bytes(d, "big") for d in actual_digests] == expected_ints

    # ALIASES
    # intdigest
    actual_ints = [ry_xxh.xxh128_intdigest(data, seed=s) for s in seeds]
    assert actual_ints == expected_ints

    # hexdigest
    actual_hexes = [ry_xxh.xxh128_hexdigest(data, seed=s) for s in seeds]
    assert [int(h, 16) for h in actual_hexes] == expected_ints

    # digest
    actual_digests = [ry_xxh.xxh128_digest(data, seed=s) for s in seeds]
    assert [int.from_bytes(d, "big") for d in actual_digests] == expected_ints

    # oneshot on hasher
    actual_ints_oneshot = [ry_xxh.xxh3_128.oneshot(data, seed=s) for s in seeds]
    assert actual_ints_oneshot == expected_ints

    # hasher init
    actual_ints_hasher = [ry_xxh.xxh3_128(data, seed=s).intdigest() for s in seeds]
    assert actual_ints_hasher == expected_ints


# -----------------------------------------------------------------------------
# TESTS
# -----------------------------------------------------------------------------


@pytest.mark.parametrize("rec", XXHASH_TEST_DATA)
def test_xxh32(rec: XXHashDataRecord) -> None:
    data = _bytes_from_record(rec)

    # We have three seeds: 0, 1, and 0xFFFFFFFF
    seeds = [0, 1, 2**32 - 1]
    expected_hexes = [
        rec["xxh32_0x00000000"],
        rec["xxh32_0x00000001"],
        rec["xxh32_0xFFFFFFFF"],
    ]
    _assert_xxh32_all_forms(data, seeds, expected_hexes)


@pytest.mark.parametrize("rec", XXHASH_TEST_DATA)
def test_xxh32_hasher(rec: XXHashDataRecord) -> None:
    """Tests xxh32_{intdigest,hexdigest,digest} for each seed using Hasher"""
    data = _bytes_from_record(rec)
    expected_0 = int(rec["xxh32_0x00000000"], 16)
    expected_1 = int(rec["xxh32_0x00000001"], 16)
    expected_ff = int(rec["xxh32_0xFFFFFFFF"], 16)

    # Check intdigest
    assert ry_xxh.xxh32(data).intdigest() == expected_0
    assert ry_xxh.xxh32(data, seed=1).intdigest() == expected_1
    assert ry_xxh.xxh32(data, seed=2**32 - 1).intdigest() == expected_ff

    # Check hexdigest
    assert int(ry_xxh.xxh32(data).hexdigest(), 16) == expected_0
    assert int(ry_xxh.xxh32(data, seed=1).hexdigest(), 16) == expected_1
    assert int(ry_xxh.xxh32(data, seed=2**32 - 1).hexdigest(), 16) == expected_ff

    # Check digest
    assert int.from_bytes(ry_xxh.xxh32(data).digest(), "big") == expected_0
    assert int.from_bytes(ry_xxh.xxh32(data, seed=1).digest(), "big") == expected_1
    assert (
        int.from_bytes(ry_xxh.xxh32(data, seed=2**32 - 1).digest(), "big")
        == expected_ff
    )


# ------------------------------------------------------------------------------
# Test xxh64
# ------------------------------------------------------------------------------


@pytest.mark.parametrize("rec", XXHASH_TEST_DATA)
def test_xxh64_oneshot_fns(rec: XXHashDataRecord) -> None:
    data = _bytes_from_record(rec)

    # Seeds: 0, 1, 0xFFFFFFFFFFFFFFFF
    seeds = [0, 1, 2**64 - 1]
    expected_hexes = [
        rec["xxh64_0x00000000"],
        rec["xxh64_0x00000001"],
        rec["xxh64_0xFFFFFFFFFFFFFFFF"],
    ]
    _assert_xxh64_all_forms(data, seeds, expected_hexes)


# ------------------------------------------------------------------------------
# Test xxh3_64
# ------------------------------------------------------------------------------
@pytest.mark.parametrize("rec", XXHASH_TEST_DATA)
def test_xxh3_64_oneshot_fns(rec: XXHashDataRecord) -> None:
    data = _bytes_from_record(rec)

    # Seeds: 0, 1, 0xFFFFFFFFFFFFFFFF
    seeds = [0, 1, 2**64 - 1]
    expected_hexes = [
        rec["xxh3_64_0x00000000"],
        rec["xxh3_64_0x00000001"],
        rec["xxh3_64_0xFFFFFFFFFFFFFFFF"],
    ]
    _assert_xxh3_64_all_forms(data, seeds, expected_hexes)


# ------------------------------------------------------------------------------
# Test xxh3_128
# ------------------------------------------------------------------------------
@pytest.mark.parametrize("rec", XXHASH_TEST_DATA)
def test_xxh3_128_oneshot_fns(rec: XXHashDataRecord) -> None:
    data = _bytes_from_record(rec)

    # Seeds: 0, 1, 0xFFFFFFFFFFFFFFFF
    seeds = [0, 1, 2**64 - 1]
    expected_hexes = [
        rec["xxh3_128_0x00000000"],
        rec["xxh3_128_0x00000001"],
        rec["xxh3_128_0xFFFFFFFFFFFFFFFF"],
    ]
    _assert_xxh3_128_all_forms(data, seeds, expected_hexes)
