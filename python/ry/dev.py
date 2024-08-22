"""dev entry point"""

from ry import _ry
from ry._ry import *

__version__ = _ry.__version__
__build_profile__ = _ry.__build_profile__
__build_timestamp__ = _ry.__build_timestamp__
if hasattr(_ry, "__all__"):
    __all__ = _ry.__all__
