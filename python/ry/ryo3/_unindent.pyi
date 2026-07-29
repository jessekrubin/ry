"""ryo3-unindent ~ types"""

def unindent(s: str, /) -> str:
    """Unindent a string.

    Examples
    --------
    >>> import ry
    >>> indented_str = '''
    ...     indented'''
    >>> ry.unindent(indented_str)
    'indented'

    """

def unindent_bytes(b: bytes, /) -> bytes:
    """Unindent a python bytes.

    Examples
    --------
    >>> import ry
    >>> indented_bytes = b'''
    ...     indented'''
    >>> ry.unindent_bytes(indented_bytes)
    b'indented'

    """
