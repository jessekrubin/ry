from __future__ import annotations

from pathlib import Path

import ry


def test_glob_dtype_str() -> None:
    """Test glob dtype."""

    i = ry.glob("*", dtype=str)
    assert all(isinstance(el, str) for el in i)

    i = ry.glob("*", dtype=str)
    assert all(isinstance(el, str) for el in i.collect())

    i = ry.glob("*", dtype=str)
    assert all(isinstance(el, str) for el in i.take(1))


def test_glob_dtype_path() -> None:
    """Test glob dtype."""

    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i)

    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i.collect())
    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i.take(1))


def test_glob_dtype_fspath() -> None:
    """Test glob dtype."""

    i = ry.glob("*", dtype=ry.FsPath)
    assert all(isinstance(el, ry.FsPath) for el in i)

    i = ry.glob("*", dtype=ry.FsPath)
    assert all(isinstance(el, ry.FsPath) for el in i.collect())
    i = ry.glob("*", dtype=ry.FsPath)
