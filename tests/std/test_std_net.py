from __future__ import annotations

from typing import TypeAlias

import pytest

import ry
from ry import IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6

RyIpAddrLike: TypeAlias = Ipv4Addr | Ipv6Addr | IpAddr
RySocketAddrLike: TypeAlias = SocketAddrV4 | SocketAddrV6 | SocketAddr

IP_ADDR_OBJECTS: list[RyIpAddrLike] = [
    Ipv4Addr(192, 168, 0, 1),
    Ipv6Addr("::1"),
    IpAddr(Ipv4Addr(192, 168, 0, 1)),
    IpAddr(Ipv6Addr("::1")),
]

SOCKET_ADDR_OBJECTS: list[RySocketAddrLike] = [
    SocketAddrV4(Ipv4Addr(192, 168, 0, 1), 8080),
    SocketAddrV6(Ipv6Addr("::1"), 8080),
    SocketAddr(Ipv4Addr(192, 168, 0, 1), 8080),
    SocketAddr(Ipv6Addr("::1"), 8080),
]

STD_NET_OBJECTS: list[
    Ipv4Addr | Ipv6Addr | SocketAddrV4 | SocketAddrV6 | IpAddr | SocketAddr
] = [
    *IP_ADDR_OBJECTS,
    *SOCKET_ADDR_OBJECTS,
]


class TestIpConstants:
    def test_constants_ipv4(self) -> None:
        """
        Test that the constants in the ry module are defined.
        """
        assert ry.Ipv4Addr.LOCALHOST == Ipv4Addr(127, 0, 0, 1)
        assert ry.Ipv4Addr.UNSPECIFIED == Ipv4Addr(0, 0, 0, 0)
        assert ry.Ipv4Addr.BROADCAST == Ipv4Addr(255, 255, 255, 255)

    def test_constants_ipv6(self) -> None:
        """
        Test that the constants in the ry module are defined.
        """
        assert ry.Ipv6Addr.LOCALHOST == Ipv6Addr("::1")
        assert ry.Ipv6Addr.UNSPECIFIED == Ipv6Addr("::")

    def test_constants_ipaddr(self) -> None:
        """
        Test that the constants in the ry module are defined.
        """
        assert ry.IpAddr.LOCALHOST_V4 == Ipv4Addr(127, 0, 0, 1).to_ipaddr()
        assert ry.IpAddr.LOCALHOST_V6 == Ipv6Addr("::1").to_ipaddr()

        assert ry.IpAddr.UNSPECIFIED_V4 == Ipv4Addr(0, 0, 0, 0).to_ipaddr()
        assert ry.IpAddr.UNSPECIFIED_V6 == Ipv6Addr("::").to_ipaddr()
        assert ry.IpAddr.BROADCAST == Ipv4Addr.BROADCAST.to_ipaddr()


@pytest.mark.parametrize(
    "obj",
    STD_NET_OBJECTS,
)
def test_addr_repr(obj: ry.IpAddr | ry.SocketAddrV4 | ry.SocketAddrV6) -> None:
    """
    Test that the repr of IpAddr, SocketAddrV4, and SocketAddrV6 is correct.
    """
    repr_str = "ry." + repr(obj)
    assert eval(repr_str) == obj
    assert isinstance(eval(repr_str), type(obj))


@pytest.mark.parametrize(
    "obj",
    STD_NET_OBJECTS,
)
def test_string_and_parse(obj: ry.IpAddr | ry.SocketAddrV4 | ry.SocketAddrV6) -> None:
    """
    Test that the repr of IpAddr, SocketAddrV4, and SocketAddrV6 is correct.
    """
    s = str(obj)
    cls = type(obj)
    if isinstance(obj, ry.SocketAddrV6) or (
        isinstance(obj, ry.SocketAddr) and obj.version == 6
    ):
        with pytest.raises(NotImplementedError):
            parsed = cls.parse(s)

    else:
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
