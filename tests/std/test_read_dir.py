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
