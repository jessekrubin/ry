"""Tests for ry.FsPath"""

from pathlib import Path
from typing import Type

import pytest

import ry


def test_new_path() -> None:
    pypath = Path()
    rypath = ry.FsPath()
    assert rypath == pypath


# parametrize the tests for parity with pathlib.Path
@pytest.mark.parametrize(
    "path_cls",
    [
        pytest.param(
            Path,
            id="pathlib.Path",
        ),
        pytest.param(
            ry.FsPath,
            id="ry.FsPath",
        ),
    ],
)
class TestFsPath:
    def test_new_path(self, path_cls: Type[Path]) -> None:
        pypath = Path()
        rypath = path_cls()
        assert rypath == pypath

    def test_parent(self, path_cls: type[Path]) -> None:
        pypath = Path()
        rypath = path_cls()
        assert rypath.parent == pypath.parent

    def test_absolute(self, path_cls: type[Path]) -> None:
        pypath = Path()
        rypath = path_cls()
        pypath_abs = pypath.absolute()
        rypath_abs = rypath.absolute()
        assert rypath_abs == pypath_abs

    def test_read_text(self, path_cls: type[Path], tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.write_text("hello")
        rypath = path_cls(pypath)
        assert rypath.read_text() == pypath.read_text()

    def test_read_bytes(self, path_cls: type[Path], tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.write_bytes(b"hello")
        rypath = path_cls(pypath)
        b = rypath.read_bytes()
        assert rypath.read_bytes() == pypath.read_bytes()
        assert rypath.read_bytes() == b
