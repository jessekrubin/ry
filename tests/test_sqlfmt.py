import ry


def test_sqlfmt():
    formatted = ry.sqlfmt("select * FROM foo")
    assert formatted == "SELECT\n  *\nFROM\n  foo"


def test_sqlfmt_with_indent():
    formatted = ry.sqlfmt("SELECT * FROM foo", indent=4)
    assert formatted == "SELECT\n    *\nFROM\n    foo"


def test_sqlfmt_with_indent_and_newline():
    formatted = ry.sqlfmt("SELECT * FROM foo", indent=-1)
    assert formatted == "SELECT\n\t*\nFROM\n\tfoo"


def test_sqlfmt_indexed_params():
    formated = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = ? AND tile_column = ? AND tile_row = ?",
        list(map(str, [0, 0, 0])),
    )
    assert (
        formated
        == "SELECT\n  *\nFROM\n  tiles\nWHERE\n  zoom_level = 0\n  AND tile_column = 0\n  AND tile_row = 0"
    )


def test_sqlfmt_named_params_list_strings():
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        [("zoom_level", "0"), ("tile_column", "0"), ("tile_row", "0")],
    )
    assert (
        formatted
        == "SELECT\n  *\nFROM\n  tiles\nWHERE\n  zoom_level = 0\n  AND tile_column = 0\n  AND tile_row = 0"
    )


def test_sqlfmt_named_params_list():
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        [("zoom_level", "0"), ("tile_column", 0), ("tile_row", "0")],
    )
    assert (
        formatted
        == "SELECT\n  *\nFROM\n  tiles\nWHERE\n  zoom_level = 0\n  AND tile_column = 0\n  AND tile_row = 0"
    )


def test_sqlfmt_named_params_dict_strings():
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        {"zoom_level": "0", "tile_column": "0", "tile_row": "0"},
    )
    assert (
        formatted
        == "SELECT\n  *\nFROM\n  tiles\nWHERE\n  zoom_level = 0\n  AND tile_column = 0\n  AND tile_row = 0"
    )


def test_sqlfmt_named_params_dict_ints():
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        {"zoom_level": 0, "tile_column": 0, "tile_row": 0},
    )
    assert (
        formatted
        == "SELECT\n  *\nFROM\n  tiles\nWHERE\n  zoom_level = 0\n  AND tile_column = 0\n  AND tile_row = 0"
    )
