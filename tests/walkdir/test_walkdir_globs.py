from __future__ import annotations

import typing as t

import pytest

import ry

from .walkdir_utils import mk_dir_tree

if t.TYPE_CHECKING:
    from pathlib import Path


@pytest.mark.parametrize(
    "glob_type_factory",
    [
        lambda g: g,
        lambda g: g.globset(),
        lambda g: g.globster(),
    ],
)
def test_walkdir_with_glob(
    tmp_path: Path,
    glob_type_factory: t.Callable[[ry.Glob], ry.GlobSet | ry.Glob | ry.Globster],
) -> None:
    ry_glob = ry.Glob("*.txt")
    dirtree = mk_dir_tree(tmp_path)

    glob_type = glob_type_factory(ry_glob)

    walkdir_it = ry.walkdir(tmp_path, files=True, dirs=False, glob=glob_type)
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in walkdir_it
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = {el for el in map(str, dirtree.filepaths) if el.endswith(".txt")}
    assert walkdir_paths_set == expected, (
        f"walkdir_paths_set: {walkdir_paths_set} expected: {expected}"
    )

    assert all(p.endswith(".txt") for p in walkdir_paths_set), (
        f"walkdir_paths_set: {walkdir_paths_set} type: {type(walkdir_paths_set)}"
    )


@pytest.mark.parametrize(
    "glob_strings",
    [
        "*.txt",  # string
        ["*.txt"],  # list
        ("*.txt",),  # tuple
    ],
)
def test_walkdir_with_glob_strings(
    tmp_path: Path,
    glob_strings: str | list[str] | tuple[str],
) -> None:
    dirtree = mk_dir_tree(tmp_path)
    walkdir_it = ry.walkdir(tmp_path, files=True, dirs=False, glob=glob_strings)
    walkdir_paths = [
        e if e != "" else "."
        for e in (
            str(f).replace(str(tmp_path), "").lstrip("/").lstrip("\\")
            for f in walkdir_it
        )
    ]
    walkdir_paths_set = set(walkdir_paths)
    expected = {el for el in map(str, dirtree.filepaths) if el.endswith(".txt")}
    assert walkdir_paths_set == expected, (
        f"walkdir_paths_set: {walkdir_paths_set} expected: {expected}"
    )

    assert all(p.endswith(".txt") for p in walkdir_paths_set), (
        f"walkdir_paths_set: {walkdir_paths_set} type: {type(walkdir_paths_set)}"
    )
