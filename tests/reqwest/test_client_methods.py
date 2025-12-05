from __future__ import annotations

import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from ry.ryo3 import RequestKwargs

    from .conftest import ReqtestServer


@pytest.mark.parametrize(
    "method,options",
    [
        ("get", {}),
        ("post", {}),
        ("put", {}),
        ("delete", {}),
        ("patch", {}),
        ("head", {}),
        ("options", {}),
    ],
)
@pytest.mark.parametrize(
    "use_cls_callable",
    [True, False],
)
async def test_client_methods(
    server: ReqtestServer,
    method: t.Literal[
        "get", "post", "put", "delete", "patch", "head", "options", "__call__"
    ],
    options: RequestKwargs,
    *,
    use_cls_callable: bool,
) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.HttpClient()
    if use_cls_callable:
        response = await client(str(url) + "echo", method=method, **options)
    else:
        response = await getattr(client, method)(str(url) + "echo", **options)
    assert response.status_code == 200
    assert response.headers["x-request-method"] == method.upper()
    if method.lower() != "head":
        response_data = await response.json()
        assert response_data["method"].lower() == method


@pytest.mark.parametrize(
    "method,options",
    [
        ("get", {}),
        ("post", {}),
        ("put", {}),
        ("delete", {}),
        ("patch", {}),
        ("head", {}),
        ("options", {}),
    ],
)
@pytest.mark.parametrize(
    "use_cls_callable",
    [True, False],
)
def test_blocking_client_methods(
    server: ReqtestServer,
    method: t.Literal["get", "post", "put", "delete", "patch", "head", "options"],
    options: RequestKwargs,
    *,
    use_cls_callable: bool,
) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.BlockingClient()
    if use_cls_callable:
        response = client(str(url) + "echo", method=method, **options)
    else:
        response = getattr(client, method)(str(url) + "echo", **options)
    assert response.status_code == 200
    assert response.headers["x-request-method"] == method.upper()
    if method.lower() != "head":
        response_data = response.json()
        assert response_data["method"].lower() == method
