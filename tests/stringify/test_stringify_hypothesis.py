from __future__ import annotations

import json
import typing as t

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

from ..strategies import st_json_js

_ORJSON_INSTALLED: bool = False
try:
    import orjson

    _ORJSON_INSTALLED = True
except ImportError:
    orjson = None  # type: ignore[assignment]

_pytest_mark_skip_orjson = pytest.mark.skipif(
    not _ORJSON_INSTALLED,
    reason="orjson is not installed, skipping tests that require it",
)
_pytest_mark_fmt = pytest.mark.parametrize("fmt", [False, True], ids=["compact", "fmt"])


def py_stringify(data: t.Any, *, fmt: bool = False) -> bytes:
    """Convert data to a JSON string using Python's built-in json module."""
    if fmt:
        return json.dumps(data, indent=2).encode()
    return json.dumps(data, separators=(",", ":")).encode()


def ry_stringify(
    data: t.Any,
    *,
    fmt: bool = False,
    pybytes: bool = False,
    append_newline: bool = False,
) -> ry.Bytes | bytes:
    """Convert data to a JSON string using `ry.stringify`."""
    return ry.stringify_v2(
        data, fmt=fmt, pybytes=pybytes, append_newline=append_newline
    )


def oj_stringify(data: t.Any, *, fmt: bool = False) -> bytes:
    """Convert data to a JSON string using orjson."""
    assert orjson is not None, "orjson is not installed"
    return orjson.dumps(data, option=orjson.OPT_INDENT_2 if fmt else None)


# =============================================================================
# stdlib json
# =============================================================================


@_pytest_mark_fmt
@given(data=st_json_js())
def test_stringify_json(*, data: t.Any, fmt: bool) -> None:
    """Test that `ry.stringify` produces valid JSON equivalent to `json.dumps`."""
    ry_res = ry_stringify(data, fmt=fmt)
    assert isinstance(ry_res, ry.Bytes), "Result should be a `ry.Bytes`"
    assert json.loads(ry_res.decode()) == json.loads(py_stringify(data, fmt=fmt))
    assert ry.parse_json(ry_res) == ry.parse_json(py_stringify(data, fmt=fmt))


# =============================================================================
# fmt
# =============================================================================


@given(data=st_json_js(finite_only=False))
def test_stringify_fmt_same_data_as_compact(data: t.Any) -> None:
    """Test that `fmt=True` only changes whitespace, not the data."""
    ry_compact = ry_stringify(data)
    ry_fmt = ry_stringify(data, fmt=True)
    assert ry.parse_json(ry_fmt) == ry.parse_json(ry_compact)
    assert ry.JSON.minify(ry_fmt) == ry_compact
    assert ry.JSON.fmt(ry_compact) == ry_fmt


@given(data=st_json_js(finite_only=False))
def test_stringify_fmt_whitespace(data: t.Any) -> None:
    """Test that `fmt=True` output has no trailing whitespace/newline."""
    ry_fmt = ry_stringify(data, fmt=True, pybytes=True)
    assert ry_fmt == ry_fmt.strip()
    assert (
        ry_stringify(data, fmt=True, pybytes=True, append_newline=True)
        == ry_fmt + b"\n"
    )
    if isinstance(data, (list, dict)) and data:
        assert ry_fmt[:2] in {b"[\n", b"{\n"}
        assert ry_fmt[-2:] in {b"\n]", b"\n}"}
    else:
        assert b"\n" not in ry_fmt


# =============================================================================
# orjson
# =============================================================================
def st_json_oj() -> st.SearchStrategy[t.Any]:
    """JSON-able data for which ry/orjson output is byte-for-byte identical."""
    return st.recursive(
        st.none()
        | st.booleans()
        | st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX)
        | st.floats(min_value=-1e15, max_value=1e15)
        | st.sampled_from([float("nan"), float("inf"), float("-inf")])
        | st.text()
        | st.dates(),
        extend=lambda xs: st.lists(xs) | st.dictionaries(st.text(), xs),
    )


@_pytest_mark_fmt
@_pytest_mark_skip_orjson
@given(data=st_json_oj())
def test_stringify_orjson_identical(*, data: t.Any, fmt: bool) -> None:
    assert ry_stringify(data, fmt=fmt, pybytes=True) == oj_stringify(data, fmt=fmt)


@_pytest_mark_skip_orjson
@given(data=st.floats())
def test_stringify_orjson_floats(data: float) -> None:
    """Test floats are identical aside from the `+` in positive exponents."""
    ry_json = ry_stringify(data, pybytes=True)
    assert ry_json.replace(b"e+", b"e") == oj_stringify(data)


@_pytest_mark_skip_orjson
@given(data=st.integers(min_value=2**63, max_value=2**64 - 1))
def test_stringify_orjson_u64(data: int) -> None:
    """Test ints in `(i64::MAX, u64::MAX]` are identical."""
    assert ry_stringify(data, pybytes=True) == oj_stringify(data)


@_pytest_mark_skip_orjson
@given(data=st.datetimes())
def test_stringify_orjson_datetimes(data: t.Any) -> None:
    """Test orjson/ry.stringify for datetimes."""
    ry_json = ry_stringify(data, pybytes=True).decode().strip('"')
    oj_json = oj_stringify(data).decode().strip('"')
    assert ry_json == (oj_json.rstrip("0") if data.microsecond else oj_json)
    assert ry.DateTime.parse(ry_json) == ry.DateTime.parse(oj_json)


@_pytest_mark_fmt
@_pytest_mark_skip_orjson
@given(data=st.dates())
def test_stringify_dates(*, data: t.Any, fmt: bool) -> None:
    """Test orjson/ry.stringify for dates."""
    ry_json = ry_stringify(data, fmt=fmt, pybytes=True).decode().strip('"')
    oj_json = oj_stringify(data, fmt=fmt).decode().strip('"')
    assert ry.Date.parse(ry_json) == ry.Date.parse(oj_json)


@_pytest_mark_skip_orjson
@given(data=st.times())
def test_stringify_orjson_times(data: t.Any) -> None:
    """Test orjson/ry.stringify for times."""
    ry_json = ry_stringify(data, pybytes=True).decode().strip('"')
    oj_json = oj_stringify(data).decode().strip('"')
    assert ry.Time.parse(ry_json) == ry.Time.parse(oj_json)


@_pytest_mark_skip_orjson
@given(
    data=st.integers(max_value=-(2**63) - 1) | st.integers(min_value=2**64),
)
def test_stringify_orjson_int_out_of_range(data: int) -> None:
    """Test ints outside of `i64::MIN..=u64::MAX` raise `TypeError` (like orjson)."""
    with pytest.raises(TypeError):
        oj_stringify(data)
    with pytest.raises(TypeError):
        ry_stringify(data)
