import typing as t

import pytest

import ry


@pytest.mark.parametrize(
    "t",
    [
        "file",
        "dir",
        "symlink",
    ],
)
def test_file_type(t: t.Literal["file", "dir", "symlink"]) -> None:
    ob = ry.FileType(t)
    assert isinstance(ob, ry.FileType)
    assert str(ob) == t
    assert ob.is_dir == (t == "dir")
    assert ob.is_file == (t == "file")
    assert ob.is_symlink == (t == "symlink")
    assert eval(repr(ob), {"FileType": ry.FileType}) == ob

    # probably gonna kill the to_dict for filetype...
    # but here test
    d = ob.to_dict()
    assert isinstance(d, dict)
    assert d["is_dir"] == ob.is_dir
    assert d["is_file"] == ob.is_file
    assert d["is_symlink"] == ob.is_symlink
    assert set(d.keys()) == {"is_dir", "is_file", "is_symlink"}


def test_file_type_invalid() -> None:
    with pytest.raises(ValueError, match="invalid file type string: invalid"):
        ry.FileType("invalid")  # type: ignore[arg-type]
