from __future__ import annotations

from pathlib import Path

import ry


def _test_setup(tmp_path: Path) -> None:
    """Setup function to create a temporary file for testing."""
    (tmp_path / "test_file.txt").write_text("This is a test file.")
    (tmp_path / "stuff.json").write_text('{"key": "value"}')
    (tmp_path / "test_dir").mkdir()
    (tmp_path / "test_dir" / "nested_file.txt").write_text(
        "This is a nested test file."
    )
    ry.cd(tmp_path)
    assert ry.pwd() == tmp_path.__fspath__()  # noqa: PLC2801


def test_glob_dtype_default_no_dtype_given(
    tmp_path: Path,
) -> None:
    """Test glob dtype"""
    _test_setup(tmp_path)
    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i)

    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i.collect())
    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i.take(1))


def test_glob_dtype_str(
    tmp_path: Path,
) -> None:
    """Test glob dtype"""
    _test_setup(tmp_path)

    i = ry.glob("*", dtype=str)
    assert all(isinstance(el, str) for el in i)

    i = ry.glob("*", dtype=str)
    assert all(isinstance(el, str) for el in i.collect())

    i = ry.glob("*", dtype=str)
    assert all(isinstance(el, str) for el in i.take(1))


def test_glob_dtype_path(
    tmp_path: Path,
) -> None:
    """Test glob dtype."""
    _test_setup(tmp_path)

    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i)

    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i.collect())
    i = ry.glob("*", dtype=Path)
    assert all(issubclass(el.__class__, Path) for el in i.take(1))


def test_glob_dtype_fspath(
    tmp_path: Path,
) -> None:
    """Test glob dtype."""
    _test_setup(tmp_path)

    i = ry.glob("*", dtype=ry.FsPath)
    assert all(isinstance(el, ry.FsPath) for el in i)

    i = ry.glob("*", dtype=ry.FsPath)
    assert all(isinstance(el, ry.FsPath) for el in i.collect())
    i = ry.glob("*", dtype=ry.FsPath)
    take_uno = i.take()
    assert all(isinstance(el, ry.FsPath) for el in take_uno)
