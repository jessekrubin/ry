from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from .conftest import ReqtestServer


class TestRaiseForStatusAsync:
    @pytest.mark.anyio
    async def test_ok_returns_self(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "howdy")
        assert response.raise_for_status() is response
        # body still readable after the check
        assert await response.text()

    @pytest.mark.anyio
    async def test_500_raises(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "500")
        with pytest.raises(ry.ReqwestError, match="500 Internal Server Error") as e:
            response.raise_for_status()
        assert e.value.is_status()
        assert e.value.status == 500
        assert str(e.value.url()).endswith("/500")
        # raising doe doesn't eat the body
        assert await response.text() == "Internal Server Error"

    @pytest.mark.anyio
    async def test_404_raises(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "404-nope")
        with pytest.raises(ry.ReqwestError, match="404 Not Found") as e:
            response.raise_for_status()
        assert e.value.status == 404


class TestRaiseForStatusBlocking:
    def test_ok_returns_self(self, server: ReqtestServer) -> None:
        response = ry.BlockingClient().get(str(server.url) + "howdy")
        assert response.raise_for_status() is response
        assert response.text()

    def test_500_raises(self, server: ReqtestServer) -> None:
        response = ry.BlockingClient().get(str(server.url) + "500")
        with pytest.raises(ry.ReqwestError, match="500 Internal Server Error") as e:
            response.raise_for_status()
        assert e.value.is_status()
        assert e.value.status == 500
        assert response.text() == "Internal Server Error"

    def test_404_raises(self, server: ReqtestServer) -> None:
        response = ry.BlockingClient().get(str(server.url) + "404-nope")
        with pytest.raises(ry.ReqwestError, match="404 Not Found") as e:
            response.raise_for_status()
        assert e.value.status == 404
