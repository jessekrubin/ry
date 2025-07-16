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

import orjson

import ry


def _tokens() -> dict[str, str]:
    return {
        "RY_DOCS_BUILD_TIMESTAMP": ry.ZonedDateTime.now().string(),
    }


TOKENS = {"{{#" + k + "}}": v for k, v in _tokens().items()}


def replace_tokens(content: str) -> str:
    for token, value in TOKENS.items():
        if token in content:
            content = content.replace(token, value)
    return content


def replace_section_content(section: dict[str, t.Any] | None) -> None:
    if not isinstance(section, dict) or "Chapter" not in section:
        return

    # Replace raw and url-encoded forms
    section["Chapter"]["content"] = replace_tokens(section["Chapter"]["content"])
    for sub_item in section["Chapter"]["sub_items"]:
        replace_section_content(sub_item)


def main():
    for line in sys.stdin:
        if line:
            [_context, book] = orjson.loads(line)
            for section in book["sections"]:
                replace_section_content(section)
            b = orjson.dumps(book, option=orjson.OPT_APPEND_NEWLINE)
            sys.stdout.buffer.write(b)


if __name__ == "__main__":
    main()
