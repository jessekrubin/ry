from __future__ import annotations

import dataclasses
import gc
import sys
import typing as t

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
        "translate",
        "zfill",
    ],
)
def test_bytes_not_impl(fn_name: str) -> None:
    # remove from the list when implemented
    assert hasattr(b"asdf", fn_name)
    assert not hasattr(ry.Bytes(b"asdf"), fn_name)


class TestBytesConstructor:
    def test_constructor_default_is_empty(self) -> None:
        assert ry.Bytes() == b""

    @pytest.mark.parametrize(
        "value",
        [
            b"abc",
            bytearray(b"abc"),
            memoryview(b"abc"),
        ],
    )
    def test_constructor_accepts_bytes_like(
        self, value: bytes | bytearray | memoryview
    ) -> None:
        assert ry.Bytes(value) == b"abc"

    def test_constructor_reuses_exact_bytes_instance(self) -> None:
        value = ry.Bytes(b"abc")
        assert ry.Bytes(value) is value


class TestRyBytesExtract:
    """`RyBytes` as a pyfunction arg (not `ReadableBuffer`)"""

    @pytest.mark.parametrize(
        "value",
        [b"abc", ry.Bytes(b"abc"), bytearray(b"abc"), memoryview(b"abc")],
    )
    def test_extract(self, value: bytes | ry.Bytes | bytearray | memoryview) -> None:
        msg = ry.WsMessage.binary(value)
        assert msg.payload == b"abc"

    def test_extract_non_buffer(self) -> None:
        with pytest.raises(TypeError):
            ry.WsMessage.binary("abc")  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


@pytest.mark.parametrize("py_bytes", [b"", b"asdf", b"a\x00b", bytes(range(256))])
def test_bytes_hash_matches_python_bytes(py_bytes: bytes) -> None:
    ry_bytes = ry.Bytes(py_bytes)
    assert hash(ry_bytes) == hash(py_bytes)
    assert hash(ry_bytes) == ry_bytes.py_hashbuffer()
    assert hash(ry_bytes) == hash(ry_bytes)
    py_bytes_dict: dict[bytes | ry.Bytes, str] = {ry_bytes: "found"}
    ry_bytes_dict: dict[bytes | ry.Bytes, str] = {py_bytes: "found"}
    assert py_bytes_dict[ry_bytes] == "found"
    assert ry_bytes_dict[py_bytes] == "found"


def test_bytes_pickling() -> None:
    b = ry.Bytes(b"asdf")
    import pickle

    pickled = pickle.dumps(b)
    loaded = pickle.loads(pickled)
    assert loaded == b


class TestBytesGetItemInt:
    @pytest.mark.parametrize("ix", [3, 100, -4, -100])
    def test_getitem_out_of_range(self, ix: int) -> None:
        with pytest.raises(IndexError, match="Index out of range"):
            _ = ry.Bytes(b"abc")[ix]

    def test_getitem_empty(self) -> None:
        with pytest.raises(IndexError):
            _ = ry.Bytes()[0]
        with pytest.raises(IndexError):
            _ = ry.Bytes()[-1]

    @pytest.mark.parametrize("key", ["a", 1.5, None, b"a", object()])
    def test_getitem_bad_key_type(self, key: t.Any) -> None:
        name = type(key).__name__
        msg = f"byte indices must be integers or slices, not {name}"
        with pytest.raises(TypeError, match=msg):
            _ = b"abc"[key]
        with pytest.raises(TypeError, match=msg):
            _ = ry.Bytes(b"abc")[key]

    @pytest.mark.parametrize("key", [2**70, -(2**70)])
    def test_getitem_huge_int(self, key: int) -> None:
        msg = "cannot fit 'int' into an index-sized integer"
        with pytest.raises(IndexError, match=msg):
            _ = b"abc"[key]
        with pytest.raises(IndexError, match=msg):
            _ = ry.Bytes(b"abc")[key]

    def test_getitem_index_dunder(self) -> None:
        class Ix:
            def __index__(self) -> int:
                return 1

        ix: t.Any = Ix()
        assert ry.Bytes(b"abc")[ix] == b"abc"[ix]


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


