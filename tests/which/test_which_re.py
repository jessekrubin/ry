from __future__ import annotations

import os
from typing import TYPE_CHECKING

import ry

from .which_fixtures import _mk_test_bin_dirs

if TYPE_CHECKING:
    from pathlib import Path


def test_which_regex_existing_exe(tmp_path: Path) -> None:
    """
    Test `which_re` with an existing executable using a regex pattern.
    """
    test_bin_dirs = _mk_test_bin_dirs(tmp_path)

    # Modify PATH to include test_bin_dirs
    original_path = os.environ["PATH"]
    search_path = os.pathsep.join(test_bin_dirs) + os.pathsep + original_path
    regex = ry.Regex(r"^uwot.*")
    result = ry.which_re(regex, search_path)
    assert isinstance(result, list)
    assert result is not None
    assert len(result) > 0, "Expected a match for executables starting with 'uwot'"


def test_which_regex_existing_exe_accepts_string(tmp_path: Path) -> None:
    """
    Test `which_re` with an existing executable using a regex pattern.
    """
    test_bin_dirs = _mk_test_bin_dirs(tmp_path)

    # Modify PATH to include test_bin_dirs
    original_path = os.environ["PATH"]
    search_path = os.pathsep.join(test_bin_dirs) + os.pathsep + original_path
    regex = r"^uwot.*"
    result = ry.which_re(regex, search_path)
    assert isinstance(result, list)
    assert result is not None
    assert len(result) > 0, "Expected a match for executables starting with 'uwot'"


def test_which_regex_nonexistent_exe(tmp_path: Path) -> None:
    """
    Test `which_re` with a regex pattern that matches no executables.
    """
    test_bin_dirs = _mk_test_bin_dirs(tmp_path)

    original_path = os.environ["PATH"]
    search_path = os.pathsep.join(test_bin_dirs) + os.pathsep + original_path

    regex = ry.Regex(r"^idontexist.*")
    result = ry.which_re(regex, search_path)
    assert not result


def test_which_regex_multiple_matches(tmp_path: Path) -> None:
    test_bin_dirs = _mk_test_bin_dirs(tmp_path)
    original_path = os.environ["PATH"]
    search_path = os.pathsep.join(test_bin_dirs) + os.pathsep + original_path
    regex = ry.Regex(r"^notavirus.*")
    results = ry.which_re(regex, path=search_path)

    assert results is not None
    assert len(results) > 0, (
        "Expected multiple matches for executables starting with 'notavirus'"
    )
    for result in results:
        assert "notavirus" in str(result)


def test_which_regex_ignore_case(tmp_path: Path) -> None:
    """
    Test `which_re` with a regex pattern that is case-insensitive.
    """
    test_bin_dirs = _mk_test_bin_dirs(tmp_path)
    original_path = os.environ["PATH"]
    search_path = os.pathsep.join(test_bin_dirs) + os.pathsep + original_path

    regex = ry.Regex(r"^NOTAVIRUS.*", case_insensitive=True)
    result = ry.which_re(regex, search_path)

    assert result is not None, "Expected a match for case-insensitive regex"
    assert isinstance(result, list)
    assert len(result) > 0, "Expected a match for executables starting with 'notavirus'"
    assert all("notavirus" in str(r).lower() for r in result)
