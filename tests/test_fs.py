from pathlib import Path

import pytest

import ry


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
    with pytest.raises(UnicodeDecodeError):
        with open("test.txt", encoding="utf-8") as f:
            f.read()
    with pytest.raises(UnicodeDecodeError):
        ry.read_text("test.txt")


def test_read_bytes(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    p.write_bytes(b"hello")
    ry.cd(tmp_path)
    assert ry.read_bytes("test.txt") == b"hello"


def test_read_file_missing(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        ry.read_bytes(str(p))
    with pytest.raises(FileNotFoundError):
        ry.read_text(str(p))


@pytest.mark.skip(reason="TODO: pathlike not implemented")
def test_read_file_missing_pathlike(tmp_path: Path) -> None:
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        ry.read_bytes(p)
    with pytest.raises(FileNotFoundError):
        ry.read_text(p)
