"""
ulid tests (adapted from `python-ulid`)

ADAPTED FROM `python-ulid`'s TESTS; REF https://github.com/mdomke/python-ulid/blob/main/tests/test_ulid.py

removed freezegun as it is not really needed nor does pyo3 stuff respsect it
"""

from __future__ import annotations

import datetime as pydt
import typing as t
import uuid

import pytest

import ry
from ry.ulid import ULID

if t.TYPE_CHECKING:
    from collections.abc import Callable


def utcnow() -> pydt.datetime:
    return pydt.datetime.now(pydt.UTC)


def datetimes_almost_equal(a: pydt.datetime, b: pydt.datetime) -> None:
    dt = abs((a - b).total_seconds())
    assert dt < 0.01, (
        f"Expected {a} and {b} to be almost equal, but they differ by {dt} seconds"
    )


def test_ulid() -> None:
    ulid = ULID()
    now = pydt.datetime.now(pydt.UTC)
    t = now.timestamp()
    assert len(ulid.bytes) == 16
    assert len(str(ulid)) == (10 + 16)

    assert isinstance(ulid.to_uuid(), uuid.UUID) or isinstance(
        ulid.to_uuid4(), ry.uuid.UUID
    )

    assert isinstance(ulid.timestamp, float)
    assert ulid.timestamp == pytest.approx(t)

    assert isinstance(ulid.datetime, pydt.datetime)
    datetimes_almost_equal(ulid.datetime, now)


def test_ulid_repr() -> None:
    ulid = ULID()
    assert isinstance(repr(ulid), str)
    assert repr(ulid).startswith("ULID(")
    assert repr(ulid).endswith(")")
    # evaling the repr should return an equiv ULID
    assert eval(repr(ulid)) == ulid


@pytest.mark.parametrize("tick", [1, 60, 3600, 86400])
def test_ulid_monotonic_sorting(tick: int) -> None:
    def _gen() -> t.Generator[ULID, None, None]:
        initial_time = utcnow()
        for i in range(1, 11):
            dt = initial_time + pydt.timedelta(seconds=i * tick)
            yield ULID.from_datetime(dt)

    ulids = list(_gen())

    assert_sorted(ulids)
    assert_sorted([str(v) for v in ulids])
    assert_sorted([int(v) for v in ulids])
    assert_sorted([v.bytes for v in ulids])


def assert_sorted(seq: list[t.Any]) -> None:
    last = seq[0]
    for item in seq[1:]:
        assert last < item
        last = item


def test_comparison() -> None:
    now = utcnow()
    ulid1 = ULID.from_datetime(now)

    assert ulid1 == ulid1
    assert ulid1 == int(ulid1)
    assert ulid1 == ulid1.bytes
    assert ulid1 == str(ulid1)
    assert (ulid1 == object()) is False

    later = now + pydt.timedelta(milliseconds=1)
    ulid2 = ULID.from_datetime(later)

    assert ulid1 < ulid2
    assert ulid1 < int(ulid2)
    assert ulid1 < ulid2.bytes
    assert ulid1 < str(ulid2)
    with pytest.raises(TypeError):
        _ = ulid1 < object()  # type: ignore[operator]


def test_repr() -> None:
    ulid = ULID()
    assert f"ULID('{ulid!s}')" == repr(ulid)


def test_idempotency() -> None:
    ulid = ULID()
    assert ULID.from_bytes(ulid.bytes) == ulid
    assert ULID.from_str(str(ulid)) == ulid
    assert ULID.from_uuid(ulid.to_uuid()) == ulid
    assert ULID.from_int(int(ulid)) == ulid
    assert ULID.from_hex(ulid.hex) == ulid
    assert ULID.parse(ulid) == ulid
    assert ULID.parse(ulid.to_uuid()) == ulid
    assert ULID.parse(str(ulid.to_uuid())) == ulid
    assert ULID.parse(ulid.to_uuid().hex) == ulid
    assert ULID.parse(str(ulid)) == ulid
    assert ULID.parse(ulid.hex) == ulid
    assert ULID.parse(ulid.to_uuid().int) == ulid
    assert ULID.parse(ulid.milliseconds).milliseconds == ulid.milliseconds
    assert ULID.parse(ulid.timestamp).timestamp == ulid.timestamp
    assert ULID.parse(ulid.datetime).datetime == ulid.datetime
    assert ULID.parse(ulid.bytes) == ulid


