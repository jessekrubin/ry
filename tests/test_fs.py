from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pathlib import Path


def test_read_string(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    p.write_text("hello")
    ry.cd(tmp_path)
    assert ry.read_text("test.txt") == "hello"


def test_read_string_invalid_utf8(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    p.write_bytes(b"\x80")
    ry.cd(tmp_path)
    with open("test.txt", "rb") as f:
        assert f.read() == b"\x80"
    # with python open and get error type
    with pytest.raises(
        UnicodeDecodeError,
        match="'utf-8' codec can't decode byte 0x80 in position 0: invalid start byte",
    ):
        with open("test.txt", encoding="utf-8") as f:
            f.read()

    with pytest.raises(UnicodeDecodeError):
        ry.read_text("test.txt")


def test_read_bytes(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    p.write_bytes(b"hello")
    ry.cd(tmp_path)
    pybytes = ry.read_bytes("test.txt")
    assert pybytes == b"hello"
    assert isinstance(pybytes, bytes)
    rybytes = ry.read("test.txt")
    assert pybytes == rybytes


def test_fs_read_write(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    ry.write("test.txt", b"hello")
    assert p.read_bytes() == b"hello"
    assert ry.read("test.txt") == b"hello"
    ry.write("test.txt", "hello")
    assert p.read_text() == "hello"
    assert ry.read("test.txt") == b"hello"


@pytest.mark.anyio
async def test_fs_read_write_async(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    await ry.write_async("test.txt", b"hello")
    assert p.read_bytes() == b"hello"
    data_read = await ry.read_async("test.txt")
    assert data_read == b"hello"


def test_read_file_missing(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        ry.read_bytes(str(p))
    with pytest.raises(FileNotFoundError):
        ry.read_text(str(p))


# @pytest.mark.skip(reason="TODO: pathlike not implemented")
def test_read_file_missing_pathlike(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        ry.read_bytes(p)
    with pytest.raises(FileNotFoundError):
        ry.read_text(p)
