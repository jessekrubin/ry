from __future__ import annotations

import datetime as pydt

import ry


def test_metadata() -> None:
    this_filepath = __file__
    md = ry.metadata(this_filepath)
    assert isinstance(md, ry.Metadata)
    d = md.to_py()
    assert "file_type" in d
    assert isinstance(d["file_type"], str)
    assert d["file_type"] in ("file", "directory", "symlink")
    assert isinstance(d["len"], int)
    assert isinstance(d["readonly"], bool)
    assert isinstance(d["accessed"], pydt.datetime)
    assert isinstance(d["created"], pydt.datetime)
    assert isinstance(d["modified"], pydt.datetime)
