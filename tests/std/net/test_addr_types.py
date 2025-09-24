from __future__ import annotations

from typing import TypeAlias

import pytest

import ry

_AnyIpAddr: TypeAlias = ry.Ipv4Addr | ry.Ipv6Addr | ry.IpAddr
_AnySocketAddr: TypeAlias = ry.SocketAddrV4 | ry.SocketAddrV6 | ry.SocketAddr
_StdNetAddrLike: TypeAlias = _AnyIpAddr | _AnySocketAddr


_IP_ADDR_OBJECTS: list[_AnyIpAddr] = [
    ry.Ipv4Addr(192, 168, 0, 1),
    ry.Ipv6Addr("::1"),
    ry.IpAddr(ry.Ipv4Addr(192, 168, 0, 1)),
    ry.IpAddr(ry.Ipv6Addr("::1")),
    # consts
    ry.Ipv4Addr.LOCALHOST,
    ry.Ipv4Addr.UNSPECIFIED,
    ry.Ipv6Addr.LOCALHOST,
    ry.Ipv6Addr.UNSPECIFIED,
    ry.IpAddr.BROADCAST,
    ry.IpAddr.LOCALHOST_V4,
    ry.IpAddr.LOCALHOST_V6,
    ry.IpAddr.UNSPECIFIED_V4,
    ry.IpAddr.UNSPECIFIED_V6,
]

_SOCKET_ADDR_OBJECTS: list[_AnySocketAddr] = [
    ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080),
    ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080),
    ry.SocketAddr(ry.Ipv4Addr(192, 168, 0, 1), 8080),
    ry.SocketAddr(ry.Ipv6Addr("::1"), 8080),
]

_STD_NET_OBJECTS: list[_StdNetAddrLike] = [
    *_IP_ADDR_OBJECTS,
    *_SOCKET_ADDR_OBJECTS,
]

_IPV4_PROPERTIES = [
    "is_benchmarking",
    "is_broadcast",
    "is_documentation",
    "is_global",
    "is_link_local",
    "is_loopback",
    "is_multicast",
    "is_private",
    "is_reserved",
    "is_shared",
    "is_unicast",
    "is_unspecified",
]
_IPV6_PROPERTIES = [
    "is_benchmarking",
    "is_documentation",
    "is_global",
    "is_ipv4_mapped",
    "is_loopback",
    "is_multicast",
    "is_reserved",
    "is_shared",
    "is_unicast",
    "is_unicast_global",
    "is_unicast_link_local",
    "is_unique_local",
    "is_unspecified",
]
_IPADDR_PROPERTIES = sorted({
    *_IPV4_PROPERTIES,
    *_IPV6_PROPERTIES,
    "is_ipv4",
    "is_ipv6",
    "version",
})


@pytest.mark.parametrize("obj", _STD_NET_OBJECTS)
def test_addr_repr(obj: _StdNetAddrLike) -> None:
    """
    Test that the repr of IpAddr, SocketAddrV4, and SocketAddrV6 is correct.
    """
    repr_str = "ry." + repr(obj)
    _globals = {
        "ry": ry,
        "Ipv4Addr": ry.Ipv4Addr,
        "Ipv6Addr": ry.Ipv6Addr,
        "IpAddr": ry.IpAddr,
        "SocketAddrV4": ry.SocketAddrV4,
        "SocketAddrV6": ry.SocketAddrV6,
    }
    evaluated = eval(repr_str, _globals)
    assert evaluated == obj
    assert isinstance(evaluated, type(obj))


@pytest.mark.parametrize("obj", _STD_NET_OBJECTS)
def test_string_and_parse(obj: _StdNetAddrLike) -> None:
    """
    Test that the repr of IpAddr, SocketAddrV4, and SocketAddrV6 is correct.
    """
    s = str(obj)
    cls = type(obj)
    parsed = cls.parse(s)
    assert parsed == obj
    assert isinstance(parsed, type(obj))


def test_properties_v4() -> None:
    ip = ry.Ipv4Addr(192, 168, 0, 1)
    socket_addr = ry.SocketAddrV4(ip, 8080)
    assert ip.is_broadcast == socket_addr.is_broadcast
    assert ip.is_link_local == socket_addr.is_link_local
    assert ip.is_loopback == socket_addr.is_loopback
    assert ip.is_multicast == socket_addr.is_multicast
    assert ip.is_private == socket_addr.is_private
    assert ip.is_unspecified == socket_addr.is_unspecified
    assert ip.to_pyipaddress() == socket_addr.to_pyipaddress()


