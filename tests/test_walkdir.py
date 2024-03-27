import json
from dataclasses import dataclass
from pathlib import Path

import ry


@dataclass
class MkDirTree:
    dirpaths: set[Path]
    filepaths: set[Path]


def mk_dir_tree(tmp_path: Path | str) -> MkDirTree:
    tmp_path = Path(tmp_path)
    abcd = tmp_path / "a" / "b" / "c" / "d"
    abcd.mkdir(parents=True)
    efgh = tmp_path / "e" / "f" / "g" / "h"
    efgh.mkdir(parents=True)
    dirpaths = set()
    filepaths = set()
    files_to_create = [
        abcd / "test.txt",
        abcd / "test2.txt",
        efgh / "test.txt",
        efgh / "test2.txt",
    ]
    for f in files_to_create:
        rel_filepath = f.relative_to(tmp_path)
        rel_dirpath = f.parent.relative_to(tmp_path)
        dirpaths.add(rel_dirpath)
        filepaths.add(rel_filepath)
        f.write_text(str(rel_filepath))
    tiles_z4 = [(x, y, z) for z in range(4) for x in range(2**z) for y in range(2**z)]
    dir_parts = {(z, x) for x, y, z in tiles_z4}
    tiles_root = tmp_path / "tiles"
    for z, x in dir_parts:
        dirpath = Path(tiles_root / str(z) / str(x))
        dirpath.mkdir(parents=True)
        dirpaths.add(dirpath)
    for x, y, z in tiles_z4:
        tile_file = tiles_root / str(z) / str(x) / f"{y}.json"
        tile_file.write_text(
            json.dumps(
                {
                    "x": x,
                    "y": y,
                    "z": z,
                }
            ),
            encoding="utf-8",
            newline="\n",
        )
        filepaths.add(tile_file)
    # make some empty dirs
    empty_dirs = [tmp_path / "nada", tmp_path / "nothing-in-here"]
    for d in empty_dirs:
        d.mkdir()
        dirpaths.add(d)

    return MkDirTree(dirpaths, filepaths)


def test_walk_dir_dirpath_string(tmp_path: Path) -> None:
    mk_dir_tree(tmp_path)

    paths = []
    for f in ry.walkdir(str(tmp_path)):
        print(f)
        paths.append(f)
    print(paths)


def test_walk_dir_dirpath_pathlib_path(tmp_path: Path) -> None:
    mk_dir_tree(tmp_path)

    paths = []
    for f in ry.walkdir(tmp_path):
        print(f)
        paths.append(f)
    print(paths)
    # assert False


def test_walk_dir_dirpath_none_use_pwd(tmp_path: Path) -> None:
    mk_dir_tree(tmp_path)
    ry.cd(tmp_path)

    paths = []
    for f in ry.walkdir():
        print(f)
        paths.append(f)
    print(paths)
    # assert False


def test_walk_dir_dirpath_string_files_only(tmp_path: Path) -> None:
    mk_dir_tree(tmp_path)

    paths = []
    for f in ry.walkdir(str(tmp_path), files=True, dirs=False):
        print(f)
        paths.append(f)
    print(paths)
    # assert False


if __name__ == "__main__":
    mk_dir_tree(".")
