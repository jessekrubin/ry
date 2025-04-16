"""ryo3-same-file types"""

from os import PathLike

def is_same_file(a: PathLike[str], b: PathLike[str]) -> bool: ...
