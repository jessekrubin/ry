from pathlib import Path

import ry


def test_ls(tmp_path: Path) -> None:
    (tmp_path / "a.txt").write_text("hello")
    (tmp_path / "b.txt").write_text("world")
    assert set(ry.ls(tmp_path)) == {"a.txt", "b.txt"}


def test_ls_pathlib(tmp_path: Path) -> None:
    ry.cd(tmp_path)
    (tmp_path / "a.txt").write_text("hello")
    (tmp_path / "b.txt").write_text("world")
    assert set(ry.ls()) == {"a.txt", "b.txt"}
