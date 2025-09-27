import datetime as pydt
import typing as t

__all__ = (
    "FromStr",
    "Strftime",
    "ToPy",
    "ToPyDate",
    "ToPyDateTime",
    "ToPyTime",
    "ToPyTimeDelta",
    "ToPyTzInfo",
    "ToString",
)

_T_co = t.TypeVar("_T_co", covariant=True)


class ToPy(t.Protocol[_T_co]):
    """Objects that can be converted to a python stdlib type (`_T_co`) via `obj.to_py()`."""

    def to_py(self) -> _T_co: ...


# =============================================================================
# TO/FROM STRING
# =============================================================================
class FromStr(t.Protocol):
    """Protocol for types that have a `.from_str()` class method."""

    @classmethod
    def from_str(cls, s: str) -> t.Self: ...


class ToString(t.Protocol):
    """Protocol for types that have a `.to_string()` method."""

    def to_string(self) -> str: ...


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
