import datetime as pydt
import typing as t

__all__ = (
    "FromStr",
    "NoInit",
    "RyIterator",
    "Strftime",
    "ToPy",
    "ToPyDate",
    "ToPyDateTime",
    "ToPyTime",
    "ToPyTimeDelta",
    "ToPyTzInfo",
    "ToString",
    "_Parse",
)

_T = t.TypeVar("_T")
_T_co = t.TypeVar("_T_co", covariant=True)


class ToPy(t.Protocol[_T_co]):
    """Objects that can be converted to a python stdlib type (`_T_co`) via `obj.to_py()`."""

    def to_py(self) -> _T_co: ...


class NoInit(t.Protocol):
    """Protocol for types that cannot be instantiated directly."""

    def __init__(self) -> t.NoReturn: ...


# =============================================================================
# TO/FROM STRING
# =============================================================================
class FromStr(t.Protocol):
    """Protocol for types that have a `.from_str()` class method."""

    @classmethod
    def from_str(cls, s: str) -> t.Self: ...


class _Parse(t.Protocol):
    """Protocol for types that have a `.parse()` class method."""

    @classmethod
    def parse(cls, s: str | bytes) -> t.Self: ...


class ToString(t.Protocol):
    """Protocol for types that have a `.to_string()` method."""

    def to_string(self) -> str: ...


# =============================================================================
# ITERABLE
# =============================================================================
class RyIterator(t.Protocol[_T]):
    def __iter__(self) -> t.Self: ...
    def __next__(self) -> _T: ...
    def collect(self) -> list[_T]: ...
    def take(self, n: int = 1) -> list[_T]: ...


class RyAsyncIterator(t.Protocol[_T]):
    def __aiter__(self) -> t.Self: ...
    async def __anext__(self) -> _T: ...
    async def collect(self) -> list[_T]: ...
    async def take(self, n: int = 1) -> list[_T]: ...


# =============================================================================
# DATETIME
# =============================================================================


class Strftime(t.Protocol):
    """Protocol for types that have a `.strftime()` method."""

    def strftime(self, fmt: str) -> str: ...


class ToPyDate(t.Protocol):
    """Objects that can be converted to a Python `datetime.date`."""

    def to_pydate(self) -> pydt.date: ...


class ToPyTime(t.Protocol):
    """Objects that can be converted to a Python `datetime.time`."""

    def to_pytime(self) -> pydt.time: ...


class ToPyDateTime(t.Protocol):
    """Objects that can be converted to a Python `datetime.datetime`."""

    def to_pydatetime(self) -> pydt.datetime: ...


class ToPyTimeDelta(t.Protocol):
    """Objects that can be converted to a Python `datetime.timedelta`."""

    def to_pytimedelta(self) -> pydt.timedelta: ...


class ToPyTzInfo(t.Protocol):
    """Objects that can be converted to a Python `datetime.tzinfo`."""

    def to_pytzinfo(self) -> pydt.tzinfo: ...
