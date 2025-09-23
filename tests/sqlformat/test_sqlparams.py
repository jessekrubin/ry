from __future__ import annotations

import pickle
import typing as t

import pytest

import ry

_PARAMS_LIST: list[tuple[str, int | str | float]] = [
    ("zoom_level", "0"),
    ("tile_column", 0),
    ("tile_row", "0"),
]

_ARAMS_ARR = [
    None,
    _PARAMS_LIST,
    [("zoom_level", "0"), ("tile_column", "0"), ("tile_row", "0")],
    {"zoom_level": "0", "tile_column": "0", "tile_row": "0"},
    {"zoom_level": 0, "tile_column": 0, "tile_row": 0},
    ["0", "0", "0"],
]


@pytest.mark.parametrize("params", _ARAMS_ARR)
def test_sqlparams(params: t.Any) -> None:
    sqlfmt_params_obj = ry.sqlfmt_params(
        params,
    )

    # test the repr
    repr_str = "ry." + repr(sqlfmt_params_obj)
    # exec
    round_tripped = eval(repr_str)
    assert sqlfmt_params_obj == round_tripped
    assert not sqlfmt_params_obj != sqlfmt_params_obj
    assert hash(sqlfmt_params_obj) == hash(round_tripped)
    if params is not None:
        assert len(sqlfmt_params_obj) == 3
        assert len(sqlfmt_params_obj) == 3
    else:
        assert len(sqlfmt_params_obj) == 0
        assert len(sqlfmt_params_obj) == 0


@pytest.mark.parametrize("params", _ARAMS_ARR)
def test_sqlparams_from_self(params: t.Any) -> None:
    sqlfmt_params_obj_inner = ry.sqlfmt_params(
        params,
    )
    sqlfmt_params_obj = ry.sqlfmt_params(
        sqlfmt_params_obj_inner,
    )

    # test the repr
    repr_str = "ry." + repr(sqlfmt_params_obj)
    # exec
    round_tripped = eval(repr_str)
    assert sqlfmt_params_obj == round_tripped
    assert not sqlfmt_params_obj != sqlfmt_params_obj
    assert hash(sqlfmt_params_obj) == hash(round_tripped)
    if params is not None:
        assert len(sqlfmt_params_obj) == 3
        assert len(sqlfmt_params_obj) == 3
    else:
        assert len(sqlfmt_params_obj) == 0
        assert len(sqlfmt_params_obj) == 0


@pytest.mark.parametrize("params", _ARAMS_ARR)
def test_sqlparams_pickling(params: t.Any) -> None:
    sqlfmt_params_obj = ry.sqlfmt_params(
        params,
    )
    round_tripped = pickle.loads(pickle.dumps(sqlfmt_params_obj))
    assert sqlfmt_params_obj == round_tripped
