"""ry.ryo3.dev"""

import typing as t

# =============================================================================
# SUBPROCESS (VERY MUCH WIP)
# =============================================================================
def run(
    *args: str | list[str],
    capture_output: bool = True,
    input: bytes | None = None,  # noqa: A002
) -> t.Any: ...

# =============================================================================
# STRING-DEV
# =============================================================================

def anystr_noop(s: t.AnyStr) -> t.AnyStr: ...
def string_noop(s: str) -> str: ...
def bytes_noop(s: bytes) -> bytes: ...
