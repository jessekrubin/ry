from __future__ import annotations

from typing import Literal

FORMAT_SIZE_BASE = Literal[2, 10]  # default=2
FORMAT_SIZE_STYLE = Literal[  # default="default"
    "default",
    "abbreviated",
    "abbreviated_lowercase",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
    "full_lowercase",
]

def fmt_size(
    n: int,
    *,
    base: FORMAT_SIZE_BASE | None = 2,
    style: FORMAT_SIZE_STYLE | None = "default",
) -> str:
    """Return human-readable string representation of bytes-size."""

def parse_size(s: str) -> int:
    """Return integer representation of human-readable bytes-size string.

    Raises:
        ValueError: If string is not a valid human-readable bytes-size string.
    """

class SizeFormatter:
    """Human-readable bytes-size formatter."""

    def __init__(
        self,
        base: FORMAT_SIZE_BASE | None = 2,
        style: FORMAT_SIZE_STYLE | None = "default",
    ):
        self.base = base
        self.style = style

    def format(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __call__(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __repr__(self) -> str: ...
