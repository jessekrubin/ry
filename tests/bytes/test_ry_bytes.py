from __future__ import annotations

import dataclasses

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


class TestBytesStripIdentity:
    def test_strip_returns_same_object_when_unchanged(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.strip() is rs_bytes
        assert rs_bytes.strip(b"") is rs_bytes
        assert rs_bytes.strip(b"x") is rs_bytes

    def test_lstrip_returns_same_object_when_unchanged(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.lstrip() is rs_bytes
        assert rs_bytes.lstrip(b"") is rs_bytes
        assert rs_bytes.lstrip(b"x") is rs_bytes

    def test_rstrip_returns_same_object_when_unchanged(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.rstrip() is rs_bytes
        assert rs_bytes.rstrip(b"") is rs_bytes
        assert rs_bytes.rstrip(b"x") is rs_bytes

    def test_strip_returns_new_object_when_changed(self) -> None:
        rs_bytes = ry.Bytes(b" asdf ")
        assert rs_bytes.strip() is not rs_bytes
        assert rs_bytes.lstrip() is not rs_bytes
        assert rs_bytes.rstrip() is not rs_bytes


@dataclasses.dataclass
class _ReplaceTestCase:
    b: bytes
    old: bytes
    new: bytes
    count: int
    desc: str | None = None


class TestBytesReplace:
    @given(
        st.binary(), st.binary(), st.binary(), st.integers(min_value=-5, max_value=5)
    )
    def test_replace_matches_python(
        self,
        b: bytes,
        old: bytes,
        new: bytes,
        count: int,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        py_res = b.replace(old, new, count)
        rs_res = ry_bytes.replace(old, new, count)
        assert rs_res == py_res, (
            f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}.replace({old!r}, {new!r}, {count!r})"
        )

    @pytest.mark.parametrize(
        "data",
        [
            _ReplaceTestCase(
                b"abc", b"", b"-", -1, desc="replace empty byte with something else"
            ),
            _ReplaceTestCase(
                b"abc",
                b"",
                b"-",
                0,
                desc="replace empty byte with something else, count 0",
            ),
            _ReplaceTestCase(
                b"abc",
                b"",
                b"-",
                2,
                desc="replace empty byte with something else, count 2",
            ),
            _ReplaceTestCase(
                b"aaaa", b"aa", b"b", -1, desc="replace 'aa' with 'b', count -1"
            ),
            _ReplaceTestCase(
                b"aaaa", b"aa", b"b", 1, desc="replace 'aa' with 'b', count 1"
            ),
            _ReplaceTestCase(
                b"abc", b"x", b"y", -1, desc="replace 'x' with 'y', count -1"
            ),
            _ReplaceTestCase(
                b"abc", b"a", b"a", -1, desc="replace 'a' with 'a', count -1"
            ),
            # replace single byte
            _ReplaceTestCase(
                b"abc" * 10, b"b", b"x", -1, desc="replace 'b' with 'x' in 'abc' * 10"
            ),
            _ReplaceTestCase(
                b"abc" * 10,
                b"b",
                b"x",
                3,
                desc="replace 'b' with 'x' in 'abc' * 10, count 0",
            ),
            # remove single byte
            _ReplaceTestCase(
                b"abc" * 10, b"b", b"", -1, desc="remove 'b' from 'abc' * 10"
            ),
            _ReplaceTestCase(
                b"abc" * 10, b"b", b"", 0, desc="remove 'b' from 'abc' * 10, count 0"
            ),
            _ReplaceTestCase(
                b"abc" * 10, b"b", b"", 5, desc="remove 'b' from 'abc' * 10, count 5"
            ),
            _ReplaceTestCase(
                b"abc" * 10, b"b", b"", 15, desc="remove 'b' from 'abc' * 10, count 15"
            ),
            _ReplaceTestCase(
                b"abc" * 10, b"x", b"", -1, desc="remove 'b' from 'abc' * 10, count -1"
            ),
            # equal length replacements
            _ReplaceTestCase(
                b"abc" * 10,
                b"ab",
                b"xy",
                -1,
                desc="replace 'ab' with 'xy' in 'abc' * 10",
            ),
            _ReplaceTestCase(
                b"abc" * 10,
                b"ab",
                b"xy",
                0,
                desc="replace 'ab' with 'xy' in 'abc' * 10, count 0",
            ),
            _ReplaceTestCase(
                b"abc" * 10,
                b"ab",
                b"xy",
                5,
                desc="replace 'ab' with 'xy' in 'abc' * 10, count 5",
            ),
            _ReplaceTestCase(
                b"abc" * 10,
                b"ab",
                b"xy",
                15,
                desc="replace 'ab' with 'xy' in 'abc' * 10, count 15",
            ),
            # equal length replacements w/ no matches
            _ReplaceTestCase(
                b"abc" * 10,
                b"xy",
                b"ab",
                -1,
                desc="replace 'xy' with 'ab' in 'abc' * 10",
            ),
            # single replacement
            _ReplaceTestCase(
                b"abc" * 10,
                b"abc",
                b"xyz",
                1,
                desc="replace 'abc' with 'xyz' in 'abc' * 10, count 1",
            ),
            # single replacement w/ no matches
            _ReplaceTestCase(
                b"abc" * 10,
                b"xyz",
                b"abc",
                1,
                desc="replace 'xyz' with 'abc' in 'abc' * 10, count 1",
            ),
        ],
    )
    def test_replace_edge_cases(
        self,
        data: _ReplaceTestCase,
    ) -> None:
        ry_bytes = ry.Bytes(data.b)
        assert ry_bytes.replace(data.old, data.new, data.count) == data.b.replace(
            data.old, data.new, data.count
        )


class TestBytesReplaceIdentity:
    def test_replace_returns_same_object_when_no_replacement_occurs(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.replace(b"x", b"y") is rs_bytes
        assert rs_bytes.replace(b"a", b"b", 0) is rs_bytes

    def test_replace_returns_new_object_when_replacement_occurs(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.replace(b"a", b"z") is not rs_bytes
        assert rs_bytes.replace(b"a", b"a") is rs_bytes
        assert rs_bytes.replace(b"", b"") is rs_bytes


class TestBytesFindAndIndex:
    @given(
        b=st.binary(),
        sub=st.one_of(st.binary(), st.integers(min_value=0, max_value=255)),
        start=st.one_of(st.none(), st.integers(min_value=-20, max_value=20)),
        end=st.one_of(st.none(), st.integers(min_value=-20, max_value=20)),
    )
    def test_find_matches_python(
        self,
        b: bytes,
        sub: bytes | int,
        start: int | None,
        end: int | None,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes.find(sub, start, end) == b.find(sub, start, end)
        assert ry_bytes.rfind(sub, start, end) == b.rfind(sub, start, end)

    @given(
        b=st.binary(),
        sub=st.one_of(st.binary(), st.integers(min_value=0, max_value=255)),
        start=st.one_of(st.none(), st.integers(min_value=-20, max_value=20)),
        end=st.one_of(st.none(), st.integers(min_value=-20, max_value=20)),
    )
    def test_index_matches_python(
        self,
        b: bytes,
        sub: bytes | int,
        start: int | None,
        end: int | None,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        find_res = b.find(sub, start, end)
        if find_res == -1:
            with pytest.raises(ValueError, match="subsection not found"):
                ry_bytes.index(sub, start, end)
            with pytest.raises(ValueError, match="subsection not found"):
                ry_bytes.rindex(sub, start, end)
        else:
            assert ry_bytes.index(sub, start, end) == find_res
            assert ry_bytes.rindex(sub, start, end) == b.rfind(sub, start, end)

    @pytest.mark.parametrize(
        ("b", "needle", "start", "end"),
        [
            (b"abcabc", b"ab", 1, None),
            (b"abcabc", b"ab", -3, None),
            (b"abcabc", b"ab", -99, 99),
            (b"abcabc", b"", 3, 1),
            (b"abcabc", b"", 99, None),
            (b"abcabc", b"", -99, -99),
            (b"abcabc", 97, 1, 4),
            (b"abcabc", True, None, None),
            (b"abcabc", False, None, None),
        ],
    )
    def test_find_edge_cases(
        self,
        b: bytes,
        needle: bytes | int | bool,  # noqa: FBT001
        start: int | None,
        end: int | None,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes.find(needle, start, end) == b.find(needle, start, end)
        assert ry_bytes.rfind(needle, start, end) == b.rfind(needle, start, end)

    @pytest.mark.parametrize(
        ("b", "needle", "start", "end"),
        [
            (b"abcabc", b"ab", 1, None),
            (b"abcabc", b"ab", -3, None),
            (b"abcabc", b"ab", -99, 99),
            (b"abcabc", b"", 3, 1),
            (b"abcabc", b"", 99, None),
            (b"abcabc", b"", -99, -99),
            (b"abcabc", 97, 1, 4),
            (b"abcabc", True, None, None),
            (b"abcabc", False, None, None),
        ],
    )
    def test_index_edge_cases(
        self,
        b: bytes,
        needle: bytes | int | bool,  # noqa: FBT001
        start: int | None,
        end: int | None,
    ) -> None:
        should_raise = b.find(needle, start, end) == -1
        if should_raise:
            with pytest.raises(ValueError, match="subsection not found"):
                ry.Bytes(b).index(needle, start, end)
            with pytest.raises(ValueError, match="subsection not found"):
                ry.Bytes(b).rindex(needle, start, end)
            return

        ry_bytes = ry.Bytes(b)
        assert ry_bytes.index(needle, start, end) == b.index(needle, start, end)
        assert ry_bytes.rindex(needle, start, end) == b.rindex(needle, start, end)

    @pytest.mark.parametrize(("sub"), [300, -1])
    @pytest.mark.parametrize(("fnname"), ["find", "rfind", "index", "rindex"])
    def test_rejects_bad_int(self, sub: int, fnname: str) -> None:
        ry_bytes = ry.Bytes(b"abc")
        with pytest.raises(ValueError, match="byte must be in range"):
            getattr(ry_bytes, fnname)(sub)

    @pytest.mark.parametrize("sub", [1.2, "a", object()])
    @pytest.mark.parametrize(("fnname"), ["find", "rfind", "index", "rindex"])
    def test_err_on_bad_input(self, sub: object, fnname: str) -> None:
        ry_bytes = ry.Bytes(b"abc")
        with pytest.raises(
            TypeError, match="argument should be integer or bytes-like object"
        ):
            getattr(ry_bytes, fnname)(sub)

    @pytest.mark.parametrize(("fnname"), ["find", "rfind", "index", "rindex"])
    def test_no_kwargs(self, fnname: str) -> None:
        with pytest.raises(TypeError):
            getattr(ry.Bytes(b"abc"), fnname)(sub=b"a")


class TestBytesPartition:
    @given(
        b=st.binary(),
        sep=st.binary(min_size=1),
    )
    def test_partition_matches_python(self, b: bytes, sep: bytes) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes.partition(sep) == b.partition(sep)  # type: ignore[comparison-overlap]

    @given(
        b=st.binary(),
        sep=st.binary(min_size=1),
    )
    def test_rpartition_matches_python(self, b: bytes, sep: bytes) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes.rpartition(sep) == b.rpartition(sep)  # type: ignore[comparison-overlap]

    @pytest.mark.parametrize("fnname", ["partition", "rpartition"])
    def test_rejects_empty_separator(self, fnname: str) -> None:
        with pytest.raises(ValueError, match="empty separator"):
            getattr(ry.Bytes(b"abc"), fnname)(b"")

    @pytest.mark.parametrize("sep", [1, 1.2, "a", object()])
    @pytest.mark.parametrize("fnname", ["partition", "rpartition"])
    def test_err_on_bad_input(self, sep: object, fnname: str) -> None:
        with pytest.raises(TypeError):
            getattr(ry.Bytes(b"abc"), fnname)(sep)

    def test_partition_not_found_reuses_instances(self) -> None:
        ry_bytes = ry.Bytes(b"abc")
        head, sep, tail = ry_bytes.partition(b"x")
        assert head is ry_bytes
        assert sep is tail
        assert sep == b""

    def test_rpartition_not_found_reuses_instances(self) -> None:
        ry_bytes = ry.Bytes(b"abc")
        head, sep, tail = ry_bytes.rpartition(b"x")
        assert head is sep
        assert head == b""
        assert tail is ry_bytes


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


@pytest.mark.parametrize("b", [b"", b"\xb9\x01\xef", b"abcdef", bytes(range(8))])
@pytest.mark.parametrize("sep", [":", "-", " "])
def test_hex_sep_matches_python(
    b: bytes,
    sep: str,
) -> None:
    ry_bytes = ry.Bytes(b)
    assert ry_bytes.hex(sep=sep) == b.hex(sep=sep)


@pytest.mark.parametrize("b", [b"", b"\xb9\x01\xef", b"abcdef", bytes(range(8))])
@pytest.mark.parametrize("sep", [":", "-", " "])
@pytest.mark.parametrize("bytes_per_sep", [1, 2, 3, 4, 0])
def test_hex_sep_and_bytes_per_sep_matches_python(
    b: bytes,
    sep: str,
    bytes_per_sep: int,
) -> None:
    ry_bytes = ry.Bytes(b)
    assert ry_bytes.hex(sep=sep, bytes_per_sep=bytes_per_sep) == b.hex(
        sep=sep,
        bytes_per_sep=bytes_per_sep,
    )


@pytest.mark.parametrize("sep", ["::", "ab"])
def test_hex_rejects_multi_char_sep(
    sep: str,
) -> None:
    ry_bytes = ry.Bytes(b"\xb9\x01\xef")
    with pytest.raises((TypeError, ValueError)):
        ry_bytes.hex(sep=sep)


@pytest.mark.parametrize("b", [b"", b"\xb9\x01\xef", b"abcdef", bytes(range(8))])
@pytest.mark.parametrize("bytes_per_sep", [1, 2, 3, 4])
def test_hex_default_sep_matches_python(
    b: bytes,
    bytes_per_sep: int,
) -> None:
    ry_bytes = ry.Bytes(b)
    assert ry_bytes.hex(bytes_per_sep=bytes_per_sep) == b.hex(
        bytes_per_sep=bytes_per_sep
    )


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
        "__mod__",
        "__rmod__",
        "center",
        "count",
        "join",
        "ljust",
        "maketrans",
        "rjust",
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
    if fn_name in ["__mod__", "__rmod__"]:
        with pytest.raises(NotImplementedError):
            method(1)  # provide an argument
    else:
        with pytest.raises(NotImplementedError):
            method()
