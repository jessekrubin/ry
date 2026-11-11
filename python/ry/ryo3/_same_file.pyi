"""ryo3-same-file ~ types"""

from os import PathLike

def is_same_file(left: str | PathLike[str], right: str | PathLike[str]) -> bool: ...
