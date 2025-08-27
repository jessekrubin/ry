from __future__ import annotations

from pathlib import Path

import ry

from .walkdir_utils import mk_dir_tree


def test_walk_dir_dirpath_string(tmp_path: Path) -> None:
    dirtree = mk_dir_tree(tmp_path)
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in ry.walkdir(str(tmp_path))
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = set(map(str, dirtree.dirpaths.union(dirtree.filepaths)))
    assert walkdir_paths_set == expected


def test_walk_dir_dirpath_string_collect(tmp_path: Path) -> None:
    dirtree = mk_dir_tree(tmp_path)
    walkdir_iterable = ry.walkdir(str(tmp_path))
    walkdir_list = walkdir_iterable.collect()
    assert isinstance(walkdir_list, list)
    post_collect_iter = list(walkdir_iterable)
    assert not post_collect_iter

    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in walkdir_list
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = set(map(str, dirtree.dirpaths.union(dirtree.filepaths)))
    assert walkdir_paths_set == expected


def test_walkdir_types(tmp_path: Path) -> None:
    _dirtree = mk_dir_tree(tmp_path)
    assert all(isinstance(e, str) for e in ry.walkdir(tmp_path))
    assert all(isinstance(e, str) for e in ry.walkdir(tmp_path).collect())
    assert all(isinstance(e, str) for e in ry.walkdir(tmp_path).take())

    assert all(not isinstance(e, str) for e in ry.walkdir(tmp_path, objects=True))
    assert all(
        not isinstance(e, str) for e in ry.walkdir(tmp_path, objects=True).collect()
    )
    assert all(
        not isinstance(e, str) for e in ry.walkdir(tmp_path, objects=True).take()
    )


def test_walk_dir_dirpath_pathlib_path(tmp_path: Path) -> None:
    dirtree = mk_dir_tree(tmp_path)
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in ry.walkdir(tmp_path)
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = set(map(str, dirtree.dirpaths.union(dirtree.filepaths)))
    assert walkdir_paths_set == expected


def test_walk_dir_dirpath_none_use_pwd(tmp_path: Path) -> None:
    dirtree = mk_dir_tree(tmp_path)
    tmp_fspath = ry.FsPath(tmp_path)
    ry.cd(tmp_fspath)
    assert ry.pwd() == tmp_fspath
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in ry.walkdir(tmp_path)
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = set(map(str, dirtree.dirpaths.union(dirtree.filepaths)))
    assert walkdir_paths_set == expected


def test_walk_dir_dirpath_string_files_only(tmp_path: Path) -> None:
    dirtree = mk_dir_tree(tmp_path)
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in ry.walkdir(tmp_path, files=True, dirs=False)
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = set(map(str, dirtree.filepaths))
    assert walkdir_paths_set == expected


if __name__ == "__main__":
    mk_dir_tree(Path("../../crates/_ryo3-dev/src"))
