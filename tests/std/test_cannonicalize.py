import pathlib

import ry


def test_canonicalize_str(
    tmp_path: pathlib.Path,
) -> None:
    tmp_path.joinpath("a/b/c").mkdir(parents=True)
    file_path = tmp_path.joinpath("a/b/c/file.txt")
    file_path.write_text("test content")

    ry.cd(tmp_path)

    _uncannonical_path = "a/../a/b/../b/c/./file.txt"
    _expected_trailer = str(pathlib.Path("a/b/c/file.txt"))
    cannonical_path = ry.canonicalize(_uncannonical_path)
    assert str(cannonical_path).endswith(_expected_trailer)

    # test pathlib
    cannonical_path_pathlib = ry.canonicalize(pathlib.Path(_uncannonical_path))
    assert str(cannonical_path_pathlib).endswith(_expected_trailer)
