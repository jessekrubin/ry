from __future__ import annotations

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


def test_bytes_pickling() -> None:
    b = ry.Bytes(b"asdf")
    import pickle

    pickled = pickle.dumps(b)
    loaded = pickle.loads(pickled)
    assert loaded == b


class TestBytesIsFns:
    @given(
        py_bytes=st.binary(),
    )
    @pytest.mark.parametrize(
        "fn_name",
        [
            "isalnum",
            "isalpha",
            "isascii",
            "isdigit",
            "islower",
            "isspace",
            "istitle",
            "isupper",
        ],
    )
    def test_bytes_is_fns(
        self,
        fn_name: str,
        py_bytes: bytes,
    ) -> None:
        """Test Bytes.is*() works like python bytes"""
        ry_bytes = ry.Bytes(py_bytes)
        py_res = getattr(py_bytes, fn_name)()
        rs_res = getattr(ry_bytes, fn_name)()
        assert py_res == rs_res, f"py: {py_res}, rs: {rs_res} ~ {py_bytes!r}, {fn_name}"


@given(st.binary())
def test_bytes_capitalize(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_capitalize = b.capitalize()
    ry_capitalize = ry_bytes.capitalize()
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {b!r}"
    )


@given(st.binary())
def test_bytes_swapcase(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_capitalize = b.swapcase()
    ry_capitalize = ry_bytes.swapcase()
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {b!r}"
    )


@given(st.binary())
def test_bytes_title(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_capitalize = b.title()
    ry_capitalize = ry_bytes.title()
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {b!r}"
    )


@given(st.binary())
def test_bytes_expandtabs(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_capitalize = b.expandtabs()
    ry_capitalize = ry_bytes.expandtabs()
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {b!r}"
    )


@given(st.binary())
def test_bytes_strip_no_arg(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_capitalize = b.strip()
    ry_capitalize = ry_bytes.strip()
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {b!r}"
    )


@pytest.mark.parametrize(
    "b",
    [
        # bytes with thing in middle
        *(bytes([i]) + b"howdy" + bytes([i]) for i in range(256)),
        # just raw byte
        *(bytes([i]) for i in range(256)),
    ],
)
def test_bytes_strip_no_arg_all_bytes(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_capitalize = b.strip()
    ry_capitalize = ry_bytes.strip()
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {b!r}"
    )


@pytest.mark.parametrize(
    "bytes2strip",
    [
        b" ",
        b"\n",
        b"\t",
        b" \n\t",
        b"\n\t ",
        b" \n\t ",
    ],
)
@pytest.mark.parametrize(
    "bytes2strip_from",
    [
        b"  \n\t  ",
        b" \n\t  ",
        b"\n\t  ",
        b" \n\t",
        b"\n\t",
    ],
)
def test_bytes_strip_with_arg(
    bytes2strip: bytes,
    bytes2strip_from: bytes,
) -> None:
    """Test Bytes.strip() works like python bytes with an argument"""
    ry_bytes = ry.Bytes(bytes2strip_from)
    py_capitalize = bytes2strip_from.strip(bytes2strip)
    ry_capitalize = ry_bytes.strip(bytes2strip)
    assert ry_capitalize == py_capitalize, (
        f"py: {py_capitalize!r}, rs: {ry_capitalize!r} ~ {bytes2strip_from!r}"
    )


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
        # "capitalize",
        "center",
        "count",
        # "expandtabs",
        "find",
        "index",
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
        # "strip",
        # "swapcase",
        # "title",
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
