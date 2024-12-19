from __future__ import annotations

from pathlib import Path

import ry


class TestFsPathPush:
    def test_fspath_push_string(self) -> None:
        p = ry.FsPath("a")
        p._push("b")
        assert p == ry.FsPath("a/b")

    def test_fspath_push_path(self) -> None:
        p = ry.FsPath("a")
        p._push(Path("b"))
        assert p == ry.FsPath("a/b")

    def test_fspath_push_fspath(self) -> None:
        p = ry.FsPath("a")
        p._push(ry.FsPath("b"))
        assert p == ry.FsPath("a/b")

    def test_fspath_push_string_path_chained(self) -> None:
        p = ry.FsPath("a")
        p._push(
            "b",
        )._push(
            Path("c"),
        )
        assert p == ry.FsPath("a/b/c")


class TestFsPathPop:
    def test_fspath_pop(self) -> None:
        p = ry.FsPath("a/b/c")
        p._pop()
        assert p == ry.FsPath("a/b")

    def test_fspath_pop_twice(self) -> None:
        p = ry.FsPath("a/b/c")
        p._pop()._pop()
        assert p == ry.FsPath("a")

    def test_fspath_pop_too_far(self) -> None:
        p = ry.FsPath("a")
        p._pop()
        assert p == ry.FsPath("")

    def test_fspath_pop_empty(self) -> None:
        p = ry.FsPath("")
        p._pop()
        assert p == ry.FsPath("")


class TestFsPathSetExtension:
    def test_fspath_set_extension(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_extension("txt")
        assert p == ry.FsPath("a/b/c.txt")

    def test_fspath_set_extension_twice(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_extension("txt")._set_extension("json")
        assert p == ry.FsPath("a/b/c.json")

    def test_fspath_set_extension_empty(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_extension("")
        assert p == ry.FsPath("a/b/c")

    def test_fspath_set_extension_no_dot(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_extension("txt")
        assert p == ry.FsPath("a/b/c.txt")

    def test_fspath_set_extension_no_dot_twice(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_extension("txt")._set_extension("json")
        assert p == ry.FsPath("a/b/c.json")

    def test_fspath_set_extension_no_dot_empty(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_extension("")
        assert p == ry.FsPath("a/b/c")


class TestFsPathSetFileName:
    def test_fspath_set_filename(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_file_name("d")
        assert p == ry.FsPath("a/b/d")

    def test_fspath_set_filename_twice(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_file_name("d")._set_file_name("e")
        assert p == ry.FsPath("a/b/e")

    def test_fspath_set_filename_empty(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_file_name("")
        assert p == ry.FsPath("a/b/")

    def test_fspath_set_filename_no_dot(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_file_name("d.txt")
        assert p == ry.FsPath("a/b/d.txt")

    def test_fspath_set_filename_no_dot_twice(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_file_name("d.txt")._set_file_name("e.json")
        assert p == ry.FsPath("a/b/e.json")

    def test_fspath_set_filename_no_dot_empty(self) -> None:
        p = ry.FsPath("a/b/c")
        p._set_file_name("")
        assert p == ry.FsPath("a/b/")
