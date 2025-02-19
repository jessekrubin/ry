from pathlib import Path

import ry


def test_fspath2pathlib(tmp_path: Path) -> None:
    p = ry.FsPath(tmp_path)
    pypath = Path(tmp_path)
    pypath_conversion = p.to_pathlib()
    print(pypath_conversion, type(pypath_conversion))
    assert isinstance(pypath_conversion, tmp_path.__class__)
    assert p == pypath
    assert pypath == pypath_conversion
