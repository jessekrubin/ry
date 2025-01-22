from __future__ import annotations

import random
import typing as t

import pytest

import ry.dev as ry


def random_byte_int() -> int:
    return random.randint(0, 255)


def make_random_bytes(length: int = 32) -> bytes:
    return bytes(random_byte_int() for _ in range(length))


def expected_repr(b: bytes) -> str:
    s = repr(b)
    return f"Bytes(b'{s[2:-1]}')"


@pytest.mark.parametrize(
    "b",
    [bytes([i]) for i in range(256)],
)
def test_random_bytes_repr(b: bytes) -> None:
    ry_bytes = ry.Bytes(b)
    ry_bytes_str = str(ry_bytes)
    ry_bytes_str_eval = eval("ry." + ry_bytes_str)
    assert ry_bytes_str_eval == ry_bytes


class TestBytesSlice:
    def test_zero_step_value_err(self) -> None:
        ry_bytes = ry.Bytes(b"abcdefg")
        py_bytes = b"abcdefg"
        with pytest.raises(ValueError):
            _py_new = py_bytes[0:4:0]

        with pytest.raises(ValueError):
            _ry_new = ry_bytes[0:4:0]

    def test_slice_forward(self) -> None:
        ry_bytes = ry.Bytes(b"abcdefg")
        py_bytes = bytes(b"abcdefg")
        for start, stop, step, sliced in all_slices(py_bytes):
            print("======")
            sliced_str = str(sliced)  # mypy doesn't complain with `str-bytes-safe`
            print(f"start={start}, stop={stop}, step={step}, sliced={sliced_str}")
            new_py = py_bytes[start:stop:step]
            new_py_str = str(new_py)  # mypy doesn't complain with `str-bytes-safe`
            print(f"new_py={new_py_str}")
            new_ry = ry_bytes[start:stop:step]
            print("new_ry={new_ry}")
            if len(new_py) == 0:
                continue
            assert new_ry == new_py


def all_slices(b: bytes) -> t.Generator[tuple[int, int, int, bytes], None, None]:
    """
    Yield all tuples (start, stop, step, sliced_result) that do NOT raise ValueError.
    We pick a range that goes a bit beyond len(b) in both negative and positive directions,
    to test boundary cases like b[-999:], b[:999:], b[-999:999:2], etc.
    """
    length = len(b)
    # You can adjust how far you go beyond length in each direction.
    # e.g. range(-length-2, length+3) is a reasonable coverage.
    indices_range = range(-length - 2, length + 3)

    for start in indices_range:
        for stop in indices_range:
            for step in range(-3, 4):  # -3, -2, -1, 0, 1, 2, 3
                if step == 0:
                    # Python raises ValueError if step == 0
                    continue
                try:
                    sliced = b[start:stop:step]
                    yield (start, stop, step, sliced)
                except ValueError:
                    # In practice, Python won't raise ValueError except for step=0,
                    # but we guard just in case.
                    pass


@pytest.mark.parametrize("b", [bytes([i]) for i in range(256)])
def test_hex_str(b: bytes) -> None:
    ry_bytes = ry.Bytes(b)
    ry_hex_str = ry_bytes.hex()
    py_hex_str = b.hex()
    assert ry_hex_str == py_hex_str


@pytest.mark.parametrize(
    "fn_name",
    [
        "__bytes__",
        "__getnewargs__",
        "__iter__",
        "__mod__",
        "__rmod__",
        "capitalize",
        "center",
        "count",
        "endswith",
        "expandtabs",
        "find",
        "fromhex",
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
        "splitlines",
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
