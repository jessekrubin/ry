"""orjson + ry types

orjson-types: https://github.com/ijl/orjson/blob/master/pysrc/orjson/__init__.pyi
"""

import typing as t

import orjson

def orjson_default(obj: t.Any) -> orjson.Fragment:
    """Fn to be used with `orjson.dumps` to serialize ry-compatible types

    Example:
        >>> import orjson
        >>> from ry import orjson_default, Date
        >>> data = {"key": "value", "date": Date(2023, 10, 1)}
        >>> orjson.dumps(data, default=orjson_default)
        b'{"key":"value","date":"2023-10-01"}'

    """
