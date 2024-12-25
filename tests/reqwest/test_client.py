from __future__ import annotations

import pytest

import ry.dev as ry

from .conftest import ReqtestServer


@pytest.mark.anyio
async def test_get(server: ReqtestServer) -> None:
    print(server)
    url = server.url
    client = ry.AsyncClient()
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'
    # assert response.http_version == "HTTP/1.1"
    # assert response.headers

    # async with ry.AsyncClient() as client:
    #     response = await client.get(url)
    # assert response.status_code == 200
    # assert response.text == "Hello, world!"
    # assert response.http_version == "HTTP/1.1"
    # assert response.headers
    # assert repr(response) == "<Response [200 OK]>"
    # assert response.elapsed > timedelta(seconds=0)


@pytest.mark.anyio
async def test_get_json(server: ReqtestServer) -> None:
    url = server.url
    client = ry.AsyncClient()
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json == {"howdy": "partner"}
    headers = response.headers
    assert isinstance(headers, dict)
    assert headers["content-type"] == "application/json"


async def test_get_stream(server: ReqtestServer) -> None:
    url = server.url
    client = ry.AsyncClient()
    response = await client.get(str(url) + "long")

    expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
    parts = b""
    async for thing in response.bytes_stream():
        parts += thing
    assert parts == expected
    # async with ry.AsyncClient() as client:
    #     response = await client.get(url)
    #     assert response.status_code == 200
    #     assert response.text == "Hello, world!"
    #     assert response.http_version == "HTTP/1.1"
    #     assert response.headers
    #     assert repr(response) == "<Response [200 OK]>"
    #     assert response.elapsed > timedelta(seconds=0)


async def test_client_default_headers_get(server: ReqtestServer) -> None:
    url = server.url
    client = ry.AsyncClient(headers={"User-Agent": "ry-test", "babydog": "dingo"})
    response = await client.get(str(url) + "echo")
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


async def test_client_post(server: ReqtestServer) -> None:
    url = server.url
    client = ry.AsyncClient()
    response = await client.post(str(url) + "echo", body=b"BOOM")

    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["body"] == "BOOM"
