"""ryo3-sqlformat types"""

import typing as t

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
) -> str: ...
