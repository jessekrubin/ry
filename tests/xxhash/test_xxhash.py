from __future__ import annotations

import pytest

import ry

from ._xxhash_test_data import (
    XX3_64_TEST_DATA,
    XX32_TEST_DATA,
    XX64_TEST_DATA,
    XX128_TEST_DATA,
)


class TestXxh32Hasher:
    def test_xxh32_hasher_digest(self) -> None:
        assert ry.xxh32(b"a").digest() == (1426945110).to_bytes(4, "big")
        assert ry.xxh32(b"a", 0).digest() == (1426945110).to_bytes(4, "big")
        assert ry.xxh32(b"a", 1).digest() == (4111757423).to_bytes(4, "big")
        assert ry.xxh32(b"a", 2**32 - 1).digest() == (3443684653).to_bytes(4, "big")

    def test_xxh32_hasher_intdigest(self) -> None:
        assert ry.xxh32(b"a").intdigest() == 1426945110
        assert ry.xxh32(b"a", 0).intdigest() == 1426945110
        assert ry.xxh32(b"a", 1).intdigest() == 4111757423
        assert ry.xxh32(b"a", 2**32 - 1).intdigest() == 3443684653

    def test_xxh32_hasher_hexdigest(self) -> None:
        assert ry.xxh32(b"a").hexdigest() == (1426945110).to_bytes(4, "big").hex()
        assert ry.xxh32(b"a", 0).hexdigest() == (1426945110).to_bytes(4, "big").hex()
        assert ry.xxh32(b"a", 1).hexdigest() == (4111757423).to_bytes(4, "big").hex()
        assert (
            ry.xxh32(b"a", 2**32 - 1).hexdigest()
            == (3443684653).to_bytes(4, "big").hex()
        )

    def test_xxh32_hasher_copy(self) -> None:
        h = ry.xxh32()
        h.update(b"hello")
        h2 = h.copy()
        assert h.digest() == h2.digest()
        assert h.intdigest() == h2.intdigest()
        assert h.hexdigest() == h2.hexdigest()
        h2.update(b"world")
        assert h.digest() != h2.digest()
        assert h.intdigest() != h2.intdigest()
        assert h.hexdigest() != h2.hexdigest()

        assert h2.digest() == ry.xxh32(b"helloworld").digest()
        assert h2.intdigest() == ry.xxh32(b"helloworld").intdigest()
        assert h2.hexdigest() == ry.xxh32(b"helloworld").hexdigest()


def test_xxh32_digest() -> None:
    assert ry.xxh32_digest(b"a") == (1426945110).to_bytes(4, "big")
    assert ry.xxh32_digest(b"a", 0) == (1426945110).to_bytes(4, "big")
    assert ry.xxh32_digest(b"a", 1) == (4111757423).to_bytes(4, "big")
    assert ry.xxh32_digest(b"a", 2**32 - 1) == (3443684653).to_bytes(4, "big")


def test_xxh32_intdigest() -> None:
    assert ry.xxh32_intdigest(b"a") == 1426945110
    assert ry.xxh32_intdigest(b"a", 0) == 1426945110
    assert ry.xxh32_intdigest(b"a", 1) == 4111757423
    assert ry.xxh32_intdigest(b"a", 2**32 - 1) == 3443684653


def test_xxh32_hexdigest() -> None:
    assert ry.xxh32_hexdigest(b"a") == (1426945110).to_bytes(4, "big").hex()
    assert ry.xxh32_hexdigest(b"a", 0) == (1426945110).to_bytes(4, "big").hex()
    assert ry.xxh32_hexdigest(b"a", 1) == (4111757423).to_bytes(4, "big").hex()
    assert ry.xxh32_hexdigest(b"a", 2**32 - 1) == (3443684653).to_bytes(4, "big").hex()


@pytest.mark.parametrize("data, expected", XX32_TEST_DATA)
def test_xxh32(data: bytes, expected: tuple[int, int, int]) -> None:
    expected_0, expected_1, expected_0xffffff = expected
    int_digest_0, int_digest_1, int_digest_0xffffff = (
        ry.xxh32_intdigest(data),
        ry.xxh32_intdigest(data, seed=1),
        ry.xxh32_intdigest(data, seed=2**32 - 1),
    )
    assert int_digest_0 == expected_0
    assert int_digest_1 == expected_1
    assert int_digest_0xffffff == expected_0xffffff

    # test the hexdigest
    hex_digest_0, hex_digest_1, hex_digest_0xffffff = (
        ry.xxh32_hexdigest(data),
        ry.xxh32_hexdigest(data, seed=1),
        ry.xxh32_hexdigest(data, seed=2**32 - 1),
    )
    assert int(hex_digest_0, 16) == expected_0
    assert int(hex_digest_1, 16) == expected_1
    assert int(hex_digest_0xffffff, 16) == expected_0xffffff

    # test the digest
    digest_0, digest_1, digest_0xffffff = (
        ry.xxh32_digest(data),
        ry.xxh32_digest(data, seed=1),
        ry.xxh32_digest(data, seed=2**32 - 1),
    )
    assert int.from_bytes(digest_0, "big") == expected_0
    assert int.from_bytes(digest_1, "big") == expected_1
    assert int.from_bytes(digest_0xffffff, "big") == expected_0xffffff


