from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from ry.ryo3 import ClientConfig

_DEFAULT_CONFIG: ClientConfig = {
    "headers": None,
    "cookies": False,
    "user_agent": "ry/0.0.71",
    "timeout": None,
    "read_timeout": None,
    "connect_timeout": None,
    "redirect": 10,
    "referer": True,
    "gzip": True,
    "brotli": True,
    "deflate": True,
    "zstd": True,
    "hickory_dns": True,
    "http1_only": False,
    "https_only": False,
    "http1_title_case_headers": False,
    "http1_allow_obsolete_multiline_headers_in_responses": False,
    "http1_allow_spaces_after_header_name_in_responses": False,
    "http1_ignore_invalid_headers_in_responses": False,
    "http2_prior_knowledge": False,
    "http2_initial_stream_window_size": None,
    "http2_initial_connection_window_size": None,
    "http2_adaptive_window": False,
    "http2_max_frame_size": None,
    "http2_max_header_list_size": None,
    "http2_keep_alive_interval": None,
    "http2_keep_alive_timeout": None,
    "http2_keep_alive_while_idle": False,
    "pool_idle_timeout": ry.Duration(secs=90, nanos=0),
    "pool_max_idle_per_host": ry.USIZE_MAX,
    "tcp_keepalive": ry.Duration(secs=15, nanos=0),
    "tcp_keepalive_interval": ry.Duration(secs=15, nanos=0),
    "tcp_keepalive_retries": 3,
    "tcp_nodelay": True,
    "root_certificates": None,
    "tls_min_version": None,
    "tls_max_version": None,
    "tls_info": False,
    "tls_sni": True,
    "danger_accept_invalid_certs": False,
    "danger_accept_invalid_hostnames": False,
}


# param fixture
@pytest.fixture(params=[ry.HttpClient, ry.BlockingClient])
def client_cls(
    request: pytest.FixtureRequest,
) -> type[ry.HttpClient | ry.BlockingClient]:
    return t.cast("type[ry.HttpClient | ry.BlockingClient]", request.param)


def test_config_equality(client_cls: type[ry.HttpClient | ry.BlockingClient]) -> None:
    client = client_cls()
    assert isinstance(client.config(), dict)
    assert client.config() == _DEFAULT_CONFIG


def test_client_config_headers(
    client_cls: type[ry.HttpClient | ry.BlockingClient],
) -> None:
    headers = {"user-agent": "ryo3-reqwest-test", "accept": "application/json"}
    client = client_cls(headers=headers)
    config = client.config()
    assert isinstance(config["headers"], ry.Headers)
    assert config["headers"].to_dict() == headers
