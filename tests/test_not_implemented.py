"""`__eq__`/binary ops return `NotImplemented` for unknown types (no `TypeError` built + discarded)"""

from __future__ import annotations

import dataclasses
import datetime as pydt
import typing as t

import pytest

import ry

UNKNOWN = [None, "x", 1.5j, object()]
# `str` is a valid `/` operand for `FsPath`/`URL`
UNKNOWN_BINOP = [None, 1.5j, object()]
_INSTANT = ry.Instant.now()

_LOCALHOST_V4 = ry.Ipv4Addr(127, 0, 0, 1)


_T = t.TypeVar("_T")


@dataclasses.dataclass(frozen=True)
class _EqTestCase(t.Generic[_T]):
    build: t.Callable[[], _T]  # called twice ~ the 2 results must be equal
    ordered: bool = False  # supports `<`/`<=`/... (vs same type only)


_EQ_CASES: dict[str, _EqTestCase[t.Any]] = {
    "regex": _EqTestCase(lambda: ry.Regex(r"a+")),
    "glob": _EqTestCase(lambda: ry.Glob("*.py")),
    "glob_pattern": _EqTestCase(lambda: ry.GlobPattern("*.py")),
    "duration": _EqTestCase(lambda: ry.Duration(1, 5), ordered=True),
    "instant": _EqTestCase(lambda: _INSTANT, ordered=True),
    "socket_addr_v4": _EqTestCase(
        lambda: ry.SocketAddrV4(_LOCALHOST_V4, 80), ordered=True
    ),
    "socket_addr": _EqTestCase(lambda: ry.SocketAddr(_LOCALHOST_V4, 80), ordered=True),
    "ipv4_addr": _EqTestCase(lambda: ry.Ipv4Addr(127, 0, 0, 1), ordered=True),
    "ipv6_addr": _EqTestCase(lambda: ry.Ipv6Addr("::1"), ordered=True),
    "ip_addr": _EqTestCase(lambda: ry.IpAddr(_LOCALHOST_V4), ordered=True),
    "url": _EqTestCase(lambda: ry.URL("https://example.com/a"), ordered=True),
    "fs_path": _EqTestCase(lambda: ry.FsPath("a/b"), ordered=True),
    "headers": _EqTestCase(lambda: ry.Headers({"a": "b"})),
    "cookie": _EqTestCase(lambda: ry.Cookie("a", "b")),
    "sql_formatter": _EqTestCase(ry.SqlFormatter),
    "http_status": _EqTestCase(lambda: ry.HttpStatus(200), ordered=True),
}


@pytest.mark.parametrize("case", _EQ_CASES.values(), ids=_EQ_CASES.keys())
@pytest.mark.parametrize("other", UNKNOWN)
def test_eq_unknown_types(case: _EqTestCase[t.Any], other: object) -> None:
    a, b = case.build(), case.build()
    assert a == b
    assert not (a != b)
    assert a.__eq__(other) is NotImplemented  # noqa: PLC2801
    assert a != other
    assert not (a == other)
    if case.ordered:
        assert a <= b
        with pytest.raises(TypeError):
            _ = a < other


_OpDunder: t.TypeAlias = t.Literal[
    "__add__",
    "__radd__",
    "__sub__",
    "__rsub__",
    "__mul__",
    "__rmul__",
    "__truediv__",
    "__rtruediv__",
    "__or__",
]


@dataclasses.dataclass(frozen=True)
class _DunderOpsTestCase:
    obj: object
    ops: tuple[_OpDunder, ...]


_MATHS: tuple[_OpDunder, ...] = (
    "__add__",
    "__radd__",
    "__sub__",
    "__rsub__",
    "__mul__",
    "__rmul__",
    "__truediv__",
)
_DUNDER_OPS_CASES: dict[str, _DunderOpsTestCase] = {
    "duration": _DunderOpsTestCase(ry.Duration(1, 5), _MATHS),
    "signed_duration": _DunderOpsTestCase(ry.SignedDuration(5, 0), _MATHS),
    "instant": _DunderOpsTestCase(ry.Instant.now(), ("__add__", "__radd__", "__sub__")),
    "date": _DunderOpsTestCase(ry.date(2024, 1, 1), ("__add__", "__radd__", "__sub__")),
    "datetime": _DunderOpsTestCase(ry.datetime(2024, 1, 1, 1), ("__add__", "__sub__")),
    "time": _DunderOpsTestCase(ry.time(1, 2), ("__add__", "__sub__")),
    "timestamp": _DunderOpsTestCase(ry.Timestamp.now(), ("__add__", "__sub__")),
    "zoned_datetime": _DunderOpsTestCase(
        ry.ZonedDateTime.now(), ("__add__", "__sub__")
    ),
    "offset": _DunderOpsTestCase(ry.Offset(1), ("__add__", "__sub__")),
    "timespan": _DunderOpsTestCase(
        ry.timespan(days=1), ("__add__", "__sub__", "__mul__")
    ),
    "fs_path": _DunderOpsTestCase(ry.FsPath("a"), ("__truediv__", "__rtruediv__")),
    "url": _DunderOpsTestCase(
        ry.URL("https://x.com/a/"), ("__truediv__", "__rtruediv__")
    ),
    "headers": _DunderOpsTestCase(ry.Headers({"a": "b"}), ("__or__",)),
}


def _dunder_ops_params() -> list[t.Any]:
    return [
        pytest.param(case.obj, op, id=f"{name}-{op}")
        for name, case in _DUNDER_OPS_CASES.items()
        for op in case.ops
    ]


@pytest.mark.parametrize(("obj", "op"), _dunder_ops_params())
@pytest.mark.parametrize("other", UNKNOWN_BINOP)
def test_binop_unknown_types(obj: object, op: str, other: object) -> None:
    assert getattr(obj, op)(other) is NotImplemented


def test_binop_supported_still_work() -> None:
    sd: t.Any = ry.SignedDuration(5, 0)
    dur: t.Any = ry.Duration(2)
    assert sd * 2 == ry.SignedDuration(10, 0)
    assert sd * 2.5 == ry.SignedDuration(12, 500_000_000)
    assert 2 * sd == sd * 2
    assert sd / 2 == ry.SignedDuration(2, 500_000_000)
    assert dur * 2.5 == ry.Duration(5)
    assert dur / 2 == ry.Duration(1)
    assert dur + pydt.timedelta(seconds=1) == ry.Duration(3)
    assert pydt.timedelta(seconds=1) + dur == ry.Duration(3)
    assert ry.timespan(days=1) * 2 == ry.timespan(days=2)
    assert ry.date(2024, 1, 1) + ry.timespan(days=1) == ry.date(2024, 1, 2)
    assert ry.timespan(days=1) + ry.timespan(days=1) == ry.timespan(days=2)
    assert ry.FsPath("a") / "b" == ry.FsPath("a/b")
    assert "a" / ry.FsPath("b") == ry.FsPath("a/b")
    assert ry.URL("https://x.com/a/") / "b" == ry.URL("https://x.com/a/b")
    assert (ry.Headers({"a": "b"}) | {"c": "d"})["c"] == "d"
    # right type, bad value: still unsupported (pyo3 already did this)
    with pytest.raises(TypeError):
        _ = dur * -1
