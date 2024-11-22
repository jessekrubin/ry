"""dev entry point"""

from __future__ import annotations

from ry import ryo3
from ry.ryo3 import *  # noqa: F403

__version__ = ryo3.__version__
__build_profile__ = ryo3.__build_profile__
__build_timestamp__ = ryo3.__build_timestamp__
if hasattr(ryo3, "__all__"):
    __all__ = ryo3.__all__

# assign all things in _ry to this module

for _k in dir(ryo3):
    if not _k.startswith("_"):
        globals()[_k] = getattr(ryo3, _k)
