from __future__ import annotations

import hashlib
import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

    from ry.ryo3._aws_lc import (
        _Sha,
    )


def _random_bytes(size: int) -> bytes:
    # make random bytes
    return bytes([i % 256 for i in range(size)])


_BYTES = [
    ("1kib", _random_bytes(1024)),
    ("10kib", _random_bytes(1024 * 10)),
    ("100kib", _random_bytes(1024 * 100)),
    ("1mib", _random_bytes(1024 * 1024)),
    ("10mib", _random_bytes(1024 * 1024 * 10)),
]
_HASHERS = [
    ("sha1", ry.sha1, hashlib.sha1),
    ("sha224", ry.sha224, hashlib.sha224),
    ("sha256", ry.sha256, hashlib.sha256),
    ("sha384", ry.sha384, hashlib.sha384),
    ("sha3_256", ry.sha3_256, hashlib.sha3_256),
    ("sha3_384", ry.sha3_384, hashlib.sha3_384),
    ("sha3_512", ry.sha3_512, hashlib.sha3_512),
    ("sha512", ry.sha512, hashlib.sha512),
]
_RY_HASHERS = [(name, "ry", cls) for name, cls, _ in _HASHERS]
_PY_HASHERS = [(name, "py", cls) for name, _, cls in _HASHERS]


@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize(
    "impl", [*_RY_HASHERS, *_PY_HASHERS], ids=lambda impl: f"{impl[0]}-{impl[1]}"
)
def test_bench_aws_lc_digest_sha(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, str, type]
) -> None:
    size, data = id_data
    benchmark.group = f"aws-lc-{impl[0]}-{size}"

    def _fn() -> None:
        h = impl[2]()
        h.update(data)
        h.digest()

    benchmark(_fn)


@pytest.mark.parametrize("id_data", _BYTES, ids=lambda t: t[0])
@pytest.mark.parametrize("impl", _RY_HASHERS, ids=lambda t: f"{t[0]}-{t[1]}")
def test_bench_aws_lc_digest_oneshot(
    benchmark: BenchmarkFixture,
    id_data: tuple[str, bytes],
    impl: tuple[str, str, _Sha[str, int, int]],
) -> None:
    size, data = id_data
    benchmark.group = f"aws-lc-{impl[0]}-{size}"

    def _fn() -> None:
        impl[2].oneshot(data)

    benchmark(_fn)
