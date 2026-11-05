"""uuid benchmarks"""

from __future__ import annotations

import operator
import pickle
import random
import uuid as pyuuid
from typing import TYPE_CHECKING

import pytest

import ry.uuid as ryuuid

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_UUID_STR = "550e8400-e29b-41d4-a716-446655440000"
_RY = ryuuid.UUID(_UUID_STR)
_PY = pyuuid.UUID(_UUID_STR)

# `format` / `__format__` are new ~ skip on older builds so this file still
# runs against a "before" build
_HAS_FORMAT = hasattr(ryuuid.UUID, "format")
_needs_format = pytest.mark.skipif(
    not _HAS_FORMAT, reason="ry.UUID.format missing (old build)"
)

_LIBS = [
    pytest.param(ryuuid, id="ry"),
    pytest.param(pyuuid, id="stdlib"),
]
_OBJS = [
    pytest.param(_RY, id="ry"),
    pytest.param(_PY, id="stdlib"),
]


# =============================================================================
# generation
# =============================================================================


@pytest.mark.benchmark(group="uuid-gen-uuid4")
@pytest.mark.parametrize("lib", _LIBS)
def test_uuid4(benchmark: BenchmarkFixture, lib: object) -> None:
    benchmark(lib.uuid4)  # type: ignore[attr-defined]


@pytest.mark.benchmark(group="uuid-gen-uuid7")
@pytest.mark.parametrize("lib", _LIBS)
def test_uuid7(benchmark: BenchmarkFixture, lib: object) -> None:
    if not hasattr(lib, "uuid7"):
        pytest.skip("no uuid7 (stdlib < 3.14)")
    benchmark(lib.uuid7)  # type: ignore[attr-defined]


@pytest.mark.benchmark(group="uuid-gen-uuid8")
def test_uuid8_random_ry(benchmark: BenchmarkFixture) -> None:
    # no args => random a/b/c via `rand::rng()`
    benchmark(ryuuid.uuid8)


@pytest.mark.benchmark(group="uuid-gen-uuid8")
def test_uuid8_random_stdlib(benchmark: BenchmarkFixture) -> None:
    if not hasattr(pyuuid, "uuid8"):
        pytest.skip("no uuid8 (stdlib < 3.14)")
    benchmark(pyuuid.uuid8)  # type: ignore[attr-defined]


# =============================================================================
# stringify
# =============================================================================


@pytest.mark.benchmark(group="uuid-str")
@pytest.mark.parametrize("u", _OBJS)
def test_str(benchmark: BenchmarkFixture, u: object) -> None:
    benchmark(str, u)


@pytest.mark.benchmark(group="uuid-str")
def test_to_string_ry(benchmark: BenchmarkFixture) -> None:
    benchmark(_RY.to_string)


@pytest.mark.benchmark(group="uuid-repr")
@pytest.mark.parametrize("u", _OBJS)
def test_repr(benchmark: BenchmarkFixture, u: object) -> None:
    benchmark(repr, u)


@pytest.mark.benchmark(group="uuid-hex")
@pytest.mark.parametrize("u", _OBJS)
def test_hex(benchmark: BenchmarkFixture, u: pyuuid.UUID) -> None:
    benchmark(lambda: u.hex)


@pytest.mark.benchmark(group="uuid-urn")
@pytest.mark.parametrize("u", _OBJS)
def test_urn(benchmark: BenchmarkFixture, u: pyuuid.UUID) -> None:
    benchmark(lambda: u.urn)


# =============================================================================
# format method + f-string dunder (new)
# =============================================================================

_FORMATS = ["hyphenated", "simple", "urn", "braced"]


@_needs_format
@pytest.mark.benchmark(group="uuid-format-method")
@pytest.mark.parametrize("fmt", _FORMATS)
def test_format_method_ry(benchmark: BenchmarkFixture, fmt: str) -> None:
    benchmark(_RY.format, fmt)  # type: ignore[attr-defined]


@_needs_format
@pytest.mark.benchmark(group="uuid-format-method")
def test_format_method_default_ry(benchmark: BenchmarkFixture) -> None:
    benchmark(_RY.format)  # type: ignore[attr-defined]


@pytest.mark.benchmark(group="uuid-format-method")
def test_format_braced_stdlib(benchmark: BenchmarkFixture) -> None:
    benchmark(lambda: "{" + str(_PY) + "}")


