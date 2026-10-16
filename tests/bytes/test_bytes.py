from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

from ry import Bytes

if TYPE_CHECKING:
    from collections.abc import Iterable

ALL_BYTES = b"".join([bytes([i]) for i in range(256)])


def test_empty_eq() -> None:
    """Test that empty bytes and Bytes are equal."""
    assert Bytes(b"") == b""


def test_repr() -> None:
    """Test the repr of Bytes and bytes."""
    py_buf = b"foo\nbar\nbaz"
    rust_buf = Bytes(py_buf)
    # Assert reprs are the same excluding the prefix and suffix
    assert repr(py_buf)[2:-1] == repr(rust_buf)[8:-2]


@pytest.mark.parametrize(
    "b",
    [bytes([i]) for i in range(256)],
)
def test_uno_byte_bytes_repr(b: bytes) -> None:
    """Test the repr of Bytes and bytes for single byte values."""
    rust_bytes = Bytes(b)
    rust_bytes_str = repr(rust_bytes)
    rust_bytes_str_eval = eval(rust_bytes_str)
    assert rust_bytes_str_eval == rust_bytes == b


class TestBytesRemovePrefixSuffix:
    """Test the remove_prefix and remove_suffix methods."""

    def test_remove_prefix(self) -> None:
        """Test that remove_prefix works as expected."""
        rust_bytes = Bytes(b"asdf")
        assert rust_bytes.removeprefix(b"as") == Bytes(b"df")
        assert rust_bytes.removeprefix(b"asdf") == Bytes(b"")

    def test_remove_suffix(self) -> None:
        """Test that remove_suffix works as expected."""
        rust_bytes = Bytes(b"asdf")
        assert rust_bytes.removesuffix(b"df") == Bytes(b"as")
        assert rust_bytes.removesuffix(b"asdf") == Bytes(b"")


class TestBytesSlice:
    """Test suite for Bytes slicing."""

    def test_zero_step_value_err(self) -> None:
        """Test that slicing with step=0 raises ValueError."""
        rs_bytes = Bytes(b"abcdefg")
        py_bytes = b"abcdefg"
        with pytest.raises(ValueError, match="slice step cannot be zero"):
            _py_new = py_bytes[0:4:0]  # ty:ignore[zero-stepsize-in-slice]

        with pytest.raises(ValueError, match="slice step cannot be zero"):
            _rs_bytes = rs_bytes[0:4:0]

    @pytest.mark.parametrize(
        "py_bytes",
        [b"", b"a", b"ab", b"abcdefg"],
    )
    def test_slice_o_bytes(self, py_bytes: bytes) -> None:
        """Run every slice in range on both bytes and Bytes and assert they are equal."""
        rs_bytes = Bytes(py_bytes)
        for start, stop, step in self._bytes_slices(py_bytes):
            new_py = py_bytes[start:stop:step]
            new_rs = rs_bytes[start:stop:step]
            assert new_rs == new_py, f"[{start}:{stop}:{step}] ~ {py_bytes!r}"

    @pytest.mark.parametrize(
        "py_bytes",
        [b"", b"x", ALL_BYTES],
        ids=["empty", "one", "all_bytes"],
    )
    @pytest.mark.parametrize(
        "s",
        [
            slice(None, None, -1),
            slice(None, None, -2),
            slice(None, None, -300),
            slice(5, None, -1),
            slice(-1, None, -1),
            slice(0, 0, -1),
            slice(0, None, -1),
            slice(None, 2**62),
            slice(-(2**62), None, -1),
            slice(2**62, -(2**62), -3),
            slice(-(2**62), 2**62, 7),
        ],
        ids=str,
    )
    def test_slice_edges(self, py_bytes: bytes, s: slice) -> None:
        assert Bytes(py_bytes)[s] == py_bytes[s]

    @staticmethod
    def _bytes_slices(
        b: bytes,
        range_buffer: int = 3,
    ) -> Iterable[tuple[int | None, int | None, int | None]]:
        """Yield (start, stop, step) for all slices of b (incl. `None`s)."""
        b_len = len(b)
        indices = (None, *range(-b_len - range_buffer, b_len + range_buffer + 1))
        steps = (None, *(i for i in range(-(b_len + 2), b_len + 3) if i != 0))
        return (
            (start, stop, step)
            for start in indices
            for stop in indices
            for step in steps
        )


@pytest.mark.parametrize("b", [bytes([i]) for i in range(256)])
def test_hex_str(b: bytes) -> None:
    ry_bytes = Bytes(b)
    ry_hex_str = ry_bytes.hex()
    py_hex_str = b.hex()
    assert ry_hex_str == py_hex_str
    assert ry_bytes == Bytes.fromhex(ry_hex_str)
