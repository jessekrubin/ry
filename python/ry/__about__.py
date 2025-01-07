"""Package metadata/info"""

from __future__ import annotations

from ry.ryo3 import __authors__, __build_profile__, __build_timestamp__, __version__

__all__ = (
    "__authors__",
    "__build_profile__",
    "__build_timestamp__",
    "__description__",
    "__pkgroot__",
    "__title__",
    "__version__",
)
__title__ = "ry"
__description__ = "ry = rust + python - most of the letters"
__pkgroot__ = __file__.replace("__about__.py", "").rstrip("/\\")
