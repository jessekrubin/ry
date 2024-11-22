from __future__ import annotations

import ry


def test_glob_str_repr_methods() -> None:
    glob = ry.glob("*.py")
    assert str(glob) == 'Glob("*.py")'
    assert repr(glob) == str(glob)
    assert glob.__module__ == "ryo3"


def test_globset_str_repr_methods() -> None:
    globset = ry.GlobSet(["*.py", "*.txt"])
    assert str(globset) == 'GlobSet(["*.py", "*.txt"])'
    assert str(globset) == repr(globset)
    assert globset.__module__ == "ryo3"


def test_globster_str_repr_methods() -> None:
    globset = ry.globs(["*.py", "*.txt"])
    assert str(globset) == 'Globster(["*.py", "*.txt"])'
    assert str(globset) == repr(globset)
    assert globset.__module__ == "ryo3"


def test_single_globster() -> None:
    matcher = ry.glob("*.py")
    assert matcher.is_match("file.py")
    assert not matcher.is_match("file.txt")


def test_single_globster_callable() -> None:
    matcher = ry.glob("*.py")
    assert matcher("file.py")
    assert not matcher("file.txt")


def test_multiple_globsters() -> None:
    gset = ry.globs(["*.py", "*.txt"])
    assert gset.is_match("file.py")
    assert gset.is_match("file.txt")
    assert not gset.is_match("file.exe")


def test_multiple_globsters_callable() -> None:
    gset = ry.globs(
        ["*.py", "*.txt"],
    )
    assert gset("file.py")
    assert gset("file.txt")
    assert gset("path/to/a/file.txt")

    assert not gset("file.PY")
    assert not gset("file.TXT")
    assert not gset("file.TxT")
    assert not gset("file.exe")


def test_multiple_globsters_negative() -> None:
    gset = ry.globs(["*.py", "!*.txt"])
    assert gset("file.py")
    assert gset("file.txt") is False
    assert not gset("path/to/a/file.txt")
    assert not gset("file.exe")


def test_multiple_globsters_case_insensitive() -> None:
    globset = ry.globs(["*.py", "*.txt"], case_insensitive=True)
    assert globset("file.py")
    assert globset("file.PY")
    assert globset("file.txt")
    assert globset("file.TXT")
    assert globset("file.TxT")
    assert not globset("file.exe")


def test_glob_paths() -> None:
    strings = [
        "/a",
        "/a/sub_aa",
        "/a/sub_aa/aaa",
        "/a/sub_aa/subsub",
        "/b",
        "/c",
    ]
    globber = ry.globs(
        ["/a/*", "!/a/*/*"], case_insensitive=True, literal_separator=True
    )
    matches = [el for el in strings if globber(el)]
    assert matches == [
        "/a/sub_aa",
    ]
