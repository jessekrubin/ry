from __future__ import annotations

import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from .conftest import ReqtestServer


def test_get(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    response = client.get(str(url) + "howdy")
    assert not response.body_used
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_text = response.text()
    assert response.body_used
    assert res_text == '{"howdy": "partner"}'
    assert response.ok
    assert bool(response)
    assert (
        f"{response!r}"
        == f"<BlockingResponse [{response.status_code}; {response.url}]>"
    )
    assert response.content_encoding is None
    if response.remote_addr is not None:
        assert isinstance(response.remote_addr, ry.SocketAddr)
    assert response.content_length == len(res_text)


def test_bytes(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    response = client.get(str(url) + "howdy")
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_bin = response.bytes()
    assert res_bin == b'{"howdy": "partner"}'
    assert response.ok
    assert bool(response)


def test_get_query(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    query_params = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = client.fetch(str(url) + "howdy", query=query_params)
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_text = response.text()
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
def test_form_data(server: ReqtestServer, method: str, form_data: t.Any) -> None:
    url = server.url / "echo"
    client = ry.BlockingClient()
    form_data = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = client.fetch(
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
    res_json = response.json()

    expected_body = "dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    assert res_json["body"] == expected_body


def test_get_query_url_already_has_param(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    query_params = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = client.fetch(str(url) + "howdy?doggy=bruf", query=query_params)
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_text = response.text()
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


def test_get_url(server: ReqtestServer) -> None:
    url_str = str(server.url) + "howdy"
    url_obj = ry.URL(url_str)
    client = ry.BlockingClient()

    response = client.get(url_obj)
    assert response.status_code == 200
    res_text = response.text()
    assert res_text == '{"howdy": "partner"}'


def test_get_json(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    response = client.get(str(url) + "howdy")
    assert response.status_code == 200
    res_json = response.json()
    assert res_json == {"howdy": "partner"}
    headers = response.headers
    assert isinstance(headers, ry.Headers)
    assert headers["content-type"] == "application/json"
    headers_dict = dict(headers)
    assert headers_dict["content-type"] == "application/json"


class TestResponseJson:
    def test_get_json_broken_is_broken(self, server: ReqtestServer) -> None:
        url = server.url / "broken-json"
        client = ry.BlockingClient()
        response = client.get(url)
        with pytest.raises(ValueError):
            _data = response.json()

    def test_get_json_broken_is_broken_allow_partial(
        self, server: ReqtestServer
    ) -> None:
        url = server.url / "broken-json"
        client = ry.BlockingClient()
        response = client.get(url)
        data = response.json(partial_mode=True)
        expected = {
            "dog": "dingo",
            "is-dingo": True,
            "bluey-fam-size": 4,
            "fraction-red-heelers": 0.5,
            "activities": [
                "screwing up the garden",
                "barking at strangers for existing",
            ],
        }
        assert data == expected


class TestStream:
    @staticmethod
    def test_get_bytes_stream(server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient()
        response = client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        parts = b""
        for thing in response.bytes_stream():
            parts += thing
        assert parts == expected

    @staticmethod
    def test_get_stream(server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient()
        response = client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        parts = b""
        for thing in response.stream():
            parts += thing
        assert parts == expected

    @staticmethod
    def test_get_stream_take_collect(server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient()
        response = client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        response_stream = response.bytes_stream()

        take1 = response_stream.take(1)
        take2 = response_stream.take(2)
        assert len(take1) == 1
        assert len(take2) == 2
        rest = response_stream.collect()
        joined = b"".join(take1 + take2 + rest)
        assert joined == expected
        expected_len = len(expected) - (
            sum(len(t) for t in take1) + sum(len(t) for t in take2)
        )
        rest_total_inner_len = sum(len(t) for t in rest)
        assert rest_total_inner_len == expected_len

    @staticmethod
    def test_get_stream_collect_join(server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient()
        response = client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        response_stream = response.bytes_stream()
        collected = response_stream.collect(join=True)
        assert isinstance(collected, ry.Bytes)
        assert collected == expected


def test_client_headers_req(server: ReqtestServer) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.BlockingClient()
    headers = {"User-Agent": "ry-test", "babydog": "dingo"}
    response = client.get(str(url) + "echo", headers=headers)
    assert response.status_code == 200
    res_json = response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


def test_client_headers_obj_req(server: ReqtestServer) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = ry.BlockingClient()
    headers = {"User-Agent": "ry-test", "babydog": "dingo"}
    response = client.get(str(url) + "echo", headers=ry.Headers(headers))
    assert response.status_code == 200
    res_json = response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


def test_client_default_headers_get(server: ReqtestServer) -> None:
    """Test that default headers are sent with the request and work good"""
    url = server.url
    client = ry.BlockingClient(headers={"User-Agent": "ry-test", "babydog": "dingo"})
    response = client.get(str(url) + "echo")
    assert response.status_code == 200
    res_json = response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


def test_client_post(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    response = client.post(str(url) + "echo", body=b"BABOOM")

    assert response.status_code == 200
    res_json = response.json()
    assert res_json["body"] == "BABOOM"


def test_client_post_json(server: ReqtestServer) -> None:
    url = server.url
    client = ry.BlockingClient()
    response = client.post(str(url) + "echo", json={"body": "BABOOM"})
    assert response.status_code == 200
    res_json = response.json()
    assert res_json["headers"]["content-type"] == "application/json"
    assert res_json["body"] == '{"body":"BABOOM"}'


def test_client_post_json_and_form_errors(server: ReqtestServer) -> None:
    url = server.url / "echo"
    client = ry.BlockingClient()
    with pytest.raises(
        ValueError, match="body, json, form, multipart are mutually exclusive"
    ):
        _response = client.post(url, json={"body": "BABOOM"}, form={"a": 1})


class TestTimeout:
    def test_client_timeout_dev(self, server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient(timeout=ry.Duration.from_secs_f64(0.1))
        res = client.get(str(url) + "slow")
        assert res.status_code == 200
        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = res.text()

    def test_client_timeout_get_both_same_time(self, server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient()
        res = client.get(str(url) + "slow")
        text_future = res.text()
        with pytest.raises(ValueError):
            _bytes_future = res.bytes()
        text = text_future
        assert text == "".join([f"howdy partner {i}\n" for i in range(10)])

    def test_client_timeout(self, server: ReqtestServer) -> None:
        url = server.url
        client = ry.BlockingClient(timeout=ry.Duration.from_secs_f64(0.1))
        with pytest.raises(ry.ReqwestError):
            res = client.get(str(url) + "slow")
            _text = res.text()


class TestCookies:
    def test_client_cookie_jar_cookies_disabled(self, server: ReqtestServer) -> None:
        """Test for cookies being set and sent back

        Should not be set in the echo response, as cookies are not enabled
        """

        url = server.url
        client = ry.BlockingClient()
        response = client.get(str(url) + "cookies")
        assert response.status_code == 200, f"response: {response}"
        res_json = response.json()

        header_set_cookie = response.headers["set-cookie"]
        assert header_set_cookie == "ryo3=ryo3; Path=/"

        # send to echo endpoint
        response = client.get(str(url) + "echo")
        assert response.status_code == 200, f"response: {response}"
        res_json = response.json()
        assert "cookie" not in res_json["headers"] or res_json["headers"]["cookie"] in (
            None,
            "",
        ), "cookie should not be set in the echo response"

    def test_client_cookie_jar_cookies_enabled(self, server: ReqtestServer) -> None:
        """Test for cookies being set and sent back

        Should be set in the echo response, as cookies are enabled
        """
        url = server.url
        client = ry.BlockingClient(cookies=True)
        response = client.get(str(url) + "cookies")
        assert response.status_code == 200, f"response: {response}"
        _res_json = response.json()

        c = response.cookies
        assert isinstance(c, list) and len(c) == 1

        assert isinstance(c[0], ry.Cookie)
        assert c[0].name == "ryo3"
        assert c[0].value == "ryo3"
        assert c[0].path == "/"

        header_set_cookie = response.headers["set-cookie"]
        assert header_set_cookie == "ryo3=ryo3; Path=/", (
            f"header_set_cookie: {header_set_cookie}"
        )

        # send to echo endpoint
        response = client.get(str(url) + "echo")
        assert response.status_code == 200, f"response: {response}"
        res_json = response.json()
        assert res_json["headers"]["cookie"] == "ryo3=ryo3", f"res_json: {res_json}"


class TestTodo:
    def test_response_new_errs(self) -> None:
        with pytest.raises(NotImplementedError):
            _res = ry.Response()  # type: ignore[var-annotated]

    def test_post_multipart_not_impl(
        self,
    ) -> None:
        c = ry.BlockingClient()
        with pytest.raises(NotImplementedError):
            _r = c.post("http://example.com", multipart={"a": 1})

    def test_client_fetch_multipart_not_impl(
        self,
    ) -> None:
        c = ry.BlockingClient()
        with pytest.raises(NotImplementedError):
            _r = c.fetch("http://example.com", method="POST", multipart={"a": 1})

    def test_fetch_multipart_not_impl(
        self,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = ry.fetch("http://example.com", method="POST", multipart={"a": 1})
