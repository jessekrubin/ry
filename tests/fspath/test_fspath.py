"""Tests for ry.FsPath"""

from __future__ import annotations

import itertools as it
import os
from pathlib import Path
from typing import TypeAlias

import pytest

import ry

TPath: TypeAlias = type[Path] | type[ry.FsPath]
is_windows = os.name == "nt"


def test_new_path() -> None:
    pypath = Path()
    rypath = ry.FsPath()
    assert rypath == pypath


def test_hash_path() -> None:
    rypath = ry.FsPath(".").resolve()
    another_rypath = rypath.parent
    assert hash(rypath) != hash(another_rypath)
    assert rypath != another_rypath


# parametrize the tests for parity with pathlib.Path
@pytest.mark.parametrize(
    "path_cls",
    [
        pytest.param(
            Path,
            id="pathlib.Path",
        ),
        pytest.param(
            ry.FsPath,
            id="ry.FsPath",
        ),
    ],
)
class TestFsPath:
    def test_new_path(self, path_cls: TPath) -> None:
        pypath = Path()
        rypath = path_cls()
        assert rypath == pypath

    def test_parent(self, path_cls: TPath) -> None:
        pypath = Path()
        rypath = path_cls()
        assert rypath.parent == pypath.parent

    def test_absolute(self, path_cls: TPath) -> None:
        pypath = Path()
        rypath = path_cls()
        pypath_abs = pypath.absolute()
        rypath_abs = rypath.absolute()
        assert rypath_abs == pypath_abs

    def test_read_text(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.write_text("hello")
        rypath = path_cls(pypath)
        assert rypath.read_text() == pypath.read_text()

    def test_read_bytes(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.write_bytes(b"hello")
        rypath = path_cls(pypath)
        b = rypath.read_bytes()
        assert rypath.read_bytes() == pypath.read_bytes()
        assert rypath.read_bytes() == b

    def test_write_text(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        rypath = path_cls(pypath)
        rypath.write_text("new content")
        assert pypath.read_text() == "new content"

    def test_write_bytes(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        rypath = path_cls(pypath)
        rypath.write_bytes(b"new content")
        assert pypath.read_bytes() == b"new content"

    def test_joinpath(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.joinpath("child") == pypath.joinpath("child")

    def test_exists(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.touch()
        rypath = path_cls(pypath)
        assert rypath.exists() == pypath.exists()

    def test_is_file(self, path_cls: TPath, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.touch()
        rypath = path_cls(pypath)
        assert rypath.is_file() == pypath.is_file()

    def test_is_dir(self, path_cls: TPath, tmp_path: Path) -> None:
        rypath = path_cls(tmp_path)
        assert rypath.is_dir() == tmp_path.is_dir()

    def test_with_name(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.with_name("newfile.txt") == pypath.with_name("newfile.txt")

    def test_with_suffix(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.with_suffix(".md") == pypath.with_suffix(".md")

    def test_stem(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.stem == pypath.stem

    def test_suffix(self, path_cls: TPath) -> None:
        pypath = Path("file.txt")
        rypath = path_cls("file.txt")
        assert rypath.suffix == pypath.suffix

    def test_iterdir(self, path_cls: TPath, tmp_path: Path) -> None:
        (tmp_path / "file1.txt").touch()
        (tmp_path / "file2.txt").touch()
        pypath = tmp_path
        rypath = path_cls(tmp_path)
        assert sorted(rypath.iterdir()) == sorted(pypath.iterdir())

    def test_relative_to(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.txt")
        rypath = path_cls("/some/path/file.txt")
        if path_cls is ry.FsPath:
            with pytest.raises(NotImplementedError):
                relative_resolved = rypath.relative_to("/some")
                assert relative_resolved == pypath.relative_to("/some")
        else:
            relative_resolved = rypath.relative_to("/some")
            assert relative_resolved == pypath.relative_to("/some")

    def test_as_posix(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.txt")
        rypath = path_cls("/some/path/file.txt")
        assert rypath.as_posix() == pypath.as_posix()

    def test_equality(self, path_cls: TPath) -> None:
        pypath1 = Path("/some/path")
        pypath2 = Path("/some/path")
        rypath1 = path_cls("/some/path")
        rypath2 = path_cls("/some/path")
        for a, b in it.combinations([pypath1, pypath2, rypath1, rypath2], 2):
            assert a == b, f"{a} != {b} ({type(a)} != {type(b)})"

    def test_inequality(self, path_cls: TPath) -> None:
        rypath1 = path_cls("/some/path")
        rypath2 = path_cls("/other/path")
        assert rypath1 != rypath2

    def test_truediv_operators(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath / "file.txt" == pypath / "file.txt"
        assert "file.txt" / rypath == "file.txt" / pypath
        assert rypath / Path("file.txt") == pypath / Path("file.txt")
        assert Path("file.txt") / rypath == Path("file.txt") / pypath

    def test_root(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.root == pypath.root

    def test_bytes(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        pathbytes_fslash = bytes(rypath).replace(b"\\", b"/")
        assert pathbytes_fslash == bytes(pypath).replace(
            b"\\", b"/"
        )  # todo: reevaluate

    def test_parts(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.parts == pypath.parts
        assert type(rypath.parts) is type(pypath.parts)
        assert isinstance(rypath.parts, tuple)

    def test_parents(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.txt")
        rypath: ry.FsPath | Path = path_cls("/some/path/file.txt")
        assert len(rypath.parents) == len(pypath.parents)
        for rp, pp in zip(rypath.parents, pypath.parents, strict=False):
            rp_posix = rp.as_posix()  # type: ignore[attr-defined]
            pp_posix = pp.as_posix()
            assert rp_posix == pp_posix

    def test_name(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.txt")
        rypath = path_cls("/some/path/file.txt")
        assert rypath.name == pypath.name

    def test_suffixes(self, path_cls: TPath) -> None:
        pypath = Path("/some/path/file.tar.gz")
        rypath = path_cls("/some/path/file.tar.gz")
        assert rypath.suffixes == pypath.suffixes
        assert type(rypath.suffixes) is type(pypath.suffixes)
        assert isinstance(rypath.suffixes, list)

    def test_home(self, path_cls: TPath) -> None:
        pypath = Path.home()
        rypath = path_cls.home()
        assert rypath == pypath

    def test_cwd(self, path_cls: TPath) -> None:
        pypath = Path.cwd()
        rypath = path_cls.cwd()
        assert rypath == pypath


class TestFsPathRustMethods:
    def test_extensions(self) -> None:
        fsp = ry.FsPath("/some/path/file.tar.gz")
        assert fsp.extension() == "gz"

    def test_file_name(self) -> None:
        fsp = ry.FsPath("/some/path/file.tar.gz")
        assert fsp.file_name() == "file.tar.gz"

    def test_file_prefix(self) -> None:
        fsp = ry.FsPath("/some/path/file.tar.gz")
        assert fsp.file_prefix() == "file.tar"

    def test_file_stem(self) -> None:
        fsp = ry.FsPath("/some/path/file.tar.gz")
        assert fsp.file_stem() == "file.tar"

    def test_is_relative(self) -> None:
        fsp = ry.FsPath("file.tar.gz")
        assert fsp.is_relative()
        another = ry.FsPath.home()
        assert not another.is_relative()


class TestFsPathBytes:
    def test_read(self, tmp_path: Path) -> None:
        """Test reading returning ry.Bytes"""
        pypath = tmp_path / "test.txt"
        pypath.write_bytes(b"hello")
        rypath = ry.FsPath(pypath)
        b = rypath.read()
        assert rypath.read() == pypath.read_bytes()
        assert rypath.read() == b

    def test_write(self, tmp_path: Path) -> None:
        """Test writing bytes"""
        pypath = tmp_path / "test.txt"
        rypath = ry.FsPath(pypath)
        rypath.write(b"new content")
        assert pypath.read_bytes() == b"new content"

        # write as ry.Bytes
        rypath.write(ry.Bytes(b"newer content"))
        assert pypath.read_bytes() == b"newer content"


class TestFsPathStringMethods:
    # test `starts_with` and `ends_with` methods, `strip_prefix`
    # 'with_extension, `with_file_name`
    def test_starts_with(self) -> None:
        fsp = ry.FsPath("some/path/file.tar.gz")
        assert fsp.starts_with("some")
        assert not fsp.starts_with("other")

    def test_ends_with(self) -> None:
        fsp = ry.FsPath("some/path/file.tar.gz")
        assert fsp.ends_with("file.tar.gz")
        assert not fsp.ends_with("file.txt")

    def test_strip_prefix(self) -> None:
        fsp = ry.FsPath("some/path/file.tar.gz")
        stripped = fsp.strip_prefix("some/path")
        assert stripped == ry.FsPath("file.tar.gz")
        with pytest.raises(ValueError):
            _ = fsp.strip_prefix("other/path")

    def test_with_extension(self) -> None:
        fsp = ry.FsPath("some/path/file.tar.gz")
        new_fsp = fsp.with_extension("md")
        assert new_fsp == ry.FsPath("some/path/file.tar.md")


class TestFsPathPosix:
    @pytest.mark.parametrize(
        "path_cls",
        [
            pytest.param(
                Path,
                id="pathlib.Path",
            ),
            pytest.param(
                ry.FsPath,
                id="ry.FsPath",
            ),
        ],
    )
    @pytest.mark.skipif(is_windows, reason="Posix specific tests")
    def test_root(self, path_cls: TPath) -> None:
        pypath = Path("/some/path")
        rypath = path_cls("/some/path")
        assert rypath.root == pypath.root


@pytest.fixture(
    name="tmp_fspath",
    scope="function",
)
def tmp_fspath(tmp_path: Path) -> ry.FsPath:
    return ry.FsPath(tmp_path)


class TestFsPathReadDir:
    def test_iterdir(self, tmp_fspath: ry.FsPath) -> None:
        (tmp_fspath / "file1.txt").touch()
        (tmp_fspath / "file2.txt").touch()
        read_dir_list = list(tmp_fspath.read_dir())
        read_dir_collect = tmp_fspath.read_dir().collect()
        assert sorted(read_dir_list) == sorted(read_dir_collect)

    def test_read_dir_empty(self, tmp_fspath: ry.FsPath) -> None:
        read_dir_list = list(tmp_fspath.read_dir())
        read_dir_collect = tmp_fspath.read_dir().collect()
        assert read_dir_list == []
        assert read_dir_collect == []

    def test_read_dir_nonexistent(self, tmp_path: Path) -> None:
        nonexist_path = tmp_path / "nonexistent"
        fspath = ry.FsPath(nonexist_path)
        with pytest.raises(FileNotFoundError):
            _ = list(fspath.read_dir())

    def test_read_dir_take(self, tmp_fspath: ry.FsPath) -> None:
        (tmp_fspath / "file1.txt").touch()
        (tmp_fspath / "file2.txt").touch()
        read_dir = tmp_fspath.read_dir()
        taken = read_dir.take(1)
        assert len(taken) == 1


class TestTouch:
    def test_touch_creates_file(self, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        rypath = ry.FsPath(pypath)
        created = rypath.touch()
        assert created is True
        assert pypath.exists()

    def test_touch_existing_file(self, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.touch()
        rypath = ry.FsPath(pypath)
        created = rypath.touch()
        assert created is False
        assert pypath.exists()

    def test_touch_no_parent_dir(self, tmp_path: Path) -> None:
        pypath = tmp_path / "nonexistent_dir" / "test.txt"
        rypath = ry.FsPath(pypath)
        with pytest.raises(FileNotFoundError, match="No such file or directory"):
            _ = rypath.touch()

    def test_touch_with_mode_not_implemented(self, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        rypath = ry.FsPath(pypath)
        with pytest.raises(NotImplementedError):
            _ = rypath.touch(mode=0o644)

    def test_touch_exists_ok_false(self, tmp_path: Path) -> None:
        pypath = tmp_path / "test.txt"
        pypath.touch()
        rypath = ry.FsPath(pypath)
        with pytest.raises(FileExistsError):
            _ = rypath.touch(exist_ok=False)


@pytest.mark.parametrize(
    "path_cls",
    [
        pytest.param(
            Path,
            id="pathlib.Path",
        ),
        pytest.param(
            ry.FsPath,
            id="ry.FsPath",
        ),
    ],
)
@pytest.mark.skipif(not is_windows, reason="Windows specific tests")
class TestFsPathWindows:
    def test_drive(self, path_cls: TPath) -> None:
        # windows
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        assert rypath.drive == pypath.drive

    def test_anchor(self, path_cls: TPath) -> None:
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        assert rypath.anchor == pypath.anchor

    def test_name(self, path_cls: TPath) -> None:
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        assert rypath.name == pypath.name

    def test_as_uri(self, path_cls: TPath) -> None:
        pypath = Path("C:/some/path")
        rypath = path_cls("C:/some/path")
        if path_cls is ry.FsPath:
            with pytest.raises(NotImplementedError):
                rypath.as_uri()
        else:
            assert rypath.as_uri() == pypath.as_uri()


@pytest.mark.skipif(is_windows, reason="Windows specific tests")
class TestFsPathNonWindows:
    def test_drive(self) -> None:
        p = ry.FsPath("/some/path")
        assert p.drive is None


def test_ancestors_repr() -> None:
    p = ry.FsPath("rooot") / "dir1" / "dir2" / "file.txt"
    ancestors_iter = p.ancestors()
    assert repr(ancestors_iter) == "FsPathAncestors<rooot/dir1/dir2/file.txt>"


def test_ancestors() -> None:
    p = ry.FsPath("rooot") / "dir1" / "dir2" / "file.txt"
    ancestors = list(p.ancestors())
    ancestor_paths = [a.as_posix() for a in ancestors]
    expected_paths = [
        p.as_posix(),
        p.parent.as_posix(),
        p.parent.parent.as_posix(),
        p.parent.parent.parent.as_posix(),
    ]
    assert ancestor_paths == expected_paths
    collected = p.ancestors().collect()
    collected_paths = [a.as_posix() for a in collected]
    assert collected_paths == expected_paths
    taken = p.ancestors().take(2)
    taken_paths = [a.as_posix() for a in taken]
    expected_taken_paths = expected_paths[:2]
    assert taken_paths == expected_taken_paths


def test_read_text_unidecode_err(tmp_path: Path) -> None:
    pypath = tmp_path / "test.txt"
    # write bytes that are invalid utf-8
    bad_bytes = b"oh-shit-this-aint-no-utf8-\xff\xfe\xfd"
    pypath.write_bytes(bad_bytes)
    rypath = ry.FsPath(pypath)
    b = rypath.read_bytes()
    assert b == bad_bytes

    with pytest.raises(UnicodeDecodeError) as e:
        _t = rypath.read_text()
        err = e.value
        assert err.start == 24
        # now test that the err is shit
