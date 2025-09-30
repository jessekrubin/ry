from __future__ import annotations

import dataclasses
import itertools as it
import pickle
import typing as t
from pathlib import Path

import pytest

import ry


def test_pattern_type() -> None:
    """Test glob pattern type"""
    pattern = ry.Pattern("*.py")
    assert isinstance(pattern, ry.Pattern)

    python_file = "file.py"
    text_file = "file.txt"

    assert pattern.pattern == "*.py"

    assert pattern.matches(python_file)
    assert not pattern.matches(text_file)

    assert pattern.matches_path(Path(python_file))
    assert not pattern.matches_path(Path(text_file))

    assert pattern.matches_path(ry.FsPath(python_file))
    assert not pattern.matches_path(ry.FsPath(text_file))

    # call
    assert pattern(python_file)
    assert not pattern(text_file)

    # case sensitive
    assert not pattern.matches("FILE.PY")
    assert not pattern(Path("FILE.PY"))
    assert not pattern(ry.FsPath("FILE.PY"))


def test_escape() -> None:
    """Test glob pattern escape"""
    s = "_[_]_?_*_!_"
    assert ry.Pattern.escape(s) == "_[[]_[]]_[?]_[*]_!_"
    assert ry.Pattern(ry.Pattern.escape(s)).matches(s)


class _MatchKwargs(t.TypedDict, total=False):
    case_sensitive: bool
    require_literal_separator: bool
    require_literal_leading_dot: bool


class _PatternTestCaseDict(t.TypedDict):
    pattern: str
    ob: str | Path | ry.FsPath
    matches: bool
    kwargs: _MatchKwargs | None


@dataclasses.dataclass
class _PatternTestCase:
    pattern: str
    path_str: str
    matches: bool
    kwargs: _MatchKwargs | None = None

    def _dtype_cases(self) -> list[_PatternTestCaseDict]:
        return [
            {
                "pattern": self.pattern,
                "ob": self.path_str,
                "matches": self.matches,
                "kwargs": self.kwargs,
            },
            {
                "pattern": self.pattern,
                "ob": Path(self.path_str),
                "matches": self.matches,
                "kwargs": self.kwargs,
            },
            {
                "pattern": self.pattern,
                "ob": ry.FsPath(self.path_str),
                "matches": self.matches,
                "kwargs": self.kwargs,
            },
        ]


_TEST_CASES_RAW = [
    _PatternTestCase(pattern="*.py", path_str="file.py", matches=True),
    _PatternTestCase(pattern="*.py", path_str="file.txt", matches=False),
    _PatternTestCase(pattern="*.py", path_str="FILE.PY", matches=False),
    _PatternTestCase(
        pattern="*.PY",
        path_str="FILE.PY",
        matches=True,
        kwargs={"case_sensitive": False},
    ),
    _PatternTestCase(pattern="data?.csv", path_str="data1.csv", matches=True),
    _PatternTestCase(pattern="data?.csv", path_str="data12.csv", matches=False),
]
_TEST_CASES = list(it.chain.from_iterable(tc._dtype_cases() for tc in _TEST_CASES_RAW))


@pytest.mark.parametrize(
    "case",
    _TEST_CASES,
)
def test_pattern(case: _PatternTestCaseDict) -> None:
    """Test glob pattern with various dtypes"""
    pattern = ry.Pattern(case["pattern"])

    if case["kwargs"] is None:
        assert pattern(case["ob"]) == case["matches"]
    else:
        assert pattern(case["ob"], **case["kwargs"]) == case["matches"]

    if isinstance(case["ob"], str):
        if case["kwargs"] is None:
            assert pattern.matches(case["ob"]) == case["matches"]
        else:
            assert (
                pattern.matches_with(case["ob"], **(case["kwargs"] or {}))
                == case["matches"]
            )
    elif isinstance(case["ob"], (Path, ry.FsPath)):
        if case["kwargs"] is None:
            assert pattern.matches_path(case["ob"]) == case["matches"]
        else:
            assert (
                pattern.matches_path_with(case["ob"], **(case["kwargs"] or {}))
                == case["matches"]
            )


def _pattern_kwargs_permutations() -> t.Generator[_MatchKwargs | None, None, None]:
    kw_opts = [True, False, None]

    for case in it.product(kw_opts, repeat=3):
        case_dict: _MatchKwargs = {}
        if case[0] is not None:
            case_dict["case_sensitive"] = case[0]
        if case[1] is not None:
            case_dict["require_literal_separator"] = case[1]
        if case[2] is not None:
            case_dict["require_literal_leading_dot"] = case[2]
        yield case_dict if case_dict else None


@pytest.mark.parametrize(
    "pattern_kwargs",
    _pattern_kwargs_permutations(),
)
def test_pattern_kwargs(pattern_kwargs: _MatchKwargs | None) -> None:
    """Test glob pattern with various kwargs"""
    p = ry.Pattern("*.py", **(pattern_kwargs or {}))

    repr_str = repr(p)
    # evaluta
    from_eval = eval("ry." + repr_str)
    assert from_eval == p

    assert hash(from_eval) == hash(p)
    assert not (from_eval != p)


@pytest.mark.parametrize(
    "pattern_kwargs",
    _pattern_kwargs_permutations(),
)
def test_pattern_pickle(pattern_kwargs: _MatchKwargs | None) -> None:
    """Test glob pattern with various kwargs"""

    p = ry.Pattern("*.py", **(pattern_kwargs or {}))

    pickled = pickle.dumps(p)
    unpickled = pickle.loads(pickled)
    assert unpickled == p
    assert not (unpickled != p)
    assert repr(unpickled) == repr(p)
    assert hash(unpickled) == hash(p)
