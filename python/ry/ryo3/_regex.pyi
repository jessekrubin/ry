"""ryo3-regex types"""

import typing as t

# =============================================================================
# Regex
# =============================================================================

@t.final
class Regex:
    def __init__(
        self,
        pattern: str,
        *,
        case_insensitive: bool = False,
        crlf: bool = False,
        dot_matches_new_line: bool = False,
        ignore_whitespace: bool = False,
        line_terminator: bytes | int | None = None,
        multi_line: bool = False,
        octal: bool = False,
        size_limit: int | None = None,
        swap_greed: bool = False,
        unicode: bool = False,
    ) -> None: ...
    def is_match(self, haystack: str) -> bool: ...
    def test(self, haystack: str) -> bool: ...
    def find(self, haystack: str) -> str | None: ...
    def find_all(self, haystack: str) -> list[tuple[int, int]]: ...
    def findall(self, haystack: str) -> list[tuple[int, int]]: ...
    def replace(self, haystack: str, replacement: str) -> str: ...
    def replace_all(self, haystack: str, replacement: str) -> str: ...
    def split(self, haystack: str) -> list[str]: ...
    def splitn(self, haystack: str, n: int) -> list[str]: ...
