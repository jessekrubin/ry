"""ry.ryo3.dev"""

from __future__ import annotations

import typing as t

# =============================================================================
# SUBPROCESS (VERY MUCH WIP)
# =============================================================================
def run(
    *args: str | list[str],
    capture_output: bool = True,
    input: bytes | None = None,
) -> t.Any: ...

# =============================================================================
# STRING-DEV
# =============================================================================

def anystr_noop(s: t.AnyStr) -> t.AnyStr: ...
def string_noop(s: str) -> str: ...
def bytes_noop(s: bytes) -> bytes: ...

# =============================================================================
# REQWEST
# =============================================================================

class AsyncClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
        gzip: bool = True,
        brotli: bool = True,
        deflate: bool = True,
    ) -> None: ...
    async def get(
        self, url: str, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def post(
        self,
        url: str,
        *,
        body: bytes | None = None,
    ) -> Response: ...
    async def put(
        self,
        url: str,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def delete(
        self,
        url: str,
        *,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def head(
        self,
        url: str,
        *,
        headers: dict[str, str] | None = None,
    ) -> Response: ...

class Response:
    status_code: int
    headers: dict[str, str]

    async def text(self) -> str: ...
    async def json(self) -> t.Any: ...
    async def bytes(self) -> bytes: ...
    def bytes_stream(self) -> ResponseStream: ...

class ResponseStream:
    def __aiter__(self) -> ResponseStream: ...
    async def __anext__(self) -> bytes: ...
