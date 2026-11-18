from __future__ import annotations

import ssl
import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pathlib import Path

    from .conftest import ReqtestServer


def _der(cert_path: Path) -> bytes:
    return ssl.PEM_cert_to_DER_cert(cert_path.read_text())


_CLIENT_KW: dict[str, t.Any] = {"tls_danger_accept_invalid_certs": True}


class TestTlsInfoAsync:
    @pytest.mark.anyio
    async def test_tls_info_on(
        self, https_server: ReqtestServer, https_cert: tuple[Path, Path]
    ) -> None:
        client = ry.Client(tls_info=True, **_CLIENT_KW)
        response = await client.get(str(https_server.url) + "howdy")
        cert = response.peer_certificate
        assert isinstance(cert, ry.Certificate)
        assert bytes(cert) == _der(https_cert[0])
        assert response.tls_version in ("1.2", "1.3")

    @pytest.mark.anyio
    async def test_tls_info_off(self, https_server: ReqtestServer) -> None:
        response = await ry.Client(**_CLIENT_KW).get(str(https_server.url) + "howdy")
        assert response.peer_certificate is None
        assert response.tls_version is None

    @pytest.mark.anyio
    async def test_plain_http(self, server: ReqtestServer) -> None:
        response = await ry.Client(tls_info=True).get(str(server.url) + "howdy")
        assert response.peer_certificate is None
        assert response.tls_version is None


class TestTlsInfoBlocking:
    def test_tls_info_on(
        self, https_server: ReqtestServer, https_cert: tuple[Path, Path]
    ) -> None:
        client = ry.BlockingClient(tls_info=True, **_CLIENT_KW)
        response = client.get(str(https_server.url) + "howdy")
        cert = response.peer_certificate
        assert isinstance(cert, ry.Certificate)
        assert bytes(cert) == _der(https_cert[0])
        assert response.tls_version in ("1.2", "1.3")

    def test_tls_info_off(self, https_server: ReqtestServer) -> None:
        client = ry.BlockingClient(**_CLIENT_KW)
        response = client.get(str(https_server.url) + "howdy")
        assert response.peer_certificate is None
        assert response.tls_version is None