class TestBytesExpandtabs:
    @given(st.binary())
    def test_bytes_expandtabs(self, b: bytes) -> None:
        assert ry.Bytes(b).expandtabs() == b.expandtabs()

    @pytest.mark.parametrize(
        "b",
        [
            b"\x0c\t",
            *_BYTES_TYPES,
        ],
    )
    def test_bytes_expandtabs_ext(self, b: bytes) -> None:
        assert ry.Bytes(b).expandtabs() == b.expandtabs()

    @pytest.mark.parametrize("b", [b"a\tb", b"\t\t", b"no tabs", b"\ta\n\tb\r\tc", b""])
    def test_expandtabs_zero(self, b: bytes) -> None:
        assert ry.Bytes(b).expandtabs(0) == b.expandtabs(0)

    @pytest.mark.parametrize(
        ("b", "tabsize", "exc"),
        [
            (b"\t" * 16, 2**60, OverflowError),  # 2**64: doesn't fit usize
            (b"\t" * 1024, 2**40, MemoryError),  # 2**50: past the address space
        ],
    )
    def test_bytes_expandtabs_huge(
        self, b: bytes, tabsize: int, exc: type[Exception]
    ) -> None:
        with pytest.raises(exc):
            ry.Bytes(b).expandtabs(tabsize)


class TestBytesOperators:
    @given(st.binary(max_size=64), st.integers(min_value=-3, max_value=8))
    def test_bytes_mul_hypothesis(self, b: bytes, n: int) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes * n == b * n
        assert n * ry_bytes == n * b

    @pytest.mark.parametrize("b", [b"", b"a", b"abc"])
    @pytest.mark.parametrize("n", [-(2**63), -5, -1, 0, 1, 2, 7, True, False])
    def test_bytes_mul_matches_python(self, b: bytes, n: int) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes * n == b * n
        assert n * ry_bytes == n * b

    @pytest.mark.parametrize("n", [1.5, "2", None])
    def test_bytes_mul_non_int_type_error(self, n: object) -> None:
        with pytest.raises(TypeError):
            _ = ry.Bytes(b"ab") * n  # type: ignore[operator]  # ty: ignore[unsupported-operator]
        with pytest.raises(TypeError):
            _ = n * ry.Bytes(b"ab")  # type: ignore[operator]  # ty: ignore[unsupported-operator]

    @pytest.mark.parametrize(
        ("b", "n", "exc"),
        [
            (b"ab", 2**62, OverflowError),  # len * n > isize::MAX
            (b"abc", 2**62, OverflowError),  # len * n overflows usize
            (b"a", 2**62, MemoryError),  # fits isize, but no allocator says yes
        ],
    )
    def test_bytes_mul_too_big(self, b: bytes, n: int, exc: type[Exception]) -> None:
        # pypy raises MemoryError not OverflowError
        if sys.implementation.name == "cpython":
            with pytest.raises(exc):
                _ = b * n
        with pytest.raises(exc):
            _ = ry.Bytes(b) * n
        with pytest.raises(exc):
            _ = n * ry.Bytes(b)

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


_STRIP_FNS = ("strip", "lstrip", "rstrip")


