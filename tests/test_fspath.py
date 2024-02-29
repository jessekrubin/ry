"""Tests for ry.FsPath"""

import ry
from pathlib import Path
import pytest


def test_new_path():
    pypath = Path()
    rypath = ry.FsPath()
    assert rypath == pypath


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

    def test_new_path(self, path_cls):
        pypath = Path()
        rypath = path_cls()
        assert rypath == pypath

    def test_parent(self, path_cls):
        pypath = Path()
        rypath = path_cls()
        assert rypath.parent == pypath.parent

    def test_absolute(self, path_cls):
        pypath = Path()
        rypath = path_cls()
        pypath_abs = pypath.absolute()
        rypath_abs = rypath.absolute()
        assert rypath_abs == pypath_abs
