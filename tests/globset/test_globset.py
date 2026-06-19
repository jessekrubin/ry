from __future__ import annotations

import pickle

import ry


class TestReprs:
    def test_glob_str_repr_methods(self) -> None:
        glob = ry.Glob("*.py")
        assert str(glob) == 'Glob("*.py")'
        assert repr(glob) == str(glob)
        assert glob.__module__ == "ry.ryo3"

    def test_globset_str_repr_methods(self) -> None:
        globset = ry.GlobSet(["*.py", "*.txt"])
        assert str(globset) == 'GlobSet(["*.py", "*.txt"])'
        assert str(globset) == repr(globset)
        assert globset.__module__ == "ry.ryo3"

    def test_globster_str_repr_methods(self) -> None:
        globset = ry.globster(["*.py", "*.txt"])
        assert str(globset) == 'Globster(["*.py", "*.txt"])'
        assert str(globset) == repr(globset)
        assert globset.__module__ == "ry.ryo3"


class TestGlob:
    def test_negative_glob_strips_bang_for_matcher(self) -> None:
        matcher = ry.Glob("!*.txt")
        assert not matcher("file.txt")
        assert matcher("file.py")
        assert str(matcher) == 'Glob("!*.txt")'


class TestGlobSet:
    def test_globset_varargs_patterns(self) -> None:
        globset = ry.GlobSet("*.py", "*.txt")
        assert globset("file.py")
        assert globset("file.txt")
        assert not globset("file.exe")
        assert globset.patterns == ("*.py", "*.txt")

    def test_globset_mixed_varargs_and_sequences(self) -> None:
        globset = ry.GlobSet("*.py", ("*.txt",), ["*.md"])
        assert globset("file.py")
        assert globset("file.txt")
        assert globset("file.md")
        assert not globset("file.exe")
        assert globset.patterns == ("*.py", "*.txt", "*.md")

    def test_globset_pickle_roundtrip_preserves_globs(self) -> None:
        globset = ry.GlobSet(ry.Glob("*.PY", case_insensitive=True), "*.txt")
        loaded = pickle.loads(pickle.dumps(globset))

        assert loaded.patterns == ("*.PY", "*.txt")
        assert loaded("file.py")
        assert loaded("file.PY")
        assert loaded("file.txt")
        assert not loaded("file.exe")


