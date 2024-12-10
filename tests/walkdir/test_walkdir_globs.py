from __future__ import annotations

from pathlib import Path

import ry

from .walkdir_utils import mk_dir_tree


def test_walkdir_with_glob(tmp_path: Path) -> None:
    globset = ry.glob("*.txt").globset()
    dirtree = mk_dir_tree(tmp_path)
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in ry.walkdir(tmp_path, files=True, dirs=False, globs=globset)
        )
    ]

    walkdir_paths_set = set(walkdir_paths)
    expected = {el for el in map(str, dirtree.filepaths) if el.endswith(".txt")}
    assert walkdir_paths_set == expected

    assert all(p.endswith(".txt") for p in walkdir_paths_set)
