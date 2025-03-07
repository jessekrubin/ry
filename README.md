# ry

ry = rust and python and bears, oh my!


[![PyPI](https://img.shields.io/pypi/v/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Wheel](https://img.shields.io/pypi/wheel/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Status](https://img.shields.io/pypi/status/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - License](https://img.shields.io/pypi/l/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)

**DOCS:** [ryo3.dev](https://ryo3.dev) (WIP)

- `ry` is a library of python shims/bindings to popular rust crates
- `ryo3-*` is the collection of rust crates providing the shims used by ry and possibly your `pyo3` rust-python library

**THIS IS A WORK IN PROGRESS ~ FEEDBACK/PRs WELCOME!**

## Install

```bash
pip install ry
uv add ry
```

**Check install:** `python -m ry`

## Quickstart

Check out the [examples](https://github.com/jessekrubin/ry/tree/main/examples) directory for some quickstart examples.

___


## What and why?

This is a collection of pyo3-wrappers for rust crates I wish existed in python.

It all started with me wanting a fast python `xxhash` and `fnv-64`

## Who?

- jessekrubin <jessekrubin@gmail.com>
- possibly you!?

## FAQ

_(aka: questions that I have been asking myself)_

- **Q:** Why?
  - **A:** I (jesse) needed several hashing functions for python and then kept
    adding things as I needed them
- **Q:** Does this have anything to do with the (excellent) package manager
  `rye`?
  - **A:** short answer: no. long answer: no, it does not.
- **Q:** Why is the repo split into `ry` and `ryo3`?
  - **A:** `ry` is the python package, `ryo3` is a rust crate setup to let you
    "register" functions you may want if you were writing your own pyo3-python
    bindings library; maybe someday the `ryo3::libs` module will be split up
    into separate packages

## Crate bindings

ryo3-std
- wrapped crates:
  - `heck`
  - `jiter`
  - `shlex`
  - `sqlformat`
  - `url`
  - `which`
  - compression:
    - `brotli`
    - `bzip2`
    - `flate2`
    - `zstd`
  - hashing:
    - `fnv`
    - `xxhash`
  - burnt-sushi:
    - `globset` (formerly [globsters](https://pypi.org/project/globsters/))
    - `jiff`
    - `walkdir`

### FUTURE?

- `subprocess.redo` (subprocesses that are lessy finicky and support tee-ing)
- wrappers:
  - `ignore`
  - `http`
  - `regex`
  - `reqwest` (async http client / waiting on pyo3 asyncio to stabilize and for me to have more time)
  - `tokio` (`fs` and `process`)
  - `tracing` (could be nicer than python's awful logging lib -- currently a part of ry/ryo3 for my dev purposes - currently has impl thingy in utiles)
  - `tracing` (eg logging)
  - `uuid`
- organization
  - split up the `ryo3` type annotations?
  - chunk things into smaller sub-packages within the `ry` package?

___

## API

<!-- API-START -->
## `ry.ryo3`

```python
"""ry api ~ type annotations"""

from __future__ import annotations

import datetime as pydt
import typing as t

from ry import dirs as dirs  # noqa: RUF100
from ry import http as http  # noqa: RUF100
from ry import xxhash as xxhash  # noqa: RUF100
from ry._types import Buffer as Buffer  # noqa: RUF100
from ry._types import FsPathLike
from ry.http import Headers as Headers  # noqa: RUF100
from ry.http import HttpStatus as HttpStatus  # noqa: RUF100

from ._brotli import brotli as brotli
from ._brotli import brotli_decode as brotli_decode
from ._brotli import brotli_encode as brotli_encode
from ._bytes import Bytes as Bytes
from ._bzip2 import bzip2 as bzip2
from ._bzip2 import bzip2_decode as bzip2_decode
from ._bzip2 import bzip2_encode as bzip2_encode
from ._flate2 import gunzip as gunzip
from ._flate2 import gzip as gzip
from ._flate2 import gzip_decode as gzip_decode
from ._flate2 import gzip_encode as gzip_encode
from ._fnv import FnvHasher as FnvHasher
from ._fnv import fnv1a as fnv1a
from ._fspath import FsPath as FsPath
from ._globset import Glob as Glob
from ._globset import GlobSet as GlobSet
from ._globset import Globster as Globster
from ._globset import glob as glob
from ._globset import globster as globster
from ._heck import camel_case as camel_case
from ._heck import kebab_case as kebab_case
from ._heck import pascal_case as pascal_case
from ._heck import shouty_kebab_case as shouty_kebab_case
from ._heck import shouty_snake_case as shouty_snake_case
from ._heck import snake_case as snake_case
from ._heck import snek_case as snek_case
from ._heck import title_case as title_case
from ._heck import train_case as train_case
from ._jiff import Date as Date
from ._jiff import DateDifference as DateDifference
from ._jiff import DateTime as DateTime
from ._jiff import DateTimeDifference as DateTimeDifference
from ._jiff import DateTimeRound as DateTimeRound
from ._jiff import ISOWeekDate as ISOWeekDate
from ._jiff import Offset as Offset
from ._jiff import SignedDuration as SignedDuration
from ._jiff import Time as Time
from ._jiff import TimeDifference as TimeDifference
from ._jiff import TimeSpan as TimeSpan
from ._jiff import Timestamp as Timestamp
from ._jiff import TimestampDifference as TimestampDifference
from ._jiff import TimestampRound as TimestampRound
from ._jiff import TimeZone as TimeZone
from ._jiff import ZonedDateTime as ZonedDateTime
from ._jiff import ZonedDateTimeDifference as ZonedDateTimeDifference
from ._jiff import ZonedDateTimeRound as ZonedDateTimeRound
from ._jiff import date as date
from ._jiff import datetime as datetime
from ._jiff import offset as offset
from ._jiff import time as time
from ._jiff import timespan as timespan
from ._jiter import JsonParseKwargs as JsonParseKwargs
from ._jiter import JsonPrimitive as JsonPrimitive
from ._jiter import JsonValue as JsonValue
from ._jiter import json_cache_clear as json_cache_clear
from ._jiter import json_cache_usage as json_cache_usage
from ._jiter import parse_json as parse_json
from ._jiter import parse_json_bytes as parse_json_bytes
from ._quick_maths import quick_maths as quick_maths
from ._regex import Regex as Regex
from ._reqwest import HttpClient as HttpClient
from ._reqwest import ReqwestError as ReqwestError
from ._reqwest import Response as Response
from ._reqwest import ResponseStream as ResponseStream
from ._reqwest import fetch as fetch
from ._same_file import is_same_file as is_same_file
from ._shlex import shplit as shplit
from ._size import Size as Size
from ._size import SizeFormatter as SizeFormatter
from ._size import fmt_size as fmt_size
from ._size import parse_size as parse_size
from ._sqlformat import SqlfmtQueryParams as SqlfmtQueryParams
from ._sqlformat import sqlfmt as sqlfmt
from ._sqlformat import sqlfmt_params as sqlfmt_params
from ._std import Duration as Duration
from ._std import FileType as FileType
from ._std import Instant as Instant
from ._std import Metadata as Metadata
from ._std import canonicalize as canonicalize
from ._std import copy as copy
from ._std import create_dir as create_dir
from ._std import create_dir_all as create_dir_all
from ._std import exists as exists
from ._std import instant as instant
from ._std import is_dir as is_dir
from ._std import is_file as is_file
from ._std import is_symlink as is_symlink
from ._std import metadata as metadata
from ._std import read as read
from ._std import read_bytes as read_bytes
from ._std import read_stream as read_stream
from ._std import read_text as read_text
from ._std import remove_dir as remove_dir
from ._std import remove_dir_all as remove_dir_all
from ._std import remove_file as remove_file
from ._std import rename as rename
from ._std import sleep as sleep
from ._std import write as write
from ._std import write_bytes as write_bytes
from ._std import write_text as write_text
from ._tokio import asleep as asleep
from ._tokio import copy_async as copy_async
from ._tokio import create_dir_async as create_dir_async
from ._tokio import metadata_async as metadata_async
from ._tokio import read_async as read_async
from ._tokio import read_dir_async as read_dir_async
from ._tokio import remove_dir_async as remove_dir_async
from ._tokio import remove_file_async as remove_file_async
from ._tokio import rename_async as rename_async
from ._tokio import sleep_async as sleep_async
from ._tokio import write_async as write_async
from ._unindent import unindent as unindent
from ._unindent import unindent_bytes as unindent_bytes
from ._url import URL as URL
from ._walkdir import WalkDirEntry as WalkDirEntry
from ._walkdir import WalkdirGen as WalkdirGen
from ._walkdir import walkdir as walkdir
from ._which import which as which
from ._which import which_all as which_all
from ._which import which_re as which_re
from ._zstd import zstd as zstd
from ._zstd import zstd_decode as zstd_decode
from ._zstd import zstd_encode as zstd_encode
from .errors import FeatureNotEnabledError as FeatureNotEnabledError

# =============================================================================
# CONSTANTS
# =============================================================================
__version__: str
__authors__: str
__build_profile__: str
__build_timestamp__: str
__pkg_name__: str
__description__: str


# =============================================================================
# SH
# =============================================================================
def pwd() -> str: ...
def home() -> str: ...
def cd(path: FsPathLike) -> None: ...
@t.overload
def ls(
    path: FsPathLike | None = None,
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[False] = False,
) -> list[str]:
    """List directory contents - returns list of strings"""


@t.overload
def ls(
    path: FsPathLike | None = None,
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[True] = True,
) -> list[FsPath]:
    """List directory contents - returns list of FsPath objects"""

```
## `ry.ryo3.JSON`

```python
"""ry.ryo3.JSON"""

from typing import Literal

JsonPrimitive = None | bool | int | float | str
JsonValue = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)


def parse_json(
    data: bytes | str,
    /,
    *,
    allow_inf_nan: bool = True,
    cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    float_mode: Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...
def parse_json_bytes(
    data: bytes,
    /,
    *,
    allow_inf_nan: bool = True,
    cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    float_mode: Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...

```
## `ry.ryo3._brotli`

```python
"""ryo3-brotli types"""

from __future__ import annotations


# =============================================================================
# BROTLI
# =============================================================================
def brotli_encode(
    input: bytes, quality: int = 11, magic_number: bool = False
) -> bytes: ...
def brotli_decode(input: bytes) -> bytes: ...
def brotli(
    input: bytes, quality: int = 11, magic_number: bool = False
) -> bytes:
    """Alias for brotli_encode"""

```
## `ry.ryo3._bytes`

```python
import sys
from typing import overload

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

    def __init__(self, buf: Buffer = b"") -> None:
        """Construct a new Bytes object.

        This will be a zero-copy view on the Python byte slice.
        """

    def __add__(self, other: Buffer) -> Bytes: ...
    def __buffer__(self, flags: int) -> memoryview[int]: ...
    def __contains__(self, other: Buffer) -> bool: ...
    def __eq__(self, other: object) -> bool: ...
    @overload
    def __getitem__(self, other: int) -> int: ...
    @overload
    def __getitem__(self, other: slice) -> Bytes: ...
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

    def decode(self, encoding: str = "utf-8", errors: str = "strict") -> str:
        """Decode the binary data using the given encoding."""

    def hex(
        self, sep: str | None = None, bytes_per_sep: int | None = None
    ) -> str:
        """Return a hexadecimal representation of the binary data."""

    @classmethod
    def fromhex(cls, hexstr: str) -> Bytes:
        """Construct a `Bytes` object from a hexadecimal string."""


BytesLike = Buffer | bytes | bytearray | memoryview | Bytes

```
## `ry.ryo3._bzip2`

```python
"""ryo3-bzip2 types"""

from __future__ import annotations


# =============================================================================
# BZIP2
# =============================================================================
def bzip2_encode(input: bytes, quality: int = 9) -> bytes: ...
def bzip2_decode(input: bytes) -> bytes: ...
def bzip2(input: bytes, quality: int = 9) -> bytes:
    """Alias for bzip2_encode"""

```
## `ry.ryo3._dev`

```python
"""ry.ryo3.dev"""

from __future__ import annotations

import typing as t


# =============================================================================
# SUBPROCESS (VERY MUCH WIP)
# =============================================================================
def run(
    *args: str | list[str],
    capture_output: bool = True,
    input: bytes | None = None,
) -> t.Any: ...


# =============================================================================
# STRING-DEV
# =============================================================================


def anystr_noop(s: t.AnyStr) -> t.AnyStr: ...
def string_noop(s: str) -> str: ...
def bytes_noop(s: bytes) -> bytes: ...

```
## `ry.ryo3._flate2`

```python
"""ryo3-flate2 types"""

from __future__ import annotations


# =============================================================================
# GZIP
# =============================================================================
def gzip_encode(input: bytes, quality: int = 9) -> bytes: ...
def gzip_decode(input: bytes) -> bytes: ...
def gzip(input: bytes, quality: int = 9) -> bytes:
    """Alias for gzip_encode"""


def gunzip(input: bytes) -> bytes:
    """Alias for gzip_decode"""

```
## `ry.ryo3._fnv`

```python
"""ryo3-fnv types"""

import typing as t


# =============================================================================
# FNV
# =============================================================================
class FnvHasher:
    name: t.Literal["fnv1a"]

    def __init__(self, input: bytes | None = None) -> None: ...
    def update(self, input: bytes) -> None: ...
    def digest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> FnvHasher: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


def fnv1a(input: bytes) -> FnvHasher: ...

```
## `ry.ryo3._fspath`

```python
"""ryo3-fspath types"""

from __future__ import annotations

import typing as t
from os import PathLike
from pathlib import Path

from ry import Bytes
from ry._types import Buffer


# =============================================================================
# FSPATH
# =============================================================================
class FsPath:
    def __init__(self, path: PathLike[str] | str | None = None) -> None: ...
    def __fspath__(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __hash__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: PathLike[str] | str) -> bool: ...
    def __le__(self, other: PathLike[str] | str) -> bool: ...
    def __gt__(self, other: PathLike[str] | str) -> bool: ...
    def __ge__(self, other: PathLike[str] | str) -> bool: ...
    def __truediv__(self, other: PathLike[str] | str) -> FsPath: ...
    def __rtruediv__(self, other: PathLike[str] | str) -> FsPath: ...
    def __bytes__(self) -> bytes: ...
    def to_pathlib(self) -> Path: ...
    def read(self) -> Bytes: ...
    def read_text(self) -> str: ...
    def read_bytes(self) -> bytes: ...
    def absolute(self) -> FsPath: ...
    def resolve(self) -> FsPath: ...
    def write(self, data: Buffer | bytes) -> None: ...
    def write_bytes(self, data: Buffer | bytes) -> None: ...
    def write_text(self, data: str) -> None: ...
    def joinpath(self, *paths: str) -> FsPath: ...
    def exists(self) -> bool: ...
    def with_name(self, name: str) -> FsPath: ...
    def with_suffix(self, suffix: str) -> FsPath: ...
    def iterdir(self) -> t.Iterator[FsPath]: ...
    def relative_to(self, other: PathLike[str] | str | FsPath) -> FsPath: ...
    def as_posix(self) -> str: ...
    def as_uri(self) -> str: ...
    def equiv(self, other: PathLike[str] | str | FsPath) -> bool: ...
    def string(self) -> str: ...
    def clone(self) -> FsPath: ...

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def cwd(cls) -> FsPath: ...
    @classmethod
    def home(cls) -> FsPath: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def anchor(self) -> str: ...
    @property
    def drive(self) -> str: ...
    @property
    def name(self) -> str: ...
    @property
    def parent(self) -> FsPath: ...
    @property
    def parents(self) -> t.Sequence[FsPath]: ...
    @property
    def parts(self) -> tuple[str, ...]: ...
    @property
    def root(self) -> str: ...
    @property
    def stem(self) -> str: ...
    @property
    def suffix(self) -> str: ...
    @property
    def suffixes(self) -> list[str]: ...

    # =========================================================================
    # std::path::PathBuf (deref -> std::path::Path)
    # =========================================================================
    def ancestors(self) -> t.Iterator[FsPath]: ...
    def canonicalize(self) -> FsPath: ...
    def components(self) -> t.Iterator[FsPath]: ...
    def display(self) -> str: ...
    def ends_with(self, path: PathLike[str] | str) -> bool: ...
    def extension(self) -> str: ...
    def file_name(self) -> str: ...
    def file_prefix(self) -> FsPath: ...
    def file_stem(self) -> str: ...
    def has_root(self) -> bool: ...
    def is_absolute(self) -> bool: ...
    def is_dir(self) -> bool: ...
    def is_file(self) -> bool: ...
    def is_relative(self) -> bool: ...
    def is_symlink(self) -> bool: ...
    def starts_with(self, path: PathLike[str] | str) -> bool: ...
    def strip_prefix(self, prefix: PathLike[str] | str) -> FsPath: ...
    def with_extension(self, ext: str) -> FsPath: ...
    def with_file_name(self, name: str) -> FsPath: ...

```
## `ry.ryo3._globset`

```python
"""ryo3-globset types"""

from __future__ import annotations

from os import PathLike


class Glob:
    """globset::Glob wrapper"""

    def __init__(
        self,
        pattern: str,
        /,
        *,
        case_insensitive: bool | None = None,
        literal_separator: bool | None = None,
        backslash_escape: bool | None = None,
    ) -> None: ...
    def regex(self) -> str: ...
    def is_match(self, path: str | PathLike[str]) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    def __invert__(self) -> Glob: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def globset(self) -> GlobSet: ...
    def globster(self) -> Globster: ...


class GlobSet:
    """globset::GlobSet wrapper"""

    def __init__(
        self,
        patterns: list[str],
        /,
        *,
        case_insensitive: bool | None = None,
        literal_separator: bool | None = None,
        backslash_escape: bool | None = None,
    ) -> None: ...
    def is_empty(self) -> bool: ...
    def is_match(self, path: str) -> bool: ...
    def matches(self, path: str) -> list[int]: ...
    def __call__(self, path: str) -> bool: ...
    def __invert__(self) -> GlobSet: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def globster(self) -> Globster: ...


class Globster:
    """Globster is a matcher with claws!

    Note: The north american `Globster` is similar to the european `Globset`
          but allows for negative patterns (prefixed with '!')

    """

    def __init__(
        self,
        patterns: list[str],
        /,
        *,
        case_insensitive: bool | None = None,
        literal_separator: bool | None = None,
        backslash_escape: bool | None = None,
    ) -> None: ...
    def is_empty(self) -> bool: ...
    def is_match(self, path: str | PathLike[str]) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    def __invert__(self) -> GlobSet: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


def glob(
    pattern: str,
    /,
    *,
    case_insensitive: bool | None = None,
    literal_separator: bool | None = None,
    backslash_escape: bool | None = None,
) -> Glob: ...
def globster(
    patterns: list[str] | tuple[str, ...],
    /,
    *,
    case_insensitive: bool | None = None,
    literal_separator: bool | None = None,
    backslash_escape: bool | None = None,
) -> Globster: ...

```
## `ry.ryo3._heck`

```python
"""ryo3-heck types"""

from __future__ import annotations


def camel_case(string: str) -> str: ...
def kebab_case(string: str) -> str: ...
def pascal_case(string: str) -> str: ...
def shouty_kebab_case(string: str) -> str: ...
def shouty_snake_case(string: str) -> str: ...
def snake_case(string: str) -> str: ...
def snek_case(string: str) -> str: ...
def title_case(string: str) -> str: ...
def train_case(string: str) -> str: ...

```
## `ry.ryo3._jiff`

```python
"""jiff types"""

import datetime as pydt
import typing as t

import typing_extensions as te

from ry._types import (
    DateTimeTypedDict,
    DateTypedDict,
    TimeSpanTypedDict,
    TimeTypedDict,
)
from ry.ryo3 import Duration

T = t.TypeVar("T")
# =============================================================================
# JIFF
# =============================================================================
JIFF_UNIT = t.Literal[
    "year",
    "month",
    "week",
    "day",
    "hour",
    "minute",
    "second",
    "millisecond",
    "microsecond",
    "nanosecond",
]

JIFF_ROUND_MODE = t.Literal[
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half_ceil",
    "half_floor",
    "half_expand",
    "half_trunc",
    "half_even",
]

WEEKDAY = t.Literal[
    "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"
]

JIFF_WEEKDAY_INT = t.Literal[
    1,  # Monday
    2,  # Tuesday
    3,  # Wednesday
    4,  # Thursday
    5,  # Friday
    6,  # Saturday
    7,  # Sunday
]


class Date:
    MIN: Date
    MAX: Date
    ZERO: Date

    def __init__(self, year: int, month: int, day: int) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def string(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # PYTHON_CONVERSIONS
    # =========================================================================
    def to_pydate(self) -> pydt.date: ...
    @classmethod
    def from_pydate(cls: type[Date], date: pydt.date) -> Date: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def year(self) -> int: ...
    @property
    def month(self) -> int: ...
    @property
    def day(self) -> int: ...
    @property
    def weekday(self) -> int: ...

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def from_iso_week_date(
        cls: type[Date], year: int, week: int, weekday: int
    ) -> Date: ...
    @classmethod
    def today(cls: type[Date]) -> Date: ...

    # =========================================================================
    # STRPTIME/STRFTIME
    # =========================================================================
    @classmethod
    def strptime(cls: type[Date], format: str, string: str) -> Date: ...
    def strftime(self, format: str) -> str: ...

    # =========================================================================
    # OPERATORS
    # =========================================================================
    def __add__(self, other: TimeSpan | SignedDuration | Duration) -> Date: ...
    def __iadd__(self, other: TimeSpan | SignedDuration | Duration) -> Date: ...
    @t.overload
    def __sub__(self, other: Date) -> TimeSpan: ...
    @t.overload
    def __sub__(self, other: TimeSpan | SignedDuration | Duration) -> Date: ...
    @t.overload
    def __isub__(self, other: Date) -> TimeSpan: ...
    @t.overload
    def __isub__(self, other: TimeSpan | SignedDuration | Duration) -> Date: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def at(
        self, hour: int, minute: int, second: int, nanosecond: int
    ) -> DateTime: ...
    def asdict(self) -> DateTypedDict: ...
    def astuple(self) -> tuple[int, int, int]: ...
    def checked_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...
    def day_of_year(self) -> int: ...
    def day_of_year_no_leap(self) -> int | None: ...
    def days_in_month(self) -> int: ...
    def days_in_year(self) -> int: ...
    def duration_since(self, other: Date) -> Date: ...
    def duration_until(self, other: Date) -> Date: ...
    def era_year(self) -> tuple[int, t.Literal["BCE", "CE"]]: ...
    def first_of_month(self) -> Date: ...
    def first_of_year(self) -> Date: ...
    def iso_week_date(self) -> ISOWeekDate: ...
    def in_leap_year(self) -> bool: ...
    def in_tz(self, tz: str) -> ZonedDateTime: ...
    def intz(self, tz: str) -> ZonedDateTime: ...
    def last_of_month(self) -> Date: ...
    def last_of_year(self) -> Date: ...
    def nth_weekday(
        self, nth: int, weekday: WEEKDAY | JIFF_WEEKDAY_INT
    ) -> Date: ...
    def nth_weekday_of_month(
        self, nth: int, weekday: WEEKDAY | JIFF_WEEKDAY_INT
    ) -> Date: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...
    def series(self, span: TimeSpan) -> JiffSeries[Date]: ...
    def to_datetime(self, time: Time) -> DateTime: ...
    def tomorrow(self) -> Date: ...
    def yesterday(self) -> Date: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: DateDifference) -> TimeSpan: ...
    def _until(self, other: DateDifference) -> TimeSpan: ...
    def since(
        self,
        other: Date | DateTime | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...
    def until(
        self,
        other: Date | DateTime | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...

    # =========================================================================
    # INSTANCE METHODS W/ OVERLOADS
    # =========================================================================
    @t.overload
    def checked_sub(self, other: Date) -> TimeSpan: ...
    @t.overload
    def checked_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...


class DateDifference:
    def __init__(
        self,
        date: Date,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def smallest(self, unit: JIFF_UNIT) -> DateDifference: ...
    def largest(self, unit: JIFF_UNIT) -> DateDifference: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> DateDifference: ...
    def increment(self, increment: int) -> DateDifference: ...


class Time:
    MIN: Time
    MAX: Time

    def __init__(
        self,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        nanosecond: int = 0,
    ) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def string(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __add__(self, other: TimeSpan | SignedDuration | Duration) -> Time: ...
    def __iadd__(self, other: TimeSpan | SignedDuration | Duration) -> Time: ...
    @t.overload
    def __sub__(self, other: Time) -> TimeSpan: ...
    @t.overload
    def __sub__(self, other: TimeSpan | SignedDuration | Duration) -> Time: ...
    @t.overload
    def __isub__(self, other: Time) -> TimeSpan: ...
    @t.overload
    def __isub__(self, other: TimeSpan | SignedDuration | Duration) -> Time: ...

    # =========================================================================
    # STRPTIME/STRFTIME/PARSE
    # =========================================================================
    @classmethod
    def strptime(cls: type[Time], format: str, string: str) -> Time: ...
    def strftime(self, format: str) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    def to_pytime(self) -> pydt.time: ...
    @classmethod
    def from_pytime(cls: type[Time], t: pydt.time) -> Time: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def midnight(cls: type[Time]) -> Time: ...
    @classmethod
    def now(cls: type[Time]) -> Time: ...
    @classmethod
    def parse(cls: type[Time], s: str) -> Time: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def hour(self) -> int: ...
    @property
    def minute(self) -> int: ...
    @property
    def second(self) -> int: ...
    @property
    def millisecond(self) -> int: ...
    @property
    def microsecond(self) -> int: ...
    @property
    def nanosecond(self) -> int: ...
    @property
    def subsec_nanosecond(self) -> None: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def astuple(self) -> tuple[int, int, int, int]: ...
    def asdict(self) -> TimeTypedDict: ...
    def series(self, span: TimeSpan) -> JiffSeries[Time]: ...
    def checked_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Time: ...
    @t.overload
    def checked_sub(self, other: Time) -> TimeSpan: ...
    @t.overload
    def checked_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Time: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Time: ...
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Time: ...
    def wrapping_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Time: ...
    def wrapping_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Time: ...
    def on(self, year: int, month: int, day: int) -> DateTime: ...
    def duration_until(self, other: Time) -> SignedDuration: ...
    def duration_since(self, other: Time) -> SignedDuration: ...
    def round(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> Time: ...
    def to_datetime(self, d: Date) -> DateTime: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: TimeDifference) -> TimeSpan: ...
    def _until(self, other: TimeDifference) -> TimeSpan: ...
    def since(
        self,
        other: Time | DateTime | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...
    def until(
        self,
        other: Time | DateTime | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...


class TimeDifference:
    def __init__(
        self,
        date: Time,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def smallest(self, unit: JIFF_UNIT) -> TimeDifference: ...
    def largest(self, unit: JIFF_UNIT) -> TimeDifference: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> TimeDifference: ...
    def increment(self, increment: int) -> TimeDifference: ...


class DateTime:
    MIN: DateTime
    MAX: DateTime
    ZERO: DateTime

    def __init__(
        self,
        year: int,
        month: int,
        day: int,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        nanosecond: int = 0,
    ) -> None: ...
    def __str__(self) -> str: ...
    def string(self) -> str: ...

    # =========================================================================
    # STRPTIME/STRFTIME/PARSE
    # =========================================================================
    def strftime(self, format: str) -> str: ...
    @classmethod
    def strptime(cls: type[DateTime], format: str, string: str) -> DateTime: ...
    @classmethod
    def parse(cls: type[DateTime], s: str) -> DateTime: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pydatetime(cls: type[DateTime], dt: pydt.datetime) -> DateTime: ...
    def to_pydatetime(self) -> pydt.datetime: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def now(cls: type[DateTime]) -> DateTime: ...
    @classmethod
    def from_parts(cls: type[DateTime], date: Date, time: Time) -> DateTime: ...

    # =========================================================================
    # OPERATORS
    # =========================================================================
    def __repr__(self) -> str: ...
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...
    def __iadd__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...
    @t.overload
    def __sub__(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...
    @t.overload
    def __isub__(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def __isub__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def checked_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...
    def in_tz(self, tz: str) -> ZonedDateTime: ...
    def intz(self, tz: str) -> ZonedDateTime: ...
    def iso_week_date(self) -> tuple[int, int, int]: ...
    def date(self) -> Date: ...
    def time(self) -> Time: ...
    def series(self, span: TimeSpan) -> JiffSeries[DateTime]: ...
    def asdict(self) -> DateTimeTypedDict: ...
    def round(
        self,
        smallest: JIFF_UNIT | None = None,
        *,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> DateTime: ...
    def _round(self, options: DateTimeRound) -> DateTime: ...
    def yesterday(self) -> DateTime: ...

    # =========================================================================
    # INSTANCE METHODS W/ OVERLOADS
    # =========================================================================
    @t.overload
    def checked_sub(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def checked_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...
    @t.overload
    def saturating_sub(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> DateTime: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def year(self) -> int: ...
    @property
    def month(self) -> int: ...
    @property
    def day(self) -> int: ...
    @property
    def hour(self) -> int: ...
    @property
    def minute(self) -> int: ...
    @property
    def second(self) -> int: ...
    @property
    def millisecond(self) -> int: ...
    @property
    def microsecond(self) -> int: ...
    @property
    def nanosecond(self) -> int: ...
    @property
    def subsec_nanosecond(self) -> int: ...
    @property
    def weekday(self) -> int: ...

    # =========================================================================
    # NOT IMPLEMENTED & NOT TYPED
    # =========================================================================
    def first_of_month(self) -> DateTime: ...
    def first_of_year(self) -> DateTime: ...
    def day_of_year(self) -> int: ...
    def day_of_year_no_leap(self) -> int | None: ...
    def days_in_month(self) -> int: ...
    def days_in_year(self) -> int: ...
    def era_year(self) -> tuple[int, t.Literal["BCE", "CE"]]: ...
    def duration_since(self, other: DateTime) -> SignedDuration: ...
    def duration_until(self, other: DateTime) -> SignedDuration: ...
    def in_leap_year(self) -> bool: ...
    def end_of_day(self) -> DateTime: ...
    def nth_weekday(
        self, nth: int, weekday: WEEKDAY | JIFF_WEEKDAY_INT
    ) -> DateTime: ...
    def nth_weekday_of_month(
        self, nth: int, weekday: WEEKDAY | JIFF_WEEKDAY_INT
    ) -> DateTime: ...
    def last_of_month(self) -> DateTime: ...
    def last_of_year(self) -> DateTime: ...
    def start_of_day(self) -> DateTime: ...
    def tomorrow(self) -> DateTime: ...
    def to_zoned(self, tz: TimeZone) -> ZonedDateTime: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: DateTimeDifference) -> TimeSpan: ...
    def _until(self, other: DateTimeDifference) -> TimeSpan: ...
    def since(
        self,
        other: Date | Time | DateTime | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...
    def until(
        self,
        other: Date | Time | DateTime | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...


class DateTimeDifference:
    def __init__(
        self,
        date: DateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def smallest(self, unit: JIFF_UNIT) -> DateTimeDifference: ...
    def largest(self, unit: JIFF_UNIT) -> DateTimeDifference: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> DateTimeDifference: ...
    def increment(self, increment: int) -> DateTimeDifference: ...


class TimeZone:
    def __init__(self, name: str) -> None: ...
    def __eq__(self, other: object) -> bool: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    def to_pytzinfo(self) -> pydt.tzinfo: ...
    @classmethod
    def from_pytzinfo(cls: type[TimeZone], tz: pydt.tzinfo) -> TimeZone: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def name(self) -> str: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def fixed(cls: type[TimeZone], offset: Offset) -> TimeZone: ...
    @classmethod
    def get(cls: type[TimeZone], name: str) -> TimeZone: ...
    @classmethod
    def posix(cls: type[TimeZone], name: str) -> TimeZone: ...
    @classmethod
    def system(cls: type[TimeZone]) -> TimeZone: ...
    @classmethod
    def try_system(cls: type[TimeZone]) -> TimeZone: ...
    @classmethod
    def tzif(cls: type[TimeZone], name: str, data: bytes) -> TimeZone: ...
    @classmethod
    def utc(cls: type[TimeZone]) -> TimeZone: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def iana_name(self) -> str | None: ...
    def to_datetime(self, dt: Timestamp) -> DateTime: ...
    def to_offset(self, timestamp: Timestamp) -> Offset: ...
    def to_timestamp(self, dt: DateTime) -> Timestamp: ...
    def to_zoned(self, other: DateTime) -> ZonedDateTime: ...

    # =========================================================================
    # NOT IMPLEMENTED
    # =========================================================================
    def to_ambiguous_timestamp(self) -> t.NoReturn: ...
    def to_ambiguous_zoned(self) -> t.NoReturn: ...


class SignedDuration:
    MIN: SignedDuration
    MAX: SignedDuration
    ZERO: SignedDuration

    def __init__(self, secs: int = 0, nanos: int = 0) -> None: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __hash__(self) -> int: ...
    def __mul__(self, other: int) -> SignedDuration: ...
    def __rmul__(self, other: int) -> SignedDuration: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: object) -> bool: ...
    def __le__(self, other: object) -> bool: ...
    def __gt__(self, other: object) -> bool: ...
    def __ge__(self, other: object) -> bool: ...
    def __neg__(self) -> SignedDuration: ...
    def __add__(self, other: SignedDuration) -> SignedDuration: ...
    def __abs__(self) -> SignedDuration: ...
    def __div__(self, other: int) -> SignedDuration: ...
    def abs(self) -> SignedDuration: ...
    def unsigned_abs(self) -> Duration: ...
    def __richcmp__(
        self, other: SignedDuration | pydt.timedelta, op: int
    ) -> bool: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...
    def string(self, human: bool = False) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(
        cls: type[SignedDuration], td: pydt.timedelta
    ) -> SignedDuration: ...
    def to_pytimedelta(self) -> pydt.timedelta: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def parse(cls: type[SignedDuration], s: str) -> SignedDuration: ...
    @classmethod
    def from_hours(cls: type[SignedDuration], n: int) -> SignedDuration: ...
    @classmethod
    def from_micros(cls: type[SignedDuration], n: int) -> SignedDuration: ...
    @classmethod
    def from_millis(cls: type[SignedDuration], n: int) -> SignedDuration: ...
    @classmethod
    def from_mins(cls: type[SignedDuration], n: int) -> SignedDuration: ...
    @classmethod
    def from_nanos(cls: type[SignedDuration], n: int) -> SignedDuration: ...
    @classmethod
    def from_secs(cls: type[SignedDuration], n: int) -> SignedDuration: ...
    @classmethod
    def from_secs_f32(
        cls: type[SignedDuration], n: float
    ) -> SignedDuration: ...
    @classmethod
    def from_secs_f64(
        cls: type[SignedDuration], n: float
    ) -> SignedDuration: ...
    @classmethod
    def try_from_secs_f32(
        cls: type[SignedDuration], n: float
    ) -> SignedDuration: ...
    @classmethod
    def try_from_secs_f64(
        cls: type[SignedDuration], n: float
    ) -> SignedDuration: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def is_negative(self) -> bool: ...
    @property
    def is_zero(self) -> bool: ...
    @property
    def secs(self) -> int: ...
    @property
    def nanos(self) -> int: ...
    @property
    def days(self) -> int: ...
    @property
    def seconds(self) -> int: ...
    @property
    def microseconds(self) -> int: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def as_hours(self) -> int: ...
    def as_micros(self) -> int: ...
    def as_millis(self) -> int: ...
    def as_millis_f32(self) -> float: ...
    def as_millis_f64(self) -> float: ...
    def as_mins(self) -> int: ...
    def as_nanos(self) -> int: ...
    def as_secs(self) -> int: ...
    def as_secs_f32(self) -> float: ...
    def as_secs_f64(self) -> float: ...
    def checked_add(self, other: SignedDuration) -> SignedDuration | None: ...
    def checked_div(self, other: int) -> SignedDuration | None: ...
    def checked_mul(self, other: int) -> SignedDuration | None: ...
    def checked_neg(self) -> SignedDuration | None: ...
    def checked_sub(self, other: SignedDuration) -> SignedDuration | None: ...
    def div_duration_f32(self, other: SignedDuration) -> float: ...
    def div_duration_f64(self, other: SignedDuration) -> float: ...
    def div_f32(self, other: int) -> float: ...
    def div_f64(self, other: int) -> float: ...
    def is_positive(self) -> bool: ...
    def mul_f32(self, other: int) -> SignedDuration: ...
    def mul_f64(self, other: int) -> SignedDuration: ...
    def saturating_add(self, other: SignedDuration) -> SignedDuration: ...
    def saturating_mul(self, other: int) -> SignedDuration: ...
    def saturating_sub(self, other: SignedDuration) -> SignedDuration: ...
    def signum(self) -> t.Literal[-1, 0, 1]: ...
    def subsec_micros(self) -> int: ...
    def subsec_millis(self) -> int: ...
    def subsec_nanos(self) -> int: ...
    def to_timespan(self) -> TimeSpan: ...


# put in quotes to avoid ruff F821 - undefined name
_TimeSpanArithmeticSingle = TimeSpan | Duration | SignedDuration
_TimeSpanArithmeticTuple = tuple[
    _TimeSpanArithmeticSingle, ZonedDateTime | Date | DateTime
]
TimeSpanArithmetic = _TimeSpanArithmeticSingle | _TimeSpanArithmeticTuple


class TimeSpan:
    def __init__(
        self,
        years: int = 0,
        months: int = 0,
        weeks: int = 0,
        days: int = 0,
        hours: int = 0,
        minutes: int = 0,
        seconds: int = 0,
        milliseconds: int = 0,
        microseconds: int = 0,
        nanoseconds: int = 0,
    ) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def string(self, human: bool = False) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def repr_full(self) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(cls, td: pydt.timedelta) -> TimeSpan: ...
    def to_pytimedelta(self) -> pydt.timedelta: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def parse(cls, s: str) -> TimeSpan: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def is_positive(self) -> bool: ...
    @property
    def is_negative(self) -> bool: ...
    @property
    def is_zero(self) -> bool: ...
    @property
    def years(self) -> int: ...
    @property
    def months(self) -> int: ...
    @property
    def weeks(self) -> int: ...
    @property
    def days(self) -> int: ...
    @property
    def hours(self) -> int: ...
    @property
    def minutes(self) -> int: ...
    @property
    def seconds(self) -> int: ...
    @property
    def milliseconds(self) -> int: ...
    @property
    def microseconds(self) -> int: ...
    @property
    def nanoseconds(self) -> int: ...

    # =========================================================================
    # OPERATORS
    # =========================================================================
    def __add__(
        self,
        val: TimeSpanArithmetic,
    ) -> te.Self: ...
    def __sub__(
        self,
        val: TimeSpanArithmetic,
    ) -> te.Self: ...
    def __mul__(self, other: int) -> te.Self: ...
    def __neg__(self) -> te.Self: ...
    def __abs__(self) -> te.Self: ...
    def __invert__(self) -> te.Self: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: TimeSpan) -> bool: ...
    def __gt__(self, other: TimeSpan) -> bool: ...
    def __le__(self, other: TimeSpan) -> bool: ...
    def __lt__(self, other: TimeSpan) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __rmul__(self, other: TimeSpan) -> bool: ...
    def __hash__(self) -> int: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================

    def abs(self) -> te.Self: ...
    def asdict(self) -> TimeSpanTypedDict: ...
    def checked_add(self, val: TimeSpanArithmetic) -> te.Self: ...
    def checked_mul(self, other: int) -> te.Self: ...
    def checked_sub(self, val: TimeSpanArithmetic) -> te.Self: ...
    def compare(
        self,
        other: TimeSpan,
        relative: ZonedDateTime | DateTime | Date | None = None,
        days_are_24_hours: bool = False,
    ) -> int: ...
    def negate(self) -> te.Self: ...
    def replace(
        self,
        years: int | None = None,
        months: int | None = None,
        weeks: int | None = None,
        days: int | None = None,
        hours: int | None = None,
        minutes: int | None = None,
        seconds: int | None = None,
        milliseconds: int | None = None,
        microseconds: int | None = None,
        nanoseconds: int | None = None,
    ) -> te.Self: ...
    def round(
        self,
        smallest: JIFF_UNIT,
        increment: int = 1,
        *,
        relative: ZonedDateTime | Date | DateTime | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
    ) -> te.Self: ...
    def signum(self) -> t.Literal[-1, 0, 1]: ...
    def to_signed_duration(
        self, relative: ZonedDateTime | Date | DateTime
    ) -> SignedDuration: ...
    def total(
        self,
        unit: JIFF_UNIT,
        relative: ZonedDateTime | Date | DateTime | None = None,
        days_are_24_hours: bool = False,
    ) -> int: ...
    def total_seconds(self) -> int: ...
    def try_years(self, years: int) -> te.Self: ...
    def try_months(self, months: int) -> te.Self: ...
    def try_weeks(self, weeks: int) -> te.Self: ...
    def try_days(self, days: int) -> te.Self: ...
    def try_hours(self, hours: int) -> te.Self: ...
    def try_minutes(self, minutes: int) -> te.Self: ...
    def try_seconds(self, seconds: int) -> te.Self: ...
    def try_milliseconds(self, milliseconds: int) -> te.Self: ...
    def try_microseconds(self, microseconds: int) -> te.Self: ...
    def try_nanoseconds(self, nanoseconds: int) -> te.Self: ...

    # -------------------------------------------------------------------------
    # PANIC-INDUCING METHODS
    # -------------------------------------------------------------------------
    def _years(self, years: int) -> te.Self: ...
    def _months(self, months: int) -> te.Self: ...
    def _weeks(self, weeks: int) -> te.Self: ...
    def _days(self, days: int) -> te.Self: ...
    def _hours(self, hours: int) -> te.Self: ...
    def _minutes(self, minutes: int) -> te.Self: ...
    def _seconds(self, seconds: int) -> te.Self: ...
    def _milliseconds(self, milliseconds: int) -> te.Self: ...
    def _microseconds(self, microseconds: int) -> te.Self: ...
    def _nanoseconds(self, nanoseconds: int) -> te.Self: ...


class Timestamp:
    """
    A representation of a timestamp with second and nanosecond precision.
    """

    MIN: Timestamp
    MAX: Timestamp
    UNIX_EPOCH: Timestamp

    def __init__(
        self, second: int | None = None, nanosecond: int | None = None
    ) -> None: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def now(cls) -> Timestamp: ...
    @classmethod
    def parse(cls, s: str) -> Timestamp: ...
    @classmethod
    def from_millisecond(cls, millisecond: int) -> Timestamp: ...
    @classmethod
    def from_microsecond(cls, microsecond: int) -> Timestamp: ...
    @classmethod
    def from_nanosecond(cls, nanosecond: int) -> Timestamp: ...
    @classmethod
    def from_second(cls, second: int) -> Timestamp: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: Timestamp) -> bool: ...
    def __gt__(self, other: Timestamp) -> bool: ...
    def __iadd__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    def __le__(self, other: Timestamp) -> bool: ...
    def __lt__(self, other: Timestamp) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __richcmp__(self, other: Timestamp, op: int) -> bool: ...

    # =========================================================================
    # OPERATORS/DUNDERS W/ OVERLOADS
    # =========================================================================
    @t.overload
    def __isub__(self, other: Timestamp) -> TimeSpan: ...
    @t.overload
    def __isub__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    @t.overload
    def __sub__(self, other: Timestamp) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...

    # =========================================================================
    # STRPTIME/STRFTIME
    # =========================================================================
    def strftime(self, format: str) -> str: ...
    @classmethod
    def strptime(cls, format: str, input: str) -> Timestamp: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================

    def as_microsecond(self) -> int: ...
    def as_millisecond(self) -> int: ...
    def as_nanosecond(self) -> int: ...
    def as_second(self) -> int: ...
    def checked_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Timestamp: ...
    @t.overload
    def checked_sub(self, other: Timestamp) -> TimeSpan: ...
    @t.overload
    def checked_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Timestamp: ...
    def display_with_offset(self, offset: Offset) -> str: ...
    def in_tz(self, tz: str) -> ZonedDateTime: ...
    def intz(self, tz: str) -> ZonedDateTime:
        """Deprecated ~ use `in_tz`"""

    def is_zero(self) -> bool: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Timestamp: ...
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> Timestamp: ...
    def series(self, span: TimeSpan) -> JiffSeries[Timestamp]: ...
    def signum(self) -> t.Literal[-1, 0, 1]: ...
    def string(self) -> str: ...
    def subsec_microsecond(self) -> int: ...
    def subsec_millisecond(self) -> int: ...
    def subsec_nanosecond(self) -> int: ...
    def to_zoned(self, time_zone: TimeZone) -> ZonedDateTime: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: TimestampDifference) -> TimeSpan: ...
    def _until(self, other: TimestampDifference) -> TimeSpan: ...
    def since(
        self,
        other: Timestamp | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...
    def until(
        self,
        other: Timestamp | ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...
    def duration_since(self, other: Timestamp) -> SignedDuration: ...
    def duration_until(self, other: Timestamp) -> SignedDuration: ...
    def round(
        self,
        unit: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> Timestamp: ...
    def _round(self, options: TimestampRound) -> Timestamp: ...


class TimestampDifference:
    def __init__(
        self,
        date: Timestamp,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def smallest(self, unit: JIFF_UNIT) -> TimestampDifference: ...
    def largest(self, unit: JIFF_UNIT) -> TimestampDifference: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> TimestampDifference: ...
    def increment(self, increment: int) -> TimestampDifference: ...


class ZonedDateTime:
    def __init__(self, timestamp: Timestamp, time_zone: TimeZone) -> None: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pydatetime(
        cls: type[ZonedDateTime], dt: pydt.datetime
    ) -> ZonedDateTime: ...
    def to_pydatetime(self) -> pydt.datetime: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def now(
        cls: type[ZonedDateTime], tz: str | None = None
    ) -> ZonedDateTime: ...
    @classmethod
    def utcnow(cls: type[ZonedDateTime]) -> ZonedDateTime: ...
    @classmethod
    def parse(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...
    @classmethod
    def from_rfc2822(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...

    # =========================================================================
    # STRPTIME/STRFTIME
    # =========================================================================
    @classmethod
    def strptime(
        cls: type[ZonedDateTime], format: str, input: str
    ) -> ZonedDateTime: ...
    def strftime(self, format: str) -> str: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def year(self) -> int: ...
    @property
    def month(self) -> int: ...
    @property
    def day(self) -> int: ...
    @property
    def hour(self) -> int: ...
    @property
    def minute(self) -> int: ...
    @property
    def second(self) -> int: ...
    @property
    def millisecond(self) -> int: ...
    @property
    def microsecond(self) -> int: ...
    @property
    def nanosecond(self) -> int: ...
    @property
    def subsec_nanosecond(self) -> int: ...
    @property
    def weekday(self) -> int: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...
    def string(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: ZonedDateTime) -> bool: ...
    def __gt__(self, other: ZonedDateTime) -> bool: ...
    def __hash__(self) -> int: ...
    def __iadd__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    def __le__(self, other: ZonedDateTime) -> bool: ...
    def __lt__(self, other: ZonedDateTime) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __richcmp__(self, other: ZonedDateTime, op: int) -> bool: ...

    # =========================================================================
    # OPERATORS/DUNDERS W/ OVERLOADS
    # =========================================================================
    @t.overload
    def __isub__(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def __isub__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    @t.overload
    def __sub__(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def astimezone(self, tz: str) -> ZonedDateTime: ...
    def checked_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    @t.overload
    def checked_sub(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def checked_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    def date(self) -> Date: ...
    def datetime(self) -> DateTime: ...
    def iso_week_date(self) -> ISOWeekDate: ...
    def day_of_year(self) -> int: ...
    def day_of_year_no_leap(self) -> int | None: ...
    def days_in_month(self) -> int: ...
    def days_in_year(self) -> int: ...
    def duration_since(self, other: ZonedDateTime) -> SignedDuration: ...
    def duration_until(self, other: ZonedDateTime) -> SignedDuration: ...
    def end_of_day(self) -> ZonedDateTime: ...
    def era_year(self) -> tuple[int, t.Literal["CE", "BCE"]]: ...
    def first_of_month(self) -> ZonedDateTime: ...
    def first_of_year(self) -> ZonedDateTime: ...
    def in_leap_year(self) -> bool: ...
    def in_tz(self, tz: str) -> te.Self: ...
    def intz(self, tz: str) -> te.Self: ...
    def inutc(self) -> ZonedDateTime: ...
    def last_of_month(self) -> ZonedDateTime: ...
    def last_of_year(self) -> ZonedDateTime: ...
    def nth_weekday(
        self, nth: int, weekday: WEEKDAY | JIFF_WEEKDAY_INT
    ) -> Date: ...
    def nth_weekday_of_month(
        self, nth: int, weekday: WEEKDAY | JIFF_WEEKDAY_INT
    ) -> Date: ...
    def offset(self) -> Offset: ...
    def round(
        self,
        smallest: JIFF_UNIT | None = None,
        *,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> DateTime: ...
    def _round(self, options: ZonedDateTimeRound) -> DateTime: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    @t.overload
    def saturating_sub(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration
    ) -> te.Self: ...
    def start_of_day(self) -> ZonedDateTime: ...
    def time(self) -> Time: ...
    def time_zone(self) -> TimeZone: ...
    def timestamp(self) -> Timestamp: ...
    def timezone(self) -> TimeZone: ...
    def to_rfc2822(self) -> str: ...
    def tomorrow(self) -> ZonedDateTime: ...
    def with_time_zone(self, tz: TimeZone) -> ZonedDateTime: ...
    def yesterday(self) -> ZonedDateTime: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def since(
        self,
        other: ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...
    def until(
        self,
        other: ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> TimeSpan: ...


class ZonedDateTimeDifference:
    def __init__(
        self,
        date: ZonedDateTime,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def smallest(self, unit: JIFF_UNIT) -> ZonedDateTimeDifference: ...
    def largest(self, unit: JIFF_UNIT) -> ZonedDateTimeDifference: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> ZonedDateTimeDifference: ...
    def increment(self, increment: int) -> ZonedDateTimeDifference: ...


class ISOWeekDate:
    MIN: ISOWeekDate
    MAX: ISOWeekDate
    ZERO: ISOWeekDate

    def __init__(
        self, year: int, week: int, weekday: JIFF_WEEKDAY_INT | WEEKDAY
    ) -> None: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: ISOWeekDate) -> bool: ...
    def __le__(self, other: ISOWeekDate) -> bool: ...
    def __gt__(self, other: ISOWeekDate) -> bool: ...
    def __ge__(self, other: ISOWeekDate) -> bool: ...
    def __hash__(self) -> int: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __repr__(self) -> str: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def from_date(cls: type[ISOWeekDate], date: Date) -> ISOWeekDate: ...
    @classmethod
    def today(cls: type[ISOWeekDate]) -> ISOWeekDate: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def year(self) -> int: ...
    @property
    def week(self) -> int: ...
    @property
    def weekday(self) -> JIFF_WEEKDAY_INT: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def date(self) -> Date: ...


class TimestampRound:
    def __init__(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int = 1,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> TimestampRound: ...
    def smallest(self, smallest: JIFF_UNIT) -> TimestampRound: ...
    def increment(self, increment: int) -> TimestampRound: ...
    def _smallest(self) -> JIFF_UNIT: ...
    def _mode(self) -> JIFF_ROUND_MODE: ...
    def _increment(self) -> int: ...
    def replace(
        self,
        smallest: JIFF_UNIT | None,
        mode: JIFF_ROUND_MODE | None,
        increment: int | None,
    ) -> TimestampRound: ...


class DateTimeRound:
    def __init__(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int = 1,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> DateTimeRound: ...
    def smallest(self, smallest: JIFF_UNIT) -> DateTimeRound: ...
    def increment(self, increment: int) -> DateTimeRound: ...
    def _smallest(self) -> JIFF_UNIT: ...
    def _mode(self) -> JIFF_ROUND_MODE: ...
    def _increment(self) -> int: ...
    def replace(
        self,
        smallest: JIFF_UNIT | None,
        mode: JIFF_ROUND_MODE | None,
        increment: int | None,
    ) -> DateTimeRound: ...


class ZonedDateTimeRound:
    def __init__(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int = 1,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> DateTimeRound: ...
    def smallest(self, smallest: JIFF_UNIT) -> DateTimeRound: ...
    def increment(self, increment: int) -> DateTimeRound: ...
    def _smallest(self) -> JIFF_UNIT: ...
    def _mode(self) -> JIFF_ROUND_MODE: ...
    def _increment(self) -> int: ...
    def replace(
        self,
        smallest: JIFF_UNIT | None,
        mode: JIFF_ROUND_MODE | None,
        increment: int | None,
    ) -> DateTimeRound: ...


class Offset:
    MIN: Offset
    MAX: Offset
    UTC: Offset
    ZERO: Offset

    def __init__(
        self, hours: int | None = None, seconds: int | None = None
    ) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def string(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __neg__(self) -> Offset: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Offset) -> bool: ...
    def __le__(self, other: Offset) -> bool: ...
    def __gt__(self, other: Offset) -> bool: ...
    def __ge__(self, other: Offset) -> bool: ...
    def __hash__(self) -> int: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def seconds(self) -> int: ...
    @property
    def is_negative(self) -> bool: ...
    @property
    def is_positive(self) -> bool: ...

    # =========================================================================
    # FROM
    # =========================================================================
    @classmethod
    def utc(cls: type[Offset]) -> Offset: ...
    @classmethod
    def from_hours(cls: type[Offset], hours: int) -> Offset: ...
    @classmethod
    def from_seconds(cls: type[Offset], seconds: int) -> Offset: ...

    # =========================================================================
    # TO
    # =========================================================================
    def to_datetime(self, timestamp: Timestamp) -> DateTime: ...
    def to_timestamp(self, datetime: DateTime) -> Timestamp: ...
    def to_timezone(self) -> TimeZone: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def checked_add(
        self, other: Duration | SignedDuration | TimeSpan
    ) -> Offset: ...
    def checked_sub(
        self, other: Duration | SignedDuration | TimeSpan
    ) -> Offset: ...
    def duration_since(self, other: Offset) -> SignedDuration: ...
    def duration_until(self, other: Offset) -> SignedDuration: ...
    def negate(self) -> Offset: ...
    def saturating_add(
        self, other: Duration | SignedDuration | TimeSpan
    ) -> Offset: ...
    def saturating_sub(
        self, other: Duration | SignedDuration | TimeSpan
    ) -> Offset: ...
    def since(self, other: Offset) -> TimeSpan: ...
    def until(self, other: Offset) -> TimeSpan: ...


class JiffSeries(
    t.Generic[T],
):
    def __iter__(self) -> t.Iterator[T]: ...
    def __next__(self) -> T: ...
    def take(self, n: int) -> list[T]: ...


def date(year: int, month: int, day: int) -> Date: ...
def time(
    hour: int = 0, minute: int = 0, second: int = 0, nanosecond: int = 0
) -> Time: ...
def datetime(
    year: int,
    month: int,
    day: int,
    hour: int = 0,
    minute: int = 0,
    second: int = 0,
    nanosecond: int = 0,
) -> DateTime: ...
def timespan(
    *,
    years: int = 0,
    months: int = 0,
    weeks: int = 0,
    days: int = 0,
    hours: int = 0,
    minutes: int = 0,
    seconds: int = 0,
    milliseconds: int = 0,
    microseconds: int = 0,
    nanoseconds: int = 0,
    unchecked: bool = False,
) -> TimeSpan: ...
def offset(hours: int) -> Offset: ...

```
## `ry.ryo3._jiter`

```python
from __future__ import annotations

import typing as t

import typing_extensions as te

from ry._types import Buffer

# =============================================================================
# JSON
# =============================================================================
JsonPrimitive = None | bool | int | float | str
JsonValue = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)


class JsonParseKwargs(t.TypedDict, total=False):
    allow_inf_nan: bool
    """Allow parsing of `Infinity`, `-Infinity`, `NaN` ~ default: True"""
    cache_mode: t.Literal[True, False, "all", "keys", "none"]
    """Cache mode for JSON parsing ~ default: `all` """
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"]
    """Partial mode for JSON parsing ~ default: False"""
    catch_duplicate_keys: bool
    """Catch duplicate keys in JSON objects ~ default: False"""
    float_mode: t.Literal["float", "decimal", "lossless-float"] | bool
    """Mode for parsing JSON floats ~ default: False"""


def parse_json(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse_json_bytes(
    data: bytes,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...

```
## `ry.ryo3._quick_maths`

```python
"""ryo3-quick-maths types"""

from __future__ import annotations

import typing as t


def quick_maths() -> t.Literal[3]:
    """Performs quick-maths

    Implements the algorithm for performing "quick-maths" as described by
    Big Shaq in his PHD thesis, 2017, in which he states:

    > "2 plus 2 is 4, minus one that's 3, quick maths." (Big Shaq et al., 2017)

    Reference:
        https://youtu.be/3M_5oYU-IsU?t=60

    Example:
        >>> import ry
        >>> result = ry.quick_maths()
        >>> assert result == 3

    NOTE: THIS IS FROM MY TEMPLATE RY03-MODULE
    """

```
## `ry.ryo3._regex`

```python
"""ryo3-regex types"""

from __future__ import annotations

# =============================================================================
# Regex
# =============================================================================


class Regex:
    def __init__(
        self,
        pattern: str,
        *,
        case_insensitive: bool = False,
        crlf: bool = False,
        dot_matches_new_line: bool = False,
        ignore_whitespace: bool = False,
        line_terminator: str | None = None,
        multi_line: bool = False,
        octal: bool = False,
        size_limit: int | None = None,
        swap_greed: bool = False,
        unicode: bool = False,
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def is_match(self, string: str) -> bool: ...

```
## `ry.ryo3._reqwest`

```python
import typing as t
from http import HTTPStatus

import ry

if t.TYPE_CHECKING:
    from ry.http import Headers
    from ry.ryo3 import URL, Duration


class HttpClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | None = None,
        user_agent: str | None = None,  # default ~ 'ry-reqwest/<VERSION> ...'
        timeout: Duration | None = None,
        connect_timeout: Duration | None = None,
        read_timeout: Duration | None = None,
        gzip: bool = True,
        brotli: bool = True,
        deflate: bool = True,
    ) -> None: ...
    async def get(
        self, url: str | URL, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def post(
        self,
        url: str | URL,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def put(
        self,
        url: str | URL,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def delete(
        self, url: str | URL, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def patch(
        self,
        url: str | URL,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def head(
        self, url: str | URL, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def fetch(
        self,
        url: str | URL,
        *,
        method: str = "GET",
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...


class ReqwestError(Exception):
    def __init__(self, *args: t.Any, **kwargs: t.Any) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __dbg__(self) -> str: ...
    def is_body(self) -> bool: ...
    def is_builder(self) -> bool: ...
    def is_connect(self) -> bool: ...
    def is_decode(self) -> bool: ...
    def is_redirect(self) -> bool: ...
    def is_request(self) -> bool: ...
    def is_status(self) -> bool: ...
    def is_timeout(self) -> bool: ...
    def status(self) -> HTTPStatus | None: ...
    def url(self) -> URL | None: ...


class Response:
    status_code: int

    @property
    def headers(self) -> Headers: ...
    async def text(self) -> str: ...
    async def json(self) -> t.Any: ...
    async def bytes(self) -> ry.Bytes: ...
    def bytes_stream(self) -> ResponseStream: ...


class ResponseStream:
    def __aiter__(self) -> ResponseStream: ...
    async def __anext__(self) -> ry.Bytes: ...


async def fetch(
    url: str | URL,
    *,
    client: HttpClient | None = None,
    method: str = "GET",
    body: bytes | None = None,
    headers: dict[str, str] | None = None,
) -> Response: ...

```
## `ry.ryo3._same_file`

```python
"""ryo3-same-file types"""

from __future__ import annotations

from os import PathLike


def is_same_file(a: PathLike[str], b: PathLike[str]) -> bool: ...

```
## `ry.ryo3._shlex`

```python
"""ryo3-shlex types"""

from __future__ import annotations


def shplit(s: str) -> list[str]:
    """shlex::split wrapper much like python's stdlib shlex.split but faster"""
    ...

```
## `ry.ryo3._size`

```python
from __future__ import annotations

from typing import Literal

FORMAT_SIZE_BASE = Literal[2, 10]  # default=2
FORMAT_SIZE_STYLE = Literal[  # default="default"
    "default",
    "abbreviated",
    "abbreviated_lowercase",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
    "full_lowercase",
]


def fmt_size(
    n: int,
    *,
    base: FORMAT_SIZE_BASE | None = 2,
    style: FORMAT_SIZE_STYLE | None = "default",
) -> str:
    """Return human-readable string representation of bytes-size."""


def parse_size(s: str) -> int:
    """Return integer representation of human-readable bytes-size string.

    Raises:
        ValueError: If string is not a valid human-readable bytes-size string.
    """


class SizeFormatter:
    """Human-readable bytes-size formatter."""

    def __init__(
        self,
        base: FORMAT_SIZE_BASE | None = 2,
        style: FORMAT_SIZE_STYLE | None = "default",
    ):
        self.base = base
        self.style = style

    def format(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __call__(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __repr__(self) -> str: ...


class Size:
    """Bytes-size object."""

    def __init__(self, size: int) -> None: ...
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __hash__(self) -> int: ...
    def __abs__(self) -> Size: ...
    def __neg__(self) -> Size: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Size | int | float) -> bool: ...
    def __le__(self, other: Size | int | float) -> bool: ...
    def __gt__(self, other: Size | int | float) -> bool: ...
    def __ge__(self, other: Size | int | float) -> bool: ...
    def __bool__(self) -> bool: ...
    def __pos__(self) -> Size: ...
    def __invert__(self) -> Size: ...
    def __add__(self, other: Size | int | float) -> Size: ...
    def __sub__(self, other: Size | int | float) -> Size: ...
    def __mul__(self, other: Size | int | float) -> Size: ...
    @property
    def bytes(self) -> int: ...
    def format(
        self,
        base: FORMAT_SIZE_BASE | None = 2,
        style: FORMAT_SIZE_STYLE | None = "default",
    ) -> str: ...

    # =========================================================================
    # CLASS-METHODS
    # =========================================================================

    # -------------------------------------------------------------------------
    # PARSING
    # -------------------------------------------------------------------------
    @classmethod
    def parse(cls: type[Size], size: str) -> Size: ...
    @classmethod
    def from_str(cls: type[Size], size: str) -> Size: ...

    # -------------------------------------------------------------------------
    # BYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_bytes(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # KILOBYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_kb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_kib(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_kibibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_kilobytes(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # MEGABYTES
    # -------------------------------------------------------------------------

    @classmethod
    def from_mb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_mebibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_megabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_mib(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # GIGABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_gb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_gib(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_gibibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_gigabytes(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # TERABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_tb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_tebibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_terabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_tib(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # PETABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_pb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_pebibytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_petabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_pib(cls: type[Size], size: int | float) -> Size: ...

    # -------------------------------------------------------------------------
    # EXABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_eb(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_eib(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_exabytes(cls: type[Size], size: int | float) -> Size: ...
    @classmethod
    def from_exbibytes(cls: type[Size], size: int | float) -> Size: ...

```
## `ry.ryo3._sqlformat`

```python
from __future__ import annotations

import typing as t

# =============================================================================
# SQLFORMAT
# =============================================================================
SqlfmtParamValue = str | int | float | bool
TSqlfmtParamValue_co = t.TypeVar(
    "TSqlfmtParamValue_co", bound=SqlfmtParamValue, covariant=True
)
SqlfmtParamsLike = (
    dict[str, TSqlfmtParamValue_co]
    | t.Sequence[tuple[str, TSqlfmtParamValue_co]]
    | t.Sequence[TSqlfmtParamValue_co]
)


class SqlfmtQueryParams:
    def __init__(
        self, params: SqlfmtParamsLike[TSqlfmtParamValue_co]
    ) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


def sqlfmt_params(
    params: SqlfmtParamsLike[TSqlfmtParamValue_co] | SqlfmtQueryParams,
) -> SqlfmtQueryParams: ...
def sqlfmt(
    sql: str,
    params: SqlfmtParamsLike[TSqlfmtParamValue_co]
    | SqlfmtQueryParams
    | None = None,
    *,
    indent: int = 2,  # -1 or any negative value will use tabs
    uppercase: bool | None = True,
    lines_between_statements: int = 1,
) -> str: ...

```
## `ry.ryo3._std`

```python
"""ryo3-std types"""

from __future__ import annotations

import datetime as pydt
import typing as t

from ry import Bytes, FsPath
from ry._types import Buffer, FsPathLike


# =============================================================================
# STD::TIME
# =============================================================================
class Duration:
    ZERO: Duration
    MIN: Duration
    MAX: Duration
    NANOSECOND: Duration
    MICROSECOND: Duration
    MILLISECOND: Duration
    SECOND: Duration

    def __init__(self, secs: int = 0, nanos: int = 0) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Duration) -> bool: ...
    def __le__(self, other: Duration) -> bool: ...
    def __gt__(self, other: Duration) -> bool: ...
    def __ge__(self, other: Duration) -> bool: ...
    def __hash__(self) -> int: ...
    def __richcmp__(
        self, other: Duration | pydt.timedelta, op: int
    ) -> bool: ...
    def __str__(self) -> str: ...
    def abs_diff(self, other: Duration) -> Duration: ...
    def sleep(self) -> None: ...

    # =========================================================================
    # PYTHON_CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(
        cls: type[Duration], td: pydt.timedelta
    ) -> Duration: ...
    def to_pytimedelta(self) -> pydt.timedelta: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def is_zero(self) -> bool: ...
    @property
    def nanos(self) -> int: ...
    @property
    def secs(self) -> int: ...
    @property
    def days(self) -> int: ...
    @property
    def seconds(self) -> int: ...
    @property
    def microseconds(self) -> int: ...
    @property
    def subsec_micros(self) -> int: ...
    @property
    def subsec_millis(self) -> int: ...
    @property
    def subsec_nanos(self) -> int: ...

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def from_hours(cls, hours: int) -> Duration: ...
    @classmethod
    def from_micros(cls, micros: int) -> Duration: ...
    @classmethod
    def from_millis(cls, millis: int) -> Duration: ...
    @classmethod
    def from_mins(cls, mins: int) -> Duration: ...
    @classmethod
    def from_nanos(cls, nanos: int) -> Duration: ...
    @classmethod
    def from_secs(cls, secs: int) -> Duration: ...
    @classmethod
    def from_secs_f32(cls, secs: float) -> Duration: ...
    @classmethod
    def from_secs_f64(cls, secs: float) -> Duration: ...
    @classmethod
    def from_days(cls, days: int) -> Duration: ...
    @classmethod
    def from_weeks(cls, weeks: int) -> Duration: ...
    def as_micros(self) -> int: ...
    def as_millis(self) -> int: ...
    def as_nanos(self) -> int: ...
    def as_secs(self) -> int: ...
    def as_secs_f32(self) -> float: ...
    def as_secs_f64(self) -> float: ...

    # =========================================================================
    # NOT IMPLEMENTED
    # =========================================================================
    def checked_add(self, other: Duration) -> Duration | None: ...
    def checked_div(self, other: Duration) -> Duration | None: ...
    def checked_mul(self, other: Duration) -> Duration | None: ...
    def checked_sub(self, other: Duration) -> Duration | None: ...
    def div_duration_f32(self, other: Duration) -> float: ...
    def div_duration_f64(self, other: Duration) -> float: ...
    def div_f32(self, other: float) -> Duration: ...
    def div_f64(self, other: float) -> Duration: ...
    def mul_f32(self, other: float) -> Duration: ...
    def mul_f64(self, other: float) -> Duration: ...
    def saturating_add(self, other: Duration) -> Duration: ...
    def saturating_mul(self, other: Duration) -> Duration: ...
    def saturating_sub(self, other: Duration) -> Duration: ...


class Instant:
    def __init__(self) -> None: ...
    @classmethod
    def now(cls) -> Instant: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Instant) -> bool: ...
    def __le__(self, other: Instant) -> bool: ...
    def __gt__(self, other: Instant) -> bool: ...
    def __ge__(self, other: Instant) -> bool: ...
    def __hash__(self) -> int: ...
    def __add__(self, other: Duration) -> Instant: ...
    def __iadd__(self, other: Duration) -> Instant: ...
    @t.overload
    def __sub__(self, other: Duration) -> Instant: ...
    @t.overload
    def __sub__(self, other: Instant) -> Duration: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def checked_add(self, other: Duration) -> Instant | None: ...
    def checked_duration_since(self, earlier: Instant) -> Duration | None: ...
    def checked_sub(self, other: Duration) -> Instant | None: ...
    def duration_since(self, earlier: Instant) -> Duration: ...
    def elapsed(self) -> Duration: ...
    def saturating_duration_since(self, earlier: Instant) -> Duration: ...


def instant() -> Instant: ...
def sleep(seconds: float) -> float: ...


# =============================================================================
# STD::FS
# =============================================================================
class FileType:
    def __repr__(self) -> str: ...
    @property
    def is_dir(self) -> bool: ...
    @property
    def is_file(self) -> bool: ...
    @property
    def is_symlink(self) -> bool: ...


class Metadata:
    def __repr__(self) -> str: ...
    @property
    def file_type(self) -> FileType: ...
    @property
    def len(self) -> int: ...
    @property
    def is_empty(self) -> bool: ...
    @property
    def modified(self) -> pydt.datetime: ...
    @property
    def accessed(self) -> pydt.datetime: ...
    @property
    def created(self) -> pydt.datetime: ...
    @property
    def is_dir(self) -> bool: ...
    @property
    def is_file(self) -> bool: ...
    @property
    def is_symlink(self) -> bool: ...


# =============================================================================
# STD::FS ~ functions
# =============================================================================
def read(path: FsPathLike) -> Bytes: ...
def read_bytes(path: FsPathLike) -> bytes: ...
def read_text(path: FsPathLike) -> str: ...
def read_stream(
    path: FsPathLike,
    chunk_size: int = 65536,
    *,
    offset: int = 0,
) -> t.Iterator[Bytes]: ...
def write(path: FsPathLike, data: Buffer | str) -> int: ...
def write_bytes(path: FsPathLike, data: bytes) -> int: ...
def write_text(path: FsPathLike, data: str) -> int: ...
def canonicalize(path: FsPathLike) -> FsPath: ...
def copy(from_path: FsPathLike, to_path: FsPathLike) -> int: ...
def create_dir(path: FsPathLike) -> None: ...
def create_dir_all(path: FsPathLike) -> None: ...
def exists(path: FsPathLike) -> bool: ...
def is_dir(path: FsPathLike) -> bool: ...
def is_file(path: FsPathLike) -> bool: ...
def is_symlink(path: FsPathLike) -> bool: ...
def metadata(path: FsPathLike) -> Metadata: ...
def remove_dir(path: FsPathLike) -> None: ...
def remove_dir_all(path: FsPathLike) -> None: ...
def remove_file(path: FsPathLike) -> None: ...
def rename(from_path: FsPathLike, to_path: FsPathLike) -> None: ...

```
## `ry.ryo3._tokio`

```python
"""ryo3-tokio types"""

from __future__ import annotations

from typing import NoReturn

from ry import Bytes
from ry._types import Buffer, FsPathLike


# =============================================================================
# FS
# =============================================================================
async def copy_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def create_dir_async(path: FsPathLike) -> None: ...
async def metadata_async(path: FsPathLike) -> None: ...
async def read_async(path: FsPathLike) -> Bytes: ...
async def read_dir_async(path: FsPathLike) -> NoReturn: ...
async def remove_dir_async(path: FsPathLike) -> None: ...
async def remove_file_async(path: FsPathLike) -> None: ...
async def rename_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def write_async(path: FsPathLike, data: Buffer) -> None: ...


# =============================================================================
# SLEEP
# =============================================================================
async def sleep_async(seconds: float) -> float: ...
async def asleep(seconds: float) -> float:
    """Alias for sleep_async"""
    ...

```
## `ry.ryo3._unindent`

```python
"""ryo3-unindent types"""

from __future__ import annotations


def unindent(string: str) -> str: ...
def unindent_bytes(string: bytes) -> bytes: ...

```
## `ry.ryo3._url`

```python
from __future__ import annotations

from ipaddress import IPv4Address


class URL:
    def __init__(
        self, url: str | URL, *, params: dict[str, str] | None = None
    ) -> None: ...

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def parse(cls, url: str) -> URL: ...
    @classmethod
    def parse_with_params(cls, url: str, params: dict[str, str]) -> URL: ...
    @classmethod
    def from_directory_path(cls, path: str) -> URL: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDER
    # =========================================================================
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: URL) -> bool: ...
    def __gt__(self, other: URL) -> bool: ...
    def __hash__(self) -> int: ...
    def __le__(self, other: URL) -> bool: ...
    def __lt__(self, other: URL) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __rtruediv__(self, relative: str) -> URL: ...
    def __truediv__(self, relative: str) -> URL: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def authority(self) -> str: ...
    @property
    def fragment(self) -> str | None: ...
    @property
    def host(self) -> str | None: ...
    @property
    def host_str(self) -> str | None: ...
    @property
    def netloc(self) -> str: ...
    @property
    def password(self) -> str | None: ...
    @property
    def path(self) -> str: ...
    @property
    def path_segments(self) -> tuple[str, ...]: ...
    @property
    def port(self) -> int | None: ...
    @property
    def port_or_known_default(self) -> int | None: ...
    @property
    def query(self) -> str | None: ...
    @property
    def query_pairs(self) -> list[tuple[str, str]]: ...
    @property
    def scheme(self) -> str: ...
    @property
    def username(self) -> str: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def has_authority(self) -> bool: ...
    def has_host(self) -> bool: ...
    def is_special(self) -> bool: ...
    def join(self, *parts: str) -> URL: ...
    def to_filepath(self) -> str: ...
    def replace_fragment(self, fragment: str | None = None) -> URL: ...
    def replace_host(self, host: str | None = None) -> URL: ...
    def replace_ip_host(self, host: IPv4Address | IPv4Address) -> URL: ...
    def replace_password(self, password: str | None = None) -> URL: ...
    def replace_path(self, path: str) -> URL: ...
    def replace_port(self, port: int | None = None) -> URL: ...
    def replace_query(self, query: str | None = None) -> URL: ...
    def replace_scheme(self, scheme: str) -> URL: ...
    def replace_username(self, username: str) -> URL: ...
    def socket_addrs(self) -> None: ...

```
## `ry.ryo3._walkdir`

```python
"""ryo3-walkdir types"""

from __future__ import annotations

import typing as t
from os import PathLike

from ry import FileType, FsPath, Glob, GlobSet, Globster


class WalkDirEntry:
    def __fspath__(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    @property
    def path(self) -> FsPath: ...
    @property
    def file_name(self) -> str: ...
    @property
    def depth(self) -> int: ...
    @property
    def path_is_symlink(self) -> bool: ...
    @property
    def file_type(self) -> FileType: ...
    @property
    def is_dir(self) -> bool: ...
    @property
    def is_file(self) -> bool: ...
    @property
    def is_symlink(self) -> bool: ...
    @property
    def len(self) -> int: ...


class WalkdirGen:
    """walkdir::Walkdir iterable wrapper"""

    files: bool
    dirs: bool

    def __next__(self) -> str: ...
    def __iter__(self) -> t.Iterator[str]: ...
    def collect(self) -> list[str]: ...


def walkdir(
    path: str | PathLike[str] | None = None,
    *,
    files: bool = True,
    dirs: bool = True,
    contents_first: bool = False,
    min_depth: int = 0,
    max_depth: int | None = None,
    follow_links: bool = False,
    same_file_system: bool = False,
    glob: Glob | GlobSet | Globster | t.Sequence[str] | str | None = None,
) -> WalkdirGen: ...

```
## `ry.ryo3._which`

```python
"""ryo3-which types"""

from __future__ import annotations

from ry.ryo3._regex import Regex


def which(cmd: str, path: None | str = None) -> str | None: ...
def which_all(cmd: str, path: None | str = None) -> list[str]: ...
def which_re(regex: str | Regex, path: None | str = None) -> list[str]: ...

```
## `ry.ryo3._zstd`

```python
"""ryo3-zstd types"""

from __future__ import annotations

# =============================================================================
# ZSTD
# =============================================================================


def zstd_decode(input: bytes) -> bytes: ...
def zstd_encode(input: bytes, level: int = 3) -> bytes: ...
def zstd(input: bytes, level: int = 3) -> bytes:
    """Alias for zstd_encode"""

```
## `ry.ryo3.errors`

```python
from __future__ import annotations


class FeatureNotEnabledError(RuntimeError):
    """Raised when a feature is not enabled in the current build."""

```
<!-- API-END -->

___

## DEV

- `just` is used to run tasks
- Do not use the phrase `blazing fast` or any emojis in any PRs or issues or
  docs
- type annotations are required
- `ruff` used for formatting and linting

___

## SEE ALSO

- utiles (web-map tile utils): https://github.com/jessekrubin/utiles
- jsonc2json (jsonc to json converter):
  https://github.com/jessekrubin/jsonc2json
