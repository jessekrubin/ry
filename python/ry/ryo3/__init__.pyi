"""ry api ~ type annotations"""

import datetime as pydt
import typing as t
from os import PathLike
from pathlib import Path

import typing_extensions as te

from ry._types import Buffer

from . import http as http
from ._bytes import Bytes as Bytes
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
from ._size import SizeFormatter as SizeFormatter
from ._size import fmt_size as fmt_size
from ._size import parse_size as parse_size
from ._url import URL as URL
from .http import Headers as Headers
from .http import HttpStatus as HttpStatus
from .reqwest import HttpClient as HttpClient
from .reqwest import ReqwestError as ReqwestError
from .reqwest import Response as Response
from .reqwest import ResponseStream as ResponseStream
from .reqwest import fetch as fetch
from .tokio import read_async as read_async
from .tokio import write_async as write_async

__version__: str
__authors__: str
__build_profile__: str
__build_timestamp__: str
__pkg_name__: str
__description__: str

# =============================================================================
# TYPE ALIASES
# =============================================================================
JsonPrimitive = None | bool | int | float | str
JsonValue = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)

# =============================================================================
# STD
# =============================================================================

# -----------------------------------------------------------------------------
# STD::TIME
# -----------------------------------------------------------------------------
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

# -----------------------------------------------------------------------------
# STD::FS
# -----------------------------------------------------------------------------
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
# RY03-CORE
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
    def read_text(self) -> str: ...
    def read_bytes(self) -> bytes: ...
    def absolute(self) -> FsPath: ...
    def resolve(self) -> FsPath: ...
    def write_text(self, data: str) -> None: ...
    def write_bytes(self, data: bytes) -> None: ...
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
    # std::path::PathBuf
    # =========================================================================
    def _pop(self) -> FsPath: ...
    def _push(self, path: PathLike[str] | str) -> FsPath: ...
    def _set_extension(self, ext: str) -> FsPath: ...
    def _set_file_name(self, name: str) -> FsPath: ...

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

FsPathLike = str | FsPath | PathLike[str]

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

# =============================================================================
# SLEEP
# =============================================================================
def sleep(seconds: float) -> float: ...
async def sleep_async(seconds: float) -> float: ...
async def asleep(seconds: float) -> float:
    """Alias for sleep_async"""
    ...

# =============================================================================
# FILESYSTEM
# =============================================================================
def read(path: FsPathLike) -> Bytes: ...
def read_bytes(path: FsPathLike) -> bytes: ...
def read_text(path: FsPathLike) -> str: ...
def write(path: FsPathLike, data: Buffer | str) -> None: ...
def write_bytes(path: FsPathLike, data: bytes) -> None: ...
def write_text(path: FsPathLike, data: str) -> None: ...

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

# =============================================================================
# WHICH
# =============================================================================
def which(cmd: str, path: None | str = None) -> str | None: ...
def which_all(cmd: str, path: None | str = None) -> list[str]: ...
def which_re(regex: str | Regex, path: None | str = None) -> list[str]: ...

# =============================================================================
# HECK
# =============================================================================

def camel_case(string: str) -> str: ...
def kebab_case(string: str) -> str: ...
def pascal_case(string: str) -> str: ...
def shouty_kebab_case(string: str) -> str: ...
def shouty_snake_case(string: str) -> str: ...
def snake_case(string: str) -> str: ...
def snek_case(string: str) -> str: ...
def title_case(string: str) -> str: ...
def train_case(string: str) -> str: ...

# =============================================================================
# GLOBSET
# =============================================================================
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
    def is_match(self, path: FsPathLike) -> bool: ...
    def __call__(self, path: FsPathLike) -> bool: ...
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
    def is_match(self, path: FsPathLike) -> bool: ...
    def __call__(self, path: FsPathLike) -> bool: ...
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
    dirs: bool = True,
    contents_first: bool = False,
    min_depth: int = 0,
    max_depth: int | None = None,
    follow_links: bool = False,
    same_file_system: bool = False,
    glob: Glob | GlobSet | Globster | t.Sequence[str] | str | None = None,
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
# JSON
# =============================================================================

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
    data: bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse_json_bytes(
    data: bytes,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse_json_str(
    data: str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def jiter_cache_clear() -> None: ...
def jiter_cache_usage() -> int: ...

# =============================================================================
# FORMATTING
# =============================================================================
def unindent(string: str) -> str: ...
def unindent_bytes(string: bytes) -> bytes: ...

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
    def __init__(self, params: SqlfmtParamsLike[TSqlfmtParamValue_co]) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

def sqlfmt_params(
    params: SqlfmtParamsLike[TSqlfmtParamValue_co] | SqlfmtQueryParams,
) -> SqlfmtQueryParams: ...
def sqlfmt(
    sql: str,
    params: SqlfmtParamsLike[TSqlfmtParamValue_co] | SqlfmtQueryParams | None = None,
    *,
    indent: int = 2,  # -1 or any negative value will use tabs
    uppercase: bool | None = True,
    lines_between_statements: int = 1,
) -> str: ...
