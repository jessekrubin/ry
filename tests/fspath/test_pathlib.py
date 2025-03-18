from __future__ import annotations

from pathlib import Path

import ry


def test_fspath2pathlib(tmp_path: Path) -> None:
    p = ry.FsPath(tmp_path)
    pypath = Path(tmp_path)
    pypath_conversion = p.to_pathlib()
    pypath_conversion_as_py = p.to_py()
    assert pypath_conversion == pypath_conversion_as_py
    assert isinstance(pypath_conversion, tmp_path.__class__)
    assert p == pypath
    assert pypath == pypath_conversion
