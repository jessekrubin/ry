import typing as t

import pytest

import ry


@pytest.mark.parametrize(
    ("input_str", "expected"),
    [
        # file
        ("file", "file"),
        ("f", "file"),
        # dir
        ("dir", "dir"),
        ("d", "dir"),
        ("directory", "dir"),
        # symlink
        ("symlink", "symlink"),
        ("link", "symlink"),
        ("s", "symlink"),
        # unix
        ("block-device", "block-device"),
        ("char-device", "char-device"),
        ("fifo", "fifo"),
        ("socket", "socket"),
        # windows
        ("symlink-dir", "symlink-dir"),
        ("symlink-file", "symlink-file"),
        # unknown
        ("unknown", "unknown"),
    ],
)
def test_file_type(
    input_str: t.Literal["file", "dir", "symlink"], expected: str
) -> None:
    ob = ry.FileType(input_str)
    assert isinstance(ob, ry.FileType)
    assert str(ob) == expected
    assert ob.is_dir == (expected == "dir")
    assert ob.is_file == (expected == "file")
    assert ob.is_symlink == (expected == "symlink")
    assert ob.is_block_device == (expected == "block-device")
    assert ob.is_char_device == (expected == "char-device")
    assert ob.is_fifo == (expected == "fifo")
    assert ob.is_socket == (expected == "socket")
    assert ob.is_symlink_dir == (expected == "symlink-dir")
    assert ob.is_symlink_file == (expected == "symlink-file")
    assert ob.is_unknown == (expected == "unknown")
    assert eval(repr(ob), {"FileType": ry.FileType}) == ob


def test_file_type_invalid() -> None:
    with pytest.raises(ValueError, match="invalid file type string: invalid"):
        ry.FileType("invalid")  # type: ignore[arg-type]
