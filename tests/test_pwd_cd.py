import os
import pathlib

import pytest

import ry


def test_pwd():
    assert ry.pwd() == os.getcwd()


class TestRyPath:
    def test_path(self):
        p = ry.FsPath(os.getcwd())
        assert p == pathlib.Path(os.getcwd())


def test_cd():
    old_pwd = ry.pwd()
    ry.cd("..")
    assert ry.pwd() != old_pwd
    assert ry.pwd() == os.path.dirname(old_pwd)
    ry.cd(old_pwd)
    assert ry.pwd() == old_pwd


def test_cd_pathlib_object():
    new_dir = pathlib.Path("..")
    old_pwd = ry.pwd()
    ry.cd(new_dir)
    assert ry.pwd() != old_pwd
    assert ry.pwd() == os.path.dirname(old_pwd)


def test_cd_pathlib_nonexistent():
    new_dir = pathlib.Path("nonexistent")
    old_pwd = ry.pwd()
    with pytest.raises(FileNotFoundError):
        ry.cd(new_dir)
    assert ry.pwd() == old_pwd


def test_cd_nonexistent_ry():
    with pytest.raises(FileNotFoundError):
        ry.cd("nonexistent")


def test_cd_nonexistent_py():
    with pytest.raises(FileNotFoundError):
        os.chdir("nonexistent")
