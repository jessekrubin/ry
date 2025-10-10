"""ryo3-sqlformat types"""

import typing as t

Dialect: t.TypeAlias = t.Literal["generic", "postgresql", "sqlserver"]
Indent: t.TypeAlias = t.Literal["tabs", "\t"] | int
SqlfmtParamValue: t.TypeAlias = str | int | float | bool
_TSqlfmtParamValue_co = t.TypeVar(
    "_TSqlfmtParamValue_co", bound=SqlfmtParamValue, covariant=True
)
SqlfmtParamsLike: t.TypeAlias = (
    dict[str, _TSqlfmtParamValue_co]
    | t.Sequence[tuple[str, _TSqlfmtParamValue_co]]
    | t.Sequence[_TSqlfmtParamValue_co]
)

class SqlfmtQueryParams:
    def __init__(self, params: SqlfmtParamsLike[_TSqlfmtParamValue_co]) -> None: ...
    def __len__(self) -> int: ...

def sqlfmt_params(
    params: SqlfmtParamsLike[_TSqlfmtParamValue_co] | SqlfmtQueryParams,
) -> SqlfmtQueryParams: ...
def sqlfmt(
    sql: str,
    params: SqlfmtParamsLike[_TSqlfmtParamValue_co] | SqlfmtQueryParams | None = None,
    *,
    indent: int | t.Literal["tabs", "\t"] = 2,
    uppercase: bool = False,
    lines_between_statements: int = 1,
    ignore_case_convert: list[str] | None = None,
    inline: bool = False,
    max_inline_block: int = 50,
    max_inline_arguments: int | None = None,
    max_inline_top_level: int | None = None,
    joins_as_top_level: bool = False,
    dialect: t.Literal["generic", "postgresql", "sqlserver"] = "generic",
) -> str: ...

class _SqlFormatterDict(t.TypedDict):
    indent: int | t.Literal["tabs"]
    uppercase: bool
    lines_between_queries: int
    ignore_case_convert: list[str] | None
    inline: bool
    max_inline_block: int
    max_inline_arguments: int | None
    max_inline_top_level: int | None
    joins_as_top_level: bool
    dialect: t.Literal["generic", "postgresql", "sqlserver"]

class SqlFormatter:
    def __init__(
        self,
        *,
        indent: int | t.Literal["tabs", "\t"] = 2,
        uppercase: bool = False,
        lines_between_statements: int = 1,
        ignore_case_convert: list[str] | None = None,
        inline: bool = False,
        max_inline_block: int = 50,
        max_inline_arguments: int | None = None,
        max_inline_top_level: int | None = None,
        joins_as_top_level: bool = False,
        dialect: t.Literal["generic", "postgresql", "sqlserver"] = "generic",
    ) -> None: ...
    def to_dict(self) -> _SqlFormatterDict: ...
    def fmt(
        self,
        sql: str,
        params: SqlfmtParamsLike[_TSqlfmtParamValue_co]
        | SqlfmtQueryParams
        | None = None,
    ) -> str: ...
    def __call__(
        self,
        sql: str,
        params: SqlfmtParamsLike[_TSqlfmtParamValue_co]
        | SqlfmtQueryParams
        | None = None,
    ) -> str: ...
    def __eq__(self, value: object) -> bool: ...
    def __ne__(self, value: object) -> bool: ...
