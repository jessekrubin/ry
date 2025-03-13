from __future__ import annotations

from hypothesis import strategies as st

import ry.dev as ry

timedelta_strategy = st.timedeltas()


def test_dev() -> None:
    assert True


def test_jiff_version() -> None:
    assert ry.__version__ is not None
