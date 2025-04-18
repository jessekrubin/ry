from __future__ import annotations

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


def test_bytes_pickling() -> None:
    b = ry.Bytes(b"asdf")
    import pickle

    pickled = pickle.dumps(b)
    loaded = pickle.loads(pickled)  # noqa: S301
    assert loaded == b


@given(
    st.binary(),
)
def test_hex_and_fromhex(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_hex = b.hex()
    ry_hex = ry_bytes.hex()
    assert ry_hex == py_hex
    ry_from_hex = ry_bytes.fromhex(py_hex)

    assert ry_from_hex == b
    assert ry_from_hex == ry_bytes


# test the string decode bytes fn
@given(
    st.text(),
)
def test_bytes_decode_default(
    s: str,
) -> None:
    """Test Bytes.decode() works like python bytes"""
    py_bytes = s.encode()
    rust_bytes = ry.Bytes(py_bytes)
    assert rust_bytes.decode() == s
    assert rust_bytes.decode("utf-8") == s
    assert rust_bytes.decode("utf-8", "ignore") == s


@pytest.mark.parametrize(
    "fn_name",
    [
        "__bytes__",
        "__iter__",
        "__mod__",
        "__rmod__",
        "capitalize",
        "center",
        "count",
        "endswith",
        "expandtabs",
        "find",
        "index",
        "istitle",
        "join",
        "ljust",
        "lstrip",
        "maketrans",
        "partition",
        "replace",
        "rfind",
        "rindex",
        "rjust",
        "rpartition",
        "rsplit",
        "rstrip",
        "split",
        "strip",
        "swapcase",
        "title",
        "translate",
        "zfill",
    ],
)
def test_bytes_not_impl(fn_name: str) -> None:
    b = b"asdf"
    rust_bytes = ry.Bytes(b)
    method = getattr(rust_bytes, fn_name, None)
    if method is None:
        return
    with pytest.raises(NotImplementedError):
        if fn_name in ["__mod__", "__rmod__"]:
            method(1)  # provide an argument
        else:
            method()
