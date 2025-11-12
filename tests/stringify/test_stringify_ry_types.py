from __future__ import annotations

import pytest

import ry

_STDNET_OBJS = [
    (ry.Ipv4Addr("192.168.0.1"), '"192.168.0.1"'),
    (ry.Ipv6Addr("::1"), '"::1"'),
    (ry.IpAddr("192.168.0.1"), '"192.168.0.1"'),
    (ry.IpAddr("::1"), '"::1"'),
    (ry.SocketAddrV4(ry.Ipv4Addr("192.168.0.1"), 8080), '"192.168.0.1:8080"'),
    (ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080), '"[::1]:8080"'),
    (ry.SocketAddr(ry.Ipv4Addr("192.168.0.1"), 8080), '"192.168.0.1:8080"'),
    (ry.SocketAddr(ry.Ipv6Addr("::1"), 8080), '"[::1]:8080"'),
]


@pytest.mark.parametrize("obj_expected", _STDNET_OBJS)
def test_stringify_std_net_objs(
    obj_expected: tuple[
        ry.Ipv4Addr
        | ry.Ipv6Addr
        | ry.IpAddr
        | ry.SocketAddrV4
        | ry.SocketAddrV6
        | ry.SocketAddr,
        str,
    ],
) -> None:
    """Test that standard library network objects can be stringified."""
    obj, expected = obj_expected

    json_bytes = ry.stringify(obj)
    json_str = json_bytes.decode("utf-8")

    assert json_str == expected


def test_stringify_duration() -> None:
    assert ry.stringify(ry.Duration(secs=1)).decode() == '"PT1S"'


def test_stringify_duration_min_max() -> None:
    data = {
        "min": ry.Duration.MIN,
        "max": ry.Duration.MAX,
    }
    json_string = ry.stringify(data).decode()
    assert json_string == '{"min":"PT0S","max":"PT5124095576030431H15.999999999S"}'
    assert ry.parse_json(json_string) == {
        "min": ry.Duration.MIN.isoformat(),
        "max": ry.Duration.MAX.isoformat(),
    }


def test_stringify_duration_max() -> None:
    assert (
        ry.stringify(ry.Duration.MAX).decode() == '"PT5124095576030431H15.999999999S"'
    )


@pytest.mark.skip(reason="Duration.MAX serialization changed to ISO8601 format")
def test_stringify_duration_max_dict() -> None:
    assert (
        ry.stringify(ry.Duration.MAX).decode()
        == '{"secs":18446744073709551615,"nanos":999999999}'
    )
