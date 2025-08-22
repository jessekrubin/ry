from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path


@dataclass
class MkDirTree:
    dirpaths: set[Path]
    filepaths: set[Path]


def mk_dir_tree(tmp_path: Path) -> MkDirTree:
    tmp_path = Path(tmp_path)
    abcd = tmp_path / "a" / "b" / "c" / "d"
    abcd.mkdir(parents=True)
    efgh = tmp_path / "e" / "f" / "g" / "h"
    efgh.mkdir(parents=True)
    dirpaths: set[Path] = set()
    filepaths: set[Path] = set()

    # add tmp_path itself
    dirpaths.add(tmp_path)
    # make me some files
    files_to_create = [
        abcd / "test.txt",
        abcd / "test2.txt",
        efgh / "test.txt",
        efgh / "test2.txt",
    ]
    for f in files_to_create:
        rel_filepath = f.relative_to(tmp_path)
        dirpaths.add(f.parent)
        filepaths.add(f)
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
        with open(tile_file, "w", encoding="utf-8", newline="\n") as phile:
            phile.write(
                json.dumps({
                    "x": x,
                    "y": y,
                    "z": z,
                })
            )
        filepaths.add(tile_file)
    # make some empty dirs
    empty_dirs = [tmp_path / "nada", tmp_path / "nothing-in-here"]
    for d in empty_dirs:
        d.mkdir()
        dirpaths.add(d)

    #  ensure all paths are relative to tmp_path
    dirpaths = {d.relative_to(tmp_path) for d in dirpaths}
    filepaths = {f.relative_to(tmp_path) for f in filepaths}
    # ensure all dirpaths have parents in dirpaths
    parents2add = set()
    for d in dirpaths:
        for i in range(1, len(d.parts)):
            parent = Path(*d.parts[:i])
            if parent not in dirpaths:
                parents2add.add(parent)
    dirpaths.update(parents2add)
    return MkDirTree(dirpaths, filepaths)
