from __future__ import annotations

from hypothesis import strategies as st

import ry.dev as ry

timedelta_strategy = st.timedeltas()


def test_version() -> None:
    assert isinstance(ry.__version__, str)
    version_tuple = tuple(map(int, ry.__version__.split(".")))  # noqa: RUF048
    assert len(version_tuple) == 3


def test_dev() -> None:
    assert True