@pytest.mark.parametrize("data, expected", XX32_TEST_DATA)
def test_xxh32_hasher(data: bytes, expected: tuple[int, int, int]) -> None:
    expected_0, expected_1, expected_0xffffff = expected
    int_digest_0, int_digest_1, int_digest_0xffffff = (
        ry.xxh32(data).intdigest(),
        ry.xxh32(data, seed=1).intdigest(),
        ry.xxh32(data, seed=2**32 - 1).intdigest(),
    )
    assert int_digest_0 == expected_0
    assert int_digest_1 == expected_1
    assert int_digest_0xffffff == expected_0xffffff

    # test the hexdigest
    hex_digest_0, hex_digest_1, hex_digest_0xffffff = (
        ry.xxh32(data).hexdigest(),
        ry.xxh32(data, seed=1).hexdigest(),
        ry.xxh32(data, seed=2**32 - 1).hexdigest(),
    )
    assert int(hex_digest_0, 16) == expected_0
    assert int(hex_digest_1, 16) == expected_1
    assert int(hex_digest_0xffffff, 16) == expected_0xffffff

    # test the digest
    digest_0, digest_1, digest_0xffffff = (
        ry.xxh32(data).digest(),
        ry.xxh32(data, seed=1).digest(),
        ry.xxh32(data, seed=2**32 - 1).digest(),
    )
    assert int.from_bytes(digest_0, "big") == expected_0
    assert int.from_bytes(digest_1, "big") == expected_1
    assert int.from_bytes(digest_0xffffff, "big") == expected_0xffffff


@pytest.mark.parametrize("data, expected", XX64_TEST_DATA)
def test_xxh64_const_fns(data: bytes, expected: tuple[int, int, int]) -> None:
    expected_0, expected_1, expected_0xffffffff = expected
    int_digest_0, int_digest_1, int_digest_0xffffffff = (
        ry.xxh64_intdigest(data),
        ry.xxh64_intdigest(data, seed=1),
        ry.xxh64_intdigest(data, seed=2**64 - 1),
    )
    assert int_digest_0 == expected_0
    assert int_digest_1 == expected_1
    assert int_digest_0xffffffff == expected_0xffffffff

    # test the hexdigest
    hex_digest_0, hex_digest_1, hex_digest_0xffffffff = (
        ry.xxh64_hexdigest(data),
        ry.xxh64_hexdigest(data, seed=1),
        ry.xxh64_hexdigest(data, seed=2**64 - 1),
    )
    assert int(hex_digest_0, 16) == expected_0
    assert int(hex_digest_1, 16) == expected_1
    assert int(hex_digest_0xffffffff, 16) == expected_0xffffffff

    # test the digest
    digest_0, digest_1, digest_0xffffffff = (
        ry.xxh64_digest(data),
        ry.xxh64_digest(data, seed=1),
        ry.xxh64_digest(data, seed=2**64 - 1),
    )
    assert int.from_bytes(digest_0, "big") == expected_0
    assert int.from_bytes(digest_1, "big") == expected_1
    assert int.from_bytes(digest_0xffffffff, "big") == expected_0xffffffff


@pytest.mark.parametrize("data, expected", XX128_TEST_DATA)
def test_xxh128_const_fns(data: bytes, expected: tuple[int, int, int]) -> None:
    expected_0, expected_1, expected_2 = expected
    int_digest_0, int_digest_1, int_digest_2 = (
        ry.xxh128_intdigest(data),
        ry.xxh128_intdigest(data, seed=1),
        ry.xxh128_intdigest(data, seed=2**64 - 1),
    )
    assert int_digest_0 == expected_0
    assert int_digest_1 == expected_1
    assert int_digest_2 == expected_2

    # test the hexdigest
    hex_digest_0, hex_digest_1, hex_digest_2 = (
        ry.xxh128_hexdigest(data),
        ry.xxh128_hexdigest(data, seed=1),
        ry.xxh128_hexdigest(data, seed=2**64 - 1),
    )
    assert int(hex_digest_0, 16) == expected_0
    assert int(hex_digest_1, 16) == expected_1
    assert int(hex_digest_2, 16) == expected_2

    # test the digest
    digest_0, digest_1, digest_2 = (
        ry.xxh128_digest(data),
        ry.xxh128_digest(data, seed=1),
        ry.xxh128_digest(data, seed=2**64 - 1),
    )
    assert int.from_bytes(digest_0, "big") == expected_0
    assert int.from_bytes(digest_1, "big") == expected_1
    assert int.from_bytes(digest_2, "big") == expected_2


@pytest.mark.parametrize("data, expected", XX3_64_TEST_DATA)
def test_xx3_64(data: bytes, expected: tuple[int, int, int]) -> None:
    expected_0, expected_1, expected_0xffffffff = expected
    int_digest_0, int_digest_1, int_digest_0xffffffff = (
        ry.xxh3_64_intdigest(data),
        ry.xxh3_64_intdigest(data, seed=1),
        ry.xxh3_64_intdigest(data, seed=2**64 - 1),
    )
    assert int_digest_0 == expected_0
    assert int_digest_1 == expected_1
    assert int_digest_0xffffffff == expected_0xffffffff

    # test the hexdigest
    hex_digest_0, hex_digest_1, hex_digest_0xffffffff = (
        ry.xxh3_64_hexdigest(data),
        ry.xxh3_64_hexdigest(data, seed=1),
        ry.xxh3_64_hexdigest(data, seed=2**64 - 1),
    )
    assert int(hex_digest_0, 16) == expected_0
    assert int(hex_digest_1, 16) == expected_1
    assert int(hex_digest_0xffffffff, 16) == expected_0xffffffff

    # test the digest
    digest_0, digest_1, digest_0xffffffff = (
        ry.xxh3_64_digest(data),
        ry.xxh3_64_digest(data, seed=1),
        ry.xxh3_64_digest(data, seed=2**64 - 1),
    )
    assert int.from_bytes(digest_0, "big") == expected_0
    assert int.from_bytes(digest_1, "big") == expected_1
    assert int.from_bytes(digest_0xffffffff, "big") == expected_0xffffffff
