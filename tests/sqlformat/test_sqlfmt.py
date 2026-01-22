from __future__ import annotations

import pickle
import typing as t

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


class _SqlFormatOptions(
    t.TypedDict,
):
    indent: t.Literal["tabs", "\t"] | int
    uppercase: bool
    dialect: t.Literal["generic", "postgresql", "sqlserver"]
    lines_between_queries: int
    ignore_case_convert: list[str] | None
    inline: bool
    max_inline_block: int
    max_inline_arguments: int | None
    max_inline_top_level: int | None
    joins_as_top_level: bool


_SQL_FORMAT_DEFAULTS: _SqlFormatOptions = {
    "indent": 2,
    "uppercase": False,
    "dialect": "generic",
    "lines_between_queries": 1,
    "ignore_case_convert": None,
    "inline": False,
    "max_inline_block": 50,
    "max_inline_arguments": None,
    "max_inline_top_level": None,
    "joins_as_top_level": False,
}


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


# =============================================================================
# SqlFormatter tests
# =============================================================================


def test_sql_formatter_default() -> None:
    fmt = ry.SqlFormatter()
    formatted = fmt.fmt("select * FROM foo")
    assert formatted == "select\n  *\nFROM\n  foo"


@pytest.mark.parametrize(
    "indent",
    ["\t", "tabs", 4, 2, None],
)
def test_repr_indent(indent: int | str | None) -> None:
    fmt = ry.SqlFormatter(indent=indent)  # type: ignore[arg-type]
    repr_str = repr(fmt)

    assert "SqlFormatter(" in repr_str
    if indent is None:
        assert "indent=2" in repr_str
    elif isinstance(indent, int):
        assert f"indent={indent}" in repr_str
    else:
        assert "indent=-1" in repr_str

    evaluated = eval(repr_str, {"SqlFormatter": ry.SqlFormatter})
    assert isinstance(evaluated, ry.SqlFormatter)
    assert repr(evaluated) == repr_str
    assert evaluated == fmt


def test_sql_formatter_uppercase() -> None:
    fmt = ry.SqlFormatter(uppercase=True)
    formatted = fmt.fmt("select * FROM foo")
    assert formatted == "SELECT\n  *\nFROM\n  foo"


def test_sql_formatter_default_repr() -> None:
    fmt = ry.SqlFormatter()
    repr_str = repr(fmt)
    expected = 'SqlFormatter(indent=2, lines_between_statements=1, inline=False, max_inline_block=50, joins_as_top_level=False, dialect="generic")'
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
def test_repr_ignore_case_convert_strings(icc: list[str] | None) -> None:
    fmt = ry.SqlFormatter(ignore_case_convert=icc)
    repr_str = repr(fmt)
    if icc is None or not icc:
        assert "ignore_case_convert" not in repr_str
    else:
        inner_icc_expected_str = ", ".join(f'"{s}"' for s in icc)
        assert f"ignore_case_convert=[{inner_icc_expected_str}]" in repr_str
    evaluated = eval(repr_str, {"SqlFormatter": ry.SqlFormatter})
    assert isinstance(evaluated, ry.SqlFormatter)
    assert repr(evaluated) == repr_str
    assert evaluated == fmt


def st_sqlformat_options() -> st.SearchStrategy[_SqlFormatOptions]:
    return st.fixed_dictionaries({
        "indent": st.one_of(
            st.integers(min_value=-1, max_value=ry.U8_MAX),
            st.just("tabs"),
            st.just("\t"),
        ),
        "uppercase": st.booleans(),
        "lines_between_queries": st.integers(min_value=0, max_value=ry.U8_MAX),
        "ignore_case_convert": st.one_of(
            st.none(), st.just([]), st.just(["dingo", "mcflurry", "flergen"])
        ),
        "inline": st.booleans(),
        "max_inline_block": st.integers(min_value=0, max_value=ry.U8_MAX),
        "max_inline_arguments": st.one_of(
            st.none(), st.integers(min_value=0, max_value=ry.U8_MAX)
        ),
        "max_inline_top_level": st.one_of(
            st.none(), st.integers(min_value=0, max_value=ry.U8_MAX)
        ),
        "joins_as_top_level": st.booleans(),
        "dialect": st.one_of(
            st.just("generic"),
            st.just("postgresql"),
            st.just("sqlserver"),
        ),
    }).map(
        lambda d: _SqlFormatOptions(**d)  # type: ignore[typeddict-item]
    )


def _canonicalize_options(options: _SqlFormatOptions) -> _SqlFormatOptions:
    """Convert options to their canonical form for comparison with SqlFormatter.to_dict() output."""
    canonical: _SqlFormatOptions = options.copy()
    if (
        canonical["indent"] == -1
        or canonical["indent"] == "\t"
        or canonical["indent"] == "tabs"
    ):
        canonical["indent"] = -1
    if canonical["ignore_case_convert"] is None or not canonical["ignore_case_convert"]:
        canonical["ignore_case_convert"] = None
    return canonical


@given(options=st_sqlformat_options())
def test_sql_formatter_to_dict(options: _SqlFormatOptions) -> None:
    sf = ry.SqlFormatter(**options)
    d = sf.to_dict()
    assert isinstance(d, dict)
    assert d == _canonicalize_options(options)
    assert all(key in d for key in _SQL_FORMAT_DEFAULTS)


@given(options=st_sqlformat_options())
def test_sql_formatter_pickle(options: _SqlFormatOptions) -> None:
    sf = ry.SqlFormatter(**options)
    sf_pickled = pickle.dumps(sf)
    sf_unpickled = pickle.loads(sf_pickled)
    assert sf == sf_unpickled


@given(options=st_sqlformat_options())
def test_sql_formatter_repr_eval(options: _SqlFormatOptions) -> None:
    sf = ry.SqlFormatter(**options)
    repr_str = repr(sf)

    canonical_options = _canonicalize_options(options)
    indent_kwargs = (
        "indent=-1"
        if canonical_options["indent"] == -1
        else f"indent={canonical_options['indent']}"
    )
    ignore_case_convert_kwarg = (
        f"ignore_case_convert={canonical_options['ignore_case_convert']!r}, ".replace(
            "'", '"'
        )
        if canonical_options["ignore_case_convert"]
        else ""
    )

    expected_repr = "".join([
        "SqlFormatter(",
        indent_kwargs + ", ",
        f"uppercase={canonical_options['uppercase']}, ",
        f"lines_between_queries={canonical_options['lines_between_queries']}, ",
        ignore_case_convert_kwarg,
        f"inline={canonical_options['inline']}, ",
        f"max_inline_block={canonical_options['max_inline_block']}, ",
        (
            f"max_inline_arguments={canonical_options['max_inline_arguments']!r}, "
            if canonical_options["max_inline_arguments"] is not None
            else ""
        ),
        (
            f"max_inline_top_level={canonical_options['max_inline_top_level']!r}, "
            if canonical_options["max_inline_top_level"] is not None
            else ""
        ),
        f"joins_as_top_level={canonical_options['joins_as_top_level']}, ",
        f'dialect="{canonical_options["dialect"]}"',
        ")",
    ])
    assert repr_str == expected_repr

    evaluated = eval(repr_str, {"SqlFormatter": ry.SqlFormatter})
    assert isinstance(evaluated, ry.SqlFormatter)
    assert repr(evaluated) == repr_str
    assert evaluated == sf
