# ry

ry = rust + python (quick-maths!)

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

- `shlex`
- `jiter`
- `which`
- `sqlformat`
- compression:
  - `brotli`
  - `bzip2`
  - `flat2`
  - `zstd`
- hashing:
  - `fnv`
  - `xxhash`
- burnt-sushi:
  - `walkdir`
  - `globset` (formerly [globsters](https://pypi.org/project/globsters/))
  - `jiff`

### FUTURE?

- `subprocess.redo` (subprocesses that are lessy finicky and support tee-ing)
- `regex`
- `tokio` (fs + process)
- `tracing` (could be nicer than python's awful logging lib -- currently a
  part of ry/ryo3 for my dev purposes - currently has impl thingy in utiles)
- `reqwest` (async http client / waiting on pyo3 asyncio to stabilize and for
  me to have more time)

## API

<!-- API-START -->

```python
"""ry api ~ type annotations"""

import datetime as pydatetime
import typing as t
from collections.abc import Iterator
from os import PathLike

from ry._types.jiff import (
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

# ==============================================================================
# TYPE ALIASES
# ==============================================================================
JsonPrimitive = None | bool | int | float | str
JsonValue = (
  JsonPrimitive | dict[str, JsonPrimitive | JsonValue] | list[JsonPrimitive | JsonValue]
)


# ==============================================================================
# STD
# ==============================================================================


class Duration:
  def __init__(self, seconds: int, nanoseconds: int) -> None: ...

  def __eq__(self, other: object) -> bool: ...

  def __ne__(self, other: object) -> bool: ...

  def __lt__(self, other: object) -> bool: ...

  def __le__(self, other: object) -> bool: ...

  def __gt__(self, other: object) -> bool: ...

  def __ge__(self, other: object) -> bool: ...

  def __richcmp__(self, other: Duration | pydatetime.timedelta, op: int) -> bool: ...

  def __str__(self) -> str: ...

  @classmethod
  def from_pytimedelta(cls: type[Duration], td: pydatetime.timedelta) -> Duration: ...

  def to_pytimedelta(self) -> pydatetime.timedelta: ...

  @property
  def days(self) -> int: ...

  @property
  def seconds(self) -> int: ...

  @property
  def microseconds(self) -> int: ...


# ==============================================================================
# RY03-CORE
# ==============================================================================


class FsPath:
  def __init__(self, path: PathLike[str] | str | None = None) -> None: ...

  def __fspath__(self) -> str: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...

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

  @property
  def parent(self) -> FsPath: ...

  def write_text(self, data: str) -> None: ...

  def write_bytes(self, data: bytes) -> None: ...

  def joinpath(self, *paths: str) -> FsPath: ...

  def is_dir(self) -> bool: ...

  def is_file(self) -> bool: ...

  def exists(self) -> bool: ...

  def with_name(self, name: str) -> FsPath: ...

  def with_suffix(self, suffix: str) -> FsPath: ...

  @property
  def suffix(self) -> str: ...

  @property
  def suffixes(self) -> list[str]: ...

  def iterdir(self) -> Iterator[FsPath]: ...

  def relative_to(self, other: PathLike[str] | str | FsPath) -> FsPath: ...

  def as_posix(self) -> str: ...

  # TODO
  @property
  def parents(self) -> t.Sequence[t.Self]: ...

  @property
  def root(self) -> str: ...

  def __bytes__(self) -> bytes: ...

  def as_uri(self) -> str: ...

  @property
  def parts(self) -> tuple[str, ...]: ...

  @property
  def drive(self) -> str: ...

  @property
  def anchor(self) -> str: ...

  @property
  def name(self) -> str: ...

  @property
  def stem(self) -> str: ...


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
      >>> result = quick_maths()
      >>> assert result == 3

  NOTE: THIS IS FROM MY TEMPLATE RY03-MODULE
  """


# ==============================================================================
# SLEEP
# ==============================================================================
def sleep(seconds: float) -> float: ...


async def sleep_async(seconds: float) -> float: ...


# ==============================================================================
# FILESYSTEM
# ==============================================================================
def read_text(path: FsPathLike) -> str: ...


def read_bytes(path: FsPathLike) -> bytes: ...


def write_text(path: FsPathLike, data: str) -> None: ...


def write_bytes(path: FsPathLike, data: bytes) -> None: ...


# ==============================================================================
# SUBPROCESS (VERY MUCH WIP)
# ==============================================================================
def run(
  *args: str | list[str],
  capture_output: bool = True,
  input: bytes | None = None,
) -> t.Any: ...


# ==============================================================================
# DEV
# ==============================================================================


def string_noop(s: str) -> str: ...


def bytes_noop(s: bytes) -> bytes: ...


# ------------------------------------------------------------------------------
# \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
# /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
# ------------------------------------------------------------------------------
# ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~ LIBS ~
# ------------------------------------------------------------------------------
# \/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/
# /\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\
# ------------------------------------------------------------------------------


# ==============================================================================
# WHICH
# ==============================================================================
def which(cmd: str, path: None | str = None) -> str | None: ...


def which_all(cmd: str, path: None | str = None) -> list[str]: ...


def whicha(cmd: str, path: None | str = None) -> list[str]:
  """Alias for which_all (may go away in the future)"""


# ==============================================================================
# HECK
# ==============================================================================


def camel_case(string: str) -> str: ...


def kebab_case(string: str) -> str: ...


def pascal_case(string: str) -> str: ...


def shouty_kebab_case(string: str) -> str: ...


def shouty_snake_case(string: str) -> str: ...


def snake_case(string: str) -> str: ...


def snek_case(string: str) -> str: ...


def title_case(string: str) -> str: ...


def train_case(string: str) -> str: ...


# ==============================================================================
# GLOBSET
# ==============================================================================
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

  def is_match(self, path: str) -> bool: ...

  def __call__(self, path: str) -> bool: ...

  def __invert__(self) -> Glob: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...


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

  def is_match(self, path: str) -> bool: ...

  def __call__(self, path: str) -> bool: ...

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
  patterns: list[str],
  /,
  *,
  case_insensitive: bool | None = None,
  literal_separator: bool | None = None,
  backslash_escape: bool | None = None,
) -> Globster: ...


# ==============================================================================
# WALKDIR
# ==============================================================================


class WalkdirGen:
  """walkdir::Walkdir iterable wrapper"""

  files: bool
  dirs: bool

  def __next__(self) -> str: ...

  def __iter__(self) -> Iterator[str]: ...


class FspathsGen:
  """walkdir iterable that yields FsPath objects"""

  files: bool
  dirs: bool

  def __next__(self) -> FsPath: ...

  def __iter__(self) -> Iterator[FsPath]: ...


def walkdir(
  path: FsPathLike | None = None,
  files: bool = True,
  dirs: bool = True,
  contents_first: bool = False,
  min_depth: int = 0,
  max_depth: int | None = None,
  follow_links: bool = False,
  same_file_system: bool = False,
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


# ==============================================================================
# SHLEX
# ==============================================================================
def shplit(s: str) -> list[str]:
  """shlex::split wrapper much like python's stdlib shlex.split but faster"""
  ...


# ==============================================================================
# JSON
# ==============================================================================
def parse_json(
  data: bytes | str,
  /,
  *,
  allow_inf_nan: bool = True,
  cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
  partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
  catch_duplicate_keys: bool = False,
  float_mode: t.Literal["float", "decimal", "lossless-float"] = "float",
) -> JsonValue: ...


def parse_json_bytes(
  data: bytes,
  /,
  *,
  allow_inf_nan: bool = True,
  cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
  partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
  catch_duplicate_keys: bool = False,
  float_mode: t.Literal["float", "decimal", "lossless-float"] = "float",
) -> JsonValue: ...


def parse_json_str(
  data: str,
  /,
  *,
  allow_inf_nan: bool = True,
  cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
  partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
  catch_duplicate_keys: bool = False,
  float_mode: t.Literal["float", "decimal", "lossless-float"] = "float",
) -> JsonValue: ...


def jiter_cache_clear() -> None: ...


def jiter_cache_usage() -> int: ...


# ==============================================================================
# FORMATTING
# ==============================================================================
def fmt_nbytes(nbytes: int) -> str: ...


# ==============================================================================
# FNV
# ==============================================================================
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


# ==============================================================================
# DEV
# ==============================================================================
def anystr_noop(s: t.AnyStr) -> t.AnyStr: ...


# ==============================================================================
# BROTLI
# ==============================================================================
def brotli_encode(
  input: bytes, quality: int = 11, magic_number: bool = False
) -> bytes: ...


def brotli_decode(input: bytes) -> bytes: ...


def brotli(input: bytes, quality: int = 11, magic_number: bool = False) -> bytes:
  """Alias for brotli_encode"""


# ==============================================================================
# BZIP2
# ==============================================================================
def bzip2_encode(input: bytes, quality: int = 9) -> bytes: ...


def bzip2_decode(input: bytes) -> bytes: ...


def bzip2(input: bytes, quality: int = 9) -> bytes:
  """Alias for bzip2_encode"""


# ==============================================================================
# GZIP
# ==============================================================================
def gzip_encode(input: bytes, quality: int = 9) -> bytes: ...


def gzip_decode(input: bytes) -> bytes: ...


def gzip(input: bytes, quality: int = 9) -> bytes:
  """Alias for gzip_encode"""


def gunzip(input: bytes) -> bytes:
  """Alias for gzip_decode"""


# ==============================================================================
# ZSTD
# ==============================================================================
def zstd_encode(input: bytes, level: int = 3) -> bytes: ...


def zstd(input: bytes, level: int = 3) -> bytes:
  """Alias for zstd_encode"""


def zstd_decode(input: bytes) -> bytes: ...


# ==============================================================================
# XXHASH
# ==============================================================================
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
    self, input: bytes = ..., seed: int | None = ..., secret: bytes | None = ...
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
  input: bytes | None = None, seed: int | None = None, secret: bytes | None = None
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


# ==============================================================================
# SQLFORMAT
# ==============================================================================
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


# ==============================================================================
# JIFF
# ==============================================================================


class Date:
  MIN: Date
  MAX: Date
  ZERO: Date

  def __init__(self, year: int, month: int, day: int) -> None: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...

  def at(self, hour: int, minute: int, second: int, nanosecond: int) -> DateTime: ...

  @property
  def year(self) -> int: ...

  @property
  def month(self) -> int: ...

  @property
  def day(self) -> int: ...

  def to_pydate(self) -> pydatetime.date: ...

  @classmethod
  def from_pydate(cls: type[Date], date: pydatetime.date) -> Date: ...

  def astuple(self) -> tuple[int, int, int]: ...

  def asdict(self) -> DateTypedDict: ...

  def intz(self, tz: str) -> ZonedDateTime: ...

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


class Time:
  MIN: Time
  MAX: Time

  def __init__(
    self, hour: int = 0, minute: int = 0, second: int = 0, nanosecond: int = 0
  ) -> None: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...

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

  @classmethod
  def strptime(cls: type[Time], format: str, string: str) -> Time: ...

  def strftime(self, format: str) -> str: ...

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

  def to_pytime(self) -> pydatetime.time: ...

  @classmethod
  def from_pytime(cls: type[Time], time: pydatetime.time) -> Time: ...

  def astuple(self) -> tuple[int, int, int, int]: ...

  def asdict(self) -> TimeTypedDict: ...

  def series(self, span: TimeSpan) -> TimeSeries: ...

  def until(self, other: Time) -> TimeSpan: ...


class TimeSeries:
  def __iter__(self) -> t.Iterator[Time]: ...

  def __next__(self) -> Time: ...


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

  @classmethod
  def parse(cls: type[t.Self], s: str) -> t.Self: ...

  def __repr__(self) -> str: ...

  def __add__(self, other: TimeSpan | SignedDuration | Duration) -> DateTime: ...

  def __iadd__(self, other: TimeSpan | SignedDuration | Duration) -> DateTime: ...

  @t.overload
  def __sub__(self, other: DateTime) -> TimeSpan: ...

  @t.overload
  def __sub__(self, other: TimeSpan | SignedDuration | Duration) -> DateTime: ...

  @t.overload
  def __isub__(self, other: DateTime) -> TimeSpan: ...

  @t.overload
  def __isub__(self, other: TimeSpan | SignedDuration | Duration) -> DateTime: ...

  def intz(self, tz: str) -> ZonedDateTime: ...

  def date(self) -> Date: ...

  def time(self) -> Time: ...

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
  def nanosecond(self) -> int: ...

  @property
  def subsec_nanosecond(self) -> int: ...

  @classmethod
  def from_pydatetime(cls: type[DateTime], dt: pydatetime.datetime) -> DateTime: ...

  def to_pydatetime(self) -> pydatetime.datetime: ...

  def series(self, span: TimeSpan) -> DateTimeSeries: ...

  def asdict(self) -> DateTimeTypedDict: ...

  def round(self, options: JIFF_UNIT_STRING | DateTimeRound) -> t.Self: ...


class DateTimeSeries:
  def __iter__(self) -> t.Iterator[DateTime]: ...

  def __next__(self) -> DateTime: ...


class TimeZone:
  def __init__(self, name: str) -> None: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...

  def __eq__(self, other: object) -> bool: ...

  def to_pytzinfo(self) -> pydatetime.tzinfo: ...

  @classmethod
  def from_pytzinfo(cls: type[TimeZone], tz: pydatetime.tzinfo) -> TimeZone: ...

  @classmethod
  def utc(cls: type[TimeZone]) -> TimeZone: ...


class SignedDuration:
  MIN: SignedDuration
  MAX: SignedDuration
  ZERO: SignedDuration

  def __init__(self, secs: int, nanos: int) -> None: ...

  def __eq__(self, other: object) -> bool: ...

  def __ne__(self, other: object) -> bool: ...

  def __lt__(self, other: object) -> bool: ...

  def __le__(self, other: object) -> bool: ...

  def __gt__(self, other: object) -> bool: ...

  def __ge__(self, other: object) -> bool: ...

  def __neg__(self) -> t.Self: ...

  def __add__(self, other: t.Self) -> t.Self: ...

  def __richcmp__(
    self, other: SignedDuration | pydatetime.timedelta, op: int
  ) -> bool: ...

  def __str__(self) -> str: ...

  def string(self) -> str: ...

  def is_negative(self) -> bool: ...

  def is_zero(self) -> bool: ...

  @classmethod
  def from_pytimedelta(
    cls: type[SignedDuration], td: pydatetime.timedelta
  ) -> SignedDuration: ...

  def to_pytimedelta(self) -> pydatetime.timedelta: ...

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


class TimeSpan:
  def __init__(
    self,
  ) -> None: ...

  def __str__(self) -> str: ...

  def string(self) -> str: ...

  def __repr__(self) -> str: ...

  def __neg__(self) -> t.Self: ...

  def negate(self) -> t.Self: ...

  def __abs__(self) -> t.Self: ...

  def __invert__(self) -> t.Self: ...

  @classmethod
  def parse(cls: type[t.Self], s: str) -> t.Self: ...

  def years(self, years: int) -> t.Self: ...

  def months(self, months: int) -> t.Self: ...

  def weeks(self, weeks: int) -> t.Self: ...

  def days(self, days: int) -> t.Self: ...

  def hours(self, hours: int) -> t.Self: ...

  def minutes(self, minutes: int) -> t.Self: ...

  def seconds(self, seconds: int) -> t.Self: ...

  def to_jiff_duration(
    self, relative: ZonedDateTime | Date | DateTime
  ) -> SignedDuration: ...

  def repr_full(self) -> str: ...

  def asdict(self) -> TimeSpanTypedDict: ...

  def to_pytimedelta(self) -> pydatetime.timedelta: ...


class Timestamp:
  """
  A representation of a timestamp with second and nanosecond precision.
  """

  def __init__(
    self, second: int | None = None, nanosecond: int | None = None
  ) -> None: ...

  @classmethod
  def now(cls: type[Timestamp]) -> Timestamp: ...

  @classmethod
  def parse(cls: type[Timestamp], s: str) -> Timestamp: ...

  @classmethod
  def from_millisecond(cls: type[Timestamp], millisecond: int) -> Timestamp: ...

  def to_zoned(self, time_zone: TimeZone) -> ZonedDateTime: ...

  def string(self) -> str: ...

  def as_second(self) -> int: ...

  def as_microsecond(self) -> int: ...

  def as_millisecond(self) -> int: ...

  def as_nanosecond(self) -> int: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...

  def __eq__(self, other: object) -> bool: ...

  def __ne__(self, other: object) -> bool: ...

  def __lt__(self, other: object) -> bool: ...

  def __le__(self, other: object) -> bool: ...

  def __gt__(self, other: object) -> bool: ...

  def __ge__(self, other: object) -> bool: ...

  def __richcmp__(self, other: Timestamp, op: int) -> bool: ...

  def series(self, span: TimeSpan) -> TimestampSeries: ...

  def __add__(self, other: TimeSpan | SignedDuration | Duration) -> Timestamp: ...

  def __iadd__(self, other: TimeSpan | SignedDuration | Duration) -> Timestamp: ...

  @t.overload
  def __sub__(self, other: Timestamp) -> TimeSpan: ...

  @t.overload
  def __sub__(self, other: TimeSpan | SignedDuration | Duration) -> Timestamp: ...

  @t.overload
  def __isub__(self, other: Timestamp) -> TimeSpan: ...

  @t.overload
  def __isub__(self, other: TimeSpan | SignedDuration | Duration) -> Timestamp: ...


class TimestampSeries:
  def __iter__(self) -> t.Iterator[Timestamp]: ...

  def __next__(self) -> Timestamp: ...


class ZonedDateTime:
  def __init__(self, timestamp: Timestamp, time_zone: TimeZone) -> None: ...

  @classmethod
  def now(cls: type[ZonedDateTime]) -> ZonedDateTime: ...

  @classmethod
  def parse(cls: type[ZonedDateTime], s: str) -> ZonedDateTime: ...

  def __str__(self) -> str: ...

  def string(self) -> str: ...

  @classmethod
  def from_pydatetime(
    cls: type[ZonedDateTime], dt: pydatetime.datetime
  ) -> ZonedDateTime: ...

  def to_pydatetime(self) -> pydatetime.datetime: ...

  @classmethod
  def strptime(cls: type[ZonedDateTime], format: str, input: str) -> ZonedDateTime: ...

  def strftime(self, format: str) -> str: ...

  def __richcmp__(self, other: ZonedDateTime, op: int) -> bool: ...

  def __eq__(self, other: object) -> bool: ...

  def __ne__(self, other: object) -> bool: ...

  def __lt__(self, other: object) -> bool: ...

  def __le__(self, other: object) -> bool: ...

  def __gt__(self, other: object) -> bool: ...

  def __ge__(self, other: object) -> bool: ...

  def __hash__(self) -> int: ...

  def __add__(self, other: TimeSpan | SignedDuration | Duration) -> ZonedDateTime: ...

  def __iadd__(self, other: TimeSpan | SignedDuration | Duration) -> ZonedDateTime: ...

  @t.overload
  def __sub__(self, other: ZonedDateTime) -> TimeSpan: ...

  @t.overload
  def __sub__(self, other: TimeSpan | SignedDuration | Duration) -> ZonedDateTime: ...

  @t.overload
  def __isub__(self, other: ZonedDateTime) -> TimeSpan: ...

  @t.overload
  def __isub__(self, other: TimeSpan | SignedDuration | Duration) -> ZonedDateTime: ...

  def intz(self, tz: str) -> ZonedDateTime: ...

  def checked_add(self, span: TimeSpan) -> ZonedDateTime: ...

  def round(self, options: JIFF_UNIT_STRING | DateTimeRound) -> ZonedDateTime: ...

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
  def nanosecond(self) -> int: ...

  @property
  def subsec_nanosecond(self) -> int: ...

  def timezone(self) -> TimeZone: ...

  def timestamp(self) -> Timestamp: ...

  def time(self) -> Time: ...

  def datetime(self) -> DateTime: ...


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
  def __init__(self, hours: int) -> None: ...

  def __str__(self) -> str: ...

  def __repr__(self) -> str: ...


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
) -> TimeSpan: ...


def timespan_unchecked(
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

```

<!-- API-END -->

## DEV

- `just` is used to run tasks
- Do not use the phrase `blazing fast` or any emojis in any PRs or issues or
  docs
- type annotations are required
- `ruff` used for formatting and linting

## SEE ALSO

- utiles (web-map tile utils): https://github.com/jessekrubin/utiles
- jsonc2json (jsonc to json converter):
  https://github.com/jessekrubin/jsonc2json