@pytest.mark.parametrize("fnname", _STRIP_FNS)
class TestBytesStrip:
    @given(b=st.binary(), chars=st.none() | st.binary())
    def test_strip_hypothesis(self, fnname: str, b: bytes, chars: bytes | None) -> None:
        py_res = getattr(b, fnname)(chars)
        rs_res = getattr(ry.Bytes(b), fnname)(chars)
        assert rs_res == py_res, (
            f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}.{fnname}({chars!r})"
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
    def test_strip_no_arg_all_bytes(self, fnname: str, b: bytes) -> None:
        assert getattr(ry.Bytes(b), fnname)() == getattr(b, fnname)()

    @pytest.mark.parametrize(
        "chars",
        [b"", b" ", b"\n", b"\t", b" \n\t", b"\n\t ", b" \n\t "],
    )
    @pytest.mark.parametrize(
        "b",
        [b"", b"  \n\t  ", b" \n\t  ", b"\n\t  ", b" \n\t", b"\n\t"],
    )
    def test_strip_with_arg(self, fnname: str, b: bytes, chars: bytes) -> None:
        assert getattr(ry.Bytes(b), fnname)(chars) == getattr(b, fnname)(chars)

    @pytest.mark.parametrize("chars", [b" \t\n\r\x0b\x0c", b"\x0c\x0b\r\n\t "])
    def test_strip_explicit_ascii_whitespace(self, fnname: str, chars: bytes) -> None:
        b = b" \t\n\x0b\x0cabc \r\n\x0b\x0c\t"
        assert getattr(ry.Bytes(b), fnname)(chars) == getattr(b, fnname)(chars)


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
            # byte -> many
            _ReplaceTestCase(b"a-b-c", b"-", b"::", -1, desc="byte to many"),
            _ReplaceTestCase(b"a-b-c", b"-", b"::", 1, desc="byte to many, count 1"),
            _ReplaceTestCase(b"---", b"-", b"<>", 2, desc="byte to many, count 2"),
            _ReplaceTestCase(b"abc", b"-", b"::", -1, desc="byte to many, no match"),
            _ReplaceTestCase(b"-", b"-", b"abc", -1, desc="byte to many, whole buf"),
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


class TestBytesReplaceHugeOutput:
    """Outputs of 2**49 bytes (past the 47-bit address space) -> MemoryError, not abort."""

    _BUF = ry.Bytes(b"a" * 2**23)
    _NEW = ry.Bytes(b"b" * 2**26)

    @pytest.mark.parametrize(
        ("old", "new"),
        [
            pytest.param(b"", _NEW, id="empty-old"),
            pytest.param(b"a", _NEW, id="byte-to-many"),
            pytest.param(b"aa", _NEW, id="substring-to-many"),
        ],
    )
    def test_replace_huge_output_memory_error(self, old: bytes, new: ry.Bytes) -> None:
        with pytest.raises(MemoryError):
            self._BUF.replace(old, new)


class TestBytesRemovePrefixSuffixIdentity:
    def test_removeprefix_no_match_returns_self(self) -> None:
        b = ry.Bytes(b"abc")
        assert b.removeprefix(b"x") is b

    def test_removesuffix_no_match_returns_self(self) -> None:
        b = ry.Bytes(b"abc")
        assert b.removesuffix(b"x") is b

    def test_empty_affix_returns_self(self) -> None:
        b = ry.Bytes(b"abc")
        assert b.removeprefix(b"") is b
        assert b.removesuffix(b"") is b

    def test_match_returns_new_object(self) -> None:
        b = ry.Bytes(b"abc")
        assert b.removeprefix(b"a") is not b
        assert b.removesuffix(b"c") is not b


class TestBytesSliceIdentity:
    @pytest.mark.parametrize(
        "s",
        [
            slice(None),
            slice(None, None, 1),
            slice(0, 3),
            slice(-3, None),
            slice(-100, 100),
        ],
        ids=str,
    )
    def test_whole_slice_returns_self(self, s: slice) -> None:
        b = ry.Bytes(b"abc")
        assert b[s] is b

    @pytest.mark.parametrize("value", [b"", b"x"])
    @pytest.mark.parametrize("s", [slice(None), slice(None, None, -1)], ids=str)
    def test_tiny_buffer_any_step_returns_self(self, value: bytes, s: slice) -> None:
        b = ry.Bytes(value)
        assert b[s] is b

    @pytest.mark.parametrize(
        "s",
        [slice(None, None, -1), slice(1, None), slice(None, 2), slice(None, None, 2)],
        ids=str,
    )
    def test_partial_slice_returns_new_object(self, s: slice) -> None:
        b = ry.Bytes(b"abc")
        assert b[s] is not b
        assert b[s] == b"abc"[s]


