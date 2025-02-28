"""ryo3-regex types"""

from __future__ import annotations

# =============================================================================
# Regex
# =============================================================================

class Regex:
    def __init__(
        self,
        pattern: str,
        *,
        case_insensitive: bool = False,
        crlf: bool = False,
        dot_matches_new_line: bool = False,
        ignore_whitespace: bool = False,
        line_terminator: str | None = None,
        multi_line: bool = False,
        octal: bool = False,
        size_limit: int | None = None,
        swap_greed: bool = False,
        unicode: bool = False,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def is_match(self, string: str) -> bool: ...
