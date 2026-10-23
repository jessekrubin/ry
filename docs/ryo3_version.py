# SPDX-License-Identifier: MIT OR Apache-2.0
# ruff: noqa: S101
# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "orjson",
#     "ry",
# ]
# ///
"""mdbook preprocessor to inject build stuff into docs

BASED ON: https://github.com/PyO3/pyo3/blob/main/guide/pyo3_version.py

Replaces:
  - `{{#RY_DOCS_BUILD_TIMESTAMP}}` with build timestamp

# TODO: switch to using ry
"""

from __future__ import annotations

import sys
import typing as t

import ry  # ty: ignore[unresolved-import]

_DEBUG = False


def _tokens() -> dict[str, str]:
    return {
        "RY_DOCS_BUILD_TIMESTAMP": str(
            ry.ZonedDateTime.now().in_tz("America/Los_Angeles")
        ),
    }


def _dbg(*args: t.Any) -> None:  # ruff: ignore[any-type]
    print(*args, file=sys.stderr)  # ruff: ignore[print]


def _noop(*args: t.Any) -> None:  # ruff: ignore[any-type]
    pass


dbg = _dbg if _DEBUG else _noop

_TOKENS = {"{{#" + k + "}}": v for k, v in _tokens().items()}


def _replace_tokens(content: str) -> str:
    for token, value in _TOKENS.items():
        if token in content:
            content = content.replace(token, value)
    return content


def _replace_section_content(section: dict[str, t.Any] | None) -> None:
    if not isinstance(section, dict):
        return
    if "Chapter" not in section:
        return
    # Replace raw and url-encoded forms
    section["Chapter"]["content"] = _replace_tokens(section["Chapter"]["content"])
    for sub_item in section["Chapter"]["sub_items"]:
        _replace_section_content(sub_item)


def _main() -> None:
    for line in sys.stdin:
        if line:
            parsed = ry.JSON.parse(line)
            assert isinstance(parsed, list)
            assert len(parsed) == 2
            [_context, book] = parsed
            assert isinstance(book, dict)
            assert "items" in book
            assert isinstance(book["items"], list)
            for section in book["items"]:
                if not isinstance(section, dict):
                    continue
                _replace_section_content(section)
            b = ry.JSON.dumps(book, append_newline=True)
            sys.stdout.buffer.write(b)


if __name__ == "__main__":
    _main()
