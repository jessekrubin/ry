"""ry api ~ type annotations"""

from __future__ import annotations

import datetime as pydt
import typing as t
from os import PathLike
from pathlib import Path

from ry import dirs as dirs  # noqa: RUF100
from ry import http as http  # noqa: RUF100
from ry import xxhash as xxhash  # noqa: RUF100
from ry._types import Buffer as Buffer  # noqa: RUF100
from ry.http import Headers as Headers  # noqa: RUF100
from ry.http import HttpStatus as HttpStatus  # noqa: RUF100

from ._bytes import Bytes as Bytes
from ._fnv import FnvHasher as FnvHasher
from ._fnv import fnv1a as fnv1a
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
from ._regex import Regex as Regex
from ._reqwest import HttpClient as HttpClient
from ._reqwest import ReqwestError as ReqwestError
from ._reqwest import Response as Response
from ._reqwest import ResponseStream as ResponseStream
from ._reqwest import fetch as fetch
from ._size import SizeFormatter as SizeFormatter
from ._size import fmt_size as fmt_size
from ._size import parse_size as parse_size
from ._sqlformat import SqlfmtQueryParams as SqlfmtQueryParams
from ._sqlformat import sqlfmt as sqlfmt
from ._sqlformat import sqlfmt_params as sqlfmt_params
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
from ._url import URL as URL
from ._which import which as which
from ._which import which_all as which_all
from ._which import which_re as which_re
from .errors import FeatureNotEnabledError as FeatureNotEnabledError

__version__: str
__authors__: str
__build_profile__: str
__build_timestamp__: str
__pkg_name__: str
__description__: str

# =============================================================================
# TYPE ALIASES
# =============================================================================

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
    def __richcmp__(self, other: Duration | pydt.timedelta, op: int) -> bool: ...
    def __str__(self) -> str: ...
    def abs_diff(self, other: Duration) -> Duration: ...
    def sleep(self) -> None: ...

    # =========================================================================
    # PYTHON_CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(cls: type[Duration], td: pydt.timedelta) -> Duration: ...
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
    offset: int = 0,  # noqa: F811
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

    # TODO
    def __bytes__(self) -> bytes: ...
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

FsPathLike = str | PathLike[str]

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

# -----------------------------------------------------------------------------
# \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
# /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
# -----------------------------------------------------------------------------
# |~|~ LIBS ~|~ LIBS ~|~ LIBS ~|~ LIBS ~|~ LIBS ~|~ LIBS ~|~ LIBS ~|~ LIBS ~|~|
# -----------------------------------------------------------------------------
# \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
# /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
# -----------------------------------------------------------------------------

# =============================================================================
# WALKDIR
# =============================================================================
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
    path: FsPathLike | None = None,
    *,
    files: bool = True,
    dirs: bool = True,  # noqa: F811
    contents_first: bool = False,
    min_depth: int = 0,
    max_depth: int | None = None,
    follow_links: bool = False,
    same_file_system: bool = False,
    glob: Glob | GlobSet | Globster | t.Sequence[str] | str | None = None,  # noqa: F811
) -> WalkdirGen: ...

# =============================================================================
# SAME_FILE
# =============================================================================

def is_same_file(a: PathLike[str], b: PathLike[str]) -> bool: ...

# =============================================================================
# SHLEX
# =============================================================================
def shplit(s: str) -> list[str]:
    """shlex::split wrapper much like python's stdlib shlex.split but faster"""
    ...

# =============================================================================
# FORMATTING
# =============================================================================
def unindent(string: str) -> str: ...
def unindent_bytes(string: bytes) -> bytes: ...

# =============================================================================
# BROTLI
# =============================================================================
def brotli_encode(
    input: bytes, quality: int = 11, magic_number: bool = False
) -> bytes: ...
def brotli_decode(input: bytes) -> bytes: ...
def brotli(input: bytes, quality: int = 11, magic_number: bool = False) -> bytes:
    """Alias for brotli_encode"""

# =============================================================================
# BZIP2
# =============================================================================
def bzip2_encode(input: bytes, quality: int = 9) -> bytes: ...
def bzip2_decode(input: bytes) -> bytes: ...
def bzip2(input: bytes, quality: int = 9) -> bytes:
    """Alias for bzip2_encode"""

# =============================================================================
# GZIP
# =============================================================================
def gzip_encode(input: bytes, quality: int = 9) -> bytes: ...
def gzip_decode(input: bytes) -> bytes: ...
def gzip(input: bytes, quality: int = 9) -> bytes:
    """Alias for gzip_encode"""

def gunzip(input: bytes) -> bytes:
    """Alias for gzip_decode"""

# =============================================================================
# ZSTD
# =============================================================================
def zstd_encode(input: bytes, level: int = 3) -> bytes: ...
def zstd(input: bytes, level: int = 3) -> bytes:
    """Alias for zstd_encode"""

def zstd_decode(input: bytes) -> bytes: ...
