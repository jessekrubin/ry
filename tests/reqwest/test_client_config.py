from __future__ import annotations

import re
import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from ry.ryo3 import ClientConfig

_DEFAULT_CONFIG: ClientConfig = {
    "headers": None,
    "cookies": False,
    "user_agent": f"ry/{ry.__version__}",
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
    "tls_version_min": None,
    "tls_version_max": None,
    "tls_info": False,
    "tls_sni": True,
    "danger_accept_invalid_certs": False,
    "danger_accept_invalid_hostnames": False,
}


# param fixture
@pytest.fixture(params=[ry.HttpClient, ry.Client, ry.BlockingClient])
def client_cls(
    request: pytest.FixtureRequest,
) -> type[ry.HttpClient | ry.Client | ry.BlockingClient]:
    return t.cast("type[ry.HttpClient | ry.Client | ry.BlockingClient]", request.param)


def test_config_equality(
    client_cls: type[ry.HttpClient | ry.Client | ry.BlockingClient],
) -> None:
    client = client_cls()
    assert isinstance(client.config(), dict)
    assert client.config() == _DEFAULT_CONFIG


def test_client_config_headers(
    client_cls: type[ry.HttpClient | ry.Client | ry.BlockingClient],
) -> None:
    headers = {"user-agent": "ryo3-reqwest-test", "accept": "application/json"}
    client = client_cls(headers=headers)
    config = client.config()
    assert isinstance(config["headers"], ry.Headers)
    assert config["headers"].to_dict() == headers


def test_client_config_pickle(
    client_cls: type[ry.HttpClient | ry.Client | ry.BlockingClient],
) -> None:
    import pickle

    client = client_cls()
    pickled = pickle.dumps(client)
    unpickled = pickle.loads(pickled)
    assert isinstance(unpickled, client_cls)
    assert unpickled.config() == client.config()


class TestTlsVersions:
    @pytest.mark.parametrize(
        "tls_version_max",
        [None, "1.0", "1.1", "1.2", "1.3"],
    )
    @pytest.mark.parametrize(
        "tls_version_min",
        [None, "1.0", "1.1", "1.2", "1.3"],
    )
    def test_client_config_tls_versions(
        self,
        client_cls: type[ry.HttpClient | ry.Client | ry.BlockingClient],
        tls_version_min: t.Literal["1.0", "1.1", "1.2", "1.3"] | None,
        tls_version_max: t.Literal["1.0", "1.1", "1.2", "1.3"] | None,
    ) -> None:
        if (
            tls_version_min is not None
            and tls_version_max is not None
            and tls_version_min > tls_version_max
        ) or (
            # problem childs
            (tls_version_min, tls_version_max)
            in {
                (None, "1.0"),
                (None, "1.1"),
                ("1.0", "1.0"),
                ("1.0", "1.1"),
                ("1.1", "1.1"),
            }
        ):
            with pytest.raises(ry.ReqwestError):
                _ = client_cls(
                    tls_version_min=tls_version_min,
                    tls_version_max=tls_version_max,
                )
            return
        client = client_cls(
            tls_version_min=tls_version_min, tls_version_max=tls_version_max
        )
        config = client.config()
        assert config["tls_version_min"] == tls_version_min
        assert config["tls_version_max"] == tls_version_max

    def test_client_tls_versions_wrong_type(
        self, client_cls: type[ry.HttpClient | ry.Client | ry.BlockingClient]
    ) -> None:
        match_pat = "TLS version must be a string (options: '1.0', '1.1', '1.2', '1.3')"
        with pytest.raises(TypeError, match=re.escape(match_pat)):
            _ = client_cls(tls_version_min=1.2)  # type: ignore[arg-type]

    def test_client_config_tls_versions_value_problemo(
        self,
        client_cls: type[ry.HttpClient | ry.Client | ry.BlockingClient],
    ) -> None:
        match_pat = (
            "Invalid TLS version: snorkling (options: '1.0', '1.1', '1.2', '1.3')"
        )
        with pytest.raises(ValueError, match=re.escape(match_pat)):
            _ = client_cls(tls_version_min="snorkling")  # type: ignore[arg-type]
