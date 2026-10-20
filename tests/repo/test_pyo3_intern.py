from __future__ import annotations

from pathlib import Path

import pytest

import ry

_REPO_ROOT = Path(__file__).resolve().parent.parent.parent

_RS_FILES = sorted([
    *_REPO_ROOT.glob("crates/**/*.rs"),
    *_REPO_ROOT.glob("src/**/*.rs"),
])

# any (possibly path-qualified) `intern!` macro invocation
_RE_INTERN_MACRO = ry.Regex(r"[\w:]*\bintern!")
# `use pyo3::intern;` or `use pyo3::{..., intern, ...};`
_RE_USE_INTERN = ry.Regex(r"(?s)\buse\s+pyo3::(?:intern\s*;|\{[^}]*\bintern\b[^}]*\})")


def _line_no(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1


@pytest.mark.parametrize("filepath", _RS_FILES)
def test_pyo3_intern_fully_qualified(filepath: Path) -> None:
    """`intern!` must always be written as `pyo3::intern!` (and never imported)"""
    text = filepath.read_text(encoding="utf-8")
    rel = filepath.relative_to(_REPO_ROOT).as_posix()
    failures: list[str] = [
        # intern calls
        *(
            f"{rel}:{_line_no(text, start)}: use `pyo3::intern!` not `{text[start:end]}`"
            for start, end in _RE_INTERN_MACRO.find_all(text)
            if text[start:end] not in {"pyo3::intern!", "::pyo3::intern!"}
        ),
        # intern imports
        *(
            f"{rel}:{_line_no(text, start)}: do not import `intern`; use `pyo3::intern!`"
            for start, _end in _RE_USE_INTERN.find_all(text)
        ),
    ]
    assert not failures, "\n".join(failures)
