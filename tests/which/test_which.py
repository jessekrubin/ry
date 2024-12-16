from __future__ import annotations

import os
import shutil
from pathlib import Path

import ry

from .which_fixtures import _clean_path, _mk_test_bin_dirs


def test_which_python() -> None:
    py_which = shutil.which("python")
    ry_which = ry.which("python")
    # clean path
    py_clean = _clean_path(py_which)
    ry_clean = _clean_path(ry_which)
    assert py_clean == ry_clean


def test_which_path(tmp_path: Path) -> None:
    # make exes
    path_list = _mk_test_bin_dirs(tmp_path)
    path_kwarg = os.pathsep.join(path_list)
    py_which = shutil.which("notavirus", path=path_kwarg)
    ry_which = ry.which("notavirus", path=path_kwarg)
    # clean path
    py_clean = _clean_path(py_which)
    ry_clean = _clean_path(ry_which)
    assert py_clean == ry_clean


def test_which_all_path(tmp_path: Path) -> None:
    path_list = _mk_test_bin_dirs(tmp_path)
    path_kwarg = os.pathsep.join(path_list)
    ry_which = ry.which_all("notavirus", path=path_kwarg)
    assert len(ry_which) >= 2


def test_which_path_cwd(tmp_path: Path) -> None:
    # make exes
    path_list = _mk_test_bin_dirs(tmp_path)
    path_kwarg = os.pathsep.join(path_list)
    ry.cd(tmp_path)
    py_which = shutil.which("notavirus", path=path_kwarg)
    ry_which = ry.which("notavirus", path=path_kwarg)
    # clean path
    py_clean = _clean_path(py_which)
    ry_clean = _clean_path(ry_which)
    assert py_clean == ry_clean


def test_which_nada() -> None:
    exe = "idontexist"
    py_which = shutil.which(exe)
    ry_which = ry.which(exe)
    ry_which_all = ry.which_all(exe)
    assert py_which == ry_which
    assert len(ry_which_all) == 0 and isinstance(ry_which_all, list)
