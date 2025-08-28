from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pathlib import Path


def test_fs_read_stream(tmp_path: Path) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    p = tmp_path / "test.txt"
    string = "\n".join([str(i) for i in range(1000)])
    string_bytes = string.encode()
    with open(p, "wb") as f:
        f.write(string_bytes)
    ry.cd(tmp_path)
    chunks = list(ry.read_stream("test.txt", chunk_size=10))
    assert b"".join(chunks) == string_bytes
    assert len(chunks) == len(string_bytes) // 10 + 1

    # with offset
    chunks = list(ry.read_stream("test.txt", chunk_size=10, offset=100))
    assert b"".join(chunks) == string_bytes[100:]
    assert len(chunks) == len(string_bytes[100:]) // 10 + 1


def test_fs_read_stream_str(tmp_path: Path) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    p = tmp_path / "test.txt"
    string = "\n".join([str(i) for i in range(1000)])
    string_bytes = string.encode()
    with open(p, "wb") as f:
        f.write(string_bytes)
    ry.cd(tmp_path)
    stream = ry.read_stream("test.txt", chunk_size=10)
    uno = stream.take()
    dos = stream.take(2)
    restante = stream.collect()
    assert len(uno) == 1
    assert uno == [b"0\n1\n2\n3\n4\n"]
    assert dos == [b"5\n6\n7\n8\n9\n", b"10\n11\n12\n1"]
    assert len(restante) == 386


def test_fs_read_stream_file_not_found(tmp_path: Path) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        list(ry.read_stream("test.txt", chunk_size=10))


def test_read_stream_is_directory(tmp_path: Path) -> None:
    """Test that reading a directory raises an error."""
    ry.cd(tmp_path)
    (tmp_path / "test").mkdir(parents=True)
    with pytest.raises(OSError):
        list(ry.read_stream(tmp_path, chunk_size=10))


def test_read_offset_greater_than_file_size(tmp_path: Path) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    p = tmp_path / "test.txt"
    string = "123"
    string_bytes = string.encode()
    with open(p, "wb") as f:
        size = f.write(string_bytes)
    ry.cd(tmp_path)
    read_offset = size + 1
    chunks = list(ry.read_stream("test.txt", offset=read_offset))
    assert b"".join(chunks) == b""
    assert len(chunks) == 0
