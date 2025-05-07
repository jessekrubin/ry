from __future__ import annotations

import pytest

import ry.dev as ry

from .conftest import ReqtestServer


@pytest.mark.anyio
async def test_get(server: ReqtestServer) -> None:
    print(server)
    url = server.url
    client = ry.HttpClient()
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'


@pytest.mark.anyio
async def test_get_query(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    query_params = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = await client.fetch(str(url) + "howdy", query=query_params)
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'

    expected_query = "dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    assert response.url.query == expected_query

    assert isinstance(response.url.query_pairs, tuple)
    assert response.url.query_pairs == (
        ("dog", "dingo"),
        ("is-dingo", "true"),
        ("bluey-fam-size", "4"),
        ("fraction-red-heelers", "0.5"),
    )


@pytest.mark.anyio
async def test_get_query_url_already_has_param(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    query_params = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = await client.fetch(str(url) + "howdy?doggy=bruf", query=query_params)
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'

    expected_query = (
        "doggy=bruf&dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    )
    assert response.url.query == expected_query

    assert response.url.query_pairs == (
        ("doggy", "bruf"),
        ("dog", "dingo"),
        ("is-dingo", "true"),
        ("bluey-fam-size", "4"),
        ("fraction-red-heelers", "0.5"),
    )


@pytest.mark.anyio
async def test_get_url(server: ReqtestServer) -> None:
    url_str = str(server.url) + "howdy"
    url_obj = ry.URL(url_str)
    client = ry.HttpClient()

    response = await client.get(url_obj)
    assert response.status_code == 200
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'


@pytest.mark.anyio
async def test_get_json(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json == {"howdy": "partner"}
    headers = response.headers
    assert isinstance(headers, ry.Headers)
    # assert isinstance(headers, dict)
    assert headers["content-type"] == "application/json"
    headers_dict = dict(headers)
    assert headers_dict["content-type"] == "application/json"


async def test_get_stream(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    response = await client.get(str(url) + "long")

    expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
    parts = b""
    async for thing in response.bytes_stream():
        parts += thing
    assert parts == expected


async def test_client_headers_req(server: ReqtestServer) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.HttpClient()
    headers = {"User-Agent": "ry-test", "babydog": "dingo"}
    response = await client.get(str(url) + "echo", headers=headers)
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


async def test_client_headers_obj_req(server: ReqtestServer) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.HttpClient()
    headers = {"User-Agent": "ry-test", "babydog": "dingo"}
    response = await client.get(str(url) + "echo", headers=ry.Headers(headers))
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


async def test_client_default_headers_get(server: ReqtestServer) -> None:
    """Test that default headers are sent with the request and work good"""
    url = server.url
    client = ry.HttpClient(headers={"User-Agent": "ry-test", "babydog": "dingo"})
    response = await client.get(str(url) + "echo")
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


async def test_client_post(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    response = await client.post(str(url) + "echo", body=b"BABOOM")

    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["body"] == "BABOOM"


async def test_client_timeout_dev(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient(timeout=ry.Duration.from_secs_f64(0.1))
    res = await client.get(str(url) + "slow")
    assert res.status_code == 200
    with pytest.raises(ry.ReqwestError, match="TimedOut"):
        _text = await res.text()


async def test_client_timeout_get_both_same_time(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    res = await client.get(str(url) + "slow")
    text_future = res.text()
    with pytest.raises(ValueError):
        _bytes_future = await res.bytes()
    text = await text_future
    assert text == "".join([f"howdy partner {i}\n" for i in range(10)])


async def test_client_timeout(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient(timeout=ry.Duration.from_secs_f64(0.1))
    with pytest.raises(ry.ReqwestError):
        res = await client.get(str(url) + "slow")
        _text = await res.text()


class TestCookies:
    async def test_client_cookie_jar_cookies_disabled(
        self, server: ReqtestServer
    ) -> None:
        """Test for cookies being set and sent back

        Should not be set in the echo response, as cookies are not enabled
        """

        url = server.url
        client = ry.HttpClient()
        response = await client.get(str(url) + "cookies")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()

        header_set_cookie = response.headers["set-cookie"]
        assert header_set_cookie == "ryo3=ryo3; Path=/"

        # send to echo endpoint
        response = await client.get(str(url) + "echo")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()
        assert "cookie" not in res_json["headers"] or res_json["headers"]["cookie"] in (
            None,
            "",
        ), "cookie should not be set in the echo response"

    async def test_client_cookie_jar_cookies_enabled(
        self, server: ReqtestServer
    ) -> None:
        """Test for cookies being set and sent back

        Should be set in the echo response, as cookies are enabled
        """
        url = server.url
        client = ry.HttpClient(cookies=True)
        response = await client.get(str(url) + "cookies")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()

        header_set_cookie = response.headers["set-cookie"]
        assert header_set_cookie == "ryo3=ryo3; Path=/", (
            f"header_set_cookie: {header_set_cookie}"
        )

        # send to echo endpoint
        response = await client.get(str(url) + "echo")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()
        assert res_json["headers"]["cookie"] == "ryo3=ryo3", f"res_json: {res_json}"
