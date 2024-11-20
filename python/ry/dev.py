"""dev entry point"""

from __future__ import annotations

from ry import _ry
from ry._ry import *  # noqa: F403

__version__ = _ry.__version__
__build_profile__ = _ry.__build_profile__
__build_timestamp__ = _ry.__build_timestamp__
if hasattr(_ry, "__all__"):
    __all__ = _ry.__all__

# assign all things in _ry to this module

for _k in dir(_ry):
    if not _k.startswith("_"):
        globals()[_k] = getattr(_ry, _k)