class TestBytesAddIdentity:
    def test_add_empty_returns_self(self) -> None:
        b = ry.Bytes(b"abc")
        assert b + b"" is b
        assert b + ry.Bytes() is b

    def test_empty_add_ry_bytes_returns_other(self) -> None:
        b = ry.Bytes(b"abc")
        assert ry.Bytes() + b is b

    def test_empty_add_py_bytes_returns_new_ry_bytes(self) -> None:
        res = ry.Bytes() + b"abc"
        assert isinstance(res, ry.Bytes)
        assert res == b"abc"


class TestBytesExpandtabsIdentity:
    @pytest.mark.parametrize("tabsize", [0, 1, 8])
    def test_no_tabs_returns_self(self, tabsize: int) -> None:
        b = ry.Bytes(b"no tabs\nhere")
        assert b.expandtabs(tabsize) is b

    def test_tabs_returns_new_object(self) -> None:
        b = ry.Bytes(b"a\tb")
        assert b.expandtabs() is not b


class TestBytesReplaceIdentity:
    def test_replace_returns_same_object_when_no_replacement_occurs(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.replace(b"x", b"y") is rs_bytes
        assert rs_bytes.replace(b"a", b"b", 0) is rs_bytes
        assert rs_bytes.replace(b"a", b"a") is rs_bytes
        assert rs_bytes.replace(b"", b"") is rs_bytes

    def test_replace_returns_new_object_when_replacement_occurs(self) -> None:
        rs_bytes = ry.Bytes(b"asdf")
        assert rs_bytes.replace(b"a", b"z") is not rs_bytes


_FIND_EDGE_CASES = [
    (b"abcabc", b"ab", 1, None),
    (b"abcabc", b"ab", -3, None),
    (b"abcabc", b"ab", -99, 99),
    (b"abcabc", b"", 3, 1),
    (b"abcabc", b"", 99, None),
    (b"abcabc", b"", -99, -99),
    (b"abcabc", 97, 1, 4),
    (b"abcabc", True, None, None),
    (b"abcabc", False, None, None),
]


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
        _FIND_EDGE_CASES,
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
        _FIND_EDGE_CASES,
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


class TestBytesContains:
    @pytest.mark.parametrize("haystack", [b"", b"a", b"abc"])
    @pytest.mark.parametrize("needle", [b"", bytearray(), memoryview(b"")])
    def test_contains_empty(self, needle: bytes, haystack: bytes) -> None:
        assert (needle in ry.Bytes(haystack)) is True

    @given(st.binary(max_size=16), st.binary(max_size=64))
    def test_contains_matches_python(self, needle: bytes, haystack: bytes) -> None:
        assert (needle in ry.Bytes(haystack)) == (needle in haystack)


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
    with pytest.raises(ValueError, match="single ASCII character"):
        ry_bytes.hex(sep=sep)


def test_hex_sep_non_ascii() -> None:
    with pytest.raises(ValueError, match="single ASCII character"):
        ry.Bytes(b"abc").hex("é")


def test_hex_sep_not_str() -> None:
    with pytest.raises(ValueError, match="single ASCII character string"):
        ry.Bytes(b"abc").hex(1)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


@pytest.mark.parametrize("s", ["a", "abc", "ab c"])
def test_fromhex_odd_length(s: str) -> None:
    with pytest.raises(ValueError, match="Odd-length hex string"):
        ry.Bytes.fromhex(s)


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


@pytest.mark.xfail(raises=NotImplementedError, strict=True, reason="not impl")
class TestBytesSplit:
    @given(st.binary())
    def test_bytes_split_matches_python(
        self,
        b: bytes,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        py_res = b.split()
        rs_res = ry_bytes.split()
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}.split()"

    @given(st.binary())
    def test_bytes_rsplit_matches_python(
        self,
        b: bytes,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        py_res = b.rsplit()
        rs_res = ry_bytes.rsplit()
        assert rs_res == py_res, f"py: {py_res!r}, rs: {rs_res!r} ~ {b!r}.rsplit()"

    @pytest.mark.parametrize(
        ("b", "sep", "maxsplit"),
        [
            (b"  a  b  ", None, -1),
            (b"  a  b  ", None, 0),
            (b"  a  b  c  ", None, 1),
            (b"a,,b,", b",", -1),
            (b"a,,b,", b",", 1),
            (b"abcXXdefXX", b"XX", -1),
            (b"abcXXdefXX", b"XX", 1),
        ],
    )
    def test_bytes_split_fns(
        self,
        b: bytes,
        sep: bytes | None,
        maxsplit: int,
    ) -> None:
        ry_bytes = ry.Bytes(b)
        assert ry_bytes.split(sep, maxsplit) == b.split(sep, maxsplit)
        assert ry_bytes.rsplit(sep, maxsplit) == b.rsplit(sep, maxsplit)

    def test_bytes_split_rejects_empty_separator(self) -> None:
        with pytest.raises(ValueError, match="empty separator"):
            ry.Bytes(b"abc").split(b"")
        with pytest.raises(ValueError, match="empty separator"):
            ry.Bytes(b"abc").rsplit(b"")


class TestBytesCopyClone:
    @pytest.mark.parametrize(
        "value", [b"abc", bytearray(b"abc"), memoryview(b"abc"), ry.Bytes(b"abc")]
    )
    def test_copy_from(self, value: bytes | bytearray | memoryview | ry.Bytes) -> None:
        b = ry.Bytes.copy_from(value)
        assert isinstance(b, ry.Bytes)
        assert b == b"abc"

    def test_copy_from_is_new_object(self) -> None:
        og = ry.Bytes(b"abc")
        assert ry.Bytes.copy_from(og) is not og

    def test_clone(self) -> None:
        og = ry.Bytes(b"abc")
        cloned = og.clone()
        assert cloned == og
        assert cloned is not og


class TestBytesIsEmptyIsUnique:
    @pytest.mark.parametrize(("value", "expected"), [(b"", True), (b"a", False)])
    def test_is_empty(self, *, value: bytes, expected: bool) -> None:
        assert ry.Bytes(value).is_empty() is expected

    def test_is_unique(self) -> None:
        # vec-backed bytes are unique until cloned
        b = ry.Bytes.copy_from(b"abc")
        assert b.is_unique()
        cloned = b.clone()
        assert not b.is_unique()
        assert not cloned.is_unique()
        del cloned
        gc.collect()  # pypy doesnt free on `del`
        assert b.is_unique()


class TestBytesIter:
    def test_iter_is_self_and_len(self) -> None:
        it = iter(ry.Bytes(b"abc"))
        assert iter(it) is it  # ty:ignore[no-matching-overload]  # ty bug: iter() on protocol w/ Self __iter__
        assert len(it) == 3
        assert next(it) == ord("a")
        assert len(it) == 2
        assert list(it) == [ord("b"), ord("c")]
        assert len(it) == 0


class TestBytesWindows:
    def test_bytes_windows_iterates_slices(self) -> None:
        ry_bytes = ry.Bytes(b"abcdef")
        windows = ry_bytes.windows(3)

        assert len(windows) == 4
        assert list(windows) == [
            ry.Bytes(b"abc"),
            ry.Bytes(b"bcd"),
            ry.Bytes(b"cde"),
            ry.Bytes(b"def"),
        ]
        assert len(windows) == 0

    def test_bytes_windows_iterates_slices_in_reverse(self) -> None:
        ry_bytes = ry.Bytes(b"abcdef")
        windows = ry_bytes.windows(3, reverse=True)

        assert len(windows) == 4
        assert list(windows) == [
            ry.Bytes(b"def"),
            ry.Bytes(b"cde"),
            ry.Bytes(b"bcd"),
            ry.Bytes(b"abc"),
        ]
        assert len(windows) == 0

    def test_bytes_windows_size_one(self) -> None:
        assert list(ry.Bytes(b"abc").windows(1)) == [
            ry.Bytes(b"a"),
            ry.Bytes(b"b"),
            ry.Bytes(b"c"),
        ]

    @pytest.mark.parametrize("size", [4, 5])
    def test_bytes_windows_larger_than_buffer(self, size: int) -> None:
        windows = ry.Bytes(b"abc").windows(size)

        assert len(windows) == 0
        assert list(windows) == []

    def test_bytes_windows_zero_size(self) -> None:
        with pytest.raises(ValueError, match="invalid zero value"):
            ry.Bytes(b"abc").windows(0)

    def test_iter_is_self(self) -> None:
        it = ry.Bytes(b"abcdef").windows(2)
        assert iter(it) is it  # ty:ignore[no-matching-overload]  # ty bug: iter() on protocol w/ Self __iter__

    @pytest.mark.parametrize(
        ("reverse", "expected"), [(False, "windows"), (True, "windows-reverse")]
    )
    def test_kind(self, *, reverse: bool, expected: str) -> None:
        assert ry.Bytes(b"abc").windows(2, reverse=reverse).kind == expected

    @pytest.mark.parametrize("reverse", [False, True])
    def test_repr(self, *, reverse: bool) -> None:
        it = ry.Bytes(b"abcdef").windows(2, reverse=reverse)
        kind = "windows-reverse" if reverse else "windows"
        assert repr(it) == f"BytesSlicesIter<{kind}; len=6; remaining=5>"
        it.next()
        assert repr(it) == f"BytesSlicesIter<{kind}; len=6; remaining=4>"

    def test_next(self) -> None:
        it = ry.Bytes(b"abc").windows(2)
        assert it.next() == b"ab"
        assert it.next() == b"bc"
        assert it.next() is None

    def test_take(self) -> None:
        it = ry.Bytes(b"abcdef").windows(2)
        assert it.take() == [b"ab"]
        assert it.take(2) == [b"bc", b"cd"]
        assert it.take(100) == [b"de", b"ef"]
        assert it.take() == []

    def test_collect(self) -> None:
        it = ry.Bytes(b"abcd").windows(2, reverse=True)
        assert it.collect() == [b"cd", b"bc", b"ab"]
        assert it.collect() == []

    def test_nth(self) -> None:
        it = ry.Bytes(b"abcdef").windows(2)
        assert it.nth(0) == b"ab"
        assert it.nth(1) == b"cd"
        assert it.nth(100) is None
        assert len(it) == 0

    @pytest.mark.parametrize(("reverse", "expected"), [(False, b"ef"), (True, b"ab")])
    def test_last(self, *, reverse: bool, expected: bytes) -> None:
        it = ry.Bytes(b"abcdef").windows(2, reverse=reverse)
        assert it.last() == expected
        assert it.last() is None

    def test_count(self) -> None:
        it = ry.Bytes(b"abcdef").windows(2)
        it.next()
        assert it.count() == 4
        assert it.count() == 0

    @pytest.mark.parametrize("reverse", [False, True])
    def test_size_hint(self, *, reverse: bool) -> None:
        it = ry.Bytes(b"abcdef").windows(2, reverse=reverse)
        assert it.size_hint() == (5, 5)
        it.next()
        assert it.size_hint() == (4, 4)
        it.collect()
        assert it.size_hint() == (0, 0)
