from __future__ import annotations

import typing as t

import pytest

import ry


def test_sqlfmt_params() -> None:
    params = ry.sqlfmt_params([1, 2])
    assert str(params) == 'SqlfmtQueryParams(["1", "2"])'


def test_sqlfmt() -> None:
    formatted = ry.sqlfmt("select * FROM foo", uppercase=True)
    assert formatted == "SELECT\n  *\nFROM\n  foo"


def test_sqlfmt_with_indent() -> None:
    formatted = ry.sqlfmt("SELECT * FROM foo", indent=4)
    assert formatted == "SELECT\n    *\nFROM\n    foo"


@pytest.mark.parametrize(
    "indent",
    [
        "badstring",
        3.5,
    ],
)
def test_sqlfmt_with_indent_invalid(
    indent: str | float,
) -> None:
    with pytest.raises(TypeError):
        ry.sqlfmt("SELECT * FROM foo", indent=indent)  # type: ignore[arg-type]


@pytest.mark.parametrize("indent", [-1, "\t", "tabs"])
def test_sqlfmt_with_indent_tabs_and_newline(
    indent: int | t.Literal["\t", "tabs"],
) -> None:
    formatted = ry.sqlfmt("SELECT * FROM foo", indent=indent)
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


def test_sqlfmt_named_params_params_obj() -> None:
    params = ry.sqlfmt_params({"zoom_level": 0, "tile_column": 0, "tile_row": 0})
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


class TestSqlFormatter:
    def test_sql_formatter_default(self) -> None:
        fmt = ry.SqlFormatter()
        formatted = fmt.fmt("select * FROM foo")
        assert formatted == "select\n  *\nFROM\n  foo"

    @pytest.mark.parametrize(
        "indent",
        ["\t", "tabs", 4, 2, None],
    )
    def test_repr_indent(self, indent: int | str | None) -> None:
        if indent is None:
            fmt = ry.SqlFormatter()
        else:
            fmt = ry.SqlFormatter(indent=indent)  # type: ignore[arg-type]
        repr_str = repr(fmt)

        assert "SqlFormatter(" in repr_str
        if indent is None:
            assert "indent=2" in repr_str
        elif isinstance(indent, int):
            assert f"indent={indent}" in repr_str
        else:
            assert 'indent="\t"' in repr_str

        evaluated = eval(repr_str, {"SqlFormatter": ry.SqlFormatter})
        assert isinstance(evaluated, ry.SqlFormatter)
        assert repr(evaluated) == repr_str
        assert evaluated == fmt

    def test_sql_formatter_uppercase(self) -> None:
        fmt = ry.SqlFormatter(uppercase=True)
        formatted = fmt.fmt("select * FROM foo")
        assert formatted == "SELECT\n  *\nFROM\n  foo"

    def test_sql_formatter_default_repr(self) -> None:
        fmt = ry.SqlFormatter()
        repr_str = repr(fmt)
        expected = "".join((
            "SqlFormatter(",
            "indent=2, ",
            "lines_between_statements=1, ",
            "inline=False, ",
            "max_inline_block=50, ",
            "joins_as_top_level=False, ",
            'dialect="generic"',
            ")",
        ))
        expected = 'SqlFormatter(indent=2, lines_between_queries=1, inline=False, max_inline_block=50, joins_as_top_level=False, dialect="generic")'
        assert repr_str == expected

    @pytest.mark.parametrize(
        "icc",
        [
            None,
            [],
            ["foo"],
            ["foo", "bar", "baz"],
        ],
    )
    def test_repr_ignore_case_convert_strings(self, icc: list[str] | None) -> None:
        fmt = ry.SqlFormatter(
            ignore_case_convert=icc  # type: ignore[arg-type]
        )
        repr_str = repr(fmt)
        if icc is None:
            assert "ignore_case_convert" not in repr_str
        else:
            inner_icc_expected_str = ", ".join(f'"{s}"' for s in icc)
            assert f"ignore_case_convert=[{inner_icc_expected_str}]" in repr_str
        evaluated = eval(repr_str, {"SqlFormatter": ry.SqlFormatter})
        assert isinstance(evaluated, ry.SqlFormatter)
        assert repr(evaluated) == repr_str
        assert evaluated == fmt
