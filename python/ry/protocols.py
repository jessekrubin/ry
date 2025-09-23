import datetime as pydt
import typing as t

__all__ = (
    "FromStr",
    "ToPyDate",
    "ToPyDateTime",
    "ToPyTime",
    "ToPyTimeDelta",
    "ToPyTzInfo",
    "ToString",
)

T_co = t.TypeVar("T_co", covariant=True)


class FromStr(t.Protocol):
    """Protocol for types that have a `.from_str()` class method."""

    @classmethod
    def from_str(cls, s: str) -> t.Self: ...


class ToString(t.Protocol):
    """Protocol for types that have a `.to_string()` method."""

    def to_string(self) -> str: ...


class ToPy(t.Protocol[T_co]):
    """Objects that can be converted to a python stdlib type (`T_co`) via `obj.to_py()`."""

    def to_py(self) -> T_co: ...


class ToPyDate(t.Protocol):
    """Objects that can be converted to a Python `datetime.date`."""

    def to_pydate(self) -> pydt.date: ...


class ToPyTime(t.Protocol):
    """Objects that can be converted to a Python `datetime.time`."""

    def to_pytime(self) -> pydt.time: ...


class ToPyDateTime(t.Protocol):
    def to_pydatetime(self) -> pydt.datetime: ...


class ToPyTimeDelta(t.Protocol):
    def to_pytimedelta(self) -> pydt.timedelta: ...


class ToPyTzInfo(t.Protocol):
    def to_pytzinfo(self) -> pydt.tzinfo: ...
