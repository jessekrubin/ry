"""Package metadata/info"""
from ry._ry import __build_profile__, __version__
__all__ = (
    "__title__",
    "__description__",
    "__pkgroot__",
    "__version__",
    "__build_profile__",
)
__title__ = "utiles"
__description__ = "utiles = utils + tiles + rust"
__pkgroot__ = __file__.replace("__about__.py", "").rstrip("/\\")
