"""ryo3-same-file types"""

from __future__ import annotations

from os import PathLike

def is_same_file(a: PathLike[str], b: PathLike[str]) -> bool: ...