class TestGlobster:
    def test_single_globster(self) -> None:
        matcher = ry.Glob("*.py")
        assert matcher.is_match("file.py")
        assert not matcher.is_match("file.txt")

    def test_single_globster_callable(self) -> None:
        matcher = ry.Glob("*.py")
        assert matcher("file.py")
        assert not matcher("file.txt")

    def test_multiple_globsters(self) -> None:
        gset = ry.globster(["*.py", "*.txt"])
        assert gset.is_match("file.py")
        assert gset.is_match("file.txt")
        assert not gset.is_match("file.exe")

    def test_multiple_globsters_tuple(self) -> None:
        gset = ry.globster(("*.py", "*.txt"))
        assert gset.is_match("file.py")
        assert gset.is_match("file.txt")
        assert not gset.is_match("file.exe")

    def test_globster_single_string_pattern(self) -> None:
        gset = ry.globster("*.py")
        assert gset("file.py")
        assert not gset("file.txt")
        assert gset.patterns == ("*.py",)

    def test_globster_varargs_patterns(self) -> None:
        gset = ry.globster("*.py", "*.txt")
        assert gset("file.py")
        assert gset("file.txt")
        assert not gset("file.exe")
        assert gset.patterns == ("*.py", "*.txt")

    def test_globster_mixed_varargs_and_sequences(self) -> None:
        gset = ry.globster("*.py", ("*.txt",), ["!skip.txt"])
        assert gset("file.py")
        assert gset("file.txt")
        assert not gset("skip.txt")
        assert gset.patterns == ("*.py", "*.txt", "!skip.txt")

    def test_globster_class_accepts_varargs_and_sequences(self) -> None:
        gset = ry.Globster("*.py", ("*.txt",))
        assert gset("file.py")
        assert gset("file.txt")
        assert not gset("file.exe")
        assert gset.patterns == ("*.py", "*.txt")

    def test_globster_accepts_glob(self) -> None:
        glob = ry.Glob("*.PY", case_insensitive=True)
        gset = ry.globster(glob, "*.txt")
        assert gset("file.py")
        assert gset("file.PY")
        assert gset("file.txt")
        assert not gset("file.exe")
        assert gset.patterns == ("*.PY", "*.txt")

    def test_globster_accepts_globset(self) -> None:
        globset = ry.GlobSet(["*.py", "*.txt"])
        gset = ry.globster(globset, "*.md")
        assert gset("file.py")
        assert gset("file.txt")
        assert gset("file.md")
        assert not gset("file.exe")
        assert gset.patterns == ("*.py", "*.txt", "*.md")

    def test_globster_accepts_globster(self) -> None:
        inner = ry.globster("*.py", "!skip.py")
        gset = ry.globster(inner, "*.txt")
        assert gset("file.py")
        assert not gset("skip.py")
        assert gset("file.txt")
        assert not gset("file.exe")
        assert gset.patterns == ("*.py", "!skip.py", "*.txt")

    def test_globster_class_accepts_existing_matchers(self) -> None:
        glob = ry.Glob("*.py")
        globset = ry.GlobSet(["*.txt"])
        inner = ry.globster("*.md")
        gset = ry.Globster(glob, globset, inner)
        assert gset("file.py")
        assert gset("file.txt")
        assert gset("file.md")
        assert not gset("file.exe")
        assert gset.patterns == ("*.py", "*.txt", "*.md")

    def test_multiple_globsters_callable(self) -> None:
        gset = ry.globster(
            ["*.py", "*.txt"],
        )
        assert gset("file.py")
        assert gset("file.txt")
        assert gset("path/to/a/file.txt")

        assert not gset("file.PY")
        assert not gset("file.TXT")
        assert not gset("file.TxT")
        assert not gset("file.exe")

    def test_multiple_globsters_callable_fspath(self) -> None:
        gset = ry.globster(
            ["*.py", "*.txt"],
        )
        assert gset(ry.FsPath("file.py"))
        assert gset(ry.FsPath("file.txt"))
        assert gset(ry.FsPath("path/to/a/file.txt"))

        assert not gset(ry.FsPath("file.PY"))
        assert not gset(ry.FsPath("file.TXT"))
        assert not gset(ry.FsPath("file.TxT"))
        assert not gset(ry.FsPath("file.exe"))

    def test_multiple_globsters_negative(self) -> None:
        gset = ry.globster(["*.py", "!*.txt"])
        assert gset("file.py")
        assert gset("file.txt") is False
        assert not gset("path/to/a/file.txt")
        assert not gset("file.exe")

    def test_globster_ordered_last_match_wins(self) -> None:
        gset = ry.globster(["*.txt", "!keep.txt", "keep.txt"])
        assert gset("file.txt")
        assert gset("keep.txt")
        assert not gset("file.py")

    def test_globster_ordered_later_negative_wins(self) -> None:
        gset = ry.globster(["*.txt", "keep.txt", "!keep.txt"])
        assert gset("file.txt")
        assert not gset("keep.txt")
        assert not gset("file.py")

    def test_multiple_globsters_case_insensitive(self) -> None:
        globset = ry.globster(["*.py", "*.txt"], case_insensitive=True)
        assert globset("file.py")
        assert globset("file.PY")
        assert globset("file.txt")
        assert globset("file.TXT")
        assert globset("file.TxT")
        assert not globset("file.exe")

    def test_glob_paths(self) -> None:
        strings = [
            "/a",
            "/a/sub_aa",
            "/a/sub_aa/aaa",
            "/a/sub_aa/subsub",
            "/b",
            "/c",
        ]
        globber = ry.globster(
            ["/a/*", "!/a/*/*"], case_insensitive=True, literal_separator=True
        )
        matches = [el for el in strings if globber(el)]
        assert matches == [
            "/a/sub_aa",
        ]

    def test_globster_pickle_roundtrip_preserves_globs(self) -> None:
        globster = ry.globster(ry.Glob("*.PY", case_insensitive=True), "!skip.py")
        loaded = pickle.loads(pickle.dumps(globster))

        assert loaded.patterns == ("*.PY", "!skip.py")
        assert loaded("file.py")
        assert loaded("file.PY")
        assert not loaded("skip.py")
        assert not loaded("file.exe")
