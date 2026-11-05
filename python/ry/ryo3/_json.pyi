"""ryo3-json ~ types"""

import sys
import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes
from ry.ryo3._jiter import JSONDecodeError as _JSONDecodeError
from ry.ryo3._jiter import _JsonValue

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

# `dtype=` output type ~ `ry.Bytes` (default) or `bytes`
@t.overload
def stringify(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    dtype: type[Bytes] = ...,
) -> Bytes: ...
@t.overload
def stringify(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    dtype: type[bytes],
) -> bytes: ...
@t.overload
@deprecated("`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]")
def stringify(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
@deprecated("`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]")
def stringify(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[False],
) -> Bytes: ...

@t.final
class JSON:
    """`ry.JSON` namespace (not instantiable)"""

    JSONDecodeError = _JSONDecodeError

    @staticmethod
    def minify(buf: Buffer | str, /, append_newline: bool = False) -> Bytes:
        """Return minified json data (remove whitespace, newlines)

        Parameters
        ----------
        buf : Buffer | str
            JSON buffer/string to minify

        Returns
        -------
        Bytes
            Minified JSON data as a `Bytes` object.

        Examples
        --------
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

    @staticmethod
    def fmt(buf: Buffer | str, /, append_newline: bool = False) -> Bytes:
        r"""Return formatted json data (add indentation, newlines)

        Parameters
        ----------
        buf : Buffer | str
            JSON buffer/string to format

        Returns
        -------
        Bytes
            Formatted JSON data as a `Bytes` object.

        Examples
        --------
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
    @staticmethod
    def stringify(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        dtype: type[Bytes] = ...,
    ) -> Bytes: ...
    @t.overload
    @staticmethod
    def stringify(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        dtype: type[bytes],
    ) -> bytes: ...
    @t.overload
    @deprecated(
        "`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]"
    )
    @staticmethod
    def stringify(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        pybytes: t.Literal[True],
    ) -> bytes: ...
    @t.overload
    @deprecated(
        "`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]"
    )
    @staticmethod
    def stringify(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        pybytes: t.Literal[False],
    ) -> Bytes: ...
    @t.overload
    @staticmethod
    def dumps(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        dtype: type[Bytes] = ...,
    ) -> Bytes: ...
    @t.overload
    @staticmethod
    def dumps(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        dtype: type[bytes],
    ) -> bytes: ...
    @t.overload
    @deprecated(
        "`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]"
    )
    @staticmethod
    def dumps(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        pybytes: t.Literal[True],
    ) -> bytes: ...
    @t.overload
    @deprecated(
        "`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]"
    )
    @staticmethod
    def dumps(
        obj: t.Any,
        *,
        default: t.Callable[[t.Any], t.Any] | None = None,
        fmt: bool = False,
        sort_keys: bool = False,
        append_newline: bool = False,
        pybytes: t.Literal[False],
    ) -> Bytes: ...
    @staticmethod
    def loads(
        data: Buffer | bytes | str,
        *,
        allow_inf_nan: bool = False,
        cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
        partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
        catch_duplicate_keys: bool = False,
    ) -> _JsonValue: ...
    @staticmethod
    def parse(
        data: Buffer | bytes | str,
        *,
        allow_inf_nan: bool = False,
        cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
        partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
        catch_duplicate_keys: bool = False,
    ) -> _JsonValue: ...
    @staticmethod
    def cache_clear() -> None: ...
    @staticmethod
    def cache_usage() -> int: ...
