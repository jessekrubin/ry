"""ryo3-globset types"""

import typing as t
from os import PathLike

@t.final
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
    def is_match_str(self, path: str) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    def __invert__(self) -> Glob: ...
    def globset(self) -> GlobSet: ...
    def globster(self) -> Globster: ...

@t.final
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
    def is_match_str(self, path: str) -> bool: ...
    def matches(self, path: str) -> list[int]: ...
    def __call__(self, path: str) -> bool: ...
    def globster(self) -> Globster: ...
    @property
    def patterns(self) -> tuple[str, ...]: ...

@t.final
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
    def is_match_str(self, path: str) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    @property
    def patterns(self) -> tuple[str, ...]: ...

def globster(
    patterns: list[str] | tuple[str, ...],
    /,
    *,
    case_insensitive: bool | None = None,
    literal_separator: bool | None = None,
    backslash_escape: bool | None = None,
) -> Globster: ...
