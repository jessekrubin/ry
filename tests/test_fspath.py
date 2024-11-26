"""Tests for ry.FsPath"""

from __future__ import annotations

import itertools as it
import os
from pathlib import Path
from typing import Union

import pytest

import ry

TPath = Union[type[Path], type[ry.FsPath]]
is_windows = os.name == "nt"


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
    def test_new_path(self, path_cls: TPath) -> None:
        pypath = Path()
        rypath = path_cls()
        assert rypath == pypath

    def test_parent(self, path_cls: TPath) -> None:
        pypath = Path()
        rypath = path_cls()
        assert rypath.parent == pypath.parent

    def test_absolute(self, path_cls: TPath) -> None:
        pypath = Path()
        rypath = path_cls()
        pypath_abs = pypath.absolute()
        rypath_abs = rypath.absolute()
        assert rypath_abs == pypath_abs

    def test_read_text(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.write_text("hello")
        rypath = path_cls(pypath)
        assert rypath.read_text() == pypath.read_text()

    def test_read_bytes(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.write_bytes(b"hello")
        rypath = path_cls(pypath)
        b = rypath.read_bytes()
        assert rypath.read_bytes() == pypath.read_bytes()
        assert rypath.read_bytes() == b

    def test_write_text(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        rypath = path_cls(pypath)
        rypath.write_text("new content")
        assert pypath.read_text() == "new content"

    def test_write_bytes(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        rypath = path_cls(pypath)
        rypath.write_bytes(b"new content")
        assert pypath.read_bytes() == b"new content"

    def test_joinpath(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.joinpath("child") == pypath.joinpath("child")

    def test_exists(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.touch()
        rypath = path_cls(pypath)
        assert rypath.exists() == pypath.exists()

    def test_is_file(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.touch()
        rypath = path_cls(pypath)
        assert rypath.is_file() == pypath.is_file()

    def test_is_dir(self, path_cls: TPath, tmp_path: Path) -> None:
        rypath = path_cls(tmp_path)
        assert rypath.is_dir() == tmp_path.is_dir()

    def test_with_name(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.with_name("newfile.txt") == pypath.with_name("newfile.txt")

    def test_with_suffix(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.with_suffix(".md") == pypath.with_suffix(".md")

    def test_stem(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.stem == pypath.stem

    def test_suffix(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.suffix == pypath.suffix

    def test_iterdir(self, path_cls: TPath, tmp_path: Path) -> None:
        (tmp_path / "file1.txt").touch()
        (tmp_path / "file2.txt").touch()
        pypath = tmp_path
        rypath = path_cls(tmp_path)
        assert sorted(rypath.iterdir()) == sorted(pypath.iterdir())

    def test_relative_to(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.txt")
        rypath = path_cls("/some/path/file.txt")
        if path_cls is ry.FsPath:
            with pytest.raises(NotImplementedError):
                relative_resolved = rypath.relative_to("/some")
                assert relative_resolved == pypath.relative_to("/some")
        else:
            relative_resolved = rypath.relative_to("/some")
            assert relative_resolved == pypath.relative_to("/some")

    def test_as_posix(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.txt")
        rypath = path_cls("/some/path/file.txt")
        assert rypath.as_posix() == pypath.as_posix()

    def test_equality(self, path_cls: TPath) -> None:
        pypath1 = Path("/some/path")
        pypath2 = Path("/some/path")
        rypath1 = path_cls("/some/path")
        rypath2 = path_cls("/some/path")
        for a, b in it.combinations([pypath1, pypath2, rypath1, rypath2], 2):
            assert a == b, f"{a} != {b} ({type(a)} != {type(b)})"

    def test_inequality(self, path_cls: TPath) -> None:
        rypath1 = path_cls("/some/path")
        rypath2 = path_cls("/other/path")
        assert rypath1 != rypath2

    def test_truediv_operators(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath / "file.txt" == pypath / "file.txt"
        assert "file.txt" / rypath == "file.txt" / pypath
        assert rypath / Path("file.txt") == pypath / Path("file.txt")
        assert Path("file.txt") / rypath == Path("file.txt") / pypath

    def test_root(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.root == pypath.root

    def test_bytes(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        pathbytes_fslash = rypath.__bytes__().replace(b"\\", b"/")
        assert pathbytes_fslash == pypath.__bytes__().replace(
            b"\\", b"/"
        )  # todo: reevaluate

    def test_parts(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.parts == pypath.parts
        assert type(rypath.parts) is type(pypath.parts)
        assert isinstance(rypath.parts, tuple)


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
@pytest.mark.skipif(not is_windows, reason="Windows specific tests")
class TestFsPathWindows:
    def test_drive(self, path_cls: TPath) -> None:
        # windows
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        assert rypath.drive == pypath.drive

    def test_anchor(self, path_cls: TPath) -> None:
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        assert rypath.anchor == pypath.anchor

    def test_name(self, path_cls: TPath) -> None:
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        assert rypath.name == pypath.name

    def test_as_uri(self, path_cls: TPath) -> None:
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        if path_cls is ry.FsPath:
            with pytest.raises(NotImplementedError):
                rypath.as_uri()
        else:
            assert rypath.as_uri() == pypath.as_uri()
