from __future__ import annotations

import typing as t

import pytest

from ry import Bytes


def test_empty_eq() -> None:
    assert b"" == Bytes(b"")


def test_repr() -> None:
    py_buf = b"foo\nbar\nbaz"
    rust_buf = Bytes(py_buf)
    # Assert reprs are the same excluding the prefix and suffix
    assert repr(py_buf)[2:-1] == repr(rust_buf)[8:-2]


@pytest.mark.parametrize(
    "b",
    [bytes([i]) for i in range(256)],
)
def test_uno_byte_bytes_repr(b: bytes) -> None:
    rust_bytes = Bytes(b)
    rust_bytes_str = repr(rust_bytes)
    rust_bytes_str_eval = eval(rust_bytes_str)
    assert rust_bytes_str_eval == rust_bytes == b


# =============================================================================
# The following tests are not in the `pyo3-bytes` test suite
# =============================================================================


def test_decode() -> None:
    py_bytes = b"asdf"
    ry_bytes = Bytes(py_bytes)
    assert ry_bytes.decode() == "asdf"


class TestBytesRemovePrefixSuffix:
    def test_remove_prefix(self) -> None:
        ry_bytes = Bytes(b"asdf")
        assert ry_bytes.removeprefix(b"as") == Bytes(b"df")
        assert ry_bytes.removeprefix(b"asdf") == Bytes(b"")

    def test_remove_suffix(self) -> None:
        ry_bytes = Bytes(b"asdf")
        assert ry_bytes.removesuffix(b"df") == Bytes(b"as")
        assert ry_bytes.removesuffix(b"asdf") == Bytes(b"")


class TestBytesSlice:
    def test_zero_step_value_err(self) -> None:
        ry_bytes = Bytes(b"abcdefg")
        py_bytes = b"abcdefg"
        with pytest.raises(ValueError):
            _py_new = py_bytes[0:4:0]

        with pytest.raises(ValueError):
            _ry_new = ry_bytes[0:4:0]

    def test_slice_forward(self) -> None:
        ry_bytes = Bytes(b"abcdefg")
        py_bytes = b"abcdefg"
        for start, stop, step, _sliced in _bytes_slices(py_bytes):
            new_py = py_bytes[start:stop:step]
            new_ry = ry_bytes[start:stop:step]
            assert new_ry == new_py


def _bytes_slices(
    b: bytes, range_buffer: int = 3
) -> t.Generator[tuple[int, int, int, bytes], None, None]:
    """yield tuples of (start, stop, step, sliced_result) for all possible slices of b."""
    b_len = len(b)
    indices_range = range(-b_len - (range_buffer - 1), b_len + range_buffer)
    steps = (i for i in range(-(b_len + 2), b_len + 3) if i != 0)
    return (
        (start, stop, step, b[start:stop:step])
        for start in indices_range
        for stop in indices_range
        for step in steps
    )


@pytest.mark.parametrize("b", [bytes([i]) for i in range(256)])
def test_hex_str(b: bytes) -> None:
    ry_bytes = Bytes(b)
    ry_hex_str = ry_bytes.hex()
    py_hex_str = b.hex()
    assert ry_hex_str == py_hex_str
    assert ry_bytes == Bytes.fromhex(ry_hex_str)
