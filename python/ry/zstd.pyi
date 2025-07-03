from ry.ryo3.zstd import BLOCKSIZE_MAX as BLOCKSIZE_MAX
from ry.ryo3.zstd import BLOCKSIZELOG_MAX as BLOCKSIZELOG_MAX
from ry.ryo3.zstd import CLEVEL_DEFAULT as CLEVEL_DEFAULT
from ry.ryo3.zstd import CONTENTSIZE_ERROR as CONTENTSIZE_ERROR
from ry.ryo3.zstd import CONTENTSIZE_UNKNOWN as CONTENTSIZE_UNKNOWN
from ry.ryo3.zstd import MAGIC_DICTIONARY as MAGIC_DICTIONARY
from ry.ryo3.zstd import MAGIC_SKIPPABLE_MASK as MAGIC_SKIPPABLE_MASK
from ry.ryo3.zstd import MAGIC_SKIPPABLE_START as MAGIC_SKIPPABLE_START
from ry.ryo3.zstd import MAGICNUMBER as MAGICNUMBER
from ry.ryo3.zstd import VERSION_MAJOR as VERSION_MAJOR
from ry.ryo3.zstd import VERSION_MINOR as VERSION_MINOR
from ry.ryo3.zstd import VERSION_NUMBER as VERSION_NUMBER
from ry.ryo3.zstd import VERSION_RELEASE as VERSION_RELEASE
from ry.ryo3.zstd import __zstd_version__ as __zstd_version__
from ry.ryo3.zstd import compress as compress
from ry.ryo3.zstd import decode as decode
from ry.ryo3.zstd import decompress as decompress
from ry.ryo3.zstd import is_zstd as is_zstd
from ry.ryo3.zstd import unzstd as unzstd

__all__ = (
    "BLOCKSIZELOG_MAX",
    "BLOCKSIZE_MAX",
    "CLEVEL_DEFAULT",
    "CONTENTSIZE_ERROR",
    "CONTENTSIZE_UNKNOWN",
    "MAGICNUMBER",
    "MAGIC_DICTIONARY",
    "MAGIC_SKIPPABLE_MASK",
    "MAGIC_SKIPPABLE_START",
    "VERSION_MAJOR",
    "VERSION_MINOR",
    "VERSION_NUMBER",
    "VERSION_RELEASE",
    "__zstd_version__",
    "compress",
    "decode",
    "decompress",
    "is_zstd",
    "unzstd",
)
