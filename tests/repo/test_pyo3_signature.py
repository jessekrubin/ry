from __future__ import annotations

import typing as t
from pathlib import Path

import pytest

import ry

ROOT = Path(__file__).resolve().parent.parent.parent

_RS_FILES = sorted(ROOT.glob("crates/**/*.rs"))

_RE_PYO3_ATTR_WITH_SIGNATURE = ry.Regex(
    r"(?s)#\s*\[\s*pyo3\s*\(.*?\bsignature\s*=?\s*\(.*?\)\s*\)\s*\]"
)
_RE_BAD_OUTER = ry.Regex(r"signature=\(")
_RE_BAD_INNER = ry.Regex(r"\b[a-zA-Z_][a-zA-Z0-9_]*=")


def _extract_signature_block(text: str, start: int, end: int) -> str | None:
    attr = text[start:end]
    sig_start = attr.find("signature")
    if sig_start == -1:
        return None

    paren_start = attr.find("(", sig_start)
    if paren_start == -1:
        return None

    depth = 0
    for i in range(paren_start, len(attr)):
        ch = attr[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return attr[sig_start : i + 1]

    return None


def _extract_signature_blocks(text: str) -> t.Generator[str, None, None]:
    return (
        sig
        for start, end in _RE_PYO3_ATTR_WITH_SIGNATURE.find_all(text)
        if (sig := _extract_signature_block(text, start, end)) is not None
    )


@pytest.mark.parametrize("filepath", _RS_FILES)
def test_pyo3_signature_spacing(filepath: Path) -> None:
    failures: list[str] = []

    text = filepath.read_text(encoding="utf-8")

    signature_blocks = list(_extract_signature_blocks(text))
    for sig in signature_blocks:
        if _RE_BAD_OUTER.is_match(sig):
            failures.append(f"{filepath}: use `signature = (`\n{sig}\n")
            continue

        inner_start = sig.find("(")
        inner_end = sig.rfind(")")
        inner = sig[inner_start + 1 : inner_end]

        for line in inner.splitlines():
            stripped = line.strip()
            if not stripped or stripped == "*":
                continue
            if _RE_BAD_INNER.is_match(stripped):
                failures.append(
                    f"{filepath}: use spaces around `=` inside signature entries\n{sig}\n"
                )
                break

    assert not failures, "\n".join(failures)
