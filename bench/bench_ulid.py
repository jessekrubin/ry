"""ulid benchmarks"""

from __future__ import annotations

import operator
import random
import typing as t
import uuid as pyuuid

import pytest

from ry import ULID

if t.TYPE_CHECKING:
    from collections.abc import Callable

    from pytest_benchmark.fixture import BenchmarkFixture

_U = ULID.from_int(0x0196_2A8F_5B3C_7D1E_9F00_1122_3344_5566)
_U2 = ULID.from_int(int(_U))
_HI = ULID.MAX
_STR = str(_U)
_HEX = _U.hex
_BYTES = _U.bytes
_INT = int(_U)
_UUID_STR = str(_U.to_uuid())
_STDLIB_UUID = pyuuid.UUID(_UUID_STR)

_rng = random.Random(42)  # noqa: S311
_INTS = [_rng.getrandbits(128) for _ in range(1000)]
_DATA = [ULID.from_int(i) for i in _INTS]
_STRS = [str(u) for u in _DATA]
_NEEDLE = ULID.from_int(0)


# =============================================================================
# creation
# =============================================================================


@pytest.mark.benchmark(group="ulid-new")
def test_new_gen(benchmark: BenchmarkFixture) -> None:
    benchmark(ULID)


@pytest.mark.benchmark(group="ulid-new")
@pytest.mark.parametrize(
    "arg",
    [
        pytest.param(_STR, id="str"),
        pytest.param(_HEX, id="hex"),
        pytest.param(_BYTES, id="bytes"),
    ],
)
def test_new(benchmark: BenchmarkFixture, arg: object) -> None:
    benchmark(ULID, arg)


@pytest.mark.benchmark(group="ulid-from")
@pytest.mark.parametrize(
    ("fn", "arg"),
    [
        pytest.param(ULID.from_str, _STR, id="from_str(str)"),
        pytest.param(ULID.from_str, _HEX, id="from_str(hex)"),
        pytest.param(ULID.from_hex, _HEX, id="from_hex"),
        pytest.param(ULID.from_bytes, _BYTES, id="from_bytes"),
        pytest.param(ULID.from_int, _INT, id="from_int"),
        pytest.param(ULID.from_uuid, _STDLIB_UUID, id="from_uuid(stdlib)"),
    ],
)
def test_from(
    benchmark: BenchmarkFixture, fn: Callable[[object], ULID], arg: object
) -> None:
    benchmark(fn, arg)


@pytest.mark.benchmark(group="ulid-parse")
@pytest.mark.parametrize(
    "arg",
    [
        pytest.param(_U, id="ulid"),
        pytest.param(_STR, id="str"),
        pytest.param(_HEX, id="hex"),
        pytest.param(_UUID_STR, id="uuid-str"),
        pytest.param(_BYTES, id="bytes"),
        pytest.param(_INT, id="int"),
        pytest.param(_STDLIB_UUID, id="stdlib-uuid"),
    ],
)
def test_parse(benchmark: BenchmarkFixture, arg: object) -> None:
    benchmark(ULID.parse, arg)


# =============================================================================
# conversions
# =============================================================================


@pytest.mark.benchmark(group="ulid-to")
@pytest.mark.parametrize(
    "fn",
    [
        pytest.param(str, id="str"),
        pytest.param(repr, id="repr"),
        pytest.param(operator.attrgetter("hex"), id="hex"),
        pytest.param(int, id="int"),
        pytest.param(operator.attrgetter("bytes"), id="bytes"),
        pytest.param(operator.attrgetter("timestamp"), id="timestamp"),
        pytest.param(ULID.to_uuid, id="to_uuid"),
    ],
)
def test_to(benchmark: BenchmarkFixture, fn: Callable[[ULID], object]) -> None:
    benchmark(fn, _U)


@pytest.mark.benchmark(group="ulid-hash")
def test_hash(benchmark: BenchmarkFixture) -> None:
    benchmark(hash, _U)


# =============================================================================
# richcmp
# =============================================================================


@pytest.mark.benchmark(group="ulid-eq")
@pytest.mark.parametrize(
    "other",
    [
        pytest.param(_U2, id="ulid"),
        pytest.param(_INT, id="int"),
        pytest.param(_STR, id="str"),
        pytest.param(_BYTES, id="bytes"),
        pytest.param(None, id="None"),
        pytest.param(_STDLIB_UUID, id="stdlib-uuid"),
    ],
)
def test_eq(benchmark: BenchmarkFixture, other: object) -> None:
    benchmark(operator.eq, _U, other)


@pytest.mark.benchmark(group="ulid-lt")
@pytest.mark.parametrize(
    "other",
    [
        pytest.param(_HI, id="ulid"),
    ],
)
def test_lt(benchmark: BenchmarkFixture, other: object) -> None:
    benchmark(operator.lt, _U, other)


@pytest.mark.benchmark(group="ulid-bulk")
def test_sorted_1000(benchmark: BenchmarkFixture) -> None:
    benchmark(sorted, _DATA)


@pytest.mark.benchmark(group="ulid-bulk")
def test_set_1000(benchmark: BenchmarkFixture) -> None:
    benchmark(set, _DATA)


@pytest.mark.benchmark(group="ulid-bulk")
def test_in_list_1000_miss(benchmark: BenchmarkFixture) -> None:
    benchmark(operator.contains, _DATA, _NEEDLE)


@pytest.mark.benchmark(group="ulid-bulk")
def test_str_1000(benchmark: BenchmarkFixture) -> None:
    benchmark(lambda: [str(u) for u in _DATA])


@pytest.mark.benchmark(group="ulid-bulk")
def test_from_str_1000(benchmark: BenchmarkFixture) -> None:
    benchmark(lambda: [ULID.from_str(s) for s in _STRS])


# `equiv` is new ~ skip on older builds
@pytest.mark.benchmark(group="ulid-equiv")
@pytest.mark.skipif(not hasattr(ULID, "equiv"), reason="ULID.equiv missing (old build)")
@pytest.mark.parametrize(
    "other",
    [
        pytest.param(_U2, id="ulid"),
        pytest.param(_INT, id="int"),
        pytest.param(_STR, id="str"),
        pytest.param(_HEX, id="hex"),
        pytest.param(_UUID_STR, id="uuid-str"),
        pytest.param(_BYTES, id="bytes"),
        pytest.param(_STDLIB_UUID, id="stdlib-uuid"),
        pytest.param(None, id="None"),
    ],
)
def test_equiv(benchmark: BenchmarkFixture, other: object) -> None:
    benchmark(_U.equiv, other)
