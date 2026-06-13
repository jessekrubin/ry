import sys
import typing as t

from ry.protocols import RyIterator

if sys.version_info >= (3, 12):
    from collections.abc import Buffer as Buffer
else:
    from typing_extensions import Buffer as Buffer

class Bytes(Buffer):
    """
    A buffer implementing the Python buffer protocol, allowing zero-copy access
    to underlying Rust memory.

    You can pass this to `memoryview` for a zero-copy view into the underlying
    data or to `bytes` to copy the underlying data into a Python `bytes`.

    Many methods from the Python `bytes` class are implemented on this,
    """

    def __new__(cls, buf: Buffer = b"") -> t.Self:
        """Construct a new Bytes object.

        This will be a zero-copy view on the Python byte slice.
        """

    @staticmethod
    def copy_from(buf: Buffer) -> Bytes:
        """Construct a new Bytes object by copying the given buffer.

        Examples
        --------
        >>> from ry import Bytes
        >>> b = Bytes.copy_from(b"hello")
        >>> b
        Bytes(b"hello")

        """
    def __add__(self, other: Buffer) -> Bytes: ...
    def __buffer__(self, flags: int) -> memoryview: ...
    def __contains__(self, other: Buffer) -> bool: ...
    def __eq__(self, other: object) -> bool: ...
    @t.overload
    def __getitem__(self, other: int) -> int: ...
    @t.overload
    def __getitem__(self, other: slice) -> Bytes: ...
    def __mul__(self, other: int) -> Bytes: ...
    def __rmul__(self, other: int) -> Bytes: ...
    def __len__(self) -> int: ...
    def __bytes__(self) -> bytes:
        """Return the underlying data as a Python `bytes` object."""
    def __iter__(self) -> t.Iterator[int]: ...
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

    def istitle(self) -> bool:
        """
        Return `True` if the sequence is non-empty and contains only ASCII letters,
        digits, underscores, and hyphens, and starts with an ASCII letter or underscore.
        Otherwise, return `False`.
        """
    def decode(self, encoding: str = "utf-8", errors: str = "strict") -> str:
        """Decode the binary data using the given encoding."""

    def hex(self, sep: str | None = None, *, bytes_per_sep: int = 1) -> str:
        """Return a hexadecimal representation of the binary data."""

    @classmethod
    def fromhex(cls, string: str) -> Bytes:
        """Construct a `Bytes` object from a hexadecimal string."""
    def startswith(self, prefix: Buffer, /) -> bool:
        """Return `True` if the binary data starts with the prefix string, `False` otherwise."""
    def endswith(self, suffix: Buffer, /) -> bool:
        """Return `True` if the binary data ends with the suffix string, `False` otherwise."""
    def capitalize(self) -> Bytes:
        """
        Return a copy of the sequence with the first byte converted to uppercase and
        all other bytes converted to lowercase.
        """
    def strip(self, chars: Buffer | None = None, /) -> Bytes:
        """
        Return a copy of the sequence with leading and trailing bytes removed.
        If `chars` is provided, remove all bytes in `chars` from both ends.
        If `chars` is not provided, remove all ASCII whitespace bytes.
        """
    def lstrip(self, chars: Buffer | None = None, /) -> Bytes:
        """
        Return a copy of the sequence with leading bytes removed.
        If `chars` is provided, remove all bytes in `chars` from the left end.
        If `chars` is not provided, remove all ASCII whitespace bytes.
        """
    def rstrip(self, chars: Buffer | None = None, /) -> Bytes:
        """
        Return a copy of the sequence with trailing bytes removed.
        If `chars` is provided, remove all bytes in `chars` from the right end.
        If `chars` is not provided, remove all ASCII whitespace bytes.
        """
    def expandtabs(self, tabsize: int = 8) -> Bytes:
        """
        Return a copy of the sequence with all ASCII tab characters replaced by spaces.
        The number of spaces is determined by the `tabsize` parameter.
        """
    def title(self) -> Bytes:
        """
        Return a copy of the sequence with the first byte of each word converted to
        uppercase and all other bytes converted to lowercase.
        """
    def swapcase(self) -> Bytes:
        """
        Return a copy of the sequence with all uppercase ASCII characters converted to
        their corresponding lowercase counterpart and vice versa.
        """
    def replace(self, old: Buffer, new: Buffer, count: int = -1, /) -> Bytes:
        """
        Return a copy of the sequence with all occurrences of `old` replaced by `new`.
        If `count` is given and not negative, only the first `count` occurrences are
        replaced.
        """
    def find(
        self,
        sub: Buffer | int,
        start: int | None = None,
        end: int | None = None,
        /,
    ) -> int:
        """Return the lowest index where `sub` is found, or `-1`."""
    def rfind(
        self,
        sub: Buffer | int,
        start: int | None = None,
        end: int | None = None,
        /,
    ) -> int:
        """Return the highest index where `sub` is found, or `-1`."""
    def index(
        self,
        sub: Buffer | int,
        start: int | None = None,
        end: int | None = None,
        /,
    ) -> int:
        """Return the lowest index where `sub` is found or raise `ValueError`."""
    def rindex(
        self,
        sub: Buffer | int,
        start: int | None = None,
        end: int | None = None,
        /,
    ) -> int:
        """Return the highest index where `sub` is found or raise `ValueError`."""
    def partition(self, sep: Buffer, /) -> tuple[Bytes, Bytes, Bytes]:
        """Partition the bytes into three parts using the given separator.

        This will search for the separator sep in the bytes. If the separator is found,
        returns a 3-tuple containing the part before the separator, the separator
        itself, and the part after it.

        If the separator is not found, returns a 3-tuple containing the original bytes
        object and two empty bytes objects.
        """
    def rpartition(self, sep: Buffer, /) -> tuple[Bytes, Bytes, Bytes]:
        """Partition the bytes into three parts using the given separator.

        This will search for the separator sep in the bytes, starting at the end. If
        the separator is found, returns a 3-tuple containing the part before the
        separator, the separator itself, and the part after it.

        If the separator is not found, returns a 3-tuple containing two empty bytes
        objects and the original bytes object.
        """
    def is_unique(self) -> bool:
        """Return `True` if all bytes in the sequence are unique, `False` otherwise.

        Notes
        -----
        This will usually return `False` for `Bytes` objects created from Python
        byte slices, since they use `::bytes::Bytes::from_owner`.

        Examples
        --------
        >>> from ry import Bytes
        >>> b = Bytes.copy_from(b"unique-nu-yawk")
        >>> b.is_unique()
        True
        >>> zero_copy = Bytes(b)
        >>> zero_copy.is_unique()
        False

        """
    def is_empty(self) -> bool:
        """Return `True` if the sequence is empty, `False` otherwise.

        Examples
        --------
        >>> from ry import Bytes
        >>> b = Bytes(b"")
        >>> b.is_empty()
        True
        >>> b = Bytes(b"abc")
        >>> b.is_empty()
        False

        """

    def windows(self, size: int, /, *, reverse: bool = False) -> _BytesSliceIter:
        """Returns an iterator over all contiguous windows of length size.

        The windows overlap. If the slice is shorter than size, the iterator returns no values.

        Parameters
        ----------
        size : int
            The size of the windows to return.

        Examples
        --------
        >>> from ry import Bytes
        >>> b = Bytes(b"abcdefg")
        >>> list(b.windows(3))
        [Bytes(b"abc"), Bytes(b"bcd"), Bytes(b"cde"), Bytes(b"def"), Bytes(b"efg")]
        >>> list(b.windows(3, reverse=True))
        [Bytes(b"efg"), Bytes(b"def"), Bytes(b"cde"), Bytes(b"bcd"), Bytes(b"abc")]

        """

class _BytesSliceIter(t.Protocol):
    def __iter__(self) -> t.Self: ...
    def __next__(self) -> Bytes: ...
    def next(self) -> Bytes: ...
    def collect(self) -> list[Bytes]: ...
    def take(self, n: int = 1, /) -> list[Bytes]: ...
    def count(self) -> int: ...
    def size_hint(self) -> tuple[int, int | None]: ...
    def last(self) -> Bytes | None: ...
    def nth(self, n: int, /) -> Bytes | None: ...
    def kind(self) -> t.Literal["windows", "windows-reverse"]: ...
    def __len__(self) -> int: ...

ReadableBuffer: t.TypeAlias = Buffer | bytes | bytearray | memoryview | Bytes
