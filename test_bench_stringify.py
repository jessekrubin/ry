from __future__ import annotations

import json

import orjson
import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import ry.dev as ry

dataog = {
    "name": "ry",
    "version": "0.0.0",
    "description": "ry jawascript/interfacescript tools",
    "main": "index.js",
    "scripts": {"test": 'echo "Error: no test specified" && exit 1'},
    "keywords": [],
    "author": "jesse rubin <jessekrubin@gmail.com>",
    "license": "MIT OR Apache-2.0",
    "devDependencies": {
        "dprint": "^0.49.1",
        "prettier": "^3.5.3",
        "pyright": "^1.1.396",
    },
}


def test_profile():
    assert ry.__build_profile__ == "release"


data = ry.read_json("D:\\dgts\\rush.json")
data = [
    data,
    dataog,
    list(range(1000)),
]


# data = {
#     "name": "ry",
#     "version": "0.1.0",
# }
# r = ry.stringify( data)
# print(r)
def py_stringify(data):
    return json.dumps(data, indent=2, separators=(",", ":"), sort_keys=True)


def oj_stringify(data):
    return orjson.dumps(data).decode("utf-8")


@pytest.mark.benchmark(group="stringify")
def test_python_stdlib(benchmark: BenchmarkFixture):
    benchmark(lambda: py_stringify(data))


@pytest.mark.benchmark(group="stringify")
def test_ry_stringify(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.stringify_v1(data))


@pytest.mark.benchmark(group="stringify")
def test_ry_stringify_v5(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.stringify_v5(data))


# @pytest.mark.benchmark(group="stringify")
@pytest.mark.benchmark(group="stringify")
def test_ry_stringify_v3(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.stringify_v3(data))


# def test_ry_stringifyv2(benchmark: BenchmarkFixture):
#     benchmark(lambda: ry.stringifyv2(data))


# @pytest.mark.benchmark(group="stringify")
@pytest.mark.benchmark(group="stringify")
def test_ry_stringify_v4(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.stringify_v4(data))


# def test_ry_stringifyv2(benchmark: BenchmarkFixture):


@pytest.mark.benchmark(group="stringify")
def test_orjson(benchmark: BenchmarkFixture):
    benchmark(lambda: oj_stringify(data))


if __name__ == "__main__":
    print(ry.stringify_v5(data).decode())
