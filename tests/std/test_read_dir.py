from __future__ import annotations

import concurrent.futures
import os

import ry

# this files  dirpath
PWD = os.path.dirname(os.path.abspath(__file__))


def test_read_dir() -> None:
    items = os.listdir(PWD)

    for direntry in ry.read_dir(PWD):
        basename = os.path.basename(direntry)
        assert basename in items
        assert isinstance(direntry.metadata, ry.Metadata)
        assert isinstance(direntry.file_type, ry.FileType)
        assert isinstance(direntry.basename, str)


def test_read_dir_concurrent() -> None:
    i = ry.read_dir(PWD)

    total = len(os.listdir(PWD))

    def _process_direntry() -> ry.ryo3._std.DirEntry:
        de = next(i)
        assert isinstance(de.__fspath__(), str)  # dummy check thing
        return de

    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as tpe:
        # Submit the task to the executor
        futs = [tpe.submit(_process_direntry) for _ in range(total)]
        res = [f.result() for f in futs]
        assert len(res) == total
