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

        async def _do_fetch() -> str:
            res = await ry.fetch(
                url,
                timeout=ry.Duration.from_secs_f64(0.1),
            )
            assert res.status_code == 200
            return await res.text()

        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = await _do_fetch()

    @pytest.mark.anyio
    async def test_fetch_multipart_not_impl(
        self,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = await ry.fetch("http://example.com", method="POST", multipart={"a": 1})


class TestFetchSync:
    def test_fetch_timeout_on_request_sync(self, server: ReqtestServer) -> None:
        url = server.url / "slow"

        def _do_fetch() -> str:
            res = ry.fetch_sync(
                url,
                timeout=ry.Duration.from_secs_f64(0.1),
            )
            return res.text()

        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = _do_fetch()

    def test_fetch_multipart_not_impl_sync(
        self,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = ry.fetch_sync("http://example.com", method="POST", multipart={"a": 1})
