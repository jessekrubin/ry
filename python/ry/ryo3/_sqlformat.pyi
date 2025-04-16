import typing as t

import typing_extensions

# =============================================================================
# SQLFORMAT
# =============================================================================
SqlfmtParamValue: typing_extensions.TypeAlias = str | int | float | bool
_TSqlfmtParamValue_co = t.TypeVar(
    "_TSqlfmtParamValue_co", bound=SqlfmtParamValue, covariant=True
)
SqlfmtParamsLike: typing_extensions.TypeAlias = (
    dict[str, _TSqlfmtParamValue_co]
    | t.Sequence[tuple[str, _TSqlfmtParamValue_co]]
    | t.Sequence[_TSqlfmtParamValue_co]
)

class SqlfmtQueryParams:
    def __init__(self, params: SqlfmtParamsLike[_TSqlfmtParamValue_co]) -> None: ...

def sqlfmt_params(
    params: SqlfmtParamsLike[_TSqlfmtParamValue_co] | SqlfmtQueryParams,
) -> SqlfmtQueryParams: ...
def sqlfmt(
    sql: str,
    params: SqlfmtParamsLike[_TSqlfmtParamValue_co] | SqlfmtQueryParams | None = None,
    *,
    indent: int = 2,  # -1 or any negative value will use tabs
    uppercase: bool | None = True,
    lines_between_statements: int = 1,
) -> str: ...
