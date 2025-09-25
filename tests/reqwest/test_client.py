from __future__ import annotations

import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from .conftest import ReqtestServer


@pytest.mark.anyio
async def test_get(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    response = await client.get(str(url) + "howdy")
    assert not response.body_used
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_text = await response.text()
    assert response.body_used
    assert res_text == '{"howdy": "partner"}'
    assert response.ok
    assert bool(response)
    assert f"{response!r}" == f"<Response [{response.status_code}; {response.url}]>"
    assert response.content_encoding is None
    if response.remote_addr is not None:
        assert isinstance(response.remote_addr, ry.SocketAddr)
    assert response.content_length == len(res_text)


@pytest.mark.anyio
async def test_bytes(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_bin = await response.bytes()
    assert res_bin == b'{"howdy": "partner"}'
    assert response.ok
    assert bool(response)


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
@pytest.mark.parametrize("method", ["post", "put", "patch", "delete"])
@pytest.mark.parametrize(
    "form_data",
    [
        {
            "dog": "dingo",
            "is-dingo": True,
            "bluey-fam-size": 4,
            "fraction-red-heelers": 2 / 4,
        },
        [
            ("dog", "dingo"),
            ("is-dingo", True),
            ("bluey-fam-size", 4),
            ("fraction-red-heelers", 2 / 4),
        ],
    ],
)
async def test_form_data(server: ReqtestServer, method: str, form_data: t.Any) -> None:
    url = server.url / "echo"
    client = ry.HttpClient()
    form_data = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = await client.fetch(
        url,
        method=method,
        form=form_data,
    )
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert response.http_version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_json = await response.json()

    expected_body = "dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    assert res_json["body"] == expected_body


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
    assert headers["content-type"] == "application/json"
    headers_dict = dict(headers)
    assert headers_dict["content-type"] == "application/json"


class TestResponseJson:
    @pytest.mark.anyio
    async def test_get_json_broken_is_broken(self, server: ReqtestServer) -> None:
        url = server.url / "broken-json"
        client = ry.HttpClient()
        response = await client.get(url)
        with pytest.raises(ValueError):
            _data = await response.json()

    @pytest.mark.anyio
    async def test_get_json_broken_is_broken_allow_partial(
        self, server: ReqtestServer
    ) -> None:
        url = server.url / "broken-json"
        client = ry.HttpClient()
        response = await client.get(url)
        data = await response.json(partial_mode=True)
        expected = {
            "dog": "dingo",
            "is-dingo": True,
            "bluey-fam-size": 4,
            "fraction-red-heelers": 0.5,
            "activities": [
                "screwing up the garden",
                "barking at strangers for exisiting",
            ],
        }
        assert data == expected


class TestStream:
    @pytest.mark.anyio
    @staticmethod
    async def test_get_bytes_stream(server: ReqtestServer) -> None:
        url = server.url
        client = ry.HttpClient()
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        parts = b""
        async for thing in response.bytes_stream():
            parts += thing
        assert parts == expected

    @pytest.mark.anyio
    @staticmethod
    async def test_get_stream(server: ReqtestServer) -> None:
        url = server.url
        client = ry.HttpClient()
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        parts = b""
        async for thing in response.stream():
            parts += thing
        assert parts == expected

    @pytest.mark.anyio
    @staticmethod
    async def test_get_stream_take_collect(server: ReqtestServer) -> None:
        url = server.url
        client = ry.HttpClient()
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        response_stream = response.bytes_stream()

        take1 = await response_stream.take(1)
        take2 = await response_stream.take(2)
        assert len(take1) == 1
        assert len(take2) == 2
        rest = await response_stream.collect()
        joined = b"".join(take1 + take2 + rest)
        assert joined == expected
        expected_len = len(expected) - (
            sum(len(t) for t in take1) + sum(len(t) for t in take2)
        )
        rest_total_inner_len = sum(len(t) for t in rest)
        assert rest_total_inner_len == expected_len

    @pytest.mark.anyio
    @staticmethod
    async def test_get_stream_collect_join(server: ReqtestServer) -> None:
        url = server.url
        client = ry.HttpClient()
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        response_stream = response.bytes_stream()
        collected = await response_stream.collect(join=True)
        assert isinstance(collected, ry.Bytes)
        assert collected == expected


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


async def test_client_post_json(server: ReqtestServer) -> None:
    url = server.url
    client = ry.HttpClient()
    response = await client.post(str(url) + "echo", json={"body": "BABOOM"})
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["content-type"] == "application/json"
    assert res_json["body"] == '{"body":"BABOOM"}'


async def test_client_post_json_and_form_errors(server: ReqtestServer) -> None:
    url = server.url / "echo"
    client = ry.HttpClient()
    with pytest.raises(
        ValueError, match="body, json, form, multipart are mutually exclusive"
    ):
        _response = await client.post(url, json={"body": "BABOOM"}, form={"a": 1})


class TestTimeout:
    async def test_client_timeout_dev(self, server: ReqtestServer) -> None:
        url = server.url
        client = ry.HttpClient(timeout=ry.Duration.from_secs_f64(0.1))
        res = await client.get(str(url) + "slow")
        assert res.status_code == 200
        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = await res.text()

    async def test_client_timeout_on_request(self, server: ReqtestServer) -> None:
        url = server.url / "slow"
        res = await ry.fetch(
            url,
            timeout=ry.Duration.from_secs_f64(0.1),
        )
        assert res.status_code == 200
        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = await res.text()

    async def test_client_timeout_get_both_same_time(
        self, server: ReqtestServer
    ) -> None:
        url = server.url
        client = ry.HttpClient()
        res = await client.get(str(url) + "slow")
        text_future = res.text()
        with pytest.raises(ValueError):
            _bytes_future = await res.bytes()
        text = await text_future
        assert text == "".join([f"howdy partner {i}\n" for i in range(10)])

    async def test_client_timeout(self, server: ReqtestServer) -> None:
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


class TestTodo:
    def test_response_new_errs(self) -> None:
        with pytest.raises(NotImplementedError):
            _res = ry.Response()

    @pytest.mark.anyio
    async def test_post_multipart_not_impl(
        self,
    ) -> None:
        c = ry.HttpClient()
        with pytest.raises(NotImplementedError):
            _r = await c.post("http://example.com", multipart={"a": 1})

    @pytest.mark.anyio
    async def test_client_fetch_multipart_not_impl(
        self,
    ) -> None:
        c = ry.HttpClient()
        with pytest.raises(NotImplementedError):
            _r = await c.fetch("http://example.com", method="POST", multipart={"a": 1})

    @pytest.mark.anyio
    async def test_fetch_multipart_not_impl(
        self,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = await ry.fetch("http://example.com", method="POST", multipart={"a": 1})
