from __future__ import annotations

import pytest

import ry
from ry import IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6


@pytest.mark.parametrize(
    "obj",
    [
        Ipv4Addr(192, 168, 0, 1),
        Ipv6Addr("::1"),
        SocketAddrV4(Ipv4Addr(192, 168, 0, 1), 8080),
        SocketAddrV6(Ipv6Addr("::1"), 8080),
        IpAddr(Ipv4Addr(192, 168, 0, 1)),
        IpAddr(Ipv6Addr("::1")),
        SocketAddr(Ipv4Addr(192, 168, 0, 1), 8080),
        SocketAddr(Ipv6Addr("::1"), 8080),
    ],
)
def test_addr_repr(obj: ry.IpAddr | ry.SocketAddrV4 | ry.SocketAddrV6) -> None:
    """
    Test that the repr of IpAddr, SocketAddrV4, and SocketAddrV6 is correct.
    """
    repr_str = "ry." + repr(obj)
    assert eval(repr_str) == obj
    assert isinstance(eval(repr_str), type(obj))


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
