from __future__ import annotations

from typing import TYPE_CHECKING

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
