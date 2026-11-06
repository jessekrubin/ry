"""xxhash benchmarks

impls:
- `ry.xxhash` (twox-hash)
- `xxhash` (pypi, wraps the C lib)
"""

from __future__ import annotations

import typing as t

import pytest

from ry import xxhash as ry_twox

if t.TYPE_CHECKING:
    from types import ModuleType

    from pytest_benchmark.fixture import BenchmarkFixture

_IMPLS: dict[str, t.Any] = {"twox": ry_twox}
try:
    import xxhash as _pypi_xxhash

    _IMPLS["pypi"] = _pypi_xxhash
except ImportError:  # pragma: no cover
    pass

_ALGOS = ("xxh32", "xxh64", "xxh3_64", "xxh3_128")
_SIZES = {"16B": 16, "256B": 256, "4KiB": 4096, "64KiB": 65536, "1MiB": 1 << 20}
_DATA = {k: bytes(i % 251 for i in range(n)) for k, n in _SIZES.items()}

_IMPL_PARAMS = [pytest.param(m, id=k) for k, m in _IMPLS.items()]


@pytest.mark.parametrize("impl", _IMPL_PARAMS)
@pytest.mark.parametrize("size", list(_SIZES))
@pytest.mark.parametrize("algo", _ALGOS)
def test_oneshot_intdigest(
    benchmark: BenchmarkFixture, impl: ModuleType, algo: str, size: str
) -> None:
    benchmark.group = f"oneshot-intdigest-{algo}-{size}"
    fn = getattr(impl, f"{algo}_intdigest")
    data = _DATA[size]
    benchmark(fn, data)


@pytest.mark.parametrize("impl", _IMPL_PARAMS)
@pytest.mark.parametrize("kind", ["digest", "hexdigest"])
@pytest.mark.parametrize("algo", _ALGOS)
def test_oneshot_out(
    benchmark: BenchmarkFixture, impl: ModuleType, algo: str, kind: str
) -> None:
    benchmark.group = f"oneshot-{kind}-{algo}-16B"
    fn = getattr(impl, f"{algo}_{kind}")
    benchmark(fn, _DATA["16B"])


@pytest.mark.parametrize("impl", _IMPL_PARAMS)
@pytest.mark.parametrize("algo", _ALGOS)
def test_new_update_intdigest(
    benchmark: BenchmarkFixture, impl: ModuleType, algo: str
) -> None:
    benchmark.group = f"hasher-new-update-intdigest-{algo}-256B"
    cls = getattr(impl, algo)
    data = _DATA["256B"]

    def _run() -> object:
        h = cls()
        h.update(data)
        return h.intdigest()

    benchmark(_run)


@pytest.mark.parametrize("impl", _IMPL_PARAMS)
@pytest.mark.parametrize("algo", _ALGOS)
def test_new(benchmark: BenchmarkFixture, impl: ModuleType, algo: str) -> None:
    benchmark.group = f"hasher-new-{algo}"
    benchmark(getattr(impl, algo))


@pytest.mark.parametrize("impl", _IMPL_PARAMS)
@pytest.mark.parametrize("algo", _ALGOS)
def test_chunked_update(
    benchmark: BenchmarkFixture, impl: ModuleType, algo: str
) -> None:
    # 1MiB in 4KiB chunks
    benchmark.group = f"hasher-chunked-1MiB-by-4KiB-{algo}"
    cls = getattr(impl, algo)
    data = memoryview(_DATA["1MiB"])
    chunks = [data[i : i + 4096] for i in range(0, len(data), 4096)]

    def _run() -> object:
        h = cls()
        for c in chunks:
            h.update(c)
        return h.intdigest()

    benchmark(_run)
