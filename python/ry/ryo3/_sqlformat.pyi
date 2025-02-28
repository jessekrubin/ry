from __future__ import annotations

import typing as t

# =============================================================================
# SQLFORMAT
# =============================================================================
SqlfmtParamValue = str | int | float | bool
TSqlfmtParamValue_co = t.TypeVar(
    "TSqlfmtParamValue_co", bound=SqlfmtParamValue, covariant=True
)
SqlfmtParamsLike = (
    dict[str, TSqlfmtParamValue_co]
    | t.Sequence[tuple[str, TSqlfmtParamValue_co]]
    | t.Sequence[TSqlfmtParamValue_co]
)

class SqlfmtQueryParams:
    def __init__(self, params: SqlfmtParamsLike[TSqlfmtParamValue_co]) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

def sqlfmt_params(
    params: SqlfmtParamsLike[TSqlfmtParamValue_co] | SqlfmtQueryParams,
) -> SqlfmtQueryParams: ...
def sqlfmt(
    sql: str,
    params: SqlfmtParamsLike[TSqlfmtParamValue_co] | SqlfmtQueryParams | None = None,
    *,
    indent: int = 2,  # -1 or any negative value will use tabs
    uppercase: bool | None = True,
    lines_between_statements: int = 1,
) -> str: ...
