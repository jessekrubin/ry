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
    print(tmp_fspath, type(tmp_fspath))
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


# def test_walkdir_objects_and_strings(tmp_path: Path) -> None:
#     dirtree = mk_dir_tree(tmp_path)
#     walkdir_strings = [
#         e if e != "" else "."
#         for e in (
#             str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
#             for f in ry.walkdir(tmp_path, files=True, dirs=False)
#         )
#     ]
#     assert all(isinstance(e, str) for e in walkdir_strings)
#     walkdir_paths = [
#         f for f in ry.walkdir(tmp_path, files=True, dirs=False)
#     ]
#
#     for thing in walkdir_paths:
#         print(thing, type(thing))
#     print(walkdir_paths)
#     assert all(isinstance(e, ry.FsPath) for e in walkdir_paths)
#
#     assert set(walkdir_strings) == set(map(str, dirtree.filepaths))


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
