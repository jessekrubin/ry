"""ryo3-globset types"""

from __future__ import annotations

from os import PathLike

class Glob:
    """globset::Glob wrapper"""

    def __init__(
        self,
        pattern: str,
        /,
        *,
        case_insensitive: bool | None = None,
        literal_separator: bool | None = None,
        backslash_escape: bool | None = None,
    ) -> None: ...
    def regex(self) -> str: ...
    def is_match(self, path: str | PathLike[str]) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    def __invert__(self) -> Glob: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def globset(self) -> GlobSet: ...
    def globster(self) -> Globster: ...

class GlobSet:
    """globset::GlobSet wrapper"""

    def __init__(
        self,
        patterns: list[str],
        /,
        *,
        case_insensitive: bool | None = None,
        literal_separator: bool | None = None,
        backslash_escape: bool | None = None,
    ) -> None: ...
    def is_empty(self) -> bool: ...
    def is_match(self, path: str) -> bool: ...
    def matches(self, path: str) -> list[int]: ...
    def __call__(self, path: str) -> bool: ...
    def __invert__(self) -> GlobSet: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def globster(self) -> Globster: ...

class Globster:
    """Globster is a matcher with claws!

    Note: The north american `Globster` is similar to the european `Globset`
          but allows for negative patterns (prefixed with '!')

    """

    def __init__(
        self,
        patterns: list[str],
        /,
        *,
        case_insensitive: bool | None = None,
        literal_separator: bool | None = None,
        backslash_escape: bool | None = None,
    ) -> None: ...
    def is_empty(self) -> bool: ...
    def is_match(self, path: str | PathLike[str]) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    def __invert__(self) -> GlobSet: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

def globster(
    patterns: list[str] | tuple[str, ...],
    /,
    *,
    case_insensitive: bool | None = None,
    literal_separator: bool | None = None,
    backslash_escape: bool | None = None,
) -> Globster: ...
