# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "ry",
# ]
# ///
"""mdbook preprocessor to inject build stuff into docs

BASED ON: https://github.com/PyO3/pyo3/blob/main/guide/pyo3_version.py

Replaces:
  - `{{#RY_DOCS_BUILD_TIMESTAMP}}` with build timestamp

"""

import datetime
import json
import sys


def _tokens():
    try:
        import ry

        return {
            "RY_DOCS_BUILD_TIMESTAMP": ry.ZonedDateTime.now().string(),
        }
    except ImportError:
        return {
            "RY_DOCS_BUILD_TIMESTAMP": datetime.datetime.now(
                tz=datetime.timezone.utc
            ).isoformat(),
        }


TOKENS = {"{{#" + k + "}}": v for k, v in _tokens().items()}


def replace_tokens(content):
    for token, value in TOKENS.items():
        content = content.replace(token, value)
    return content


def replace_section_content(section):
    if not isinstance(section, dict) or "Chapter" not in section:
        return

    # Replace raw and url-encoded forms
    section["Chapter"]["content"] = replace_tokens(section["Chapter"]["content"])
    for sub_item in section["Chapter"]["sub_items"]:
        replace_section_content(sub_item)


def main():
    for line in sys.stdin:
        if line:
            [_context, book] = json.loads(line)
            for section in book["sections"]:
                replace_section_content(section)
            json.dump(book, fp=sys.stdout)


if __name__ == "__main__":
    main()
