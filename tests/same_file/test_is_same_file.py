from __future__ import annotations

from typing import TYPE_CHECKING

import ry

if TYPE_CHECKING:
    from pathlib import Path


def test_is_same_file(tmp_path: Path) -> None:
    a = tmp_path / "a"
    a.write_text("content")
    assert ry.is_same_file(a, a)


def test_same_file_symlink(tmp_path: Path) -> None:
    a = tmp_path / "a"
    a.write_text("content")
    b = tmp_path / "b"
    b.symlink_to(a)
    assert ry.is_same_file(a, b)


def test_not_same_file(tmp_path: Path) -> None:
    a = tmp_path / "a"
    a.write_text("content")
    b = tmp_path / "b"  # different file
    b.write_text("poopy")
    assert not ry.is_same_file(a, b)
