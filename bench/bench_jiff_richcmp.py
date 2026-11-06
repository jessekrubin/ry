"""jiff types richcmp benchmarks

# Notes

a hand-written `__richcmp__`/`__eq__` w/ `other: &Self` makes pyo3 build a
`TypeError` (+ `add_note`) on every failed extract, just to throw it away and
return `NotImplemented`. deriving the eq/richmp impls via `#[pyclass(eq, ord)]`
skips the `TypeError` and `add_note` overhead on failed type extractions.

`add_note` attaches to the GIL and calls into python to annotate the error.
"""

from __future__ import annotations

import dataclasses
import operator
import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_DT = ry.datetime(2020, 8, 26, 6, 27, 0, 0)


@dataclasses.dataclass(frozen=True, slots=True)
class _Eq:
    name: str
    a: object
    a_eq: object


@dataclasses.dataclass(frozen=True, slots=True)
class _EqOrd:
    name: str
    a: object
    a_eq: object  # equal to `a` but a different object
    gt: object  # > `a`


_EQ_ORD_CASES = (
    _EqOrd("Date", ry.date(2020, 8, 26), ry.date(2020, 8, 26), ry.date(2021, 1, 1)),
    _EqOrd("DateTime", _DT, ry.datetime(2020, 8, 26, 6, 27), ry.datetime(2021, 1, 1)),
    _EqOrd("Time", ry.time(6, 27), ry.time(6, 27), ry.time(7, 0)),
    _EqOrd(
        "Timestamp",
        ry.Timestamp.from_millisecond(1598438400000),
        ry.Timestamp.from_millisecond(1598438400000),
        ry.Timestamp.from_millisecond(1698438400000),
    ),
    _EqOrd(
        "ZonedDateTime",
        _DT.in_tz("America/New_York"),
        _DT.in_tz("America/New_York"),
        ry.datetime(2021, 1, 1).in_tz("America/New_York"),
    ),
    _EqOrd("Offset", ry.Offset(1), ry.Offset(1), ry.Offset(2)),
    _EqOrd(
        "SignedDuration",
        ry.SignedDuration(1, 1),
        ry.SignedDuration(1, 1),
        ry.SignedDuration(2, 0),
    ),
    _EqOrd(
        "ISOWeekDate",
        ry.date(2020, 8, 26).iso_week_date(),
        ry.date(2020, 8, 26).iso_week_date(),
        ry.date(2021, 1, 1).iso_week_date(),
    ),
)

_EQ_CASES = (
    _Eq("TimeZone", ry.TimeZone("America/New_York"), ry.TimeZone("America/New_York")),
    _Eq("TimeSpan", ry.timespan(hours=1), ry.timespan(hours=1)),
    _Eq("DateTimeRound", ry.DateTimeRound(), ry.DateTimeRound()),
    _Eq(
        "DateDifference",
        ry.DateDifference(ry.date(2020, 8, 26)),
        ry.DateDifference(ry.date(2020, 8, 26)),
    ),
)

_ALL = [pytest.param(c, id=c.name) for c in (*_EQ_ORD_CASES, *_EQ_CASES)]
_ORDERED = [pytest.param(c, id=c.name) for c in _EQ_ORD_CASES]


@pytest.mark.benchmark(group="jiff-eq-same")
@pytest.mark.parametrize("case", _ALL)
def test_eq_same(benchmark: BenchmarkFixture, case: _Eq) -> None:
    assert benchmark(operator.eq, case.a, case.a_eq) is True


@pytest.mark.benchmark(group="jiff-eq-none")
@pytest.mark.parametrize("case", _ALL)
def test_eq_none(benchmark: BenchmarkFixture, case: _Eq) -> None:
    assert benchmark(operator.eq, case.a, None) is False


@pytest.mark.benchmark(group="jiff-ne-str")
@pytest.mark.parametrize("case", _ALL)
def test_ne_str(benchmark: BenchmarkFixture, case: _Eq) -> None:
    assert benchmark(operator.ne, case.a, "nope") is True


@pytest.mark.benchmark(group="jiff-lt-same")
@pytest.mark.parametrize("case", _ORDERED)
def test_lt_same(benchmark: BenchmarkFixture, case: _EqOrd) -> None:
    assert benchmark(operator.lt, case.a, case.gt) is True


@pytest.mark.benchmark(group="jiff-in-mixed-list")
@pytest.mark.parametrize("case", _ALL)
def test_in_nonhomog_seq(benchmark: BenchmarkFixture, case: _Eq) -> None:
    haystack = [None, 1, "x", 1.5] * 25 + [case.a_eq]
    assert benchmark(operator.contains, haystack, case.a) is True