@pytest.mark.benchmark(group="uuid-fstring-empty")
@pytest.mark.parametrize("u", _OBJS)
def test_fstring_empty(benchmark: BenchmarkFixture, u: object) -> None:
    benchmark(format, u, "")


@_needs_format
@pytest.mark.benchmark(group="uuid-fstring-spec")
@pytest.mark.parametrize("fmt", _FORMATS)
def test_fstring_spec_ry(benchmark: BenchmarkFixture, fmt: str) -> None:
    benchmark(format, _RY, fmt)


# =============================================================================
# from_any
# =============================================================================

_FROM_ANY_INPUTS = [
    pytest.param(_RY, id="ry-uuid"),
    pytest.param(_PY, id="stdlib-uuid"),
    pytest.param(_UUID_STR, id="str"),
    pytest.param(_UUID_STR.encode(), id="bytes-str"),
    pytest.param(_PY.bytes, id="bytes-16"),
]


@pytest.mark.benchmark(group="uuid-from-any")
@pytest.mark.parametrize("value", _FROM_ANY_INPUTS)
def test_from_any(benchmark: BenchmarkFixture, value: object) -> None:
    benchmark(ryuuid.UUID.from_any, value)  # type: ignore[attr-defined]


# =============================================================================
# pickle (`__getnewargs__`)
# =============================================================================


@pytest.mark.benchmark(group="uuid-pickle-dumps")
@pytest.mark.parametrize("u", _OBJS)
def test_pickle_dumps(benchmark: BenchmarkFixture, u: object) -> None:
    benchmark(pickle.dumps, u)


@pytest.mark.benchmark(group="uuid-pickle-roundtrip")
@pytest.mark.parametrize("u", _OBJS)
def test_pickle_roundtrip(benchmark: BenchmarkFixture, u: object) -> None:
    benchmark(lambda: pickle.loads(pickle.dumps(u)))  # noqa: S301


# =============================================================================
# richcmp
# =============================================================================

_RY2 = ryuuid.UUID(_UUID_STR)  # same value, different object
_PY2 = pyuuid.UUID(_UUID_STR)
_RY_HI = ryuuid.UUID("ffffffff-ffff-4fff-bfff-ffffffffffff")

_CMP_CASES = [
    pytest.param(operator.eq, _RY, _RY2, id="eq-ry-ry"),
    pytest.param(operator.ne, _RY, _RY2, id="ne-ry-ry"),
    pytest.param(operator.lt, _RY, _RY_HI, id="lt-ry-ry"),
    pytest.param(operator.eq, _RY, _PY, id="eq-ry-stdlib"),
    pytest.param(operator.lt, _RY, _PY, id="lt-ry-stdlib"),
    pytest.param(operator.eq, _PY, _PY2, id="eq-stdlib-stdlib"),
    pytest.param(operator.lt, _PY, _PY2, id="lt-stdlib-stdlib"),
    pytest.param(operator.eq, _RY, None, id="eq-ry-none"),
    pytest.param(operator.eq, _RY, 1, id="eq-ry-int"),
    pytest.param(operator.eq, _PY, None, id="eq-stdlib-none"),
]


@pytest.mark.benchmark(group="uuid-richcmp")
@pytest.mark.parametrize(("op", "a", "b"), _CMP_CASES)
def test_richcmp(
    benchmark: BenchmarkFixture,
    op: object,
    a: object,
    b: object,
) -> None:
    try:
        op(a, b)  # type: ignore[operator]
    except TypeError as e:
        # old builds raise on `ry_uuid == <not a uuid>`
        pytest.skip(f"raises: {e}")
    benchmark(op, a, b)


_rng = random.Random(42)  # noqa: S311
_INTS = [_rng.getrandbits(128) for _ in range(1_000)]


@pytest.mark.benchmark(group="uuid-sorted-1000")
@pytest.mark.parametrize("lib", _LIBS)
def test_sorted_1000(benchmark: BenchmarkFixture, lib: object) -> None:
    data = [lib.UUID(int=i) for i in _INTS]  # type: ignore[attr-defined]
    benchmark(sorted, data)


@pytest.mark.benchmark(group="uuid-in-list-1000-miss")
@pytest.mark.parametrize("lib", _LIBS)
def test_in_list_1000_miss(benchmark: BenchmarkFixture, lib: object) -> None:
    data = [lib.UUID(int=i) for i in _INTS]  # type: ignore[attr-defined]
    needle = lib.UUID(int=0)  # type: ignore[attr-defined]
    benchmark(operator.contains, data, needle)
