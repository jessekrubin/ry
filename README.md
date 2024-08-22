# ry

[![PyPI](https://img.shields.io/pypi/v/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Wheel](https://img.shields.io/pypi/wheel/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Status](https://img.shields.io/pypi/status/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - License](https://img.shields.io/pypi/l/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)

python bindings for rust crates I wish existed in python

**THIS IS A WORK IN PROGRESS**

## Install

```bash
pip install ry
poetry add ry
pdm add ry
rye add ry
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
  - **A:** I (jesse) needed several hashing functions for python and then kept adding things as I needed them
- **Q:** Does this have anything to do with the (excellent) package manager `rye`?
  - **A:** short answer: no. long answer: no, it does not.
- **Q:** Why is the repo split into `ry` and `ryo3`?
  - **A:** `ry` is the python package, `ryo3` is a rust crate setup to let you "register" functions you may want if you
    were writing your own pyo3-python bindings library; maybe someday the `ryo3::libs` module will be split up into
    separate packages

## Crate bindings

- `brotli`
- `bzip2`
- `flate2`
- `fnv`
- `shlex`
- `walkdir`
- `which`
- `xxhash`
- `zstd`
- TBD:
  - `subprocess.redo` (subprocesses that are lessy finicky and support tee-ing)
  - `globset` (technically done, but not yet in `ry` -- [globsters](https://pypi.org/project/globsters/))
  - `regex`
  - `tokio` (fs + process)
  - `tracing` (could be nicer than python's awful logging lib -- currently a part of ry/ryo3 for my dev purposes)
  - `reqwest` (async http client / waiting on pyo3 asyncio to stablize and for me to have more time)

## API

```python
"""ry api"""

from collections.abc import Iterator
from os import PathLike
from typing import Any, AnyStr, Literal, final

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
  JsonPrimitive
  | dict[str, JsonPrimitive | JsonValue]
  | list[JsonPrimitive | JsonValue]
)


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

  def read_text(self) -> str: ...

  def read_bytes(self) -> bytes: ...

  def absolute(self) -> FsPath: ...

  @property
  def parent(self) -> FsPath: ...


FsPathLike = str | FsPath | PathLike[str]


def pwd() -> str: ...


def home() -> str: ...


def cd(path: FsPathLike) -> None: ...


def ls(path: FsPathLike | None = None) -> list[FsPath]: ...


def quick_maths() -> Literal[3]:
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
) -> Any: ...


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
  cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
  partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
  catch_duplicate_keys: bool = False,
  lossless_floats: bool = False,
) -> JsonValue: ...


def parse_json_bytes(
  data: bytes,
  /,
  *,
  allow_inf_nan: bool = True,
  cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
  partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
  catch_duplicate_keys: bool = False,
  lossless_floats: bool = False,
) -> JsonValue: ...


def parse_json_str(
  data: str,
  /,
  *,
  allow_inf_nan: bool = True,
  cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
  partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
  catch_duplicate_keys: bool = False,
  lossless_floats: bool = False,
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
def anystr_noop(s: AnyStr) -> AnyStr: ...


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
@final
class Xxh32:
  def __init__(self, input: bytes = ..., seed: int | None = ...) -> None: ...

  def update(self, input: bytes) -> None: ...

  def digest(self) -> bytes: ...

  def hexdigest(self) -> str: ...

  def intdigest(self) -> int: ...

  def copy(self) -> Xxh32: ...

  def reset(self, seed: int | None = ...) -> None: ...

  @property
  def name(self) -> str: ...

  @property
  def seed(self) -> int: ...


@final
class Xxh64:
  def __init__(self, input: bytes = ..., seed: int | None = ...) -> None: ...

  def update(self, input: bytes) -> None: ...

  def digest(self) -> bytes: ...

  def hexdigest(self) -> str: ...

  def intdigest(self) -> int: ...

  def copy(self) -> Xxh32: ...

  def reset(self, seed: int | None = ...) -> None: ...

  @property
  def name(self) -> str: ...

  @property
  def seed(self) -> int: ...


@final
class Xxh3:
  def __init__(
    self, input: bytes = ..., seed: int | None = ..., secret: bytes | None = ...
  ) -> None: ...

  def update(self, input: bytes) -> None: ...

  def digest(self) -> bytes: ...

  def hexdigest(self) -> str: ...

  def intdigest(self) -> int: ...

  @property
  def name(self) -> str: ...

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

```

## DEV

- `just` is used to run tasks
- Do not use the phrase `blazing fast` or any emojis in any PRs or issues or docs
- type annotations are required
- `ruff` used for formatting and linting

## SEE ALSO

- utiles (web-map tile utils): https://github.com/jessekrubin/utiles
- jsonc2json (jsonc to json converter): https://github.com/jessekrubin/jsonc2json
