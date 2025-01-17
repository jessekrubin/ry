import sys
from typing import NoReturn

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

class Bytes(Buffer):
    """
    A buffer implementing the Python buffer protocol, allowing zero-copy access
    to underlying Rust memory.

    You can pass this to `memoryview` for a zero-copy view into the underlying
    data or to `bytes` to copy the underlying data into a Python `bytes`.

    Many methods from the Python `bytes` class are implemented on this,
    """

    def __new__(cls, Buffer) -> Bytes:
        """
        Create a new `Bytes` object from a buffer-like object.
        """

    def __add__(self, other: Buffer) -> Bytes: ...
    def __contains__(self, other: Buffer) -> bool: ...
    def __eq__(self, other: object) -> bool: ...
    def __getitem__(self, other: int) -> int: ...
    def __mul__(self, other: Buffer) -> int: ...
    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...
    def removeprefix(self, prefix: Buffer, /) -> Bytes:
        """
        If the binary data starts with the prefix string, return `bytes[len(prefix):]`.
        Otherwise, return the original binary data.
        """

    def removesuffix(self, suffix: Buffer, /) -> Bytes:
        """
        If the binary data ends with the suffix string and that suffix is not empty,
        return `bytes[:-len(suffix)]`. Otherwise, return the original binary data.
        """

    def isalnum(self) -> bool:
        """
        Return `True` if all bytes in the sequence are alphabetical ASCII characters or
        ASCII decimal digits and the sequence is not empty, `False` otherwise.

        Alphabetic ASCII characters are those byte values in the sequence
        `b'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'`. ASCII decimal digits
        are those byte values in the sequence `b'0123456789'`.
        """

    def isalpha(self) -> bool:
        """
        Return `True` if all bytes in the sequence are alphabetic ASCII characters and
        the sequence is not empty, `False` otherwise.

        Alphabetic ASCII characters are those byte values in the sequence
        `b'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'`.
        """

    def isascii(self) -> bool:
        """
        Return `True` if the sequence is empty or all bytes in the sequence are ASCII,
        `False` otherwise.

        ASCII bytes are in the range `0-0x7F`.
        """

    def isdigit(self) -> bool:
        """
        Return `True` if all bytes in the sequence are ASCII decimal digits and the
        sequence is not empty, `False` otherwise.

        ASCII decimal digits are those byte values in the sequence `b'0123456789'`.
        """

    def islower(self) -> bool:
        """
        Return `True` if there is at least one lowercase ASCII character in the sequence
        and no uppercase ASCII characters, `False` otherwise.
        """

    def isspace(self) -> bool:
        """
        Return `True` if all bytes in the sequence are ASCII whitespace and the sequence
        is not empty, `False` otherwise.

        ASCII whitespace characters are those byte values
        in the sequence `b' \t\n\r\x0b\f'` (space, tab, newline, carriage return,
        vertical tab, form feed).
        """

    def isupper(self) -> bool:
        """
        Return `True` if there is at least one uppercase alphabetic ASCII character in
        the sequence and no lowercase ASCII characters, `False` otherwise.
        """

    def lower(self) -> Bytes:
        """
        Return a copy of the sequence with all the uppercase ASCII characters converted
        to their corresponding lowercase counterpart.
        """

    def upper(self) -> Bytes:
        """
        Return a copy of the sequence with all the lowercase ASCII characters converted
        to their corresponding uppercase counterpart.
        """

    def to_bytes(self) -> bytes:
        """Copy this buffer's contents into a Python `bytes` object."""

    # =========================================================================
    # IMPL IN RY
    # =========================================================================
    def hex(self, sep: str | None = None, bytes_per_sep: int | None = None) -> str: ...

    # =========================================================================
    # NOT IMPLEMENTED
    # =========================================================================
    def __bytes__(self) -> NoReturn: ...
    def __getnewargs__(self) -> NoReturn: ...
    def __iter__(self) -> NoReturn: ...
    def __mod__(self) -> NoReturn: ...
    def __rmod__(self) -> NoReturn: ...
    def capitalize(self) -> NoReturn: ...
    def center(self) -> NoReturn: ...
    def count(self) -> NoReturn: ...
    def decode(self) -> NoReturn: ...
    def endswith(self) -> NoReturn: ...
    def expandtabs(self) -> NoReturn: ...
    def find(self) -> NoReturn: ...
    def fromhex(self) -> NoReturn: ...
    def index(self) -> NoReturn: ...
    def istitle(self) -> NoReturn: ...
    def join(self) -> NoReturn: ...
    def ljust(self) -> NoReturn: ...
    def lstrip(self) -> NoReturn: ...
    def maketrans(self) -> NoReturn: ...
    def partition(self) -> NoReturn: ...
    def replace(self) -> NoReturn: ...
    def rfind(self) -> NoReturn: ...
    def rindex(self) -> NoReturn: ...
    def rjust(self) -> NoReturn: ...
    def rpartition(self) -> NoReturn: ...
    def rsplit(self) -> NoReturn: ...
    def rstrip(self) -> NoReturn: ...
    def split(self) -> NoReturn: ...
    def splitlines(self) -> NoReturn: ...
    def startswith(self) -> NoReturn: ...
    def strip(self) -> NoReturn: ...
    def swapcase(self) -> NoReturn: ...
    def title(self) -> NoReturn: ...
    def translate(self) -> NoReturn: ...
    def zfill(self) -> NoReturn: ...
