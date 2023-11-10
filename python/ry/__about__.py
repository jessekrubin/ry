"""Package metadata/info"""
from utiles._utiles import __build_profile__, __version_lib__
__all__ = (
    "__title__",
    "__description__",
    "__pkgroot__",
    "__version__",
    "__version_lib__",
    "__build_profile__",
)
__title__ = "utiles"
__description__ = "utiles = utils + tiles + rust"
__pkgroot__ = __file__.replace("__about__.py", "").rstrip("/\\")
__version__ = __version_lib__
