import sys

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

from ry.ryo3 import (
    ULID,
)

deprecated("`ry.ulid.ULID` is deprecated; use `ry.ULID` instead [removal: 0.0.96]")

__all__ = ("ULID",)