def test_properties_v6() -> None:
    ip = ry.Ipv6Addr("::1")
    socket_addr = ry.SocketAddrV6(ip, 8080)
    assert ip.is_loopback == socket_addr.is_loopback
    assert ip.is_multicast == socket_addr.is_multicast
    assert ip.is_unicast_link_local == socket_addr.is_unicast_link_local
    assert ip.is_unique_local == socket_addr.is_unique_local
    assert ip.is_unspecified == socket_addr.is_unspecified
    assert not socket_addr.is_ipv4_mapped
    assert not ip.is_ipv4_mapped
    assert ip.to_pyipaddress() == socket_addr.to_pyipaddress()


class TestSocketAddrV4:
    sock_v4 = ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080)

    def test_v4_str(self) -> None:
        sock = self.sock_v4
        assert str(sock) == "192.168.0.1:8080"

    def test_v4_repr(self) -> None:
        sock = self.sock_v4
        assert repr(sock) == "SocketAddrV4(Ipv4Addr('192.168.0.1'), 8080)"


def test_ip2socket_v4() -> None:
    """
    Test that IpAddr can be converted to SocketAddrV4 and SocketAddrV6.
    """
    ip_v4 = ry.Ipv4Addr(192, 168, 0, 1)

    sock_v4 = ip_v4.to_socketaddr_v4(8080)
    assert isinstance(sock_v4, ry.SocketAddrV4)


def test_ip2socket_v6() -> None:
    ip_v6 = ry.Ipv6Addr("::1")
    sock_v6 = ip_v6.to_socketaddr_v6(8080)
    assert isinstance(sock_v6, ry.SocketAddrV6)
    assert sock_v6.ip == ip_v6


def _object_properties(cls: type) -> list[str]:
    """Return a list of strings all the getters for  an object"""
    return [
        p
        for p in dir(cls)
        if isinstance(getattr(cls, p), property)
        # or  it is an 'attribute'
        or getattr(cls, p).__class__.__name__ == "getset_descriptor"
    ]


@pytest.mark.parametrize(
    "cls,expected_props",
    [
        (ry.Ipv4Addr, _IPV4_PROPERTIES),
        (ry.Ipv6Addr, _IPV6_PROPERTIES),
        (ry.IpAddr, _IPADDR_PROPERTIES),
    ],
)
def test_ipaddr_properties_list(cls: type, expected_props: list[str]) -> None:
    props = set(_object_properties(cls))
    assert props == set(expected_props), (
        f"Properties do not match for {cls.__name__} : {props} != {set(expected_props)}"
    )


class TestSocketAddressesProperties:
    def test_socketv4_has_ipv4_properties(self) -> None:
        sock_v4 = ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080)
        props = set(_object_properties(type(sock_v4)))
        # assert subset
        assert props.issuperset(_IPV4_PROPERTIES), (
            f"Missing properties: {set(_IPV4_PROPERTIES) - props}"
        )

    def test_socketv6_has_ipv6_properties(self) -> None:
        sock_v6 = ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080)
        props = set(_object_properties(type(sock_v6)))
        # assert subset
        assert props.issuperset(_IPV6_PROPERTIES), (
            f"Missing properties: {set(_IPV6_PROPERTIES) - props}"
        )

    def test_socket_has_ipaddr_properties(self) -> None:
        sock = ry.SocketAddr(ry.Ipv4Addr(192, 168, 0, 1), 8080)
        props = set(_object_properties(type(sock)))
        # assert subset
        assert props.issuperset(_IPADDR_PROPERTIES), (
            f"Missing properties: {set(_IPADDR_PROPERTIES) - props}"
        )


def test_ipaddr_has_all_ipv4_properties() -> None:
    ip_v4 = ry.Ipv4Addr(192, 168, 0, 1)
    ipaddr_v4 = ip_v4.to_ipaddr()
    assert isinstance(ipaddr_v4, ry.IpAddr)
    assert ipaddr_v4.is_ipv4
    assert not ipaddr_v4.is_ipv6
    assert ipaddr_v4.to_ipv4() == ip_v4
    assert ipaddr_v4.to_pyipaddress() == ip_v4.to_pyipaddress()
    assert ipaddr_v4.is_private
    assert not ipaddr_v4.is_loopback
    assert not ipaddr_v4.is_multicast
    assert not ipaddr_v4.is_broadcast
    assert not ipaddr_v4.is_unspecified
