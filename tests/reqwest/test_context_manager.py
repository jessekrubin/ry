from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from .conftest import ReqtestServer


def test_blocking_client_context_manager_closes(server: ReqtestServer) -> None:
    with ry.BlockingClient() as client:
        response = client.get(server.url / "howdy")
        assert response.status_code == 200

    with pytest.raises(RuntimeError, match="Client is closed"):
        _ = client.get(server.url / "howdy")


async def test_http_client_context_manager_closes(server: ReqtestServer) -> None:
    with ry.HttpClient() as client:
        response = await client.get(server.url / "howdy")
        assert response.status_code == 200

    with pytest.raises(RuntimeError, match="Client is closed"):
        _ = await client.get(server.url / "howdy")


async def test_client_async_context_manager_closes(server: ReqtestServer) -> None:
    async with ry.Client() as client:
        response = await client.get(server.url / "howdy")
        assert response.status_code == 200

    with pytest.raises(RuntimeError, match="Client is closed"):
        _ = await client.get(server.url / "howdy")


def test_blocking_client_context_manager_does_not_suppress_exceptions() -> None:
    with pytest.raises(ZeroDivisionError), ry.BlockingClient():
        _ = 1 / 0


async def test_client_async_context_manager_does_not_suppress_exceptions() -> None:
    with pytest.raises(ZeroDivisionError):
        async with ry.Client():
            _ = 1 / 0
