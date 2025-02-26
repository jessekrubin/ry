from __future__ import annotations

import dataclasses
import typing as t

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import ry.dev as ry

ALNUM_BYTES = bytes([i for i in range(256) if bytes([i]).isalnum()])
ALPHA_BYTES = bytes([i for i in range(256) if bytes([i]).isalpha()])
ASCII_BYTES = bytes([i for i in range(256) if bytes([i]).isascii()])
DIGIT_BYTES = bytes([i for i in range(256) if bytes([i]).isdigit()])
LOWER_BYTES = bytes([i for i in range(256) if bytes([i]).islower()])
SPACE_BYTES = bytes([i for i in range(256) if bytes([i]).isspace()])
UPPER_BYTES = bytes([i for i in range(256) if bytes([i]).isupper()])
ALL_BYTES = bytes(list(range(256)))
ALL_BYTES_10X = ALL_BYTES * 10
ALL_BYTES_100X = ALL_BYTES * 100


def test_benchmark_not_debug() -> None:
    assert ry.__build_profile__ == "release"


@dataclasses.dataclass
class BytesSumFn:
    id: str
    fn: t.Callable[[bytes | ry.Bytes], int]
    accepts_py_bytes: bool = False
    """True if the function accepts python bytes"""
    accepts_ry_bytes: bool = False
    """True if the function accepts ry.Bytes"""

    @classmethod
    def from_fn(
        cls,
        id: str,
        fn: t.Callable[[bytes | ry.Bytes], int],
        py_bytes: bool = False,
        ry_bytes: bool = False,
    ):
        return cls(id=id, fn=fn, accepts_py_bytes=py_bytes, accepts_ry_bytes=ry_bytes)


@dataclasses.dataclass
class BytesSumData:
    id: str
    py_bytes: bytes = dataclasses.field(repr=False)
    ry_bytes: ry.Bytes = dataclasses.field(repr=False)
    expected_sum: int = dataclasses.field()

    @classmethod
    def from_bytes(cls, data: bytes, id: str):
        expected = sum(data)
        return cls(id=id, py_bytes=data, ry_bytes=ry.Bytes(data), expected_sum=expected)


@dataclasses.dataclass
class BytesSumTest:
    id: str
    data: BytesSumData
    fn_info: BytesSumFn
    fn: t.Callable[[bytes | ry.Bytes], int]
    b: bytes | ry.Bytes = dataclasses.field(repr=False)

    @classmethod
    def from_data_fn(cls, data: BytesSumData, fn: BytesSumFn) -> list[BytesSumTest]:
        tests = []
        if fn.accepts_py_bytes:
            tests.append(
                cls(
                    id=f"{data.id}::py::{fn.id}",
                    data=data,
                    b=data.py_bytes,
                    fn_info=fn,
                    fn=fn.fn,
                )
            )
        if fn.accepts_ry_bytes:
            tests.append(
                cls(
                    id=f"{data.id}::rust::{fn.id}",
                    data=data,
                    b=data.ry_bytes,
                    fn_info=fn,
                    fn=fn.fn,
                )
            )
        return tests


def python_bytes_sum(b: bytes):
    return sum(b)


BENCH_FUNCTIONS = [
    BytesSumFn.from_fn(
        id="python_bytes_sum",
        fn=python_bytes_sum,
        py_bytes=True,
    ),
    BytesSumFn.from_fn(
        id="ry.bytes_sum_pybytes",
        fn=ry.bytes_sum_pybytes,
        py_bytes=True,
    ),
    BytesSumFn.from_fn(
        id="ry.bytes_sum_rybytes",
        fn=ry.bytes_sum_rybytes,
        ry_bytes=True,
        py_bytes=True,
    ),
    BytesSumFn.from_fn(
        id="ry.bytes_sum_rybytes_ref",
        fn=ry.bytes_sum_rybytes_ref,
        ry_bytes=True,
    ),
    BytesSumFn.from_fn(
        id="ry.bytes_sum_bytes_like",
        fn=ry.bytes_sum_bytes_like,
        ry_bytes=True,
        py_bytes=True,
    ),
]

BENCH_DATA = [
    # BytesSumData.from_bytes(b"", "empty"),
    # BytesSumData.from_bytes(b"abc", "abc"),
    BytesSumData.from_bytes(ALL_BYTES, "all-bytes-1x"),  # all bytes
    # byte_types
    # BytesSumData.from_bytes(ALNUM_BYTES, "alnum"),
    # BytesSumData.from_bytes(ALPHA_BYTES, "alpha"),
    # BytesSumData.from_bytes(ASCII_BYTES, "ascii"),
    # BytesSumData.from_bytes(DIGIT_BYTES, "digit"),
    # BytesSumData.from_bytes(LOWER_BYTES, "lower"),
    # BytesSumData.from_bytes(SPACE_BYTES, "space"),
    # BytesSumData.from_bytes(UPPER_BYTES, "upper"),
    # all bytes large
    BytesSumData.from_bytes(ALL_BYTES_10X, "all-bytes-10x"),
    BytesSumData.from_bytes(ALL_BYTES_100X, "all-bytes-100x"),
]

BENCH_TESTS = [
    test
    for tests in (
        BytesSumTest.from_data_fn(data, fn)
        for data in BENCH_DATA
        for fn in BENCH_FUNCTIONS
    )
    for test in tests
]


@pytest.mark.parametrize(
    "bench_data", [pytest.param(test, id=test.id) for test in BENCH_DATA]
)
def test_bench_sum_same(bench_data: BytesSumData) -> None:
    """Check that the sum of bytes is the same for all functions"""
    tests = [
        test
        for tests in (
            BytesSumTest.from_data_fn(bench_data, fn) for fn in BENCH_FUNCTIONS
        )
        for test in tests
    ]
    results = {}
    for test in tests:
        fn_res = test.fn(test.b)
        assert fn_res == bench_data.expected_sum
        results[test.id] = fn_res

    assert len(set(results.values())) == 1


# here are the actual benchmarks
@pytest.mark.parametrize(
    "bench_test", [pytest.param(test, id=test.id) for test in BENCH_TESTS]
)
def test_bench_sum(bench_test: BytesSumTest, benchmark: BenchmarkFixture) -> None:
    """Check that the sum of bytes is the same for all functions"""

    def __inner_bench(fn, b):
        assert fn(b) == bench_test.data.expected_sum

    benchmark.group = bench_test.data.id
    benchmark(__inner_bench, bench_test.fn, bench_test.b)
