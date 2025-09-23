from __future__ import annotations

import pickle
import typing as t

import pytest

import ry


def test_regex_repr_simple() -> None:
    re = ry.Regex(r"\w")
    assert repr(re) == r"Regex(r'\w')"


class RegexOptions(t.TypedDict, total=False):
    case_insensitive: bool
    crlf: bool
    dot_matches_new_line: bool
    ignore_whitespace: bool
    line_terminator: int | str | None
    multi_line: bool
    octal: bool
    size_limit: int | None
    swap_greed: bool
    unicode: bool


def _gen_kwargs_options() -> t.Generator[dict[str, t.Any], None, None]:
    bool_keys = [
        "case_insensitive",
        "crlf",
        "dot_matches_new_line",
        "ignore_whitespace",
        "multi_line",
        "octal",
        "swap_greed",
        "unicode",
    ]
    for bool_key in bool_keys:
        yield {bool_key: True}
        yield {bool_key: False}

    yield {"size_limit": 100000}
    yield {"size_limit": None}


_DEFAULT_OPTIONS = {
    "case_insensitive": False,
    "crlf": False,
    "dot_matches_new_line": False,
    "ignore_whitespace": False,
    "multi_line": False,
    "octal": False,
    "size_limit": None,
    "swap_greed": False,
    "unicode": True,
    "line_terminator": b"\n",
}


def _options_is_default(opts: dict[str, t.Any]) -> bool:
    for k, v in _DEFAULT_OPTIONS.items():
        if k in opts and opts[k] != v:
            return False
    return True


@pytest.mark.parametrize("kwargs", _gen_kwargs_options())
def test_regex_repr_with_options(kwargs: dict[str, t.Any]) -> None:
    re = ry.Regex(r"\w", **kwargs)
    repr_str = repr(re)

    if _options_is_default(kwargs):
        expected = r"Regex(r'\w')"
        assert repr_str == expected
        evaluated = eval(repr_str, {"Regex": ry.Regex})
        assert re == evaluated
    else:
        # Build expected kwargs string
        sorted_items = sorted(kwargs.items())
        kwargs_str = ", ".join(f"{k}={v!r}" for k, v in sorted_items)
        expected = f"Regex(r'\\w', {kwargs_str})"
        assert repr_str == expected
        evaluated = eval(repr_str, {"Regex": ry.Regex})
        assert re == evaluated


@pytest.mark.parametrize(
    "line_terminator",
    [
        None,
        *list(range(256)),
        *[bytes([i]) for i in range(256) if i != 10],
    ],
)
def test_regex_repr_line_terminator(
    line_terminator: None | int | bytes,
) -> None:
    re = ry.Regex(r"\w", line_terminator=line_terminator)
    repr_str = repr(re)
    line_terminator_pybytes = (
        bytes([line_terminator])
        if isinstance(line_terminator, int)
        else line_terminator
    )

    if (
        _options_is_default({"line_terminator": line_terminator_pybytes})
        or line_terminator is None
    ):
        expected = r"Regex(r'\w')"
        assert repr_str == expected
        evaluated = eval(repr_str, {"Regex": ry.Regex})
        assert re == evaluated
    else:
        # Build expected kwargs string
        kwargs_str = f"line_terminator={line_terminator_pybytes!r}"
        expected = f"Regex(r'\\w', {kwargs_str})"
        assert repr_str == expected
        evaluated = eval(repr_str, {"Regex": ry.Regex})
        assert re == evaluated


@pytest.mark.parametrize("kwargs", _gen_kwargs_options())
def test_regex_pickling(kwargs: dict[str, t.Any]) -> None:
    re = ry.Regex(r"\w", **kwargs)
    pickled = pickle.dumps(re)
    unpickled = pickle.loads(pickled)
    assert re == unpickled
