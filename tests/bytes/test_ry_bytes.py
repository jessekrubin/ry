from __future__ import annotations

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

_BYTES_ALL = bytes(range(256))
_BYTES_ALNUM = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
_BYTES_ALPHA = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
_BYTES_ASCII = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f"
_BYTES_DIGIT = b"0123456789"
_BYTES_LOWER = b"abcdefghijklmnopqrstuvwxyz"
_BYTES_SPACE = b"\t\n\x0b\x0c\r "
_BYTES_UPPER = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"

_BYTES_TYPES = (
    _BYTES_ALL,
    _BYTES_ALNUM,
    _BYTES_ALPHA,
    _BYTES_ASCII,
    _BYTES_DIGIT,
    _BYTES_LOWER,
    _BYTES_SPACE,
    _BYTES_UPPER,
)


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
    py_res = b.capitalize()
    rs_res = ry_bytes.capitalize()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


@given(st.binary())
def test_bytes_swapcase(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_res = b.swapcase()
    rs_res = ry_bytes.swapcase()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


@given(st.binary())
def test_bytes_title(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_res = b.title()
    rs_res = ry_bytes.title()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


@given(st.binary())
def test_bytes_expandtabs(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_res = b.expandtabs()
    rs_res = ry_bytes.expandtabs()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


@pytest.mark.parametrize(
    "b",
    [
        b"\x0c\t",
        *_BYTES_TYPES,
    ],
)
def test_bytes_expandtabs_ext(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_res = b.expandtabs()
    rs_res = ry_bytes.expandtabs()
    assert rs_res.to_bytes() == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


class TestBytesOperators:
    @given(st.binary())
    def test_bytes_mul(
        self,
        b: bytes,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        py_res = b * 2
        rs_res = ry_bytes * 2
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary())
    def test_bytes_rmul(
        self,
        b: bytes,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        py_res = 2 * b
        rs_res = 2 * ry_bytes
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary(), st.binary())
    def test_add(
        self,
        a: bytes,
        b: bytes,
    ) -> None:
        ry_a = ry.Bytes(a)
        ry_b = ry.Bytes(b)
        py_res = a + b
        rs_res = ry_a + ry_b
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {a!r} + {b!r}"


@given(st.binary())
def test_bytes_strip_no_arg(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_res = b.strip()
    rs_res = ry_bytes.strip()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


class TestBytesStripHypothesis:
    @given(st.binary())
    def test_strip_hypothesis_no_arg(
        self,
        b: bytes,
    ) -> None:
        # .strip()
        ry_bytes = ry.Bytes(b)
        py_res = b.strip()
        rs_res = ry_bytes.strip()
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary())
    def test_lstrip_hypothesis_no_arg(
        self,
        b: bytes,
    ) -> None:
        # .lstrip()
        ry_bytes = ry.Bytes(b)
        py_res = b.lstrip()
        rs_res = ry_bytes.lstrip()
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary())
    def test_rstrip_hypothesis_no_arg(
        self,
        b: bytes,
    ) -> None:
        # .rstrip()
        ry_bytes = ry.Bytes(b)
        py_res = b.rstrip()
        rs_res = ry_bytes.rstrip()
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary(), st.binary())
    def test_strip_hypothesis_arg(
        self,
        b: bytes,
        bytes2strip: bytes,
    ) -> None:
        # .strip()
        ry_bytes = ry.Bytes(b)
        py_res = b.strip(bytes2strip)
        rs_res = ry_bytes.strip(bytes2strip)
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary(), st.binary())
    def test_lstrip_hypothesis_arg(
        self,
        b: bytes,
        bytes2strip: bytes,
    ) -> None:
        # .lstrip()
        ry_bytes = ry.Bytes(b)
        py_res = b.lstrip(bytes2strip)
        rs_res = ry_bytes.lstrip(bytes2strip)
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"

    @given(st.binary(), st.binary())
    def test_rstrip_hypothesis_arg(
        self,
        b: bytes,
        bytes2strip: bytes,
    ) -> None:
        # .rstrip()
        ry_bytes = ry.Bytes(b)
        py_res = b.rstrip(bytes2strip)
        rs_res = ry_bytes.rstrip(bytes2strip)
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


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
    # .strip()
    ry_bytes = ry.Bytes(b)
    py_res = b.strip()
    rs_res = ry_bytes.strip()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"
    # .lstrip()
    ry_bytes = ry.Bytes(b)
    py_res = b.lstrip()
    rs_res = ry_bytes.lstrip()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"
    # .rstrip()
    ry_bytes = ry.Bytes(b)
    py_res = b.rstrip()
    rs_res = ry_bytes.rstrip()
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}"


@pytest.mark.parametrize(
    "bytes2strip",
    [
        b"",
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
        b"",
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

    # .strip()
    ry_bytes = ry.Bytes(bytes2strip_from)
    py_res = bytes2strip_from.strip(bytes2strip)
    rs_res = ry_bytes.strip(bytes2strip)
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {bytes2strip_from!r}"

    # .lstrip()
    ry_bytes = ry.Bytes(bytes2strip_from)
    py_res = bytes2strip_from.lstrip(bytes2strip)
    rs_res = ry_bytes.lstrip(bytes2strip)
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {bytes2strip_from!r}"

    # .rstrip()
    ry_bytes = ry.Bytes(bytes2strip_from)
    py_res = bytes2strip_from.rstrip(bytes2strip)
    rs_res = ry_bytes.rstrip(bytes2strip)
    assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {bytes2strip_from!r}"


@given(st.binary())
def test_hex_and_fromhex(
    b: bytes,
) -> None:
    ry_bytes = ry.Bytes(b)
    py_hex = b.hex()
    ry_hex = ry_bytes.hex()
    assert ry_hex == py_hex
    ry_from_hex = ry_bytes.fromhex(py_hex)
    ry_from_hex_upper = ry_bytes.fromhex(py_hex.upper())

    assert ry_from_hex == b
    assert ry_from_hex == ry_bytes
    assert ry_from_hex_upper == b
    assert ry_from_hex_upper == ry_bytes


# test the string decode bytes fn
@given(st.text())
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
        "__iter__",
        "__mod__",
        "__rmod__",
        "center",
        "count",
        "find",
        "index",
        "join",
        "ljust",
        "maketrans",
        "partition",
        "replace",
        "rfind",
        "rindex",
        "rjust",
        "rpartition",
        "rsplit",
        "split",
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
