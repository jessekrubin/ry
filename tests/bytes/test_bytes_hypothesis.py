from __future__ import annotations

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


@given(st.binary())
@pytest.mark.parametrize(
    "fnname", ["lower", "upper", "capitalize", "swapcase", "title"]
)
def test_case_fns_match_python(fnname: str, b: bytes) -> None:
    py_res = getattr(b, fnname)()
    rs_res = getattr(ry.Bytes(b), fnname)()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}.{fnname}()"


@given(st.binary(), st.binary(max_size=4))
def test_startswith_endswith_matches_python(b: bytes, affix: bytes) -> None:
    ry_b = ry.Bytes(b)
    assert ry_b.startswith(affix) == b.startswith(affix)
    assert ry_b.endswith(affix) == b.endswith(affix)


@given(st.binary(min_size=1), st.data())
def test_getitem_matches_python(b: bytes, data: st.DataObject) -> None:
    ix = data.draw(st.integers(min_value=-len(b), max_value=len(b) - 1))
    assert ry.Bytes(b)[ix] == b[ix]


@given(
    st.binary(max_size=64).flatmap(
        lambda b: st.tuples(st.just(b), st.slices(len(b) + 8))
    )
)
def test_slice_matches_python(b_and_slice: tuple[bytes, slice]) -> None:
    b, s = b_and_slice
    assert ry.Bytes(b)[s] == b[s], f"{s} ~ {b!r}"


@given(st.binary())
def test_hex_and_fromhex(b: bytes) -> None:
    ry_bytes = ry.Bytes(b)
    py_hex = b.hex()
    assert ry_bytes.hex() == py_hex
    assert ry.Bytes.fromhex(py_hex) == b
    assert ry.Bytes.fromhex(py_hex.upper()) == b


@given(st.text())
def test_decode_utf8_matches_python(s: str) -> None:
    b = s.encode()
    assert ry.Bytes(b).decode() == s
    assert ry.Bytes(b).decode("utf-8") == s


@given(st.binary())
@pytest.mark.parametrize("errors", ["ignore", "replace", "backslashreplace"])
def test_decode_errors_matches_python(errors: str, b: bytes) -> None:
    assert ry.Bytes(b).decode("utf-8", errors) == b.decode("utf-8", errors)


@given(st.binary())
@pytest.mark.parametrize("fnname", ["expandtabs"])
def test_returns_self_iff_unchanged(fnname: str, b: bytes) -> None:
    ry_b = ry.Bytes(b)
    res = getattr(ry_b, fnname)()
    assert res == getattr(b, fnname)()
    assert (res is ry_b) == (res == b)


@given(
    st.binary(max_size=64).flatmap(
        lambda b: st.tuples(st.just(b), st.slices(len(b) + 8))
    )
)
def test_slice_returns_self_iff_whole(b_and_slice: tuple[bytes, slice]) -> None:
    b, s = b_and_slice
    ry_b = ry.Bytes(b)
    res = ry_b[s]
    assert res == b[s]
    # `b"aa"[::-1] == b"aa"` but is a new obj; only whole + forward (or tiny) is self
    start, stop, step = s.indices(len(b))
    whole = len(range(start, stop, step)) == len(b) and (step == 1 or len(b) <= 1)
    assert (res is ry_b) == whole
