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
