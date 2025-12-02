"""ryo3-same-file types"""

from os import PathLike

def is_same_file(left: PathLike[str], right: PathLike[str]) -> bool: ...
