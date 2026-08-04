from __future__ import annotations

import ry.dev as ry


def test_version() -> None:
    assert isinstance(ry.__version__, str)
    version_tuple = tuple(map(int, ry.__version__.split(".")))  # noqa: RUF048
    assert len(version_tuple) == 3


def test_dev() -> None:
    assert True
