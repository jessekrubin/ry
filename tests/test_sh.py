from __future__ import annotations

import os
import pathlib
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pathlib import Path


def test_ls(tmp_path: Path) -> None:
    (tmp_path / "a.txt").write_text("hello")
    (tmp_path / "b.txt").write_text("world")
    assert set(ry.ls(tmp_path)) == {"a.txt", "b.txt"}


def test_ls_objects(tmp_path: Path) -> None:
    (tmp_path / "a.txt").write_text("hello")
    (tmp_path / "b.txt").write_text("world")
    paths = ry.ls(tmp_path, objects=True)
    assert all(isinstance(p, ry.FsPath) for p in paths)
    assert {str(e) for e in ry.ls(tmp_path, objects=True)} == {
        "a.txt",
        "b.txt",
    }


def test_ls_pathlib(tmp_path: Path) -> None:
    ry.cd(tmp_path)
    (tmp_path / "a.txt").write_text("hello")
    (tmp_path / "b.txt").write_text("world")
    assert set(ry.ls()) == {"a.txt", "b.txt"}


def test_pwd() -> None:
    assert ry.pwd() == os.getcwd()


class TestRyPath:
    def test_path(self) -> None:
        p = ry.FsPath(os.getcwd())
        assert p == pathlib.Path(os.getcwd())


def test_cd() -> None:
    old_pwd = ry.pwd()
    ry.cd("..")
    assert ry.pwd() != old_pwd
    assert ry.pwd() == os.path.dirname(old_pwd)
    ry.cd(old_pwd)
    assert ry.pwd() == old_pwd


def test_cd_pathlib_object() -> None:
    new_dir = pathlib.Path("..")
    old_pwd = ry.pwd()
    ry.cd(new_dir)
    assert ry.pwd() != old_pwd
    assert ry.pwd() == os.path.dirname(old_pwd)


def test_cd_pathlib_nonexistent() -> None:
    new_dir = pathlib.Path("nonexistent")
    old_pwd = ry.pwd()
    with pytest.raises(FileNotFoundError):
        ry.cd(new_dir)
    assert ry.pwd() == old_pwd


def test_cd_nonexistent_ry() -> None:
    with pytest.raises(FileNotFoundError):
        ry.cd("nonexistent")


def test_cd_nonexistent_py() -> None:
    with pytest.raises(FileNotFoundError):
        os.chdir("nonexistent")
