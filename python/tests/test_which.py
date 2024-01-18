import os
import shutil
from pathlib import Path

import ry


def _clean_path(path):
    res = path
    for ext in (".EXE", ".BAT", ".CMD"):
        if res.endswith(ext):
            res = res.replace(ext, ext.lower())
    return res


def _mk_test_bin_dirs(tmppath):
    # exe names
    exe_names = ("notavirus", "uwot")
    if os.name == "nt":
        # make exes
        windows_exe_filenames = [
            item
            for sublist in [
                (
                    f"{exe}.exe",
                    f"{exe}.bat",
                    f"{exe}.cmd",
                )
                for exe in exe_names
            ]
            for item in sublist
        ]

        tmppath_bin = tmppath / "bin"
        tmppath_bin.mkdir()
        for exe in windows_exe_filenames:
            with open(tmppath_bin / exe, "w") as f:
                f.write("echo %PATH%")

        tmppath_bin2 = tmppath / "bin2"
        tmppath_bin2.mkdir()
        for exe in windows_exe_filenames:
            with open(tmppath_bin2 / exe, "w") as f:
                f.write("echo %PATH%")

        return [
            str(tmppath_bin),
            str(tmppath_bin2),
        ]
    else:
        # make exes
        for exe in exe_names:
            with open(tmppath / exe, "w") as f:
                f.write("echo $PATH")
            # make executable
            os.chmod(tmppath / exe, 0o777)
        tmppath_bin = tmppath / "bin"
        tmppath_bin.mkdir()
        for exe in exe_names:
            with open(tmppath_bin / exe, "w") as f:
                f.write("echo $PATH")
            # make executable
            os.chmod(tmppath_bin / exe, 0o777)
        tmppath_bin2 = tmppath / "bin2"
        tmppath_bin2.mkdir()
        for exe in exe_names:
            with open(tmppath_bin2 / exe, "w") as f:
                f.write("echo $PATH")
            # make executable
            os.chmod(tmppath_bin2 / exe, 0o777)
        return [
            str(tmppath_bin),
            str(tmppath_bin2),
        ]

def test_which_python():
    py_which = shutil.which("python")
    print("py", py_which)
    ry_which = ry.which("python")
    print("ry", ry_which)
    # clean path
    py_clean = _clean_path(py_which)
    ry_clean = _clean_path(ry_which)
    print("py", py_clean)
    print("ry", ry_clean)
    assert py_clean == ry_clean


def test_which_path(tmpdir: Path):
    # make exes
    path_list = _mk_test_bin_dirs(tmpdir)
    path_kwarg = os.pathsep.join(path_list)
    py_which = shutil.which("notavirus", path=path_kwarg)
    print("py", py_which)
    ry_which = ry.which("notavirus", path=path_kwarg)
    print("ry", ry_which)
    # clean path
    py_clean = _clean_path(py_which)
    ry_clean = _clean_path(ry_which)
    print("py", py_clean)
    print("ry", ry_clean)
    assert py_clean == ry_clean

    # assert False


def test_which_all_path(tmpdir: Path):
    path_list = _mk_test_bin_dirs(tmpdir)
    path_kwarg = os.pathsep.join(path_list)
    ry_which = ry.which_all("notavirus", path=path_kwarg)
    assert len(ry_which) >= 2

def test_which_path_cwd(tmpdir: Path):
    # make exes
    path_list = _mk_test_bin_dirs(tmpdir)
    path_kwarg = os.pathsep.join(path_list)
    ry.cd(tmpdir)
    py_which = shutil.which("notavirus", path=path_kwarg)
    print("py", py_which)
    ry_which = ry.which("notavirus", path=path_kwarg)
    print("ry", ry_which)
    # clean path
    py_clean = _clean_path(py_which)
    ry_clean = _clean_path(ry_which)
    print("py", py_clean)
    print("ry", ry_clean)
    assert py_clean == ry_clean

    # assert False


def test_which_nada():
    exe = "idontexist"
    py_which = shutil.which(exe)
    ry_which = ry.which(exe)
    ry_which_all = ry.which_all(exe)
    print(ry_which_all)
    assert py_which == ry_which
    assert len(ry_which_all) == 0 and isinstance(ry_which_all, list)