# ry

ry = rust and python and bears, oh my!

[![PyPI](https://img.shields.io/pypi/v/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Wheel](https://img.shields.io/pypi/wheel/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Status](https://img.shields.io/pypi/status/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - License](https://img.shields.io/pypi/l/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)

**DOCS:** https://ryo3.dev (WIP)

python bindings for rust crates I wish existed in python

**THIS IS A WORK IN PROGRESS**

## Install

```bash
pip install ry
uv add ry
```

**Check install:** `python -m ry`

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
## `ry.__init__`

```python
"""ry api ~ type annotations"""

import datetime as pydt
import typing as t
from os import PathLike

from ry.types.jiff import (
    JIFF_ROUND_MODE_STRING,
    JIFF_UNIT_STRING,
    DateTimeTypedDict,
    DateTypedDict,
    TimeSpanTypedDict,
    TimeTypedDict,
)

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
    def parents(self) -> t.Sequence[t.Self]: ...

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


def ls(path: FsPathLike | None = None) -> list[FsPath]: ...


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


# =============================================================================
# FILESYSTEM
# =============================================================================
def read_text(path: FsPathLike) -> str: ...


def read_bytes(path: FsPathLike) -> bytes: ...


def write_text(path: FsPathLike, data: str) -> None: ...


def write_bytes(path: FsPathLike, data: bytes) -> None: ...


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


def which_re(regex: Regex, path: None | str = None) -> list[str]: ...


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


def globs(
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


class WalkdirGen:
    """walkdir::Walkdir iterable wrapper"""

    files: bool
    dirs: bool

    def __next__(self) -> str: ...

    def __iter__(self) -> t.Iterator[str]: ...

    def collect(self) -> list[str]: ...


class FspathsGen:
    """walkdir iterable that yields FsPath objects"""

    files: bool
    dirs: bool

    def __next__(self) -> FsPath: ...

    def __iter__(self) -> t.Iterator[FsPath]: ...


def walkdir(
        path: FsPathLike | None = None,
        files: bool = True,
        dirs: bool = True,
        contents_first: bool = False,
        min_depth: int = 0,
        max_depth: int | None = None,
        follow_links: bool = False,
        same_file_system: bool = False,
        glob: Glob | GlobSet | Globster | None = None,
) -> WalkdirGen: ...


def fspaths(
        path: FsPathLike | None = None,
        files: bool = True,
        dirs: bool = True,
        contents_first: bool = False,
        min_depth: int = 0,
        max_depth: int | None = None,
        follow_links: bool = False,
        same_file_system: bool = False,
) -> WalkdirGen: ...


# =============================================================================
# SHLEX
# =============================================================================
def shplit(s: str) -> list[str]:
    """shlex::split wrapper much like python's stdlib shlex.split but faster"""
    ...


# =============================================================================
# JSON
# =============================================================================
def parse_json(
        data: bytes | str,
        /,
        *,
        allow_inf_nan: bool = True,
        cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
        partial_mode: t.Literal[
            True, False, "off", "on", "trailing-strings"
        ] = False,
        catch_duplicate_keys: bool = False,
        float_mode: t.Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...


def parse_json_bytes(
        data: bytes,
        /,
        *,
        allow_inf_nan: bool = True,
        cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
        partial_mode: t.Literal[
            True, False, "off", "on", "trailing-strings"
        ] = False,
        catch_duplicate_keys: bool = False,
        float_mode: t.Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...


def parse_json_str(
        data: str,
        /,
        *,
        allow_inf_nan: bool = True,
        cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
        partial_mode: t.Literal[
            True, False, "off", "on", "trailing-strings"
        ] = False,
        catch_duplicate_keys: bool = False,
        float_mode: t.Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...


def jiter_cache_clear() -> None: ...


def jiter_cache_usage() -> int: ...


# =============================================================================
# FORMATTING
# =============================================================================
def fmt_nbytes(nbytes: int) -> str: ...


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


def brotli(
        input: bytes, quality: int = 11, magic_number: bool = False
) -> bytes:
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


# =============================================================================
# URL
# =============================================================================


class URL:
    def __init__(
            self, url: str, *, params: dict[str, str] | None = None
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

    def set_fragment(self, fragment: str) -> None: ...

    def set_host(self, host: str) -> None: ...

    def set_ip_host(self, host: str) -> None: ...

    def set_password(self, password: str) -> None: ...

    def set_path(self, path: str) -> None: ...

    def set_port(self, port: int) -> None: ...

    def set_query(self, query: str) -> None: ...

    def set_scheme(self, scheme: str) -> None: ...

    def set_username(self, username: str) -> None: ...

    def socket_addrs(self) -> None: ...


# =============================================================================
# JIFF
# =============================================================================


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

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def from_iso_week_date(
            cls: type[Date], year: int, week: int, weekday: int
    ) -> Date: ...

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

    def in_leap_year(self) -> bool: ...

    def intz(self, tz: str) -> ZonedDateTime: ...

    def last_of_month(self) -> Date: ...

    def last_of_year(self) -> Date: ...

    def saturating_add(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...

    def saturating_sub(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...

    def series(self, span: TimeSpan) -> t.Iterator[Date]: ...

    def to_datetime(self, time: Time) -> DateTime: ...

    def tomorrow(self) -> Date: ...

    def yesterday(self) -> Date: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: DateDifference) -> TimeSpan: ...

    def _until(self, other: DateDifference) -> TimeSpan: ...

    def since(self, other: IntoDateDifference) -> TimeSpan: ...

    def until(self, other: IntoDateDifference) -> TimeSpan: ...

    # =========================================================================
    # INSTANCE METHODS W/ OVERLOADS
    # =========================================================================
    @t.overload
    def checked_sub(self, other: Date) -> TimeSpan: ...

    @t.overload
    def checked_sub(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> Date: ...

    # =========================================================================
    # NOT IMPLEMENTED & NOT TYPED
    # =========================================================================

    def nth_weekday(self) -> t.NoReturn: ...

    def nth_weekday_of_month(self) -> t.NoReturn: ...

    def to_zoned(self) -> t.NoReturn: ...

    def weekday(self) -> t.NoReturn: ...


IntoDateDifference = (
        DateDifference
        | Date
        | DateTime
        | ZonedDateTime
        | tuple[JIFF_UNIT_STRING, Date]
        | tuple[JIFF_UNIT_STRING, DateTime]
        | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)


class DateDifference:
    def __init__(
            self,
            date: Date,
            *,
            smallest: JIFF_UNIT_STRING | None = None,
            largest: JIFF_UNIT_STRING | None = None,
            mode: JIFF_ROUND_MODE_STRING | None = None,
            increment: int | None = None,
    ) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def smallest(self, unit: JIFF_UNIT_STRING) -> DateDifference: ...

    def largest(self, unit: JIFF_UNIT_STRING) -> DateDifference: ...

    def mode(self, mode: JIFF_ROUND_MODE_STRING) -> DateDifference: ...

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
    def from_pytime(cls: type[Time], time: pydt.time) -> Time: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def midnight(cls: type[Time]) -> Time: ...

    @classmethod
    def now(cls: type[Time]) -> Time: ...

    @classmethod
    def parse(cls: type[t.Self], s: str) -> t.Self: ...

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

    def series(self, span: TimeSpan) -> t.Iterator[Time]: ...

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
            smallest: JIFF_UNIT_STRING | None = None,
            mode: JIFF_ROUND_MODE_STRING | None = None,
            increment: int | None = None,
    ) -> Time: ...

    def to_datetime(self, d: Date) -> DateTime: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: TimeDifference) -> TimeSpan: ...

    def _until(self, other: TimeDifference) -> TimeSpan: ...

    def since(self, other: IntoTimeDifference) -> TimeSpan: ...

    def until(self, other: IntoTimeDifference) -> TimeSpan: ...


IntoTimeDifference = (
        TimeDifference
        | Time
        | DateTime
        | ZonedDateTime
        | tuple[JIFF_UNIT_STRING, Time]
        | tuple[JIFF_UNIT_STRING, DateTime]
        | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)


class TimeDifference:
    def __init__(
            self,
            date: Time,
            *,
            smallest: JIFF_UNIT_STRING | None = None,
            largest: JIFF_UNIT_STRING | None = None,
            mode: JIFF_ROUND_MODE_STRING | None = None,
            increment: int | None = None,
    ) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def smallest(self, unit: JIFF_UNIT_STRING) -> TimeDifference: ...

    def largest(self, unit: JIFF_UNIT_STRING) -> TimeDifference: ...

    def mode(self, mode: JIFF_ROUND_MODE_STRING) -> TimeDifference: ...

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
    def parse(cls: type[t.Self], s: str) -> t.Self: ...

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

    def intz(self, tz: str) -> ZonedDateTime: ...

    def date(self) -> Date: ...

    def time(self) -> Time: ...

    def series(self, span: TimeSpan) -> t.Iterator[DateTime]: ...

    def asdict(self) -> DateTimeTypedDict: ...

    def round(self, options: JIFF_UNIT_STRING | DateTimeRound) -> t.Self: ...

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
    def weekday(self) -> t.NoReturn: ...

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

    def since(self, other: IntoDateTimeDifference) -> TimeSpan: ...

    def until(self, other: IntoDateTimeDifference) -> TimeSpan: ...

    # =========================================================================
    def nth_weekday(self) -> t.NoReturn: ...

    def nth_weekday_of_month(self) -> t.NoReturn: ...


IntoDateTimeDifference = (
        DateTimeDifference
        | Date
        | Time
        | DateTime
        | ZonedDateTime
        | tuple[JIFF_UNIT_STRING, Date]
        | tuple[JIFF_UNIT_STRING, Time]
        | tuple[JIFF_UNIT_STRING, DateTime]
        | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)


class DateTimeDifference:
    def __init__(
            self,
            date: DateTime,
            *,
            smallest: JIFF_UNIT_STRING | None = None,
            largest: JIFF_UNIT_STRING | None = None,
            mode: JIFF_ROUND_MODE_STRING | None = None,
            increment: int | None = None,
    ) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def smallest(self, unit: JIFF_UNIT_STRING) -> DateTimeDifference: ...

    def largest(self, unit: JIFF_UNIT_STRING) -> DateTimeDifference: ...

    def mode(self, mode: JIFF_ROUND_MODE_STRING) -> DateTimeDifference: ...

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

    def to_offset(self, timestamp: Timestamp) -> tuple[Offset, bool, str]: ...

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

    def __init__(self, secs: int, nanos: int) -> None: ...

    # =========================================================================
    # OPERATORS/DUNDERS
    # =========================================================================
    def __hash__(self) -> int: ...

    def __mul__(self, other: int) -> SignedDuration: ...

    def __eq__(self, other: object) -> bool: ...

    def __ne__(self, other: object) -> bool: ...

    def __lt__(self, other: object) -> bool: ...

    def __le__(self, other: object) -> bool: ...

    def __gt__(self, other: object) -> bool: ...

    def __ge__(self, other: object) -> bool: ...

    def __neg__(self) -> t.Self: ...

    def __add__(self, other: t.Self) -> SignedDuration: ...

    def __abs__(self) -> t.Self: ...

    def __div__(self, other: int) -> SignedDuration: ...

    def abs(self) -> t.Self: ...

    def unsigned_abs(self) -> Duration: ...

    def __richcmp__(
            self, other: SignedDuration | pydt.timedelta, op: int
    ) -> bool: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...

    def string(self) -> str: ...

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
_TimeSpanArithmeticSingle = "TimeSpan" | Duration | SignedDuration
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
    def string(self) -> str: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def repr_full(self) -> str: ...

    # =========================================================================
    # PYTHON CONVERSIONS
    # =========================================================================
    @classmethod
    def from_pytimedelta(cls: type[t.Self], td: pydt.timedelta) -> t.Self: ...

    def to_pytimedelta(self) -> pydt.timedelta: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def parse(cls: type[t.Self], s: str) -> t.Self: ...

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
    ) -> t.Self: ...

    def __sub__(
            self,
            val: TimeSpanArithmetic,
    ) -> t.Self: ...

    def __mul__(self, other: int) -> t.Self: ...

    def __neg__(self) -> t.Self: ...

    def __abs__(self) -> t.Self: ...

    def __invert__(self) -> t.Self: ...

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

    def abs(self) -> t.Self: ...

    def asdict(self) -> TimeSpanTypedDict: ...

    def checked_add(self, val: TimeSpanArithmetic) -> t.Self: ...

    def checked_mul(self, other: int) -> t.Self: ...

    def checked_sub(self, val: TimeSpanArithmetic) -> t.Self: ...

    def compare(self, other: TimeSpan) -> int: ...

    def negate(self) -> t.Self: ...

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
    ) -> t.Self: ...

    def round(self, options: JIFF_UNIT_STRING) -> t.Self: ...

    def signum(self) -> t.Literal[-1, 0, 1]: ...

    def to_signed_duration(
            self, relative: ZonedDateTime | Date | DateTime
    ) -> SignedDuration: ...

    def total(self) -> int: ...

    def total_seconds(self) -> int: ...

    def try_years(self, years: int) -> t.Self: ...

    def try_months(self, months: int) -> t.Self: ...

    def try_weeks(self, weeks: int) -> t.Self: ...

    def try_days(self, days: int) -> t.Self: ...

    def try_hours(self, hours: int) -> t.Self: ...

    def try_minutes(self, minutes: int) -> t.Self: ...

    def try_seconds(self, seconds: int) -> t.Self: ...

    def try_milliseconds(self, milliseconds: int) -> t.Self: ...

    def try_microseconds(self, microseconds: int) -> t.Self: ...

    def try_nanoseconds(self, nanoseconds: int) -> t.Self: ...

    # -------------------------------------------------------------------------
    # PANIC-INDUCING METHODS
    # -------------------------------------------------------------------------
    def _years(self, years: int) -> t.Self: ...

    def _months(self, months: int) -> t.Self: ...

    def _weeks(self, weeks: int) -> t.Self: ...

    def _days(self, days: int) -> t.Self: ...

    def _hours(self, hours: int) -> t.Self: ...

    def _minutes(self, minutes: int) -> t.Self: ...

    def _seconds(self, seconds: int) -> t.Self: ...

    def _milliseconds(self, milliseconds: int) -> t.Self: ...

    def _microseconds(self, microseconds: int) -> t.Self: ...

    def _nanoseconds(self, nanoseconds: int) -> t.Self: ...


class Timestamp:
    """
    A representation of a timestamp with second and nanosecond precision.
    """

    def __init__(
            self, second: int | None = None, nanosecond: int | None = None
    ) -> None: ...

    # =========================================================================
    # CLASS METHODS
    # =========================================================================
    @classmethod
    def now(cls: type[t.Self]) -> t.Self: ...

    @classmethod
    def parse(cls: type[t.Self], s: str) -> t.Self: ...

    @classmethod
    def from_millisecond(cls: type[t.Self], millisecond: int) -> t.Self: ...

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
    ) -> t.Self: ...

    def __eq__(self, other: object) -> bool: ...

    def __ge__(self, other: Timestamp) -> bool: ...

    def __gt__(self, other: Timestamp) -> bool: ...

    def __iadd__(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

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
    ) -> t.Self: ...

    @t.overload
    def __sub__(self, other: Timestamp) -> TimeSpan: ...

    @t.overload
    def __sub__(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def series(self, span: TimeSpan) -> t.Iterator[Timestamp]: ...

    def to_zoned(self, time_zone: TimeZone) -> ZonedDateTime: ...

    def string(self) -> str: ...

    def as_second(self) -> int: ...

    def as_microsecond(self) -> int: ...

    def as_millisecond(self) -> int: ...

    def as_nanosecond(self) -> int: ...

    # =========================================================================
    # SINCE/UNTIL
    # =========================================================================
    def _since(self, other: TimestampDifference) -> TimeSpan: ...

    def _until(self, other: TimestampDifference) -> TimeSpan: ...

    def since(self, other: IntoTimestampDifference) -> TimeSpan: ...

    def until(self, other: IntoTimestampDifference) -> TimeSpan: ...


IntoTimestampDifference = (
        TimestampDifference
        | Timestamp
        | ZonedDateTime
        | tuple[JIFF_UNIT_STRING, Timestamp]
        | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)


class TimestampDifference:
    def __init__(
            self,
            date: Timestamp,
            *,
            smallest: JIFF_UNIT_STRING | None = None,
            largest: JIFF_UNIT_STRING | None = None,
            mode: JIFF_ROUND_MODE_STRING | None = None,
            increment: int | None = None,
    ) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def smallest(self, unit: JIFF_UNIT_STRING) -> TimestampDifference: ...

    def largest(self, unit: JIFF_UNIT_STRING) -> TimestampDifference: ...

    def mode(self, mode: JIFF_ROUND_MODE_STRING) -> TimestampDifference: ...

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
    ) -> t.Self: ...

    def __eq__(self, other: object) -> bool: ...

    def __ge__(self, other: ZonedDateTime) -> bool: ...

    def __gt__(self, other: ZonedDateTime) -> bool: ...

    def __hash__(self) -> int: ...

    def __iadd__(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

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
    ) -> t.Self: ...

    @t.overload
    def __sub__(self, other: ZonedDateTime) -> TimeSpan: ...

    @t.overload
    def __sub__(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def astimezone(self, tz: str) -> ZonedDateTime: ...

    def checked_add(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

    @t.overload
    def checked_sub(self, other: ZonedDateTime) -> TimeSpan: ...

    @t.overload
    def checked_sub(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

    def date(self) -> Date: ...

    def datetime(self) -> DateTime: ...

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

    def intz(self, tz: str) -> t.Self: ...

    def inutc(self) -> ZonedDateTime: ...

    def last_of_month(self) -> ZonedDateTime: ...

    def last_of_year(self) -> ZonedDateTime: ...

    def offset(self) -> Offset: ...

    def round(self, options: JIFF_UNIT_STRING | DateTimeRound) -> t.Self: ...

    def saturating_add(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

    @t.overload
    def saturating_sub(self, other: ZonedDateTime) -> TimeSpan: ...

    @t.overload
    def saturating_sub(
            self, other: TimeSpan | SignedDuration | Duration
    ) -> t.Self: ...

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
            self, other: ZonedDateTime | tuple[JIFF_UNIT_STRING, ZonedDateTime]
    ) -> TimeSpan: ...

    def until(
            self, other: ZonedDateTime | tuple[JIFF_UNIT_STRING, ZonedDateTime]
    ) -> TimeSpan: ...

    # =========================================================================
    # NOT IMPLEMENTED & NOT TYPED
    # =========================================================================
    def nth_weekday(self) -> t.NoReturn: ...

    def nth_weekday_of_month(self) -> t.NoReturn: ...


class DateTimeRound:
    def __init__(
            self,
            smallest: JIFF_UNIT_STRING | None = None,
            mode: JIFF_ROUND_MODE_STRING | None = None,
            increment: int = 1,
    ) -> None: ...

    def __str__(self) -> str: ...

    def __repr__(self) -> str: ...

    def __eq__(self, other: object) -> bool: ...

    def mode(self, mode: JIFF_ROUND_MODE_STRING) -> DateTimeRound: ...

    def smallest(self, smallest: JIFF_UNIT_STRING) -> DateTimeRound: ...

    def increment(self, increment: int) -> DateTimeRound: ...

    def _smallest(self) -> JIFF_UNIT_STRING: ...

    def _mode(self) -> JIFF_ROUND_MODE_STRING: ...

    def _increment(self) -> int: ...

    def replace(
            self,
            smallest: JIFF_UNIT_STRING | None,
            mode: JIFF_ROUND_MODE_STRING | None,
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
## `ry.JSON`

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
def parse_json_str(
    data: str,
    /,
    *,
    allow_inf_nan: bool = True,
    cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    float_mode: Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...
def jiter_cache_clear() -> None: ...
def jiter_cache_usage() -> int: ...

```
## `ry._dev`

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
# DEV
# =============================================================================


def anystr_noop(s: t.AnyStr) -> t.AnyStr: ...
def string_noop(s: str) -> str: ...
def bytes_noop(s: bytes) -> bytes: ...

```
## `ry.xxhash`

```python
from __future__ import annotations

import typing as t


@t.final
class Xxh32:
    name: t.Literal["xxh32"]

    def __init__(self, input: bytes = ..., seed: int | None = ...) -> None: ...
    def update(self, input: bytes) -> None: ...
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

    def __init__(self, input: bytes = ..., seed: int | None = ...) -> None: ...
    def update(self, input: bytes) -> None: ...
    def digest(self) -> bytes: ...
    def hexdigest(self) -> str: ...
    def intdigest(self) -> int: ...
    def copy(self) -> Xxh32: ...
    def reset(self, seed: int | None = ...) -> None: ...
    @property
    def seed(self) -> int: ...


@t.final
class Xxh3:
    name: t.Literal["xxh3"]

    def __init__(
        self,
        input: bytes = ...,
        seed: int | None = ...,
        secret: bytes | None = ...,
    ) -> None: ...
    def update(self, input: bytes) -> None: ...
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


def xxh32(input: bytes | None = None, seed: int | None = None) -> Xxh32: ...
def xxh64(input: bytes | None = None, seed: int | None = None) -> Xxh64: ...
def xxh3(
    input: bytes | None = None,
    seed: int | None = None,
    secret: bytes | None = None,
) -> Xxh3: ...


# xxh32
def xxh32_digest(input: bytes, seed: int | None = None) -> bytes: ...
def xxh32_hexdigest(input: bytes, seed: int | None = None) -> str: ...
def xxh32_intdigest(input: bytes, seed: int | None = None) -> int: ...


# xxh64
def xxh64_digest(input: bytes, seed: int | None = None) -> bytes: ...
def xxh64_hexdigest(input: bytes, seed: int | None = None) -> str: ...
def xxh64_intdigest(input: bytes, seed: int | None = None) -> int: ...


# xxh128
def xxh128_digest(input: bytes, seed: int | None = None) -> bytes: ...
def xxh128_hexdigest(input: bytes, seed: int | None = None) -> str: ...
def xxh128_intdigest(input: bytes, seed: int | None = None) -> int: ...


# xxh3
def xxh3_64_digest(input: bytes, seed: int | None = None) -> bytes: ...
def xxh3_64_intdigest(input: bytes, seed: int | None = None) -> int: ...
def xxh3_64_hexdigest(input: bytes, seed: int | None = None) -> str: ...
def xxh3_digest(input: bytes, seed: int | None = None) -> bytes: ...
def xxh3_intdigest(input: bytes, seed: int | None = None) -> int: ...
def xxh3_hexdigest(input: bytes, seed: int | None = None) -> str: ...


# xxh128
def xxh3_128_digest(input: bytes, seed: int | None = None) -> bytes: ...
def xxh3_128_intdigest(input: bytes, seed: int | None = None) -> int: ...
def xxh3_128_hexdigest(input: bytes, seed: int | None = None) -> str: ...

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
