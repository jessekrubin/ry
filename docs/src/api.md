# API

## Table of Contents
- [`ry.ryo3.__init__`](#ry.ryo3.__init__)
- [`ry.ryo3.dirs`](#ry.ryo3.dirs)
- [`ry.ryo3.errors`](#ry.ryo3.errors)
- [`ry.ryo3.JSON`](#ry.ryo3.JSON)
- [`ry.ryo3.orjson`](#ry.ryo3.orjson)
- [`ry.ryo3.sh`](#ry.ryo3.sh)
- [`ry.ryo3.ulid`](#ry.ryo3.ulid)
- [`ry.ryo3.uuid`](#ry.ryo3.uuid)
- [`ry.ryo3.xxhash`](#ry.ryo3.xxhash)
- [`ry.ryo3.zstd`](#ry.ryo3.zstd)
- [`ry.ryo3._brotli`](#ry.ryo3._brotli)
- [`ry.ryo3._bytes`](#ry.ryo3._bytes)
- [`ry.ryo3._bzip2`](#ry.ryo3._bzip2)
- [`ry.ryo3._dev`](#ry.ryo3._dev)
- [`ry.ryo3._flate2`](#ry.ryo3._flate2)
- [`ry.ryo3._fnv`](#ry.ryo3._fnv)
- [`ry.ryo3._fspath`](#ry.ryo3._fspath)
- [`ry.ryo3._glob`](#ry.ryo3._glob)
- [`ry.ryo3._globset`](#ry.ryo3._globset)
- [`ry.ryo3._heck`](#ry.ryo3._heck)
- [`ry.ryo3._http`](#ry.ryo3._http)
- [`ry.ryo3._jiff`](#ry.ryo3._jiff)
- [`ry.ryo3._jiff_tz`](#ry.ryo3._jiff_tz)
- [`ry.ryo3._jiter`](#ry.ryo3._jiter)
- [`ry.ryo3._quick_maths`](#ry.ryo3._quick_maths)
- [`ry.ryo3._regex`](#ry.ryo3._regex)
- [`ry.ryo3._reqwest`](#ry.ryo3._reqwest)
- [`ry.ryo3._same_file`](#ry.ryo3._same_file)
- [`ry.ryo3._shlex`](#ry.ryo3._shlex)
- [`ry.ryo3._size`](#ry.ryo3._size)
- [`ry.ryo3._sqlformat`](#ry.ryo3._sqlformat)
- [`ry.ryo3._std`](#ry.ryo3._std)
- [`ry.ryo3._tokio`](#ry.ryo3._tokio)
- [`ry.ryo3._unindent`](#ry.ryo3._unindent)
- [`ry.ryo3._url`](#ry.ryo3._url)
- [`ry.ryo3._walkdir`](#ry.ryo3._walkdir)
- [`ry.ryo3._which`](#ry.ryo3._which)
- [`ry.ryo3._zstd`](#ry.ryo3._zstd)
- [`ry.dirs`](#ry.dirs)
- [`ry.JSON`](#ry.JSON)
- [`ry.ulid`](#ry.ulid)
- [`ry.uuid`](#ry.uuid)
- [`ry.xxhash`](#ry.xxhash)
- [`ry.zstd`](#ry.zstd)
<h2 id="ry.ryo3.__init__"><code>ry.ryo3.__init__</code></h2>

```python
"""ry api ~ type annotations"""

from ry import ulid as ulid  # noqa: RUF100
from ry import uuid as uuid  # noqa: RUF100
from ry.ryo3 import JSON as JSON
from ry.ryo3._brotli import brotli as brotli
from ry.ryo3._brotli import brotli_decode as brotli_decode
from ry.ryo3._brotli import brotli_encode as brotli_encode
from ry.ryo3._bytes import Bytes as Bytes
from ry.ryo3._bzip2 import bzip2 as bzip2
from ry.ryo3._bzip2 import bzip2_decode as bzip2_decode
from ry.ryo3._bzip2 import bzip2_encode as bzip2_encode
from ry.ryo3._flate2 import gunzip as gunzip
from ry.ryo3._flate2 import gzip as gzip
from ry.ryo3._flate2 import gzip_decode as gzip_decode
from ry.ryo3._flate2 import gzip_encode as gzip_encode
from ry.ryo3._flate2 import is_gzipped as is_gzipped
from ry.ryo3._fnv import FnvHasher as FnvHasher
from ry.ryo3._fnv import fnv1a as fnv1a
from ry.ryo3._fspath import FsPath as FsPath
from ry.ryo3._glob import Pattern as Pattern
from ry.ryo3._glob import glob as glob
from ry.ryo3._globset import Glob as Glob
from ry.ryo3._globset import GlobSet as GlobSet
from ry.ryo3._globset import Globster as Globster
from ry.ryo3._globset import globster as globster
from ry.ryo3._heck import camel_case as camel_case
from ry.ryo3._heck import kebab_case as kebab_case
from ry.ryo3._heck import pascal_case as pascal_case
from ry.ryo3._heck import shouty_kebab_case as shouty_kebab_case
from ry.ryo3._heck import shouty_snake_case as shouty_snake_case
from ry.ryo3._heck import snake_case as snake_case
from ry.ryo3._heck import snek_case as snek_case
from ry.ryo3._heck import title_case as title_case
from ry.ryo3._heck import train_case as train_case
from ry.ryo3._http import Headers as Headers  # noqa: RUF100
from ry.ryo3._http import HttpStatus as HttpStatus  # noqa: RUF100
from ry.ryo3._jiff import Date as Date
from ry.ryo3._jiff import DateDifference as DateDifference
from ry.ryo3._jiff import DateTime as DateTime
from ry.ryo3._jiff import DateTimeDifference as DateTimeDifference
from ry.ryo3._jiff import DateTimeRound as DateTimeRound
from ry.ryo3._jiff import ISOWeekDate as ISOWeekDate
from ry.ryo3._jiff import Offset as Offset
from ry.ryo3._jiff import SignedDuration as SignedDuration
from ry.ryo3._jiff import Time as Time
from ry.ryo3._jiff import TimeDifference as TimeDifference
from ry.ryo3._jiff import TimeSpan as TimeSpan
from ry.ryo3._jiff import Timestamp as Timestamp
from ry.ryo3._jiff import TimestampDifference as TimestampDifference
from ry.ryo3._jiff import TimestampRound as TimestampRound
from ry.ryo3._jiff import TimeZone as TimeZone
from ry.ryo3._jiff import TimeZoneDatabase as TimeZoneDatabase
from ry.ryo3._jiff import ZonedDateTime as ZonedDateTime
from ry.ryo3._jiff import ZonedDateTimeDifference as ZonedDateTimeDifference
from ry.ryo3._jiff import ZonedDateTimeRound as ZonedDateTimeRound
from ry.ryo3._jiff import date as date
from ry.ryo3._jiff import datetime as datetime
from ry.ryo3._jiff import now as now
from ry.ryo3._jiff import offset as offset
from ry.ryo3._jiff import time as time
from ry.ryo3._jiff import timespan as timespan
from ry.ryo3._jiff import utcnow as utcnow
from ry.ryo3._jiff import zoned as zoned
from ry.ryo3._jiter import JsonParseKwargs as JsonParseKwargs
from ry.ryo3._jiter import JsonPrimitive as JsonPrimitive
from ry.ryo3._jiter import JsonValue as JsonValue
from ry.ryo3._jiter import json_cache_clear as json_cache_clear
from ry.ryo3._jiter import json_cache_usage as json_cache_usage
from ry.ryo3._jiter import parse_json as parse_json
from ry.ryo3._jiter import parse_jsonl as parse_jsonl
from ry.ryo3._jiter import read_json as read_json
from ry.ryo3._quick_maths import quick_maths as quick_maths
from ry.ryo3._regex import Regex as Regex
from ry.ryo3._reqwest import HttpClient as HttpClient
from ry.ryo3._reqwest import ReqwestError as ReqwestError
from ry.ryo3._reqwest import Response as Response
from ry.ryo3._reqwest import ResponseStream as ResponseStream
from ry.ryo3._reqwest import fetch as fetch
from ry.ryo3._same_file import is_same_file as is_same_file
from ry.ryo3._shlex import shplit as shplit
from ry.ryo3._size import Size as Size
from ry.ryo3._size import SizeFormatter as SizeFormatter
from ry.ryo3._size import fmt_size as fmt_size
from ry.ryo3._size import parse_size as parse_size
from ry.ryo3._sqlformat import SqlfmtQueryParams as SqlfmtQueryParams
from ry.ryo3._sqlformat import sqlfmt as sqlfmt
from ry.ryo3._sqlformat import sqlfmt_params as sqlfmt_params
from ry.ryo3._std import Duration as Duration
from ry.ryo3._std import FileReadStream as FileReadStream
from ry.ryo3._std import FileType as FileType
from ry.ryo3._std import Instant as Instant
from ry.ryo3._std import IpAddr as IpAddr
from ry.ryo3._std import Ipv4Addr as Ipv4Addr
from ry.ryo3._std import Ipv6Addr as Ipv6Addr
from ry.ryo3._std import Metadata as Metadata
from ry.ryo3._std import canonicalize as canonicalize
from ry.ryo3._std import copy as copy
from ry.ryo3._std import create_dir as create_dir
from ry.ryo3._std import create_dir_all as create_dir_all
from ry.ryo3._std import exists as exists
from ry.ryo3._std import instant as instant
from ry.ryo3._std import is_dir as is_dir
from ry.ryo3._std import is_file as is_file
from ry.ryo3._std import is_symlink as is_symlink
from ry.ryo3._std import metadata as metadata
from ry.ryo3._std import read as read
from ry.ryo3._std import read_bytes as read_bytes
from ry.ryo3._std import read_dir as read_dir
from ry.ryo3._std import read_stream as read_stream
from ry.ryo3._std import read_text as read_text
from ry.ryo3._std import remove_dir as remove_dir
from ry.ryo3._std import remove_dir_all as remove_dir_all
from ry.ryo3._std import remove_file as remove_file
from ry.ryo3._std import rename as rename
from ry.ryo3._std import sleep as sleep
from ry.ryo3._std import write as write
from ry.ryo3._std import write_bytes as write_bytes
from ry.ryo3._std import write_text as write_text
from ry.ryo3._tokio import AsyncFile as AsyncFile
from ry.ryo3._tokio import aiopen as aiopen
from ry.ryo3._tokio import asleep as asleep
from ry.ryo3._tokio import canonicalize_async as canonicalize_async
from ry.ryo3._tokio import copy_async as copy_async
from ry.ryo3._tokio import create_dir_all_async as create_dir_all_async
from ry.ryo3._tokio import create_dir_async as create_dir_async
from ry.ryo3._tokio import exists_async as exists_async
from ry.ryo3._tokio import hard_link_async as hard_link_async
from ry.ryo3._tokio import metadata_async as metadata_async
from ry.ryo3._tokio import read_async as read_async
from ry.ryo3._tokio import read_dir_async as read_dir_async
from ry.ryo3._tokio import read_link_async as read_link_async
from ry.ryo3._tokio import read_to_string_async as read_to_string_async
from ry.ryo3._tokio import remove_dir_all_async as remove_dir_all_async
from ry.ryo3._tokio import remove_dir_async as remove_dir_async
from ry.ryo3._tokio import remove_file_async as remove_file_async
from ry.ryo3._tokio import rename_async as rename_async
from ry.ryo3._tokio import sleep_async as sleep_async
from ry.ryo3._tokio import try_exists_async as try_exists_async
from ry.ryo3._tokio import write_async as write_async
from ry.ryo3._unindent import unindent as unindent
from ry.ryo3._unindent import unindent_bytes as unindent_bytes
from ry.ryo3._url import URL as URL
from ry.ryo3._walkdir import WalkDirEntry as WalkDirEntry
from ry.ryo3._walkdir import WalkdirGen as WalkdirGen
from ry.ryo3._walkdir import walkdir as walkdir
from ry.ryo3._which import which as which
from ry.ryo3._which import which_all as which_all
from ry.ryo3._which import which_re as which_re
from ry.ryo3._zstd import is_zstd as is_zstd
from ry.ryo3._zstd import zstd_compress as zstd_compress
from ry.ryo3._zstd import zstd_decode as zstd_decode
from ry.ryo3._zstd import zstd_decompress as zstd_decompress
from ry.ryo3._zstd import zstd_encode as zstd_encode
from ry.ryo3.errors import FeatureNotEnabledError as FeatureNotEnabledError
from ry.ryo3.JSON import stringify as stringify
from ry.ryo3.orjson import orjson_default as orjson_default
from ry.ryo3.sh import cd as cd
from ry.ryo3.sh import home as home
from ry.ryo3.sh import ls as ls
from ry.ryo3.sh import mkdir as mkdir
from ry.ryo3.sh import pwd as pwd

# =============================================================================
# CONSTANTS
# =============================================================================
__version__: str
__authors__: str
__build_profile__: str
__build_timestamp__: str
__pkg_name__: str
__description__: str
__target__: str

```

<h2 id="ry.ryo3.dirs"><code>ry.ryo3.dirs</code></h2>

```python
def audio() -> str | None: ...
def audio_dir() -> str | None: ...
def cache() -> str | None: ...
def cache_dir() -> str | None: ...
def config() -> str | None: ...
def config_dir() -> str | None: ...
def config_local() -> str | None: ...
def config_local_dir() -> str | None: ...
def data() -> str | None: ...
def data_dir() -> str | None: ...
def data_local() -> str | None: ...
def data_local_dir() -> str | None: ...
def desktop() -> str | None: ...
def desktop_dir() -> str | None: ...
def document() -> str | None: ...
def document_dir() -> str | None: ...
def download() -> str | None: ...
def download_dir() -> str | None: ...
def executable() -> str | None: ...
def executable_dir() -> str | None: ...
def font() -> str | None: ...
def font_dir() -> str | None: ...
def home() -> str | None: ...
def home_dir() -> str | None: ...
def picture() -> str | None: ...
def picture_dir() -> str | None: ...
def preference() -> str | None: ...
def preference_dir() -> str | None: ...
def public() -> str | None: ...
def public_dir() -> str | None: ...
def runtime() -> str | None: ...
def runtime_dir() -> str | None: ...
def state() -> str | None: ...
def state_dir() -> str | None: ...
def template() -> str | None: ...
def template_dir() -> str | None: ...
def video() -> str | None: ...
def video_dir() -> str | None: ...

```

<h2 id="ry.ryo3.errors"><code>ry.ryo3.errors</code></h2>

```python
class FeatureNotEnabledError(RuntimeError):
    """Raised when a feature is not enabled in the current build."""

```

<h2 id="ry.ryo3.JSON"><code>ry.ryo3.JSON</code></h2>

```python
"""ry.ryo3.JSON"""

import typing as t

import typing_extensions as te

from ry._types import Buffer
from ry.ryo3._bytes import Bytes
from ry.ryo3._jiter import JsonParseKwargs, JsonValue


def minify(data: Buffer) -> Bytes:
    """Return minified json data (remove whitespace, newlines)

    Args:
        data: The JSON data to minify.

    Returns:
        Minified JSON data as a `Bytes` object.

    Examples:
        >>> import json as pyjson
        >>> from ry.ryo3 import JSON
        >>> data = {"key": "value", "number": 123, "bool": True}
        >>> json_str = pyjson.dumps(data, indent=2)
        >>> print(json_str)
        {
          "key": "value",
          "number": 123,
          "bool": true
        }
        >>> bytes(JSON.minify(json_str))
        b'{"key":"value","number":123,"bool":true}'

    """


def fmt(data: Buffer) -> Bytes:
    """Return minified json data (remove whitespace, newlines)

    Args:
        data: The JSON data to minify.

    Returns:
        Minified JSON data as a `Bytes` object.

    Examples:
        >>> import json as pyjson
        >>> from ry.ryo3 import JSON
        >>> data = {"key": "value", "number": 123, "bool": True}
        >>> json_str = pyjson.dumps(data, indent=2)
        >>> print(json_str)
        {
          "key": "value",
          "number": 123,
          "bool": true
        }
        >>> bytes(JSON.fmt(json_str)).decode()
        '{\n  "key": "value",\n  "number": 123,\n  "bool": true\n}'
        >>> print(bytes(JSON.fmt(json_str)).decode())
        {
          "key": "value",
          "number": 123,
          "bool": true
        }

    """


@t.overload
def stringify(
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
def stringify(
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[False] = False,
) -> Bytes: ...
@t.overload
def dumps(
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
def dumps(
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[False] = False,
) -> Bytes: ...
def loads(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def cache_clear() -> None: ...
def cache_usage() -> int: ...

```

<h2 id="ry.ryo3.orjson"><code>ry.ryo3.orjson</code></h2>

```python
"""orjson + ry types

orjson-types: https://github.com/ijl/orjson/blob/master/pysrc/orjson/__init__.pyi
"""

import typing as t

import orjson


def orjson_default(obj: t.Any) -> orjson.Fragment:
    """Fn to be used with `orjson.dumps` to serialize ry-compatible types

    Example:
        >>> import orjson
        >>> from ry import orjson_default, Date
        >>> data = {"key": "value", "date": Date(2023, 10, 1)}
        >>> orjson.dumps(data, default=orjson_default)
        b'{"key":"value","date":"2023-10-01"}'

    """

```

<h2 id="ry.ryo3.sh"><code>ry.ryo3.sh</code></h2>

```python
import typing as t
from os import PathLike

from ry.ryo3._fspath import FsPath


def pwd() -> str: ...
def home() -> str: ...
def cd(path: str | PathLike[str]) -> None: ...
@t.overload
def ls(
    path: str | PathLike[str] | None = None,  # defaults to '.' if None
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[False] = False,
) -> list[str]:
    """List directory contents - returns list of strings"""


@t.overload
def ls(
    path: str | PathLike[str] | None = None,  # defaults to '.' if None
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[True],
) -> list[FsPath]:
    """List directory contents - returns list of FsPath objects"""


def mkdir(path: str | PathLike[str]) -> None: ...

```

<h2 id="ry.ryo3.ulid"><code>ry.ryo3.ulid</code></h2>

```python
import builtins
import datetime as pydt
import uuid
from collections.abc import Callable as Callable
from typing import Any

from pydantic import GetCoreSchemaHandler as GetCoreSchemaHandler
from pydantic import (
    ValidatorFunctionWrapHandler as ValidatorFunctionWrapHandler,
)
from pydantic_core import CoreSchema as CoreSchema


class ULID:
    def __init__(self, value: builtins.bytes | str | None = None) -> None: ...

    # ----------------
    # INSTANCE METHODS
    # ----------------
    def to_uuid(self) -> uuid.UUID: ...
    def to_uuid4(self) -> uuid.UUID: ...

    # ----------
    # PROPERTIES
    # ----------
    @property
    def bytes(self) -> builtins.bytes: ...
    @property
    def milliseconds(self) -> int: ...
    @property
    def timestamp(self) -> float: ...
    @property
    def datetime(self) -> pydt.datetime: ...
    @property
    def hex(self) -> str: ...

    # -------------
    # CLASS METHODS
    # -------------
    @classmethod
    def from_datetime(cls, value: pydt.datetime) -> ULID: ...
    @classmethod
    def from_timestamp(cls, value: float) -> ULID: ...
    @classmethod
    def from_uuid(cls, value: uuid.UUID) -> ULID: ...
    @classmethod
    def from_bytes(cls, bytes_: builtins.bytes) -> ULID: ...
    @classmethod
    def from_hex(cls, value: str) -> ULID: ...
    @classmethod
    def from_str(cls, string: str) -> ULID: ...
    @classmethod
    def from_int(cls, value: int) -> ULID: ...
    @classmethod
    def parse(cls, value: Any) -> ULID: ...

    # --------
    # PYDANTIC
    # --------
    @classmethod
    def __get_pydantic_core_schema__(
        cls, source: Any, handler: GetCoreSchemaHandler
    ) -> CoreSchema: ...

    # -------
    # DUNDERS
    # -------
    def __bytes__(self) -> builtins.bytes: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: int | str | ULID | builtins.bytes) -> bool: ...
    def __gt__(self, other: int | str | ULID | builtins.bytes) -> bool: ...
    def __hash__(self) -> int: ...
    def __int__(self) -> int: ...
    def __le__(self, other: int | str | ULID | builtins.bytes) -> bool: ...
    def __lt__(self, other: int | str | ULID | builtins.bytes) -> bool: ...

```

<h2 id="ry.ryo3.uuid"><code>ry.ryo3.uuid</code></h2>

```python
"""ryo3-uuid types

based on typeshed types for python's builtin uuid module

REF: https://github.com/python/typeshed/blob/main/stdlib/uuid.pyi
"""

import builtins
import uuid as pyuuid
from enum import Enum
from typing import Any

from typing_extensions import TypeAlias

from ry._types import Buffer

_FieldsType: TypeAlias = tuple[int, int, int, int, int, int]


class SafeUUID(Enum):
    safe = 0
    unsafe = -1
    unknown = None


class UUID:
    NAMESPACE_DNS: UUID
    NAMESPACE_URL: UUID
    NAMESPACE_OID: UUID
    NAMESPACE_X500: UUID

    def __init__(
        self,
        hex: str | None = None,
        bytes: builtins.bytes | None = None,
        bytes_le: builtins.bytes | None = None,
        fields: _FieldsType | None = None,
        int: builtins.int | None = None,
        version: builtins.int | None = None,
        *,
        is_safe: SafeUUID = ...,
    ) -> None: ...
    @property
    def is_safe(self) -> SafeUUID: ...
    @property
    def bytes(self) -> builtins.bytes: ...
    @property
    def bytes_le(self) -> builtins.bytes: ...
    @property
    def clock_seq(self) -> builtins.int: ...
    @property
    def clock_seq_hi_variant(self) -> builtins.int: ...
    @property
    def clock_seq_low(self) -> builtins.int: ...
    @property
    def fields(self) -> _FieldsType: ...
    @property
    def hex(self) -> str: ...
    @property
    def int(self) -> builtins.int: ...
    @property
    def node(self) -> builtins.int: ...
    @property
    def time(self) -> builtins.int: ...
    @property
    def time_hi_version(self) -> builtins.int: ...
    @property
    def time_low(self) -> builtins.int: ...
    @property
    def time_mid(self) -> builtins.int: ...
    @property
    def urn(self) -> str: ...
    @property
    def variant(self) -> str: ...
    @property
    def version(self) -> builtins.int | None: ...
    def to_py(self) -> pyuuid.UUID: ...
    def __lt__(self, other: UUID) -> bool: ...
    def __le__(self, other: UUID) -> bool: ...
    def __eq__(self, other: object) -> bool: ...
    def __gt__(self, other: UUID) -> bool: ...
    def __ge__(self, other: UUID) -> bool: ...
    def __hash__(self) -> builtins.int: ...
    def __int__(self) -> builtins.int: ...


def getnode() -> builtins.int: ...
def uuid1(node: int | None = None, clock_seq: int | None = None) -> UUID: ...
def uuid2(*args: Any, **kwargs: Any) -> UUID: ...
def uuid3(namespace: UUID, name: str | builtins.bytes) -> UUID: ...
def uuid4() -> UUID: ...
def uuid5(namespace: UUID, name: str | builtins.bytes) -> UUID: ...
def uuid6(node: int | None = None, clock_seq: int | None = None) -> UUID: ...
def uuid7(timestamp: int | None = None) -> UUID: ...
def uuid8(data: Buffer) -> UUID: ...


NAMESPACE_DNS: UUID
NAMESPACE_URL: UUID
NAMESPACE_OID: UUID
NAMESPACE_X500: UUID
RESERVED_NCS: str
RFC_4122: str
RESERVED_MICROSOFT: str
RESERVED_FUTURE: str

```

<h2 id="ry.ryo3.xxhash"><code>ry.ryo3.xxhash</code></h2>

```python
import typing as t

from ry._types import Buffer


@t.final
class Xxh32:
    name: t.Literal["xxh32"]
    digest_size: t.Literal[4]
    block_size: t.Literal[16]

    def __init__(self, input: Buffer = ..., seed: int | None = ...) -> None: ...
    def update(self, input: Buffer) -> None: ...
    def digest(self) -> bytes: ...
    def hexdigest(self) -> str: ...
    def intdigest(self) -> int: ...
    def copy(self) -> Xxh32: ...
    def reset(self, seed: int | None = ...) -> None: ...
    @property
    def seed(self) -> int: ...


@t.final
class Xxh64:
    name: t.Literal["xxh64"]
    digest_size: t.Literal[8]
    block_size: t.Literal[32]

    def __init__(
        self, input: Buffer | None = None, seed: int | None = ...
    ) -> None: ...
    def update(self, input: Buffer) -> None: ...
    def digest(self) -> bytes: ...
    def hexdigest(self) -> str: ...
    def intdigest(self) -> int: ...
    def copy(self) -> Xxh64: ...
    def reset(self, seed: int | None = ...) -> None: ...
    @property
    def seed(self) -> int: ...


@t.final
class Xxh3:
    name: t.Literal["xxh3"]
    digest_size: int  # xxh3_64: 8, xxh3_128: 16
    block_size: int  # xxh3_64: 32, xxh3_128: 64

    def __init__(
        self,
        input: Buffer = ...,
        seed: int | None = ...,
        secret: bytes | None = ...,
    ) -> None: ...
    def update(self, input: Buffer) -> None: ...
    def digest(self) -> bytes: ...
    def hexdigest(self) -> str: ...
    def intdigest(self) -> int: ...
    @property
    def seed(self) -> int: ...
    def digest128(self) -> bytes: ...
    def hexdigest128(self) -> str: ...
    def intdigest128(self) -> int: ...
    def copy(self) -> Xxh3: ...
    def reset(self) -> None: ...


# constructor aliases
def xxh32(input: Buffer | None = None, seed: int | None = None) -> Xxh32: ...
def xxh64(input: Buffer | None = None, seed: int | None = None) -> Xxh64: ...
def xxh3(
    input: Buffer | None = None,
    seed: int | None = None,
    secret: bytes | None = None,
) -> Xxh3: ...


# -----------------------------------------------------------------------------
# ONE-SHOT FUNCTIONS
# -----------------------------------------------------------------------------


# xxh32
def xxh32_digest(input: Buffer, seed: int | None = None) -> bytes: ...
def xxh32_hexdigest(input: Buffer, seed: int | None = None) -> str: ...
def xxh32_intdigest(input: Buffer, seed: int | None = None) -> int: ...


# xxh64
def xxh64_digest(input: Buffer, seed: int | None = None) -> bytes: ...
def xxh64_hexdigest(input: Buffer, seed: int | None = None) -> str: ...
def xxh64_intdigest(input: Buffer, seed: int | None = None) -> int: ...


# xxh128
def xxh128_digest(input: Buffer, seed: int | None = None) -> bytes: ...
def xxh128_hexdigest(input: Buffer, seed: int | None = None) -> str: ...
def xxh128_intdigest(input: Buffer, seed: int | None = None) -> int: ...


# xxh3
def xxh3_64_digest(input: Buffer, seed: int | None = None) -> bytes: ...
def xxh3_64_intdigest(input: Buffer, seed: int | None = None) -> int: ...
def xxh3_64_hexdigest(input: Buffer, seed: int | None = None) -> str: ...
def xxh3_digest(input: Buffer, seed: int | None = None) -> bytes: ...
def xxh3_intdigest(input: Buffer, seed: int | None = None) -> int: ...
def xxh3_hexdigest(input: Buffer, seed: int | None = None) -> str: ...


# xxh128
def xxh3_128_digest(input: Buffer, seed: int | None = None) -> bytes: ...
def xxh3_128_intdigest(input: Buffer, seed: int | None = None) -> int: ...
def xxh3_128_hexdigest(input: Buffer, seed: int | None = None) -> str: ...

```

<h2 id="ry.ryo3.zstd"><code>ry.ryo3.zstd</code></h2>

```python
from ry import Bytes
from ry._types import Buffer

__zstd_version__: str  # zstd version string ("1.5.7" as of 2025-03-14)
BLOCKSIZELOG_MAX: int
BLOCKSIZE_MAX: int
CLEVEL_DEFAULT: int  # default=3 (as of 2025-03-14)
CONTENTSIZE_ERROR: int
CONTENTSIZE_UNKNOWN: int
MAGICNUMBER: int
MAGIC_DICTIONARY: int
MAGIC_SKIPPABLE_MASK: int
MAGIC_SKIPPABLE_START: int
VERSION_MAJOR: int
VERSION_MINOR: int
VERSION_NUMBER: int
VERSION_RELEASE: int


# =============================================================================
# PYFUNCTIONS
# =============================================================================
# __COMPRESSION__
def compress(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...
def encode(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...
def zstd(data: Buffer, level: int = CLEVEL_DEFAULT) -> Bytes: ...


# __DECOMPRESSION__
def decode(data: Buffer) -> Bytes: ...
def decompress(data: Buffer) -> Bytes: ...
def unzstd(data: Buffer) -> Bytes: ...


# __MAGIC__
def is_zstd(data: Buffer) -> bool: ...

```

<h2 id="ry.ryo3._brotli"><code>ry.ryo3._brotli</code></h2>

```python
"""ryo3-brotli types"""


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

<h2 id="ry.ryo3._bytes"><code>ry.ryo3._bytes</code></h2>

```python
import sys
from typing import overload

import typing_extensions

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
    def __buffer__(self, flags: int) -> memoryview: ...
    def __contains__(self, other: Buffer) -> bool: ...
    def __eq__(self, other: object) -> bool: ...
    @overload
    def __getitem__(self, other: int) -> int: ...
    @overload
    def __getitem__(self, other: slice) -> Bytes: ...
    def __mul__(self, other: Buffer) -> int: ...
    def __len__(self) -> int: ...
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

    def hex(
        self, sep: str | None = None, bytes_per_sep: int | None = None
    ) -> str:
        """Return a hexadecimal representation of the binary data."""

    @classmethod
    def fromhex(cls, hexstr: str) -> Bytes:
        """Construct a `Bytes` object from a hexadecimal string."""

    def startswith(self, prefix: Buffer) -> bool:
        """Return `True` if the binary data starts with the prefix string, `False` otherwise."""

    def endswith(self, suffix: Buffer) -> bool:
        """Return `True` if the binary data ends with the suffix string, `False` otherwise."""

    def capitalize(self) -> Bytes:
        """
        Return a copy of the sequence with the first byte converted to uppercase and
        all other bytes converted to lowercase.
        """

    def strip(self, chars: Buffer | None = None) -> Bytes:
        """
        Return a copy of the sequence with leading and trailing bytes removed.
        If `chars` is provided, remove all bytes in `chars` from both ends.
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


BytesLike: typing_extensions.TypeAlias = (
    Buffer | bytes | bytearray | memoryview | Bytes
)

```

<h2 id="ry.ryo3._bzip2"><code>ry.ryo3._bzip2</code></h2>

```python
"""ryo3-bzip2 types"""

from ry._types import Buffer


# =============================================================================
# BZIP2
# =============================================================================
def bzip2_encode(input: Buffer, quality: int = 9) -> bytes: ...
def bzip2_decode(input: Buffer) -> bytes: ...
def bzip2(input: Buffer, quality: int = 9) -> bytes:
    """Alias for bzip2_encode"""

```

<h2 id="ry.ryo3._dev"><code>ry.ryo3._dev</code></h2>

```python
"""ry.ryo3.dev"""

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

<h2 id="ry.ryo3._flate2"><code>ry.ryo3._flate2</code></h2>

```python
"""ryo3-flate2 types"""

from ry import Bytes
from ry._types import Buffer


# =============================================================================
# GZIP
# =============================================================================
def gzip_encode(input: Buffer, quality: int = 9) -> Bytes: ...
def gzip_decode(input: Buffer) -> Bytes: ...
def gzip(input: Buffer, quality: int = 9) -> Bytes:
    """Alias for gzip_encode"""


def gunzip(input: Buffer) -> Bytes:
    """Alias for gzip_decode"""


def is_gzipped(input: Buffer) -> bool: ...

```

<h2 id="ry.ryo3._fnv"><code>ry.ryo3._fnv</code></h2>

```python
"""ryo3-fnv types"""

import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes


@t.final
class FnvHasher:
    name: t.Literal["fnv1a"]
    digest_size: t.Literal[8]
    block_size: t.Literal[1]

    def __init__(
        self, input: Buffer | None = None, key: int | None = None
    ) -> None: ...
    def update(self, input: Buffer) -> None: ...
    def digest(self) -> Bytes: ...
    def intdigest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> FnvHasher: ...


def fnv1a(input: Buffer, key: int | None = None) -> FnvHasher: ...

```

<h2 id="ry.ryo3._fspath"><code>ry.ryo3._fspath</code></h2>

```python
"""ryo3-fspath types"""

import typing as t
from os import PathLike
from pathlib import Path

from ry._types import Buffer, ToPy
from ry.ryo3._bytes import Bytes
from ry.ryo3._regex import Regex
from ry.ryo3._std import Metadata


# =============================================================================
# FSPATH
# =============================================================================
@t.final
class FsPath(ToPy[Path]):
    def __init__(self, path: PathLike[str] | str | None = None) -> None: ...
    def __fspath__(self) -> str: ...
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
    def to_py(self) -> Path: ...
    def to_pathlib(self) -> Path: ...
    # =========================================================================
    # IO
    # =========================================================================
    def read(self) -> Bytes: ...
    def read_bytes(self) -> bytes: ...
    def read_text(self) -> str: ...
    def write(self, data: Buffer | bytes) -> None: ...
    def write_bytes(self, data: Buffer | bytes) -> None: ...
    def write_text(self, data: str) -> None: ...
    def open(
        self,
        mode: str,
        buffering: int = -1,
        encoding: str | None = None,
        errors: str | None = None,
        newline: str | None = None,
    ) -> t.IO[t.Any]: ...

    # =========================================================================
    # METHODS
    # =========================================================================
    def absolute(self) -> FsPath: ...
    def as_posix(self) -> str: ...
    def as_uri(self) -> str: ...
    def clone(self) -> FsPath: ...
    def equiv(self, other: PathLike[str] | str | FsPath) -> bool: ...
    def exists(self) -> bool: ...
    def iterdir(self) -> FsPathReaddir: ...
    def join(self, *paths: str) -> FsPath: ...
    def joinpath(self, *paths: str) -> FsPath: ...
    def metadata(self) -> Metadata: ...
    def mkdir(
        self, mode: int = 0o777, parents: bool = False, exist_ok: bool = False
    ) -> None: ...
    def read_dir(self) -> FsPathReaddir: ...
    def read_link(self) -> FsPath: ...
    def relative_to(self, other: PathLike[str] | str) -> FsPath: ...
    def rename(self, new_path: PathLike[str] | str) -> FsPath: ...
    def replace(self, new_path: PathLike[str] | str) -> FsPath: ...
    def resolve(self) -> FsPath: ...
    def rmdir(self, recursive: bool = False) -> None: ...
    def string(self) -> str: ...
    def unlink(
        self, missing_ok: bool = False, recursive: bool = False
    ) -> None: ...
    def with_name(self, name: str) -> FsPath: ...
    def with_suffix(self, suffix: str) -> FsPath: ...

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

    # =========================================================================
    # FEATURE: `same-file`
    # =========================================================================
    def samefile(self, other: PathLike[str] | str | FsPath) -> bool: ...
    def symlink_metadata(self) -> Metadata: ...

    # =========================================================================
    # FEATURE: `which` & `which-regex`
    # =========================================================================
    @staticmethod
    def which(cmd: str, path: str) -> FsPath | None: ...
    @staticmethod
    def which_all(cmd: str, path: str) -> list[FsPath]: ...
    @staticmethod
    def which_re(regex: str | Regex, path: str) -> FsPath | None: ...


class FsPathReaddir:
    def __init__(self) -> t.NoReturn: ...
    def __iter__(self) -> t.Iterator[FsPath]: ...
    def __next__(self) -> FsPath: ...
    def collect(self) -> list[FsPath]: ...
    def take(self, n: int) -> list[FsPath]: ...

```

<h2 id="ry.ryo3._glob"><code>ry.ryo3._glob</code></h2>

```python
"""ryo3-glob types"""

import typing as t
from os import PathLike
from pathlib import Path

import typing_extensions as te

from ry.ryo3._fspath import FsPath

_T = t.TypeVar("_T", bound=str | Path | FsPath)


class _MatchOptions(t.TypedDict, total=False):
    case_sensitive: bool
    require_literal_separator: bool
    require_literal_leading_dot: bool


@t.final
class GlobPaths(t.Generic[_T]):
    """glob::Paths iterable wrapper"""

    def __next__(self) -> _T: ...
    def __iter__(self) -> GlobPaths[_T]: ...
    def collect(self) -> list[_T]: ...
    def take(self, n: int = 1) -> list[_T]: ...


@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = False,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
) -> GlobPaths[Path]: ...
@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = False,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
    dtype: type[_T],
) -> GlobPaths[_T]: ...


@t.final
class Pattern:
    def __init__(self, pattern: str) -> None: ...
    def __call__(
        self,
        ob: str | PathLike[str],
        **kwargs: te.Unpack[_MatchOptions],
    ) -> bool: ...
    def matches(self, s: str) -> bool: ...
    def matches_path(self, path: PathLike[str]) -> bool: ...
    def matches_with(
        self,
        s: str,
        **kwargs: te.Unpack[_MatchOptions],
    ) -> bool: ...
    def matches_path_with(
        self,
        path: PathLike[str],
        **kwargs: te.Unpack[_MatchOptions],
    ) -> bool: ...
    @staticmethod
    def escape(pattern: str) -> str: ...
    @property
    def pattern(self) -> str: ...

```

<h2 id="ry.ryo3._globset"><code>ry.ryo3._globset</code></h2>

```python
"""ryo3-globset types"""

import typing as t
from os import PathLike


@t.final
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
    def is_match_str(self, path: str) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    def __invert__(self) -> Glob: ...
    def globset(self) -> GlobSet: ...
    def globster(self) -> Globster: ...


@t.final
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
    def is_match_str(self, path: str) -> bool: ...
    def matches(self, path: str) -> list[int]: ...
    def __call__(self, path: str) -> bool: ...
    def globster(self) -> Globster: ...
    @property
    def patterns(self) -> tuple[str, ...]: ...


@t.final
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
    def is_match_str(self, path: str) -> bool: ...
    def __call__(self, path: str | PathLike[str]) -> bool: ...
    @property
    def patterns(self) -> tuple[str, ...]: ...


def globster(
    patterns: list[str] | tuple[str, ...],
    /,
    *,
    case_insensitive: bool | None = None,
    literal_separator: bool | None = None,
    backslash_escape: bool | None = None,
) -> Globster: ...

```

<h2 id="ry.ryo3._heck"><code>ry.ryo3._heck</code></h2>

```python
"""ryo3-heck types"""


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

<h2 id="ry.ryo3._http"><code>ry.ryo3._http</code></h2>

```python
import typing as t
from collections.abc import Mapping

import typing_extensions as te

# fmt: off
HTTP_VERSION_LIKE: te.TypeAlias = t.Literal[
    "HTTP/0.9", "0.9", 0,
    "HTTP/1.0", "1.0", 1, 10,
    "HTTP/1.1", "1.1", 11,
    "HTTP/2.0", "2.0", 2, 20,
    "HTTP/3.0", "3.0", 3, 30,
]
# fmt: on

_STANDARD_HEADER: te.TypeAlias = t.Literal[
    "accept",
    "accept-charset",
    "accept-encoding",
    "accept-language",
    "accept-ranges",
    "access-control-allow-credentials",
    "access-control-allow-headers",
    "access-control-allow-methods",
    "access-control-allow-origin",
    "access-control-expose-headers",
    "access-control-max-age",
    "access-control-request-headers",
    "access-control-request-method",
    "age",
    "allow",
    "alt-svc",
    "authorization",
    "cache-control",
    "cache-status",
    "cdn-cache-control",
    "connection",
    "content-disposition",
    "content-encoding",
    "content-language",
    "content-length",
    "content-location",
    "content-range",
    "content-security-policy",
    "content-security-policy-report-only",
    "content-type",
    "cookie",
    "dnt",
    "date",
    "etag",
    "expect",
    "expires",
    "forwarded",
    "from",
    "host",
    "if-match",
    "if-modified-since",
    "if-none-match",
    "if-range",
    "if-unmodified-since",
    "last-modified",
    "link",
    "location",
    "max-forwards",
    "origin",
    "pragma",
    "proxy-authenticate",
    "proxy-authorization",
    "public-key-pins",
    "public-key-pins-report-only",
    "range",
    "referer",
    "referrer-policy",
    "refresh",
    "retry-after",
    "sec-websocket-accept",
    "sec-websocket-extensions",
    "sec-websocket-key",
    "sec-websocket-protocol",
    "sec-websocket-version",
    "server",
    "set-cookie",
    "strict-transport-security",
    "te",
    "trailer",
    "transfer-encoding",
    "user-agent",
    "upgrade",
    "upgrade-insecure-requests",
    "vary",
    "via",
    "warning",
    "www-authenticate",
    "x-content-type-options",
    "x-dns-prefetch-control",
    "x-frame-options",
    "x-xss-protection",
]

_HeaderName: te.TypeAlias = _STANDARD_HEADER | str
_VT = t.TypeVar("_VT", bound=str | t.Sequence[str])


@t.final
class Headers:
    """python-ryo3-http `http::HeadersMap` wrapper"""

    def __init__(
        self,
        headers: Mapping[_HeaderName, _VT] | Headers | None = None,
        /,
        **kwargs: _VT,
    ) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __dbg__(self) -> str: ...

    # =========================================================================
    # MAGIC METHODS
    # =========================================================================
    def __len__(self) -> int: ...
    def __getitem__(self, key: _HeaderName) -> str: ...
    def __setitem__(self, key: _HeaderName, value: str) -> None: ...
    def __delitem__(self, key: _HeaderName) -> None: ...
    def __contains__(self, key: _HeaderName) -> bool: ...
    def __or__(self, other: Headers | dict[str, str]) -> Headers: ...
    def __ror__(self, other: Headers | dict[str, str]) -> Headers: ...
    def __iter__(self) -> t.Iterator[_HeaderName]: ...
    def __bool__(self) -> bool: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def to_py(self) -> dict[str, str | t.Sequence[str]]: ...
    def asdict(self) -> dict[str, str | t.Sequence[str]]: ...
    def stringify(self, *, fmt: bool = False) -> str: ...
    def append(self, key: _HeaderName, value: str) -> None: ...
    def clear(self) -> None: ...
    def contains_key(self, key: _HeaderName) -> bool: ...
    def get(self, key: _HeaderName) -> str | None: ...
    def get_all(self, key: _HeaderName) -> list[str]: ...
    def insert(self, key: _HeaderName, value: str) -> None: ...
    def is_empty(self) -> bool: ...
    def keys(self) -> list[str]: ...
    def keys_len(self) -> int: ...
    def len(self) -> int: ...
    def pop(self, key: _HeaderName) -> str: ...
    def remove(self, key: _HeaderName) -> None: ...
    def update(self, headers: Headers | dict[str, str]) -> None: ...
    def values(self) -> list[str]: ...
    @property
    def is_flat(self) -> bool: ...


@t.final
class HttpStatus:
    def __init__(self, code: int) -> None: ...
    def __int__(self) -> int: ...
    def __bool__(self) -> bool: ...
    def __hash__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: HttpStatus | int) -> bool: ...
    def __le__(self, other: HttpStatus | int) -> bool: ...
    def __gt__(self, other: HttpStatus | int) -> bool: ...
    def __ge__(self, other: HttpStatus | int) -> bool: ...
    def to_py(self) -> int: ...
    @property
    def reason(self) -> str: ...
    @property
    def canonical_reason(self) -> str: ...
    @property
    def is_informational(self) -> bool: ...
    @property
    def is_success(self) -> bool: ...
    @property
    def is_redirect(self) -> bool: ...
    @property
    def is_redirection(self) -> bool: ...
    @property
    def is_client_error(self) -> bool: ...
    @property
    def is_server_error(self) -> bool: ...
    @property
    def is_error(self) -> bool: ...
    @property
    def is_ok(self) -> bool: ...
    @property
    def ok(self) -> bool: ...

    # =========================================================================
    # CONST STATUS CODES
    # =========================================================================
    CONTINUE: HttpStatus  # 100 ~ Continue
    SWITCHING_PROTOCOLS: HttpStatus  # 101 ~ Switching Protocols
    PROCESSING: HttpStatus  # 102 ~ Processing
    OK: HttpStatus  # 200 ~ OK
    CREATED: HttpStatus  # 201 ~ Created
    ACCEPTED: HttpStatus  # 202 ~ Accepted
    NON_AUTHORITATIVE_INFORMATION: (
        HttpStatus  # 203 ~ Non Authoritative Information
    )
    NO_CONTENT: HttpStatus  # 204 ~ No Content
    RESET_CONTENT: HttpStatus  # 205 ~ Reset Content
    PARTIAL_CONTENT: HttpStatus  # 206 ~ Partial Content
    MULTI_STATUS: HttpStatus  # 207 ~ Multi-Status
    ALREADY_REPORTED: HttpStatus  # 208 ~ Already Reported
    IM_USED: HttpStatus  # 226 ~ IM Used
    MULTIPLE_CHOICES: HttpStatus  # 300 ~ Multiple Choices
    MOVED_PERMANENTLY: HttpStatus  # 301 ~ Moved Permanently
    FOUND: HttpStatus  # 302 ~ Found
    SEE_OTHER: HttpStatus  # 303 ~ See Other
    NOT_MODIFIED: HttpStatus  # 304 ~ Not Modified
    USE_PROXY: HttpStatus  # 305 ~ Use Proxy
    TEMPORARY_REDIRECT: HttpStatus  # 307 ~ Temporary Redirect
    PERMANENT_REDIRECT: HttpStatus  # 308 ~ Permanent Redirect
    BAD_REQUEST: HttpStatus  # 400 ~ Bad Request
    UNAUTHORIZED: HttpStatus  # 401 ~ Unauthorized
    PAYMENT_REQUIRED: HttpStatus  # 402 ~ Payment Required
    FORBIDDEN: HttpStatus  # 403 ~ Forbidden
    NOT_FOUND: HttpStatus  # 404 ~ Not Found
    METHOD_NOT_ALLOWED: HttpStatus  # 405 ~ Method Not Allowed
    NOT_ACCEPTABLE: HttpStatus  # 406 ~ Not Acceptable
    PROXY_AUTHENTICATION_REQUIRED: (
        HttpStatus  # 407 ~ Proxy Authentication Required
    )
    REQUEST_TIMEOUT: HttpStatus  # 408 ~ Request Timeout
    CONFLICT: HttpStatus  # 409 ~ Conflict
    GONE: HttpStatus  # 410 ~ Gone
    LENGTH_REQUIRED: HttpStatus  # 411 ~ Length Required
    PRECONDITION_FAILED: HttpStatus  # 412 ~ Precondition Failed
    PAYLOAD_TOO_LARGE: HttpStatus  # 413 ~ Payload Too Large
    URI_TOO_LONG: HttpStatus  # 414 ~ URI Too Long
    UNSUPPORTED_MEDIA_TYPE: HttpStatus  # 415 ~ Unsupported Media Type
    RANGE_NOT_SATISFIABLE: HttpStatus  # 416 ~ Range Not Satisfiable
    EXPECTATION_FAILED: HttpStatus  # 417 ~ Expectation Failed
    IM_A_TEAPOT: HttpStatus  # 418 ~ I'm a teapot
    MISDIRECTED_REQUEST: HttpStatus  # 421 ~ Misdirected Request
    UNPROCESSABLE_ENTITY: HttpStatus  # 422 ~ Unprocessable Entity
    LOCKED: HttpStatus  # 423 ~ Locked
    FAILED_DEPENDENCY: HttpStatus  # 424 ~ Failed Dependency
    TOO_EARLY: HttpStatus  # 425 ~ Too Early
    UPGRADE_REQUIRED: HttpStatus  # 426 ~ Upgrade Required
    PRECONDITION_REQUIRED: HttpStatus  # 428 ~ Precondition Required
    TOO_MANY_REQUESTS: HttpStatus  # 429 ~ Too Many Requests
    REQUEST_HEADER_FIELDS_TOO_LARGE: (
        HttpStatus  # 431 ~ Request Header Fields Too Large
    )
    UNAVAILABLE_FOR_LEGAL_REASONS: (
        HttpStatus  # 451 ~ Unavailable For Legal Reasons
    )
    INTERNAL_SERVER_ERROR: HttpStatus  # 500 ~ Internal Server Error
    NOT_IMPLEMENTED: HttpStatus  # 501 ~ Not Implemented
    BAD_GATEWAY: HttpStatus  # 502 ~ Bad Gateway
    SERVICE_UNAVAILABLE: HttpStatus  # 503 ~ Service Unavailable
    GATEWAY_TIMEOUT: HttpStatus  # 504 ~ Gateway Timeout
    HTTP_VERSION_NOT_SUPPORTED: HttpStatus  # 505 ~ HTTP Version Not Supported
    VARIANT_ALSO_NEGOTIATES: HttpStatus  # 506 ~ Variant Also Negotiates
    INSUFFICIENT_STORAGE: HttpStatus  # 507 ~ Insufficient Storage
    LOOP_DETECTED: HttpStatus  # 508 ~ Loop Detected
    NOT_EXTENDED: HttpStatus  # 510 ~ Not Extended
    NETWORK_AUTHENTICATION_REQUIRED: (
        HttpStatus  # 511 ~ Network Authentication Required
    )

```

<h2 id="ry.ryo3._jiff"><code>ry.ryo3._jiff</code></h2>

```python
"""jiff types"""

import datetime as pydt
import typing as t

import typing_extensions as te

from ry._types import (
    DateTimeTypedDict,
    DateTypedDict,
    FromStr,
    TimeSpanTypedDict,
    TimeTypedDict,
    ToPy,
    ToPyDate,
    ToPyDateTime,
    ToPyTime,
    ToPyTimeDelta,
    ToPyTzInfo,
)
from ry.ryo3 import Duration
from ry.ryo3._jiff_tz import TZDB_NAMES

_T = t.TypeVar("_T")

TZ_NAME: te.TypeAlias = TZDB_NAMES | str
JIFF_UNIT: te.TypeAlias = t.Literal[
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

JIFF_ROUND_MODE: te.TypeAlias = t.Literal[
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

WEEKDAY_STR: te.TypeAlias = t.Literal[
    "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"
]

WEEKDAY_INT: te.TypeAlias = t.Literal[
    1,  # Monday
    2,  # Tuesday
    3,  # Wednesday
    4,  # Thursday
    5,  # Friday
    6,  # Saturday
    7,  # Sunday
]

WEEKDAY: te.TypeAlias = WEEKDAY_STR | WEEKDAY_INT


@t.final
class Date(ToPy[pydt.date], ToPyDate):
    MIN: Date
    MAX: Date
    ZERO: Date

    def __init__(self, year: int, month: int, day: int) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def string(self) -> str: ...
    def isoformat(self) -> str: ...

    # =========================================================================
    # PYTHON_CONVERSIONS
    # =========================================================================
    def to_py(self) -> pydt.date: ...
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
    @classmethod
    def from_str(cls: type[Date], s: str) -> Date: ...
    @classmethod
    def parse(cls: type[Date], s: str) -> Date: ...

    # =========================================================================
    # STRPTIME/STRFTIME
    # =========================================================================
    @classmethod
    def strptime(cls: type[Date], format: str, string: str) -> Date: ...
    def strftime(self, format: str) -> str: ...

    # =========================================================================
    # OPERATORS
    # =========================================================================
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...
    @t.overload
    def __sub__(self, other: Date) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...
    @t.overload
    def __isub__(self, other: Date) -> TimeSpan: ...
    @t.overload
    def __isub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...

    # =========================================================================
    # ARITHMETIC METHODS
    # =========================================================================
    def add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...
    @t.overload
    def sub(self, other: Date) -> TimeSpan: ...
    @t.overload
    def sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Date: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def at(
        self, hour: int, minute: int, second: int, nanosecond: int
    ) -> DateTime: ...
    def asdict(self) -> DateTypedDict: ...
    def astuple(self) -> tuple[int, int, int]: ...
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
    def in_tz(self, tz: TZ_NAME) -> ZonedDateTime: ...
    @te.deprecated("intz is deprecated, use in_tz instead")
    def intz(self, tz: TZ_NAME) -> ZonedDateTime: ...
    def last_of_month(self) -> Date: ...
    def last_of_year(self) -> Date: ...
    def nth_weekday(self, nth: int, weekday: WEEKDAY) -> Date: ...
    def nth_weekday_of_month(self, nth: int, weekday: WEEKDAY) -> Date: ...
    def replace(
        self,
        year: int | None = None,
        month: int | None = None,
        day: int | None = None,
        era_year: tuple[int, t.Literal["BCE", "CE"]] | None = None,
        day_of_year: int | None = None,
        day_of_year_no_leap: int | None = None,
    ) -> Date: ...
    def series(self, span: TimeSpan) -> JiffSeries[Date]: ...
    def to_datetime(self, t: Time) -> DateTime: ...
    def to_zoned(self, tz: TimeZone) -> ZonedDateTime: ...
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


@t.final
class Time(ToPy[pydt.time], ToPyTime, FromStr):
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
    def isoformat(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    @t.overload
    def __sub__(self, other: Time) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    @t.overload
    def __isub__(self, other: Time) -> TimeSpan: ...
    @t.overload
    def __isub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...

    # =========================================================================
    # STRPTIME/STRFTIME/PARSE
    # =========================================================================
    @classmethod
    def strptime(cls: type[Time], format: str, string: str) -> Time: ...
    def strftime(self, format: str) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    def to_py(self) -> pydt.time: ...
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
    def from_str(cls: type[Time], s: str) -> Time: ...
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
    # ARITHMETIC METHODS
    # =========================================================================
    def add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    @t.overload
    def sub(self, other: Time) -> TimeSpan: ...
    @t.overload
    def sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    def wrapping_add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...
    def wrapping_sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Time: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def astuple(self) -> tuple[int, int, int, int]: ...
    def asdict(self) -> TimeTypedDict: ...
    def duration_until(self, other: Time) -> SignedDuration: ...
    def duration_since(self, other: Time) -> SignedDuration: ...
    def on(self, year: int, month: int, day: int) -> DateTime: ...
    def replace(
        self,
        hour: int | None = None,
        minute: int | None = None,
        second: int | None = None,
        millisecond: int | None = None,
        microsecond: int | None = None,
        nanosecond: int | None = None,
        subsec_nanosecond: int | None = None,
    ) -> Time: ...
    def round(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> Time: ...
    def series(self, span: TimeSpan) -> JiffSeries[Time]: ...
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


@t.final
class DateTime(ToPy[pydt.datetime], ToPyDate, ToPyTime, ToPyDateTime, FromStr):
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
    def string(self) -> str: ...
    def isoformat(self) -> str: ...

    # =========================================================================
    # STRPTIME/STRFTIME/PARSE
    # =========================================================================
    def strftime(self, format: str) -> str: ...
    @classmethod
    def strptime(cls: type[DateTime], format: str, string: str) -> DateTime: ...
    @classmethod
    def from_str(cls: type[DateTime], s: str) -> DateTime: ...
    @classmethod
    def parse(cls: type[DateTime], s: str) -> DateTime: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pydatetime(cls: type[DateTime], dt: pydt.datetime) -> DateTime: ...
    def to_py(self) -> pydt.datetime: ...
    def to_pydate(self) -> pydt.date: ...
    def to_pydatetime(self) -> pydt.datetime: ...
    def to_pytime(self) -> pydt.time: ...

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
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...
    @t.overload
    def __sub__(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...
    @t.overload
    def __isub__(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def __isub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...

    # =========================================================================
    # ARITHMETIC METHODS
    # =========================================================================
    def add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...
    @t.overload
    def sub(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...
    @t.overload
    def saturating_sub(self, other: DateTime) -> TimeSpan: ...
    @t.overload
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def asdict(self) -> DateTimeTypedDict: ...
    def date(self) -> Date: ...
    def day_of_year(self) -> int: ...
    def day_of_year_no_leap(self) -> int | None: ...
    def days_in_month(self) -> int: ...
    def days_in_year(self) -> int: ...
    def duration_since(self, other: DateTime) -> SignedDuration: ...
    def duration_until(self, other: DateTime) -> SignedDuration: ...
    def end_of_day(self) -> DateTime: ...
    def era_year(self) -> tuple[int, t.Literal["BCE", "CE"]]: ...
    def first_of_month(self) -> DateTime: ...
    def first_of_year(self) -> DateTime: ...
    def in_leap_year(self) -> bool: ...
    def in_tz(self, tz: str) -> ZonedDateTime: ...
    @te.deprecated("intz is deprecated, use in_tz instead")
    def intz(self, tz: str) -> ZonedDateTime: ...
    def iso_week_date(self) -> ISOWeekDate: ...
    def last_of_month(self) -> DateTime: ...
    def last_of_year(self) -> DateTime: ...
    def nth_weekday(self, nth: int, weekday: WEEKDAY) -> DateTime: ...
    def nth_weekday_of_month(self, nth: int, weekday: WEEKDAY) -> DateTime: ...
    def replace(
        self,
        obj: Date | DateTime | Time | None = None,
        *,
        date: Date | None = None,
        time: Time | None = None,
        year: int | None = None,
        era_year: tuple[int, t.Literal["BCE", "CE"]] | None = None,
        month: int | None = None,
        day: int | None = None,
        day_of_year: int | None = None,
        day_of_year_no_leap: int | None = None,
        hour: int | None = None,
        minute: int | None = None,
        second: int | None = None,
        millisecond: int | None = None,
        microsecond: int | None = None,
        nanosecond: int | None = None,
        subsec_nanosecond: int | None = None,
    ) -> DateTime: ...
    def round(
        self,
        smallest: JIFF_UNIT | None = None,
        *,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> DateTime: ...
    def _round(self, options: DateTimeRound) -> DateTime: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> DateTime: ...
    def series(self, span: TimeSpan) -> JiffSeries[DateTime]: ...
    def start_of_day(self) -> DateTime: ...
    def time(self) -> Time: ...
    def to_zoned(self, tz: TimeZone) -> ZonedDateTime: ...
    def tomorrow(self) -> DateTime: ...
    def yesterday(self) -> DateTime: ...

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


@t.final
class TimeZone(ToPy[pydt.tzinfo], ToPyTzInfo, FromStr):
    def __init__(self, name: TZ_NAME) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __call__(self) -> te.Self: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================

    def to_py(self) -> pydt.tzinfo: ...
    def to_pytzinfo(self) -> pydt.tzinfo: ...
    @classmethod
    def from_str(cls, s: TZ_NAME) -> TimeZone: ...
    @classmethod
    def from_pytzinfo(cls: type[TimeZone], tz: pydt.tzinfo) -> TimeZone: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def name(self) -> str: ...
    @property
    def is_unknown(self) -> bool: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def fixed(cls: type[TimeZone], offset: Offset) -> TimeZone: ...
    @classmethod
    def get(cls: type[TimeZone], name: TZ_NAME) -> TimeZone: ...
    @classmethod
    def posix(cls: type[TimeZone], name: TZ_NAME) -> TimeZone: ...
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


@t.final
class SignedDuration(ToPy[pydt.timedelta], ToPyTimeDelta, FromStr):
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
    def __float__(self) -> float: ...
    def __int__(self) -> int: ...
    def __bool__(self) -> bool: ...
    def __div__(self, other: int) -> SignedDuration: ...
    def abs(self) -> SignedDuration: ...
    def unsigned_abs(self) -> Duration: ...
    def __richcmp__(
        self, other: SignedDuration | pydt.timedelta, op: int
    ) -> bool: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def string(self, human: bool = False) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(
        cls: type[SignedDuration], td: pydt.timedelta
    ) -> SignedDuration: ...
    def to_py(self) -> pydt.timedelta: ...
    def to_pytimedelta(self) -> pydt.timedelta: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def from_str(cls: type[SignedDuration], s: str) -> SignedDuration: ...
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
_TimeSpanArithmeticSingle: te.TypeAlias = TimeSpan | Duration | SignedDuration
_TimeSpanArithmeticTuple: te.TypeAlias = tuple[
    _TimeSpanArithmeticSingle, ZonedDateTime | Date | DateTime
]
TimeSpanArithmetic: te.TypeAlias = (
    _TimeSpanArithmeticSingle | _TimeSpanArithmeticTuple
)


@t.final
class TimeSpan(ToPy[pydt.timedelta], ToPyTimeDelta, FromStr):
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
    def repr_full(self) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(cls, td: pydt.timedelta) -> TimeSpan: ...
    def to_pytimedelta(self) -> pydt.timedelta: ...
    def to_py(self) -> pydt.timedelta: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def from_str(cls, s: str) -> TimeSpan: ...
    @classmethod
    def parse(cls, s: str) -> TimeSpan: ...
    @classmethod
    def parse_common_iso(cls, s: str) -> TimeSpan: ...

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
    # ARITHMETIC METHODS
    # =========================================================================
    def add(self, val: TimeSpanArithmetic) -> te.Self: ...
    def mul(self, other: int) -> te.Self: ...
    def sub(self, val: TimeSpanArithmetic) -> te.Self: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================

    def abs(self) -> te.Self: ...
    def asdict(self) -> TimeSpanTypedDict: ...
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


@t.final
class Timestamp(ToPy[pydt.datetime], ToPyDate, ToPyTime, ToPyDateTime, FromStr):
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
    def from_str(cls, s: str) -> Timestamp: ...
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
    # OPERATORS/DUNDERS
    # =========================================================================
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: Timestamp) -> bool: ...
    def __gt__(self, other: Timestamp) -> bool: ...
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
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    @t.overload
    def __sub__(self, other: Timestamp) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pydatetime(cls, dt: pydt.datetime) -> Timestamp: ...
    def to_py(self) -> pydt.datetime: ...
    def to_pydate(self) -> pydt.date: ...
    def to_pydatetime(self) -> pydt.datetime: ...
    def to_pytime(self) -> pydt.time: ...

    # =========================================================================
    # ARITHMETIC METHODS
    # =========================================================================
    def add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Timestamp: ...
    @t.overload
    def sub(self, other: Timestamp) -> TimeSpan: ...
    @t.overload
    def sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Timestamp: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Timestamp: ...
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> Timestamp: ...

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
    def display_with_offset(self, offset: Offset) -> str: ...
    def in_tz(self, tz: TZ_NAME) -> ZonedDateTime: ...
    @te.deprecated("intz is deprecated, use in_tz instead")
    def intz(self, tz: TZ_NAME) -> ZonedDateTime:
        """Deprecated ~ use `in_tz`"""

    def is_zero(self) -> bool: ...
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


@t.final
class ZonedDateTime(
    ToPy[pydt.datetime], ToPyDate, ToPyTime, ToPyDateTime, ToPyTzInfo, FromStr
):
    def __init__(
        self,
        year: int,
        month: int,
        day: int,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        nanosecond: int = 0,
        tz: str | None = None,
    ) -> None: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pydatetime(
        cls: type[ZonedDateTime], dt: pydt.datetime
    ) -> ZonedDateTime: ...
    def to_py(self) -> pydt.datetime: ...
    def to_pydate(self) -> pydt.date: ...
    def to_pydatetime(self) -> pydt.datetime: ...
    def to_pytime(self) -> pydt.time: ...
    def to_pytzinfo(self) -> pydt.tzinfo: ...

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
    def from_str(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...
    @classmethod
    def parse(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...
    @classmethod
    def from_rfc2822(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...
    @classmethod
    def parse_rfc2822(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...
    @classmethod
    def from_parts(
        cls: type[ZonedDateTime], timestamp: Timestamp, time_zone: TimeZone
    ) -> ZonedDateTime: ...

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
    @property
    def timezone(self) -> TimeZone: ...
    @property
    def tz(self) -> TimeZone: ...

    # =========================================================================
    # STRING/FORMAT
    # =========================================================================
    def string(self) -> str: ...
    def to_rfc2822(self) -> str: ...
    def format_rfc2822(self) -> str: ...
    def isoformat(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __add__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: ZonedDateTime) -> bool: ...
    def __gt__(self, other: ZonedDateTime) -> bool: ...
    def __hash__(self) -> int: ...
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
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    @t.overload
    def __sub__(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def __sub__(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...

    # =========================================================================
    # ARITHMETIC METHODS
    # =========================================================================
    def add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    @t.overload
    def sub(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    def saturating_add(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...
    @t.overload
    def saturating_sub(self, other: ZonedDateTime) -> TimeSpan: ...
    @t.overload
    def saturating_sub(
        self, other: TimeSpan | SignedDuration | Duration | pydt.timedelta
    ) -> te.Self: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def astimezone(self, tz: str) -> ZonedDateTime: ...
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
    def in_tz(self, tz: TZ_NAME) -> te.Self: ...
    @te.deprecated("intz is deprecated, use in_tz instead")
    def intz(self, tz: TZ_NAME) -> te.Self: ...
    def inutc(self) -> ZonedDateTime: ...
    def last_of_month(self) -> ZonedDateTime: ...
    def last_of_year(self) -> ZonedDateTime: ...
    def nth_weekday(self, nth: int, weekday: WEEKDAY) -> Date: ...
    def nth_weekday_of_month(self, nth: int, weekday: WEEKDAY) -> Date: ...
    def offset(self) -> Offset: ...
    def replace(
        self,
        obj: Date | DateTime | Time | Offset | None = None,
        *,
        date: Date | None = None,
        time: Time | None = None,
        year: int | None = None,
        era_year: tuple[int, t.Literal["BCE", "CE"]] | None = None,
        month: int | None = None,
        day: int | None = None,
        day_of_year: int | None = None,
        day_of_year_no_leap: int | None = None,
        hour: int | None = None,
        minute: int | None = None,
        second: int | None = None,
        millisecond: int | None = None,
        microsecond: int | None = None,
        nanosecond: int | None = None,
        subsec_nanosecond: int | None = None,
        offset: Offset | None = None,
        offset_conflict: t.Any = None,
        disambiguation: t.Any = None,
    ) -> ZonedDateTime: ...
    def round(
        self,
        smallest: JIFF_UNIT | None = None,
        *,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> DateTime: ...
    def _round(self, options: ZonedDateTimeRound) -> DateTime: ...
    def start_of_day(self) -> ZonedDateTime: ...
    def time(self) -> Time: ...
    def timestamp(self) -> Timestamp: ...
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


@t.final
class ISOWeekDate:
    MIN: ISOWeekDate
    MAX: ISOWeekDate
    ZERO: ISOWeekDate

    def __init__(self, year: int, week: int, weekday: WEEKDAY) -> None: ...

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
    def weekday(self) -> WEEKDAY_INT: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def date(self) -> Date: ...


@t.final
class Offset(ToPy[pydt.tzinfo], ToPyTzInfo):
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
    # PYTHON CONVERSIONS
    # =========================================================================
    # __FROM__
    @classmethod
    def from_pytzinfo(cls: type[Offset], tz: pydt.tzinfo) -> Offset: ...

    # __TO__
    def to_py(self) -> pydt.tzinfo: ...
    def to_pytzinfo(self) -> pydt.tzinfo: ...

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
    # ARITHMETIC METHODS
    # =========================================================================
    def add(
        self, other: Duration | SignedDuration | TimeSpan | pydt.timedelta
    ) -> Offset: ...
    def sub(
        self, other: Duration | SignedDuration | TimeSpan | pydt.timedelta
    ) -> Offset: ...
    def saturating_add(
        self, other: Duration | SignedDuration | TimeSpan | pydt.timedelta
    ) -> Offset: ...
    def saturating_sub(
        self, other: Duration | SignedDuration | TimeSpan | pydt.timedelta
    ) -> Offset: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def duration_since(self, other: Offset) -> SignedDuration: ...
    def duration_until(self, other: Offset) -> SignedDuration: ...
    def negate(self) -> Offset: ...
    def since(self, other: Offset) -> TimeSpan: ...
    def until(self, other: Offset) -> TimeSpan: ...


# =============================================================================
# DIFFERENCE
# =============================================================================
class _Difference(t.Generic[_T]):
    def __init__(
        self,
        date: _T,
        *,
        smallest: JIFF_UNIT | None = None,
        largest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int | None = None,
    ) -> None: ...
    def smallest(self, unit: JIFF_UNIT) -> te.Self: ...
    def largest(self, unit: JIFF_UNIT) -> te.Self: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> te.Self: ...
    def increment(self, increment: int) -> te.Self: ...


@t.final
class DateDifference(_Difference[Date]): ...


@t.final
class DateTimeDifference(_Difference[DateTime]): ...


@t.final
class TimeDifference(_Difference[Time]): ...


@t.final
class TimestampDifference(_Difference[Timestamp]): ...


@t.final
class ZonedDateTimeDifference(_Difference[ZonedDateTime]): ...


# =============================================================================
# ROUND
# =============================================================================
@t.final
class TimestampRound:
    def __init__(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int = 1,
    ) -> None: ...
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


@t.final
class DateTimeRound:
    def __init__(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int = 1,
    ) -> None: ...
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


@t.final
class ZonedDateTimeRound:
    def __init__(
        self,
        smallest: JIFF_UNIT | None = None,
        mode: JIFF_ROUND_MODE | None = None,
        increment: int = 1,
    ) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def mode(self, mode: JIFF_ROUND_MODE) -> ZonedDateTimeRound: ...
    def smallest(self, smallest: JIFF_UNIT) -> ZonedDateTimeRound: ...
    def increment(self, increment: int) -> ZonedDateTimeRound: ...
    def _smallest(self) -> JIFF_UNIT: ...
    def _mode(self) -> JIFF_ROUND_MODE: ...
    def _increment(self) -> int: ...
    def replace(
        self,
        smallest: JIFF_UNIT | None,
        mode: JIFF_ROUND_MODE | None,
        increment: int | None,
    ) -> ZonedDateTimeRound: ...


@t.type_check_only
class JiffSeries(
    t.Generic[_T],
):
    def __iter__(self) -> t.Iterator[_T]: ...
    def __next__(self) -> _T: ...
    def take(self, n: int) -> list[_T]: ...


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
def zoned(
    year: int,
    month: int,
    day: int,
    hour: int = 0,
    minute: int = 0,
    second: int = 0,
    nanosecond: int = 0,
    tz: str | None = None,
) -> ZonedDateTime: ...
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
) -> TimeSpan: ...
def offset(hours: int) -> Offset: ...
def now() -> ZonedDateTime: ...
def utcnow() -> ZonedDateTime: ...


# =============================================================================
# TIMEZONE-DATABASE
# =============================================================================
@t.final
class TimeZoneDatabase:
    def __init__(self) -> None:
        """Defaults to using the `self.from_env`"""

    @t.overload
    def get(self, name: TZ_NAME, err: t.Literal[False]) -> TimeZone | None:
        """Returns TimeZone or None if the timezone is not found"""

    @t.overload
    def get(self, name: TZ_NAME, err: t.Literal[True] = True) -> TimeZone:
        """Returns TimeZone, if not found raises a ValueError"""

    def available(self) -> list[str]: ...
    def __getitem__(self, name: TZ_NAME) -> TimeZone: ...
    def __len__(self) -> int: ...
    def is_definitively_empty(self) -> bool: ...
    @classmethod
    def from_env(cls) -> TimeZoneDatabase: ...
    @classmethod
    def from_dir(cls, path: str) -> TimeZoneDatabase: ...
    @classmethod
    def from_concatenated_path(cls, path: str) -> TimeZoneDatabase: ...
    @classmethod
    def bundled(cls) -> TimeZoneDatabase: ...

```

<h2 id="ry.ryo3._jiff_tz"><code>ry.ryo3._jiff_tz</code></h2>

```python
from typing import Literal

from typing_extensions import TypeAlias

TZDB_NAMES: TypeAlias = Literal[
    "Africa/Abidjan",
    "Africa/Accra",
    "Africa/Addis_Ababa",
    "Africa/Algiers",
    "Africa/Asmara",
    "Africa/Asmera",
    "Africa/Bamako",
    "Africa/Bangui",
    "Africa/Banjul",
    "Africa/Bissau",
    "Africa/Blantyre",
    "Africa/Brazzaville",
    "Africa/Bujumbura",
    "Africa/Cairo",
    "Africa/Casablanca",
    "Africa/Ceuta",
    "Africa/Conakry",
    "Africa/Dakar",
    "Africa/Dar_es_Salaam",
    "Africa/Djibouti",
    "Africa/Douala",
    "Africa/El_Aaiun",
    "Africa/Freetown",
    "Africa/Gaborone",
    "Africa/Harare",
    "Africa/Johannesburg",
    "Africa/Juba",
    "Africa/Kampala",
    "Africa/Khartoum",
    "Africa/Kigali",
    "Africa/Kinshasa",
    "Africa/Lagos",
    "Africa/Libreville",
    "Africa/Lome",
    "Africa/Luanda",
    "Africa/Lubumbashi",
    "Africa/Lusaka",
    "Africa/Malabo",
    "Africa/Maputo",
    "Africa/Maseru",
    "Africa/Mbabane",
    "Africa/Mogadishu",
    "Africa/Monrovia",
    "Africa/Nairobi",
    "Africa/Ndjamena",
    "Africa/Niamey",
    "Africa/Nouakchott",
    "Africa/Ouagadougou",
    "Africa/Porto-Novo",
    "Africa/Sao_Tome",
    "Africa/Timbuktu",
    "Africa/Tripoli",
    "Africa/Tunis",
    "Africa/Windhoek",
    "America/Adak",
    "America/Anchorage",
    "America/Anguilla",
    "America/Antigua",
    "America/Araguaina",
    "America/Argentina/Buenos_Aires",
    "America/Argentina/Catamarca",
    "America/Argentina/ComodRivadavia",
    "America/Argentina/Cordoba",
    "America/Argentina/Jujuy",
    "America/Argentina/La_Rioja",
    "America/Argentina/Mendoza",
    "America/Argentina/Rio_Gallegos",
    "America/Argentina/Salta",
    "America/Argentina/San_Juan",
    "America/Argentina/San_Luis",
    "America/Argentina/Tucuman",
    "America/Argentina/Ushuaia",
    "America/Aruba",
    "America/Asuncion",
    "America/Atikokan",
    "America/Atka",
    "America/Bahia",
    "America/Bahia_Banderas",
    "America/Barbados",
    "America/Belem",
    "America/Belize",
    "America/Blanc-Sablon",
    "America/Boa_Vista",
    "America/Bogota",
    "America/Boise",
    "America/Buenos_Aires",
    "America/Cambridge_Bay",
    "America/Campo_Grande",
    "America/Cancun",
    "America/Caracas",
    "America/Catamarca",
    "America/Cayenne",
    "America/Cayman",
    "America/Chicago",
    "America/Chihuahua",
    "America/Ciudad_Juarez",
    "America/Coral_Harbour",
    "America/Cordoba",
    "America/Costa_Rica",
    "America/Coyhaique",
    "America/Creston",
    "America/Cuiaba",
    "America/Curacao",
    "America/Danmarkshavn",
    "America/Dawson",
    "America/Dawson_Creek",
    "America/Denver",
    "America/Detroit",
    "America/Dominica",
    "America/Edmonton",
    "America/Eirunepe",
    "America/El_Salvador",
    "America/Ensenada",
    "America/Fort_Nelson",
    "America/Fort_Wayne",
    "America/Fortaleza",
    "America/Glace_Bay",
    "America/Godthab",
    "America/Goose_Bay",
    "America/Grand_Turk",
    "America/Grenada",
    "America/Guadeloupe",
    "America/Guatemala",
    "America/Guayaquil",
    "America/Guyana",
    "America/Halifax",
    "America/Havana",
    "America/Hermosillo",
    "America/Indiana/Indianapolis",
    "America/Indiana/Knox",
    "America/Indiana/Marengo",
    "America/Indiana/Petersburg",
    "America/Indiana/Tell_City",
    "America/Indiana/Vevay",
    "America/Indiana/Vincennes",
    "America/Indiana/Winamac",
    "America/Indianapolis",
    "America/Inuvik",
    "America/Iqaluit",
    "America/Jamaica",
    "America/Jujuy",
    "America/Juneau",
    "America/Kentucky/Louisville",
    "America/Kentucky/Monticello",
    "America/Knox_IN",
    "America/Kralendijk",
    "America/La_Paz",
    "America/Lima",
    "America/Los_Angeles",
    "America/Louisville",
    "America/Lower_Princes",
    "America/Maceio",
    "America/Managua",
    "America/Manaus",
    "America/Marigot",
    "America/Martinique",
    "America/Matamoros",
    "America/Mazatlan",
    "America/Mendoza",
    "America/Menominee",
    "America/Merida",
    "America/Metlakatla",
    "America/Mexico_City",
    "America/Miquelon",
    "America/Moncton",
    "America/Monterrey",
    "America/Montevideo",
    "America/Montreal",
    "America/Montserrat",
    "America/Nassau",
    "America/New_York",
    "America/Nipigon",
    "America/Nome",
    "America/Noronha",
    "America/North_Dakota/Beulah",
    "America/North_Dakota/Center",
    "America/North_Dakota/New_Salem",
    "America/Nuuk",
    "America/Ojinaga",
    "America/Panama",
    "America/Pangnirtung",
    "America/Paramaribo",
    "America/Phoenix",
    "America/Port-au-Prince",
    "America/Port_of_Spain",
    "America/Porto_Acre",
    "America/Porto_Velho",
    "America/Puerto_Rico",
    "America/Punta_Arenas",
    "America/Rainy_River",
    "America/Rankin_Inlet",
    "America/Recife",
    "America/Regina",
    "America/Resolute",
    "America/Rio_Branco",
    "America/Rosario",
    "America/Santa_Isabel",
    "America/Santarem",
    "America/Santiago",
    "America/Santo_Domingo",
    "America/Sao_Paulo",
    "America/Scoresbysund",
    "America/Shiprock",
    "America/Sitka",
    "America/St_Barthelemy",
    "America/St_Johns",
    "America/St_Kitts",
    "America/St_Lucia",
    "America/St_Thomas",
    "America/St_Vincent",
    "America/Swift_Current",
    "America/Tegucigalpa",
    "America/Thule",
    "America/Thunder_Bay",
    "America/Tijuana",
    "America/Toronto",
    "America/Tortola",
    "America/Vancouver",
    "America/Virgin",
    "America/Whitehorse",
    "America/Winnipeg",
    "America/Yakutat",
    "America/Yellowknife",
    "Antarctica/Casey",
    "Antarctica/Davis",
    "Antarctica/DumontDUrville",
    "Antarctica/Macquarie",
    "Antarctica/Mawson",
    "Antarctica/McMurdo",
    "Antarctica/Palmer",
    "Antarctica/Rothera",
    "Antarctica/South_Pole",
    "Antarctica/Syowa",
    "Antarctica/Troll",
    "Antarctica/Vostok",
    "Arctic/Longyearbyen",
    "Asia/Aden",
    "Asia/Almaty",
    "Asia/Amman",
    "Asia/Anadyr",
    "Asia/Aqtau",
    "Asia/Aqtobe",
    "Asia/Ashgabat",
    "Asia/Ashkhabad",
    "Asia/Atyrau",
    "Asia/Baghdad",
    "Asia/Bahrain",
    "Asia/Baku",
    "Asia/Bangkok",
    "Asia/Barnaul",
    "Asia/Beirut",
    "Asia/Bishkek",
    "Asia/Brunei",
    "Asia/Calcutta",
    "Asia/Chita",
    "Asia/Choibalsan",
    "Asia/Chongqing",
    "Asia/Chungking",
    "Asia/Colombo",
    "Asia/Dacca",
    "Asia/Damascus",
    "Asia/Dhaka",
    "Asia/Dili",
    "Asia/Dubai",
    "Asia/Dushanbe",
    "Asia/Famagusta",
    "Asia/Gaza",
    "Asia/Harbin",
    "Asia/Hebron",
    "Asia/Ho_Chi_Minh",
    "Asia/Hong_Kong",
    "Asia/Hovd",
    "Asia/Irkutsk",
    "Asia/Istanbul",
    "Asia/Jakarta",
    "Asia/Jayapura",
    "Asia/Jerusalem",
    "Asia/Kabul",
    "Asia/Kamchatka",
    "Asia/Karachi",
    "Asia/Kashgar",
    "Asia/Kathmandu",
    "Asia/Katmandu",
    "Asia/Khandyga",
    "Asia/Kolkata",
    "Asia/Krasnoyarsk",
    "Asia/Kuala_Lumpur",
    "Asia/Kuching",
    "Asia/Kuwait",
    "Asia/Macao",
    "Asia/Macau",
    "Asia/Magadan",
    "Asia/Makassar",
    "Asia/Manila",
    "Asia/Muscat",
    "Asia/Nicosia",
    "Asia/Novokuznetsk",
    "Asia/Novosibirsk",
    "Asia/Omsk",
    "Asia/Oral",
    "Asia/Phnom_Penh",
    "Asia/Pontianak",
    "Asia/Pyongyang",
    "Asia/Qatar",
    "Asia/Qostanay",
    "Asia/Qyzylorda",
    "Asia/Rangoon",
    "Asia/Riyadh",
    "Asia/Saigon",
    "Asia/Sakhalin",
    "Asia/Samarkand",
    "Asia/Seoul",
    "Asia/Shanghai",
    "Asia/Singapore",
    "Asia/Srednekolymsk",
    "Asia/Taipei",
    "Asia/Tashkent",
    "Asia/Tbilisi",
    "Asia/Tehran",
    "Asia/Tel_Aviv",
    "Asia/Thimbu",
    "Asia/Thimphu",
    "Asia/Tokyo",
    "Asia/Tomsk",
    "Asia/Ujung_Pandang",
    "Asia/Ulaanbaatar",
    "Asia/Ulan_Bator",
    "Asia/Urumqi",
    "Asia/Ust-Nera",
    "Asia/Vientiane",
    "Asia/Vladivostok",
    "Asia/Yakutsk",
    "Asia/Yangon",
    "Asia/Yekaterinburg",
    "Asia/Yerevan",
    "Atlantic/Azores",
    "Atlantic/Bermuda",
    "Atlantic/Canary",
    "Atlantic/Cape_Verde",
    "Atlantic/Faeroe",
    "Atlantic/Faroe",
    "Atlantic/Jan_Mayen",
    "Atlantic/Madeira",
    "Atlantic/Reykjavik",
    "Atlantic/South_Georgia",
    "Atlantic/St_Helena",
    "Atlantic/Stanley",
    "Australia/ACT",
    "Australia/Adelaide",
    "Australia/Brisbane",
    "Australia/Broken_Hill",
    "Australia/Canberra",
    "Australia/Currie",
    "Australia/Darwin",
    "Australia/Eucla",
    "Australia/Hobart",
    "Australia/LHI",
    "Australia/Lindeman",
    "Australia/Lord_Howe",
    "Australia/Melbourne",
    "Australia/North",
    "Australia/NSW",
    "Australia/Perth",
    "Australia/Queensland",
    "Australia/South",
    "Australia/Sydney",
    "Australia/Tasmania",
    "Australia/Victoria",
    "Australia/West",
    "Australia/Yancowinna",
    "Brazil/Acre",
    "Brazil/DeNoronha",
    "Brazil/East",
    "Brazil/West",
    "Canada/Atlantic",
    "Canada/Central",
    "Canada/Eastern",
    "Canada/Mountain",
    "Canada/Newfoundland",
    "Canada/Pacific",
    "Canada/Saskatchewan",
    "Canada/Yukon",
    "CET",
    "Chile/Continental",
    "Chile/EasterIsland",
    "CST6CDT",
    "Cuba",
    "EET",
    "Egypt",
    "Eire",
    "EST",
    "EST5EDT",
    "Etc/GMT",
    "Etc/GMT+0",
    "Etc/GMT+1",
    "Etc/GMT+10",
    "Etc/GMT+11",
    "Etc/GMT+12",
    "Etc/GMT+2",
    "Etc/GMT+3",
    "Etc/GMT+4",
    "Etc/GMT+5",
    "Etc/GMT+6",
    "Etc/GMT+7",
    "Etc/GMT+8",
    "Etc/GMT+9",
    "Etc/GMT-0",
    "Etc/GMT-1",
    "Etc/GMT-10",
    "Etc/GMT-11",
    "Etc/GMT-12",
    "Etc/GMT-13",
    "Etc/GMT-14",
    "Etc/GMT-2",
    "Etc/GMT-3",
    "Etc/GMT-4",
    "Etc/GMT-5",
    "Etc/GMT-6",
    "Etc/GMT-7",
    "Etc/GMT-8",
    "Etc/GMT-9",
    "Etc/GMT0",
    "Etc/Greenwich",
    "Etc/UCT",
    "Etc/Universal",
    "Etc/UTC",
    "Etc/Zulu",
    "Europe/Amsterdam",
    "Europe/Andorra",
    "Europe/Astrakhan",
    "Europe/Athens",
    "Europe/Belfast",
    "Europe/Belgrade",
    "Europe/Berlin",
    "Europe/Bratislava",
    "Europe/Brussels",
    "Europe/Bucharest",
    "Europe/Budapest",
    "Europe/Busingen",
    "Europe/Chisinau",
    "Europe/Copenhagen",
    "Europe/Dublin",
    "Europe/Gibraltar",
    "Europe/Guernsey",
    "Europe/Helsinki",
    "Europe/Isle_of_Man",
    "Europe/Istanbul",
    "Europe/Jersey",
    "Europe/Kaliningrad",
    "Europe/Kiev",
    "Europe/Kirov",
    "Europe/Kyiv",
    "Europe/Lisbon",
    "Europe/Ljubljana",
    "Europe/London",
    "Europe/Luxembourg",
    "Europe/Madrid",
    "Europe/Malta",
    "Europe/Mariehamn",
    "Europe/Minsk",
    "Europe/Monaco",
    "Europe/Moscow",
    "Europe/Nicosia",
    "Europe/Oslo",
    "Europe/Paris",
    "Europe/Podgorica",
    "Europe/Prague",
    "Europe/Riga",
    "Europe/Rome",
    "Europe/Samara",
    "Europe/San_Marino",
    "Europe/Sarajevo",
    "Europe/Saratov",
    "Europe/Simferopol",
    "Europe/Skopje",
    "Europe/Sofia",
    "Europe/Stockholm",
    "Europe/Tallinn",
    "Europe/Tirane",
    "Europe/Tiraspol",
    "Europe/Ulyanovsk",
    "Europe/Uzhgorod",
    "Europe/Vaduz",
    "Europe/Vatican",
    "Europe/Vienna",
    "Europe/Vilnius",
    "Europe/Volgograd",
    "Europe/Warsaw",
    "Europe/Zagreb",
    "Europe/Zaporozhye",
    "Europe/Zurich",
    "Factory",
    "GB",
    "GB-Eire",
    "GMT",
    "GMT+0",
    "GMT-0",
    "GMT0",
    "Greenwich",
    "Hongkong",
    "HST",
    "Iceland",
    "Indian/Antananarivo",
    "Indian/Chagos",
    "Indian/Christmas",
    "Indian/Cocos",
    "Indian/Comoro",
    "Indian/Kerguelen",
    "Indian/Mahe",
    "Indian/Maldives",
    "Indian/Mauritius",
    "Indian/Mayotte",
    "Indian/Reunion",
    "Iran",
    "Israel",
    "Jamaica",
    "Japan",
    "Kwajalein",
    "Libya",
    "MET",
    "Mexico/BajaNorte",
    "Mexico/BajaSur",
    "Mexico/General",
    "MST",
    "MST7MDT",
    "Navajo",
    "NZ",
    "NZ-CHAT",
    "Pacific/Apia",
    "Pacific/Auckland",
    "Pacific/Bougainville",
    "Pacific/Chatham",
    "Pacific/Chuuk",
    "Pacific/Easter",
    "Pacific/Efate",
    "Pacific/Enderbury",
    "Pacific/Fakaofo",
    "Pacific/Fiji",
    "Pacific/Funafuti",
    "Pacific/Galapagos",
    "Pacific/Gambier",
    "Pacific/Guadalcanal",
    "Pacific/Guam",
    "Pacific/Honolulu",
    "Pacific/Johnston",
    "Pacific/Kanton",
    "Pacific/Kiritimati",
    "Pacific/Kosrae",
    "Pacific/Kwajalein",
    "Pacific/Majuro",
    "Pacific/Marquesas",
    "Pacific/Midway",
    "Pacific/Nauru",
    "Pacific/Niue",
    "Pacific/Norfolk",
    "Pacific/Noumea",
    "Pacific/Pago_Pago",
    "Pacific/Palau",
    "Pacific/Pitcairn",
    "Pacific/Pohnpei",
    "Pacific/Ponape",
    "Pacific/Port_Moresby",
    "Pacific/Rarotonga",
    "Pacific/Saipan",
    "Pacific/Samoa",
    "Pacific/Tahiti",
    "Pacific/Tarawa",
    "Pacific/Tongatapu",
    "Pacific/Truk",
    "Pacific/Wake",
    "Pacific/Wallis",
    "Pacific/Yap",
    "Poland",
    "Portugal",
    "PRC",
    "PST8PDT",
    "ROC",
    "ROK",
    "Singapore",
    "Turkey",
    "UCT",
    "Universal",
    "US/Alaska",
    "US/Aleutian",
    "US/Arizona",
    "US/Central",
    "US/East-Indiana",
    "US/Eastern",
    "US/Hawaii",
    "US/Indiana-Starke",
    "US/Michigan",
    "US/Mountain",
    "US/Pacific",
    "US/Samoa",
    "UTC",
    "W-SU",
    "WET",
    "Zulu",
]

```

<h2 id="ry.ryo3._jiter"><code>ry.ryo3._jiter</code></h2>

```python
import typing as t
from os import PathLike

import typing_extensions as te

from ry._types import Buffer

# =============================================================================
# JSON
# =============================================================================
JsonPrimitive: te.TypeAlias = None | bool | int | float | str
JsonValue: te.TypeAlias = (
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
def parse_jsonl(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> list[JsonValue]: ...
def read_json(
    p: str | PathLike[str],
    /,
    lines: bool = False,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...

```

<h2 id="ry.ryo3._quick_maths"><code>ry.ryo3._quick_maths</code></h2>

```python
"""ryo3-quick-maths types"""

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

<h2 id="ry.ryo3._regex"><code>ry.ryo3._regex</code></h2>

```python
"""ryo3-regex types"""

import typing as t

# =============================================================================
# Regex
# =============================================================================


@t.final
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
    def is_match(self, string: str) -> bool: ...
    def find(self, string: str) -> str | None: ...
    def find_all(self, string: str) -> list[tuple[int, int]]: ...
    def findall(self, string: str) -> list[tuple[int, int]]: ...
    def replace(self, string: str, replacement: str) -> str: ...
    def replace_all(self, string: str, replacement: str) -> str: ...
    def split(self, string: str) -> list[str]: ...
    def splitn(self, string: str, n: int) -> list[str]: ...

```

<h2 id="ry.ryo3._reqwest"><code>ry.ryo3._reqwest</code></h2>

```python
import typing as t

import typing_extensions as te

import ry
from ry._types import Buffer
from ry.ryo3._http import HTTP_VERSION_LIKE, Headers, HttpStatus
from ry.ryo3._std import Duration
from ry.ryo3._url import URL


class RequestKwargs(t.TypedDict, total=False):
    body: Buffer | None
    headers: Headers | dict[str, str] | None
    query: dict[str, t.Any] | t.Sequence[tuple[str, t.Any]] | None
    json: t.Any
    form: t.Any
    multipart: t.Any
    timeout: Duration | None
    version: HTTP_VERSION_LIKE | None


@t.final
class HttpClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | None = None,
        cookies: bool = False,
        user_agent: str | None = None,  # default ~ 'ry-reqwest/<VERSION> ...'
        timeout: Duration | None = None,
        connect_timeout: Duration | None = None,
        read_timeout: Duration | None = None,
        gzip: bool = True,
        brotli: bool = True,
        deflate: bool = True,
    ) -> None: ...
    async def get(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def post(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def put(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def delete(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def patch(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def options(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def head(
        self,
        url: str | URL,
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def fetch(
        self,
        url: str | URL,
        *,
        method: str = "GET",
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...
    async def __call__(
        self,
        url: str | URL,
        *,
        method: str = "GET",
        **kwargs: te.Unpack[RequestKwargs],
    ) -> Response: ...


@t.final
class ReqwestError(Exception):
    def __init__(self, *args: t.Any, **kwargs: t.Any) -> None: ...
    def __dbg__(self) -> str: ...
    def is_body(self) -> bool: ...
    def is_builder(self) -> bool: ...
    def is_connect(self) -> bool: ...
    def is_decode(self) -> bool: ...
    def is_redirect(self) -> bool: ...
    def is_request(self) -> bool: ...
    def is_status(self) -> bool: ...
    def is_timeout(self) -> bool: ...
    def status(self) -> HttpStatus | None: ...
    def url(self) -> URL | None: ...


@t.final
class Response:
    @property
    def headers(self) -> Headers: ...
    async def text(self) -> str: ...
    async def json(self) -> t.Any: ...
    async def bytes(self) -> ry.Bytes: ...
    def bytes_stream(self) -> ResponseStream: ...
    def stream(self) -> ResponseStream: ...
    @property
    def url(self) -> URL: ...
    @property
    def version(
        self,
    ) -> t.Literal[
        "HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"
    ]: ...
    @property
    def http_version(
        self,
    ) -> t.Literal[
        "HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"
    ]: ...
    @property
    def status(self) -> int: ...
    @property
    def status_text(self) -> str: ...
    @property
    def status_code(self) -> HttpStatus: ...
    @property
    def redirected(self) -> bool: ...


@t.final
class ResponseStream:
    def __aiter__(self) -> ResponseStream: ...
    async def __anext__(self) -> ry.Bytes: ...
    async def take(self, n: int = 1) -> list[ry.Bytes]: ...
    @t.overload
    async def collect(
        self, join: t.Literal[False] = False
    ) -> list[ry.Bytes]: ...
    @t.overload
    async def collect(self, join: t.Literal[True] = True) -> ry.Bytes: ...


async def fetch(
    url: str | URL,
    *,
    client: HttpClient | None = None,
    method: str = "GET",
    **kwargs: te.Unpack[RequestKwargs],
) -> Response: ...

```

<h2 id="ry.ryo3._same_file"><code>ry.ryo3._same_file</code></h2>

```python
"""ryo3-same-file types"""

from os import PathLike


def is_same_file(a: PathLike[str], b: PathLike[str]) -> bool: ...

```

<h2 id="ry.ryo3._shlex"><code>ry.ryo3._shlex</code></h2>

```python
"""ryo3-shlex types"""


def shplit(s: str) -> list[str]:
    """shlex::split wrapper much like python's stdlib shlex.split but faster"""

```

<h2 id="ry.ryo3._size"><code>ry.ryo3._size</code></h2>

```python
import typing as t

import typing_extensions as te

FORMAT_SIZE_BASE: te.TypeAlias = t.Literal[2, 10]  # default=2
FORMAT_SIZE_STYLE: te.TypeAlias = t.Literal[  # default="default"
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


@t.final
class SizeFormatter:
    """Human-readable bytes-size formatter."""

    def __init__(
        self,
        base: FORMAT_SIZE_BASE | None = 2,
        style: FORMAT_SIZE_STYLE | None = "default",
    ) -> None:
        """Initialize human-readable bytes-size formatter."""

    def format(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""

    def __call__(self, n: int) -> str:
        """Return human-readable string representation of bytes-size."""


@t.final
class Size:
    """Bytes-size object."""

    def __init__(self, size: int) -> None: ...
    def __int__(self) -> int: ...
    def __hash__(self) -> int: ...
    def __abs__(self) -> Size: ...
    def __neg__(self) -> Size: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Size | float) -> bool: ...
    def __le__(self, other: Size | float) -> bool: ...
    def __gt__(self, other: Size | float) -> bool: ...
    def __ge__(self, other: Size | float) -> bool: ...
    def __bool__(self) -> bool: ...
    def __pos__(self) -> Size: ...
    def __invert__(self) -> Size: ...
    def __add__(self, other: Size | float) -> Size: ...
    def __sub__(self, other: Size | float) -> Size: ...
    def __mul__(self, other: Size | float) -> Size: ...
    def __rmul__(self, other: Size | float) -> Size: ...
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
    def from_bytes(cls: type[Size], size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # KILOBYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_kb(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_kib(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_kibibytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_kilobytes(cls: type[Size], size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # MEGABYTES
    # -------------------------------------------------------------------------

    @classmethod
    def from_mb(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_mebibytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_megabytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_mib(cls: type[Size], size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # GIGABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_gb(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_gib(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_gibibytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_gigabytes(cls: type[Size], size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # TERABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_tb(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_tebibytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_terabytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_tib(cls: type[Size], size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # PETABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_pb(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_pebibytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_petabytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_pib(cls: type[Size], size: float) -> Size: ...

    # -------------------------------------------------------------------------
    # EXABYTES
    # -------------------------------------------------------------------------
    @classmethod
    def from_eb(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_eib(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_exabytes(cls: type[Size], size: float) -> Size: ...
    @classmethod
    def from_exbibytes(cls: type[Size], size: float) -> Size: ...

```

<h2 id="ry.ryo3._sqlformat"><code>ry.ryo3._sqlformat</code></h2>

```python
import typing as t

import typing_extensions

# =============================================================================
# SQLFORMAT
# =============================================================================
SqlfmtParamValue: typing_extensions.TypeAlias = str | int | float | bool
_TSqlfmtParamValue_co = t.TypeVar(
    "_TSqlfmtParamValue_co", bound=SqlfmtParamValue, covariant=True
)
SqlfmtParamsLike: typing_extensions.TypeAlias = (
    dict[str, _TSqlfmtParamValue_co]
    | t.Sequence[tuple[str, _TSqlfmtParamValue_co]]
    | t.Sequence[_TSqlfmtParamValue_co]
)


class SqlfmtQueryParams:
    def __init__(
        self, params: SqlfmtParamsLike[_TSqlfmtParamValue_co]
    ) -> None: ...


def sqlfmt_params(
    params: SqlfmtParamsLike[_TSqlfmtParamValue_co] | SqlfmtQueryParams,
) -> SqlfmtQueryParams: ...
def sqlfmt(
    sql: str,
    params: SqlfmtParamsLike[_TSqlfmtParamValue_co]
    | SqlfmtQueryParams
    | None = None,
    *,
    indent: int = 2,  # -1 or any negative value will use tabs
    uppercase: bool | None = True,
    lines_between_statements: int = 1,
) -> str: ...

```

<h2 id="ry.ryo3._std"><code>ry.ryo3._std</code></h2>

```python
"""ryo3-std types"""

import datetime as pydt
import ipaddress
import pathlib
import typing as t

import typing_extensions as te

from ry._types import Buffer, FileTypeDict, FsPathLike, MetadataDict, ToPy
from ry.ryo3._bytes import Bytes


# =============================================================================
# STD::TIME
# =============================================================================
@t.final
class Duration(ToPy[pydt.timedelta]):
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
    def __bool__(self) -> bool: ...
    def __float__(self) -> float: ...
    def __int__(self) -> int: ...
    @t.overload
    def __truediv__(self, other: Duration | pydt.timedelta) -> float: ...
    @t.overload
    def __truediv__(self, other: float) -> Duration: ...
    def __mul__(self, other: float) -> Duration: ...
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
    def to_py(self) -> pydt.timedelta: ...

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


@t.final
class Instant:
    def __init__(self) -> None: ...
    @classmethod
    def now(cls) -> Instant: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Instant) -> bool: ...
    def __le__(self, other: Instant) -> bool: ...
    def __gt__(self, other: Instant) -> bool: ...
    def __ge__(self, other: Instant) -> bool: ...
    def __hash__(self) -> int: ...
    def __add__(self, other: Duration) -> Instant: ...
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
@t.final
class FileType:
    def __init__(self, *args: te.Never, **kwargs: te.Never) -> te.NoReturn: ...
    @property
    def is_dir(self) -> bool: ...
    @property
    def is_file(self) -> bool: ...
    @property
    def is_symlink(self) -> bool: ...
    def to_py(self) -> FileTypeDict: ...


@t.final
class Permissions:
    @property
    def readonly(self) -> bool: ...
    def __eq__(self, value: object) -> bool: ...
    def __ne__(self, value: object) -> bool: ...


@t.final
class Metadata:
    def __init__(self) -> te.NoReturn: ...
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
    @property
    def permissions(self) -> Permissions: ...
    @property
    def readonly(self) -> bool: ...
    def to_py(self) -> MetadataDict: ...


@t.final
class DirEntry:
    def __fspath__(self) -> str: ...
    @property
    def path(self) -> pathlib.Path: ...
    @property
    def basename(self) -> str: ...
    @property
    def metadata(self) -> Metadata: ...
    @property
    def file_type(self) -> FileType: ...


_T = t.TypeVar("_T")


class RyIterable(t.Generic[_T]):
    def __iter__(self) -> te.Self: ...
    def __next__(self) -> _T: ...
    def collect(self) -> list[_T]: ...
    def take(self, n: int = 1) -> list[_T]: ...


@t.final
class ReadDir(RyIterable[DirEntry]): ...


@t.final
class FileReadStream:
    def __init__(
        self,
        path: FsPathLike,
        *,
        chunk_size: int = 65536,
        offset: int = 0,
        buffered: bool = True,
    ) -> None: ...
    def __iter__(self) -> te.Self: ...
    def __next__(self) -> Bytes: ...
    def collect(self) -> list[Bytes]: ...
    def take(self, n: int = 1) -> list[Bytes]: ...


# ============================================================================
# STD::FS ~ functions
# =============================================================================
def read(path: FsPathLike) -> Bytes: ...
def read_bytes(path: FsPathLike) -> bytes: ...
def read_dir(
    path: FsPathLike,
) -> ReadDir: ...
def read_text(path: FsPathLike) -> str: ...
def read_stream(
    path: FsPathLike,
    chunk_size: int = 65536,
    *,
    offset: int = 0,
) -> FileReadStream: ...
def write(path: FsPathLike, data: Buffer | str) -> int: ...
def write_bytes(path: FsPathLike, data: bytes) -> int: ...
def write_text(path: FsPathLike, data: str) -> int: ...
def canonicalize(path: FsPathLike) -> pathlib.Path: ...
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


# =============================================================================
# STD::NET
# =============================================================================
@t.final
class Ipv4Addr:
    BROADCAST: Ipv4Addr
    LOCALHOST: Ipv4Addr
    UNSPECIFIED: Ipv4Addr

    @t.overload
    def __init__(self, a: int, b: int, c: int, d: int) -> None: ...
    @t.overload
    def __init__(self, iplike: int | str | bytes | Ipv4Addr) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Ipv4Addr) -> bool: ...
    def __le__(self, other: Ipv4Addr) -> bool: ...
    def __gt__(self, other: Ipv4Addr) -> bool: ...
    def __ge__(self, other: Ipv4Addr) -> bool: ...
    def __hash__(self) -> int: ...
    def to_py(self) -> ipaddress.IPv4Address: ...

    # ========================================================================
    # PROPERTIES
    # ========================================================================
    @property
    def version(self) -> int: ...
    @property
    def is_broadcast(self) -> bool: ...
    @property
    def is_documentation(self) -> bool: ...
    @property
    def is_link_local(self) -> bool: ...
    @property
    def is_loopback(self) -> bool: ...
    @property
    def is_multicast(self) -> bool: ...
    @property
    def is_private(self) -> bool: ...
    @property
    def is_unspecified(self) -> bool: ...
    @property
    def is_benchmarking(self) -> t.NoReturn: ...
    @property
    def is_global(self) -> t.NoReturn: ...
    @property
    def is_reserved(self) -> t.NoReturn: ...
    @property
    def is_shared(self) -> t.NoReturn: ...

    # ========================================================================
    # CLASSMETHODS
    # ========================================================================
    @classmethod
    def parse(cls, s: str) -> Ipv4Addr: ...
    @classmethod
    def from_bits(cls, bits: int) -> Ipv4Addr: ...
    @classmethod
    def from_octets(cls, b: bytes) -> Ipv4Addr: ...

    # =======================================================================
    # METHODS
    # =======================================================================
    def to_ipaddr(self) -> IpAddr: ...


class Ipv6Addr:
    LOCALHOST: Ipv6Addr
    UNSPECIFIED: Ipv6Addr

    @t.overload
    def __init__(
        self, a: int, b: int, c: int, d: int, e: int, f: int, g: int, h: int
    ) -> None: ...
    @t.overload
    def __init__(self, iplike: int | str | bytes | Ipv6Addr) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: Ipv6Addr) -> bool: ...
    def __le__(self, other: Ipv6Addr) -> bool: ...
    def __gt__(self, other: Ipv6Addr) -> bool: ...
    def __ge__(self, other: Ipv6Addr) -> bool: ...
    def __hash__(self) -> int: ...
    def to_py(self) -> ipaddress.IPv6Address: ...

    # ========================================================================
    # PROPERTIES
    # ========================================================================
    @property
    def version(self) -> int: ...
    @property
    def is_loopback(self) -> bool: ...
    @property
    def is_multicast(self) -> bool: ...
    @property
    def is_unicast_link_local(self) -> bool: ...
    @property
    def is_unique_local(self) -> bool: ...
    @property
    def is_unspecified(self) -> bool: ...
    @property
    def is_benchmarking(self) -> t.NoReturn: ...
    @property
    def is_documentation(self) -> t.NoReturn: ...
    @property
    def is_global(self) -> t.NoReturn: ...
    @property
    def is_ipv4_mapped(self) -> t.NoReturn: ...
    @property
    def is_unicast(self) -> t.NoReturn: ...
    @property
    def is_unicast_global(self) -> t.NoReturn: ...

    # ========================================================================
    # CLASSMETHODS
    # ========================================================================
    @classmethod
    def parse(cls, s: str) -> Ipv4Addr: ...
    @classmethod
    def from_bits(cls, bits: int) -> IpAddr: ...

    # =======================================================================
    # METHODS
    # =======================================================================
    def to_ipaddr(self) -> IpAddr: ...


class IpAddr:
    BROADCAST: IpAddr
    LOCALHOST_V4: IpAddr
    UNSPECIFIED_V4: IpAddr
    LOCALHOST_V6: IpAddr
    UNSPECIFIED_V6: IpAddr

    def __init__(
        self,
        iplike: int
        | str
        | bytes
        | ipaddress.IPv4Address
        | ipaddress.IPv6Address,
    ) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: IpAddr) -> bool: ...
    def __le__(self, other: IpAddr) -> bool: ...
    def __gt__(self, other: IpAddr) -> bool: ...
    def __ge__(self, other: IpAddr) -> bool: ...
    def __hash__(self) -> int: ...
    def to_py(self) -> ipaddress.IPv4Address | ipaddress.IPv6Address: ...
    def to_ipv4(self) -> Ipv4Addr: ...
    def to_ipv6(self) -> Ipv6Addr: ...

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def parse(cls, ip: str) -> IpAddr: ...

    # ========================================================================
    # PROPERTIES
    # ========================================================================

    @property
    def version(self) -> int: ...
    @property
    def is_benchmarking(self) -> t.NoReturn: ...
    @property
    def is_ipv4(self) -> bool: ...
    @property
    def is_ipv6(self) -> bool: ...
    @property
    def is_broadcast(self) -> bool: ...
    @property
    def is_documentation(self) -> bool: ...
    @property
    def is_loopback(self) -> bool: ...
    @property
    def is_multicast(self) -> bool: ...
    @property
    def is_private(self) -> bool: ...
    @property
    def is_unspecified(self) -> bool: ...

    # =======================================================================
    # METHODS
    # =======================================================================
    def to_canonical(self) -> IpAddr: ...

```

<h2 id="ry.ryo3._tokio"><code>ry.ryo3._tokio</code></h2>

```python
"""ryo4-tokio types"""

import pathlib
import typing as t
from collections.abc import Generator
from types import TracebackType

import typing_extensions as te

from ry import Bytes
from ry._types import Buffer, FsPathLike
from ry.ryo3._std import FileType, Metadata


# =============================================================================
# FS
# =============================================================================
async def canonicalize_async(path: FsPathLike) -> FsPathLike: ...
async def copy_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def create_dir_async(path: FsPathLike) -> None: ...
async def create_dir_all_async(path: FsPathLike) -> None: ...
async def hard_link_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def metadata_async(path: FsPathLike) -> None: ...
async def read_async(path: FsPathLike) -> Bytes: ...
async def remove_dir_async(path: FsPathLike) -> None: ...
async def remove_dir_all_async(path: FsPathLike) -> None: ...
async def remove_file_async(path: FsPathLike) -> None: ...
async def read_link_async(path: FsPathLike) -> FsPathLike: ...
async def read_to_string_async(path: FsPathLike) -> str: ...
async def rename_async(src: FsPathLike, dst: FsPathLike) -> None: ...
async def write_async(path: FsPathLike, data: Buffer) -> None: ...
async def try_exists_async(path: FsPathLike) -> bool: ...
async def exists_async(path: FsPathLike) -> bool: ...


@t.final
class DirEntryAsync:
    def __fspath__(self) -> str: ...
    @property
    def path(self) -> pathlib.Path: ...
    @property
    def basename(self) -> str: ...
    @property
    async def metadata(self) -> Metadata: ...
    @property
    async def file_type(self) -> FileType: ...


@t.final
class ReadDirAsync:
    """Async iterator for read_dir_async"""

    async def collect(self) -> list[DirEntryAsync]: ...
    async def take(self, n: int) -> list[DirEntryAsync]: ...
    def __aiter__(self) -> ReadDirAsync: ...
    async def __anext__(self) -> DirEntryAsync: ...


async def read_dir_async(path: FsPathLike) -> ReadDirAsync: ...


# =============================================================================
# SLEEP
# =============================================================================
async def sleep_async(seconds: float) -> float: ...
async def asleep(seconds: float) -> float:
    """Alias for sleep_async"""


# =============================================================================
# ASYNC-FILE
# =============================================================================
@t.final
class AsyncFile:
    def __init__(
        self, path: FsPathLike, mode: str = "r", buffering: int = -1
    ) -> None: ...
    async def close(self) -> None: ...
    async def flush(self) -> None: ...
    async def isatty(self) -> te.NoReturn: ...
    async def open(self) -> None: ...
    async def peek(self, size: int = ..., /) -> Bytes: ...
    async def read(self, size: int = ..., /) -> Bytes: ...
    async def readable(self) -> bool: ...
    async def readall(self) -> Bytes: ...
    async def readline(self, size: int | None = ..., /) -> Bytes: ...
    async def readlines(self, hint: int = ..., /) -> list[Bytes]: ...
    async def seek(self, offset: int, whence: int = ..., /) -> int: ...
    async def seekable(self) -> bool: ...
    async def tell(self) -> int: ...
    async def truncate(self, size: int | None = ..., /) -> int: ...
    async def writable(self) -> bool: ...
    async def write(self, b: Buffer, /) -> int: ...
    @property
    def closed(self) -> bool: ...
    def __await__(self) -> Generator[t.Any, t.Any, te.Self]: ...
    def __aiter__(self) -> te.Self: ...
    async def __anext__(self) -> Bytes: ...
    async def __aenter__(self) -> te.Self: ...
    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None: ...


def aiopen(
    path: FsPathLike, mode: str = "r", buffering: int = -1
) -> AsyncFile: ...

```

<h2 id="ry.ryo3._unindent"><code>ry.ryo3._unindent</code></h2>

```python
"""ryo3-unindent types"""


def unindent(string: str) -> str: ...
def unindent_bytes(string: bytes) -> bytes: ...

```

<h2 id="ry.ryo3._url"><code>ry.ryo3._url</code></h2>

```python
import typing as t
from ipaddress import IPv4Address, IPv6Address


@t.final
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
    def __fspath__(self) -> str: ...

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
    def query_pairs(self) -> tuple[tuple[str, str], ...]: ...
    @property
    def scheme(self) -> str: ...
    @property
    def username(self) -> str: ...
    @property
    def origin(self) -> str: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def has_authority(self) -> bool: ...
    def has_host(self) -> bool: ...
    def is_special(self) -> bool: ...
    def join(self, *parts: str) -> URL: ...
    def make_relative(self, u: URL) -> URL: ...
    def to_filepath(self) -> str: ...
    def replace_fragment(self, fragment: str | None = None) -> URL: ...
    def replace_host(self, host: str | None = None) -> URL: ...
    def replace_ip_host(self, host: IPv4Address | IPv6Address) -> URL: ...
    def replace_password(self, password: str | None = None) -> URL: ...
    def replace_path(self, path: str) -> URL: ...
    def replace_port(self, port: int | None = None) -> URL: ...
    def replace_query(self, query: str | None = None) -> URL: ...
    def replace_scheme(self, scheme: str) -> URL: ...
    def replace_username(self, username: str) -> URL: ...
    def socket_addrs(self) -> None: ...
    def replace(
        self,
        *,
        fragment: str | None = None,
        host: str | None = None,
        ip_host: IPv4Address | None = None,
        password: str | None = None,
        path: str | None = None,
        port: int | None = None,
        query: str | None = None,
        scheme: str | None = None,
        username: str | None = None,
    ) -> URL: ...

```

<h2 id="ry.ryo3._walkdir"><code>ry.ryo3._walkdir</code></h2>

```python
"""ryo3-walkdir types"""

import typing as t
from os import PathLike

import typing_extensions as te

from ry import FileType, FsPath, Glob, GlobSet, Globster


@t.final
class WalkDirEntry:
    def __fspath__(self) -> str: ...
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


_T_walkdir = t.TypeVar(
    "_T_walkdir",
    bound=WalkDirEntry | str,
)


@t.final
class WalkdirGen(t.Generic[_T_walkdir]):
    """walkdir::Walkdir iterable wrapper"""

    def __init__(
        self,
    ) -> te.NoReturn: ...
    def __next__(self) -> _T_walkdir: ...
    def __iter__(self) -> t.Iterator[_T_walkdir]: ...
    def collect(self) -> list[_T_walkdir]: ...
    def take(self, n: int = 1) -> list[_T_walkdir]: ...


@t.overload
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
    objects: t.Literal[True],
) -> WalkdirGen[WalkDirEntry]: ...
@t.overload
def walkdir(
    path: str | PathLike[str] | None = None,
    *,
    objects: t.Literal[False] = False,
    files: bool = True,
    dirs: bool = True,
    contents_first: bool = False,
    min_depth: int = 0,
    max_depth: int | None = None,
    follow_links: bool = False,
    same_file_system: bool = False,
    glob: Glob | GlobSet | Globster | t.Sequence[str] | str | None = None,
) -> WalkdirGen[str]: ...

```

<h2 id="ry.ryo3._which"><code>ry.ryo3._which</code></h2>

```python
"""ryo3-which types"""

from pathlib import Path

from ry.ryo3._regex import Regex


def which(cmd: str, path: None | str = None) -> Path | None: ...
def which_all(cmd: str, path: None | str = None) -> list[Path]: ...
def which_re(regex: str | Regex, path: None | str = None) -> list[Path]: ...

```

<h2 id="ry.ryo3._zstd"><code>ry.ryo3._zstd</code></h2>

```python
"""ry.ryo3 root level zstd exports"""

from ry.ryo3.zstd import compress as zstd_compress
from ry.ryo3.zstd import decode as zstd_decode
from ry.ryo3.zstd import decompress as zstd_decompress
from ry.ryo3.zstd import encode as zstd_encode
from ry.ryo3.zstd import is_zstd as is_zstd

__all__ = (
    "is_zstd",
    "zstd_compress",
    "zstd_decode",
    "zstd_decompress",
    "zstd_encode",
)

```

<h2 id="ry.dirs"><code>ry.dirs</code></h2>

```python
from ry.ryo3.dirs import audio as audio
from ry.ryo3.dirs import audio_dir as audio_dir
from ry.ryo3.dirs import cache as cache
from ry.ryo3.dirs import cache_dir as cache_dir
from ry.ryo3.dirs import config as config
from ry.ryo3.dirs import config_dir as config_dir
from ry.ryo3.dirs import config_local as config_local
from ry.ryo3.dirs import config_local_dir as config_local_dir
from ry.ryo3.dirs import data as data
from ry.ryo3.dirs import data_dir as data_dir
from ry.ryo3.dirs import data_local as data_local
from ry.ryo3.dirs import data_local_dir as data_local_dir
from ry.ryo3.dirs import desktop as desktop
from ry.ryo3.dirs import desktop_dir as desktop_dir
from ry.ryo3.dirs import document as document
from ry.ryo3.dirs import document_dir as document_dir
from ry.ryo3.dirs import download as download
from ry.ryo3.dirs import download_dir as download_dir
from ry.ryo3.dirs import executable as executable
from ry.ryo3.dirs import executable_dir as executable_dir
from ry.ryo3.dirs import font as font
from ry.ryo3.dirs import font_dir as font_dir
from ry.ryo3.dirs import home as home
from ry.ryo3.dirs import home_dir as home_dir
from ry.ryo3.dirs import picture as picture
from ry.ryo3.dirs import picture_dir as picture_dir
from ry.ryo3.dirs import preference as preference
from ry.ryo3.dirs import preference_dir as preference_dir
from ry.ryo3.dirs import public as public
from ry.ryo3.dirs import public_dir as public_dir
from ry.ryo3.dirs import runtime as runtime
from ry.ryo3.dirs import runtime_dir as runtime_dir
from ry.ryo3.dirs import state as state
from ry.ryo3.dirs import state_dir as state_dir
from ry.ryo3.dirs import template as template
from ry.ryo3.dirs import template_dir as template_dir
from ry.ryo3.dirs import video as video
from ry.ryo3.dirs import video_dir as video_dir

__all__ = (
    "audio",
    "audio_dir",
    "cache",
    "cache_dir",
    "config",
    "config_dir",
    "config_local",
    "config_local_dir",
    "data",
    "data_dir",
    "data_local",
    "data_local_dir",
    "desktop",
    "desktop_dir",
    "document",
    "document_dir",
    "download",
    "download_dir",
    "executable",
    "executable_dir",
    "font",
    "font_dir",
    "home",
    "home_dir",
    "picture",
    "picture_dir",
    "preference",
    "preference_dir",
    "public",
    "public_dir",
    "runtime",
    "runtime_dir",
    "state",
    "state_dir",
    "template",
    "template_dir",
    "video",
    "video_dir",
)

```

<h2 id="ry.JSON"><code>ry.JSON</code></h2>

```python
"""ry.JSON"""

from ry.ryo3.JSON import cache_clear as cache_clear
from ry.ryo3.JSON import cache_usage as cache_usage
from ry.ryo3.JSON import dumps as dumps
from ry.ryo3.JSON import loads as loads
from ry.ryo3.JSON import parse as parse
from ry.ryo3.JSON import stringify as stringify

__all__ = (
    "cache_clear",
    "cache_usage",
    "dumps",
    "loads",
    "parse",
    "stringify",
)

```

<h2 id="ry.ulid"><code>ry.ulid</code></h2>

```python
from ry.ryo3.ulid import ULID

__all__ = ("ULID",)

```

<h2 id="ry.uuid"><code>ry.uuid</code></h2>

```python
from ry.ryo3.uuid import (
    NAMESPACE_DNS,
    NAMESPACE_OID,
    NAMESPACE_URL,
    NAMESPACE_X500,
    RESERVED_FUTURE,
    RESERVED_MICROSOFT,
    RESERVED_NCS,
    RFC_4122,
    UUID,
    getnode,
    uuid1,
    uuid2,
    uuid3,
    uuid4,
    uuid5,
    uuid6,
    uuid7,
    uuid8,
)

__all__ = (
    "NAMESPACE_DNS",
    "NAMESPACE_OID",
    "NAMESPACE_URL",
    "NAMESPACE_X500",
    "RESERVED_FUTURE",
    "RESERVED_MICROSOFT",
    "RESERVED_NCS",
    "RFC_4122",
    "UUID",
    "getnode",
    "uuid1",
    "uuid2",
    "uuid3",
    "uuid4",
    "uuid5",
    "uuid6",
    "uuid7",
    "uuid8",
)

```

<h2 id="ry.xxhash"><code>ry.xxhash</code></h2>

```python
from ry.ryo3.xxhash import Xxh3 as Xxh3
from ry.ryo3.xxhash import Xxh32 as Xxh32
from ry.ryo3.xxhash import Xxh64 as Xxh64
from ry.ryo3.xxhash import xxh3 as xxh3
from ry.ryo3.xxhash import xxh3_64_digest as xxh3_64_digest
from ry.ryo3.xxhash import xxh3_64_hexdigest as xxh3_64_hexdigest
from ry.ryo3.xxhash import xxh3_64_intdigest as xxh3_64_intdigest
from ry.ryo3.xxhash import xxh3_128_digest as xxh3_128_digest
from ry.ryo3.xxhash import xxh3_128_hexdigest as xxh3_128_hexdigest
from ry.ryo3.xxhash import xxh3_128_intdigest as xxh3_128_intdigest
from ry.ryo3.xxhash import xxh3_digest as xxh3_digest
from ry.ryo3.xxhash import xxh3_hexdigest as xxh3_hexdigest
from ry.ryo3.xxhash import xxh3_intdigest as xxh3_intdigest
from ry.ryo3.xxhash import xxh32 as xxh32
from ry.ryo3.xxhash import xxh32_digest as xxh32_digest
from ry.ryo3.xxhash import xxh32_hexdigest as xxh32_hexdigest
from ry.ryo3.xxhash import xxh32_intdigest as xxh32_intdigest
from ry.ryo3.xxhash import xxh64 as xxh64
from ry.ryo3.xxhash import xxh64_digest as xxh64_digest
from ry.ryo3.xxhash import xxh64_hexdigest as xxh64_hexdigest
from ry.ryo3.xxhash import xxh64_intdigest as xxh64_intdigest
from ry.ryo3.xxhash import xxh128_digest as xxh128_digest
from ry.ryo3.xxhash import xxh128_hexdigest as xxh128_hexdigest
from ry.ryo3.xxhash import xxh128_intdigest as xxh128_intdigest

__all__ = (
    "Xxh3",
    "Xxh32",
    "Xxh64",
    "xxh3",
    "xxh3_64_digest",
    "xxh3_64_hexdigest",
    "xxh3_64_intdigest",
    "xxh3_128_digest",
    "xxh3_128_hexdigest",
    "xxh3_128_intdigest",
    "xxh3_digest",
    "xxh3_hexdigest",
    "xxh3_intdigest",
    "xxh32",
    "xxh32_digest",
    "xxh32_hexdigest",
    "xxh32_intdigest",
    "xxh64",
    "xxh64_digest",
    "xxh64_hexdigest",
    "xxh64_intdigest",
    "xxh128_digest",
    "xxh128_hexdigest",
    "xxh128_intdigest",
)

```

<h2 id="ry.zstd"><code>ry.zstd</code></h2>

```python
from ry.ryo3.zstd import BLOCKSIZE_MAX as BLOCKSIZE_MAX
from ry.ryo3.zstd import BLOCKSIZELOG_MAX as BLOCKSIZELOG_MAX
from ry.ryo3.zstd import CLEVEL_DEFAULT as CLEVEL_DEFAULT
from ry.ryo3.zstd import CONTENTSIZE_ERROR as CONTENTSIZE_ERROR
from ry.ryo3.zstd import CONTENTSIZE_UNKNOWN as CONTENTSIZE_UNKNOWN
from ry.ryo3.zstd import MAGIC_DICTIONARY as MAGIC_DICTIONARY
from ry.ryo3.zstd import MAGIC_SKIPPABLE_MASK as MAGIC_SKIPPABLE_MASK
from ry.ryo3.zstd import MAGIC_SKIPPABLE_START as MAGIC_SKIPPABLE_START
from ry.ryo3.zstd import MAGICNUMBER as MAGICNUMBER
from ry.ryo3.zstd import VERSION_MAJOR as VERSION_MAJOR
from ry.ryo3.zstd import VERSION_MINOR as VERSION_MINOR
from ry.ryo3.zstd import VERSION_NUMBER as VERSION_NUMBER
from ry.ryo3.zstd import VERSION_RELEASE as VERSION_RELEASE
from ry.ryo3.zstd import __zstd_version__ as __zstd_version__
from ry.ryo3.zstd import compress as compress
from ry.ryo3.zstd import decode as decode
from ry.ryo3.zstd import decompress as decompress
from ry.ryo3.zstd import is_zstd as is_zstd
from ry.ryo3.zstd import unzstd as unzstd

__all__ = (
    "BLOCKSIZELOG_MAX",
    "BLOCKSIZE_MAX",
    "CLEVEL_DEFAULT",
    "CONTENTSIZE_ERROR",
    "CONTENTSIZE_UNKNOWN",
    "MAGICNUMBER",
    "MAGIC_DICTIONARY",
    "MAGIC_SKIPPABLE_MASK",
    "MAGIC_SKIPPABLE_START",
    "VERSION_MAJOR",
    "VERSION_MINOR",
    "VERSION_NUMBER",
    "VERSION_RELEASE",
    "__zstd_version__",
    "compress",
    "decode",
    "decompress",
    "is_zstd",
    "unzstd",
)

```
