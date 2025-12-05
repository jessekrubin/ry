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
async def test_client_methods(
    server: ReqtestServer,
    method: t.Literal["get", "post", "put", "delete", "patch", "head", "options"],
    options: RequestKwargs,
) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.HttpClient()
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
def test_blocking_client_methods(
    server: ReqtestServer,
    method: t.Literal["get", "post", "put", "delete", "patch", "head", "options"],
    options: RequestKwargs,
) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.BlockingClient()
    response = getattr(client, method)(str(url) + "echo", **options)
    assert response.status_code == 200
    assert response.headers["x-request-method"] == method.upper()
    if method.lower() != "head":
        response_data = response.json()
        assert response_data["method"].lower() == method
