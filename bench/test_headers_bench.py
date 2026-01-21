"""Various benchmarks for `ry.Headers`"""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_DEFAULT_TEST_HEADERS: dict[str, str] = {
    "User-Agent": f"ry-reqwest/{ry.__version__}",
    "Accept": "*/*",
    "Accept-Encoding": "gzip, deflate, br",
    "Connection": "keep-alive",
}


def _build_headers(size: int) -> ry.Headers:
    h = ry.Headers(_DEFAULT_TEST_HEADERS)
    for i in range(size):
        h[f"X-Test-{i}"] = f"v{i}"
    return h


def _build_headers_with_duplicates(size: int) -> ry.Headers:
    h = _build_headers(size)
    for i in range(size):
        h.append("set-cookie", f"cookie-{i}")
    return h


def _build_update_dict(size: int) -> dict[str, str]:
    return {f"X-Up-{i}": f"u{i}" for i in range(size)}


@pytest.mark.benchmark(group="init")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_init_from_dict(benchmark: BenchmarkFixture, size: int) -> None:
    benchmark(_build_headers, size)


@pytest.mark.benchmark(group="init")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_init_from_headers(
    benchmark: BenchmarkFixture, size: int
) -> None:
    h = _build_headers(size)
    benchmark(ry.Headers, h)


# =============================================================================
# READ
# =============================================================================


@pytest.mark.benchmark(group="headers-read")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_get(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.get, "accept")


@pytest.mark.benchmark(group="headers-read")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_contains(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.__contains__, "accept")


@pytest.mark.benchmark(group="headers-read")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_len(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.__len__)


@pytest.mark.benchmark(group="headers-read")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_keys_len(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.keys_len)


@pytest.mark.benchmark(group="headers-read")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_get_all(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers_with_duplicates(size)
    benchmark(h.get_all, "set-cookie")


# =============================================================================
# ITER
# =============================================================================


@pytest.mark.benchmark(group="headers-iter")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_keys(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.keys)


@pytest.mark.benchmark(group="headers-iter")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_values(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.values)


@pytest.mark.benchmark(group="headers-iter")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_to_dict(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    benchmark(h.to_dict)


@pytest.mark.benchmark(group="headers-iter")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_merge(benchmark: BenchmarkFixture, size: int) -> None:
    h = _build_headers(size)
    other = _build_headers(size)
    benchmark(lambda: h | other)


# =============================================================================
# BATCH/SETUP benchmarks
# =============================================================================


@pytest.mark.benchmark(group="headers-write")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_insert(benchmark: BenchmarkFixture, size: int) -> None:
    def _setup() -> tuple[tuple[ry.Headers, str, str], dict[str, object]]:
        h = _build_headers(size)
        return (h, "x-new-header", "value"), {}

    def _fn(h: ry.Headers, key: str, value: str) -> None:
        h[key] = value

    benchmark.pedantic(_fn, setup=_setup, rounds=50, iterations=1)


@pytest.mark.benchmark(group="headers-write")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_append(benchmark: BenchmarkFixture, size: int) -> None:
    def _setup() -> tuple[tuple[ry.Headers, str, str], dict[str, object]]:
        h = _build_headers(size)
        return (h, "set-cookie", "cookie-new"), {}

    def _fn(h: ry.Headers, key: str, value: str) -> None:
        h.append(key, value)

    benchmark.pedantic(_fn, setup=_setup, rounds=50, iterations=1)


@pytest.mark.benchmark(group="headers-write")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_update_dict(benchmark: BenchmarkFixture, size: int) -> None:
    update_dict = _build_update_dict(16)

    def _setup() -> tuple[tuple[ry.Headers, dict[str, str]], dict[str, object]]:
        h = _build_headers(size)
        return (h, update_dict), {}

    def _fn(h: ry.Headers, upd: dict[str, str]) -> None:
        h.update(upd)

    benchmark.pedantic(_fn, setup=_setup, rounds=50, iterations=1)


@pytest.mark.benchmark(group="headers-write")
@pytest.mark.parametrize("size", [8, 64])
def test_bench_headers_update_headers(benchmark: BenchmarkFixture, size: int) -> None:
    update_headers = ry.Headers(_build_update_dict(16))

    def _setup() -> tuple[tuple[ry.Headers, ry.Headers], dict[str, object]]:
        h = _build_headers(size)
        return (h, update_headers), {}

    def _fn(h: ry.Headers, other: ry.Headers) -> None:
        h.update(other)

    benchmark.pedantic(_fn, setup=_setup, rounds=50, iterations=1)
