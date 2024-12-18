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

import pytest


@pytest.mark.parametrize("params", params_arr)
def test_sqlparams(params):
    sqlfmt_params_obj = ry.sqlfmt_params(
        params,
    )

    # test the repr
    repr_str = "ry." + repr(sqlfmt_params_obj)
    print(repr_str)
    # exec
    round_tripped = eval(repr_str)
    print("ry." + repr(round_tripped))
    assert sqlfmt_params_obj == round_tripped
    assert hash(sqlfmt_params_obj) == hash(round_tripped)
