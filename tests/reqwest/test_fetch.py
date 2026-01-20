from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from .conftest import ReqtestServer


class TestFetch:
    @pytest.mark.anyio
    async def test_fetch_timeout_on_request(self, server: ReqtestServer) -> None:
        url = server.url / "slow"
        res = await ry.fetch(
            url,
            timeout=ry.Duration.from_secs_f64(0.1),
        )
        assert res.status_code == 200
        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = await res.text()

    @pytest.mark.anyio
    async def test_fetch_multipart(self, server: ReqtestServer) -> None:
        url = server.url / "upload"
        multipart = ry.FormData(ry.FormPart("field", "value"))
        res = await ry.fetch(url, method="POST", multipart=multipart)
        assert res.status_code == 200
        payload = await res.json()
        assert payload["received_bytes"] > 0
        assert payload["content_type"].startswith("multipart/form-data")


class TestFetchSync:
    def test_fetch_timeout_on_request_sync(self, server: ReqtestServer) -> None:
        url = server.url / "slow"
        res = ry.fetch_sync(
            url,
            timeout=ry.Duration.from_secs_f64(0.1),
        )

        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = res.text()

    def test_fetch_multipart_sync(self, server: ReqtestServer) -> None:
        url = server.url / "upload"
        multipart = ry.FormData(ry.FormPart("field", "value"))
        res = ry.fetch_sync(url, method="POST", multipart=multipart)
        assert res.status_code == 200
        payload = res.json()
        assert payload["received_bytes"] > 0
        assert payload["content_type"].startswith("multipart/form-data")
