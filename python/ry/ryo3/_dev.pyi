"""ry.ryo3.dev"""

import typing as t

from ry.ryo3._lz4rip import Lz4FrameInfo as Lz4FrameInfo
from ry.ryo3._lz4rip import lz4_compress as lz4_compress
from ry.ryo3._lz4rip import lz4_compress_block as lz4_compress_block
from ry.ryo3._lz4rip import lz4_decompress as lz4_decompress
from ry.ryo3._lz4rip import lz4_decompress_block as lz4_decompress_block
from ry.ryo3._lz4rip import lz4_train_dict as lz4_train_dict

def devfn() -> t.Literal["_ryo3-dev"]: ...
