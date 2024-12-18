from __future__ import annotations

from typing import Any

import pytest

import ry

params_list: list[tuple[str, int | str | float]] = [
    ("zoom_level", "0"),
    ("tile_column", 0),
    ("tile_row", "0"),
]

params_arr = [
    params_list,
    [("zoom_level", "0"), ("tile_column", "0"), ("tile_row", "0")],
    {"zoom_level": "0", "tile_column": "0", "tile_row": "0"},
    {"zoom_level": 0, "tile_column": 0, "tile_row": 0},
]


@pytest.mark.parametrize("params", params_arr)
def test_sqlparams(params: Any) -> None:
    sqlfmt_params_obj = ry.sqlfmt_params(
        params,
    )

    # test the repr
    repr_str = "ry." + repr(sqlfmt_params_obj)
    # exec
    round_tripped = eval(repr_str)
    assert sqlfmt_params_obj == round_tripped
    assert hash(sqlfmt_params_obj) == hash(round_tripped)
