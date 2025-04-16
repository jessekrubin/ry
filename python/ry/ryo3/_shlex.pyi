"""ryo3-shlex types"""

def shplit(s: str) -> list[str]:
    """shlex::split wrapper much like python's stdlib shlex.split but faster"""
