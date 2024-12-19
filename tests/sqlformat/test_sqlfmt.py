from __future__ import annotations

import ry


def test_sqlfmt_params() -> None:
    params = ry.sqlfmt_params([1, 2])
    assert str(params) == 'SqlfmtQueryParams(["1", "2"])'


def test_sqlfmt() -> None:
    formatted = ry.sqlfmt("select * FROM foo")
    assert formatted == "SELECT\n  *\nFROM\n  foo"


def test_sqlfmt_with_indent() -> None:
    formatted = ry.sqlfmt("SELECT * FROM foo", indent=4)
    assert formatted == "SELECT\n    *\nFROM\n    foo"


def test_sqlfmt_with_indent_and_newline() -> None:
    formatted = ry.sqlfmt("SELECT * FROM foo", indent=-1)
    assert formatted == "SELECT\n\t*\nFROM\n\tfoo"


def test_sqlfmt_indexed_params() -> None:
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = ? AND tile_column = ? AND tile_row = ?",
        [0, 0, 0],
    )
    expected = ry.unindent(
        """
        SELECT
          *
        FROM
          tiles
        WHERE
          zoom_level = 0
          AND tile_column = 0
          AND tile_row = 0
        """
    )
    assert formatted == expected.strip()


def test_sqlfmt_named_params_list_strings() -> None:
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        [("zoom_level", "0"), ("tile_column", "0"), ("tile_row", "0")],
    )
    expected = ry.unindent(
        """
        SELECT
          *
        FROM
          tiles
        WHERE
          zoom_level = 0
          AND tile_column = 0
          AND tile_row = 0
        """
    )
    assert formatted == expected.strip()


def test_sqlfmt_named_params_list() -> None:
    params: list[tuple[str, int | str | float]] = [
        ("zoom_level", "0"),
        ("tile_column", 0),
        ("tile_row", "0"),
    ]
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        params,
    )
    expected = ry.unindent(
        """
        SELECT
          *
        FROM
          tiles
        WHERE
          zoom_level = 0
          AND tile_column = 0
          AND tile_row = 0
        """
    )
    assert formatted == expected.strip()


def test_sqlfmt_named_params_dict_strings() -> None:
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        {"zoom_level": "0", "tile_column": "0", "tile_row": "0"},
    )
    expected = ry.unindent(
        """
        SELECT
          *
        FROM
          tiles
        WHERE
          zoom_level = 0
          AND tile_column = 0
          AND tile_row = 0
        """
    )
    assert formatted == expected.strip()


def test_sqlfmt_named_params_dict_ints() -> None:
    formatted = ry.sqlfmt(
        "SELECT * FROM tiles WHERE zoom_level = :zoom_level AND tile_column = :tile_column AND tile_row = :tile_row",
        {"zoom_level": 0, "tile_column": 0, "tile_row": 0},
    )
    expected = ry.unindent(
        """
        SELECT
          *
        FROM
          tiles
        WHERE
          zoom_level = 0
          AND tile_column = 0
          AND tile_row = 0
        """
    )
    assert formatted == expected.strip()
