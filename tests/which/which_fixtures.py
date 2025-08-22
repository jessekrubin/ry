from __future__ import annotations

import os
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path


def _clean_path(path: Path | str | None) -> str | None:
    if path is None:
        return None
    res = path
    for ext in (".EXE", ".BAT", ".CMD"):
        if str(res).endswith(ext):
            res = str(res).replace(ext, ext.lower())
    return str(res)


def _mk_test_bin_dirs(tmp_path: Path) -> list[str]:
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

        tmppath_bin = tmp_path / "bin"
        tmppath_bin.mkdir()
        for exe in windows_exe_filenames:
            with open(tmppath_bin / exe, "w") as f:
                f.write("echo %PATH%")

        tmppath_bin2 = tmp_path / "bin2"
        tmppath_bin2.mkdir()
        for exe in windows_exe_filenames:
            with open(tmppath_bin2 / exe, "w") as f:
                f.write("echo %PATH%")

        return [
            str(tmppath_bin),
            str(tmppath_bin2),
        ]
    else:
        script_str = "\n".join([
            "#!/usr/bin/env bash",
            "echo $PATH",
        ])
        # make exes
        for exe in exe_names:
            with open(tmp_path / exe, "w") as f:
                f.write("echo $PATH")
            # make executable
            os.chmod(tmp_path / exe, 0o777)
        tmppath_bin = tmp_path / "bin"
        tmppath_bin.mkdir()
        for exe in exe_names:
            with open(tmppath_bin / exe, "w") as f:
                f.write(script_str)
            # make executable
            os.chmod(tmppath_bin / exe, 0o777)
        tmppath_bin2 = tmp_path / "bin2"
        tmppath_bin2.mkdir()
        for exe in exe_names:
            with open(tmppath_bin2 / exe, "w") as f:
                f.write(script_str)
            # make executable
            os.chmod(tmppath_bin2 / exe, 0o777)
        return [
            str(tmppath_bin),
            str(tmppath_bin2),
        ]