def test_to_uuid4() -> None:
    ulid = ULID()
    uuid = ulid.to_uuid4()
    assert uuid.version == 4


def test_hash() -> None:
    ulid1 = ULID()
    ulid2 = ULID()
    assert isinstance(hash(ulid1), int)
    assert hash(ulid1) == hash(ulid1)
    assert hash(ulid1) == hash(ULID.from_bytes(ulid1.bytes))
    assert hash(ulid1) != hash(ulid2)


def test_ulid_from_time() -> None:
    now = utcnow()
    t = now.timestamp()
    t_ms = int(t * 1000)

    ulid1 = ULID.from_timestamp(t)
    ulid2 = ULID.from_timestamp(t_ms)
    ulid3 = ULID.from_datetime(now)

    assert ulid1.timestamp == pytest.approx(t)
    datetimes_almost_equal(ulid1.datetime, now)

    assert ulid2.timestamp == pytest.approx(t)
    datetimes_almost_equal(ulid2.datetime, now)

    assert ulid3.timestamp == pytest.approx(t)
    datetimes_almost_equal(ulid3.datetime, now)


def test_ulid_from_timestamp() -> None:
    ts = 1749067926.527876
    ulid1 = ULID.from_timestamp(ts)
    ulid2 = ULID.from_timestamp(int(ts * 1000))
    assert ulid1.timestamp == ulid2.timestamp


Params: t.TypeAlias = bytes | str | int | float


@pytest.mark.parametrize(
    ("constructor", "value"),
    [
        (ULID, b"sdf"),  # invalid length
        (ULID.from_timestamp, b"not-a-timestamp"),  # invalid type
        # NOTE [2025-06-18]:
        #   pytest-xdist freaks (the fuck) out collecting the tests if we call
        #   `time.time()`, so the "datetime" input param has been replaced with
        #   the ret-value of `time.time()` at the `time.time()` of writing this
        #   semi-incoherent comment
        #   PREV-VERSION: `(ULID.from_datetime, time.time())`,  # invalid type
        (ULID.from_datetime, 1750259472.6533403),  # invalid type
        (ULID.from_bytes, b"not-enough"),  # invalid length
        (ULID.from_bytes, 123),  # invalid type
        (ULID.from_str, "not-enough"),  # invalid length
        (ULID.from_str, 123),  # invalid type
        (ULID.from_str, "notavalidulidnotavalidulid"),  # invalid alphabet
        (ULID.from_str, "Z" * 26),  # invalid timestamp
        (ULID.from_int, "not-an-int"),  # invalid type
        (ULID.from_uuid, "not-a-uuid"),  # invalid type
        (ULID.parse, "not-a-uuid"),  # invalid length
        (ULID.parse, []),  # invalid type
    ],
)
def test_ulid_invalid_input(
    constructor: Callable[[Params], ULID], value: Params
) -> None:
    if value == "Z" * 26:
        # rs ulid doesn't throw here?
        try:
            constructor(value)
        except ValueError as e:
            msg = "Something upstream changed"
            raise AssertionError(msg) from e
        return
    with pytest.raises((ValueError, TypeError)):
        constructor(value)


@pytest.mark.parametrize(
    ("constructor", "value"),
    [
        (ULID, b"\x00" * 16),
        (ULID.from_timestamp, 0),
        (ULID.from_bytes, b"\x00" * 16),
        (ULID.from_str, "0" * 26),
        (ULID.from_hex, "0" * 32),
        (ULID.from_uuid, uuid.UUID("0" * 32)),
    ],
)
def test_ulid_min_input(constructor: Callable[[Params], ULID], value: Params) -> None:
    constructor(value)


@pytest.mark.parametrize(
    ("constructor", "value"),
    [
        (ULID, b"\xff" * 16),
        (ULID.from_timestamp, 281474976710655),
        (ULID.from_datetime, pydt.datetime.max.replace(tzinfo=pydt.UTC)),
        (ULID.from_bytes, b"\xff" * 16),
        (ULID.from_str, "7" + "Z" * 25),
        (ULID.from_hex, "f" * 32),
        (ULID.from_uuid, uuid.UUID("f" * 32)),
    ],
)
def test_ulid_max_input(constructor: Callable[[Params], ULID], value: Params) -> None:
    constructor(value)
