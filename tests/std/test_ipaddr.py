from __future__ import annotations

import ipaddress
import ipaddress as pyip
import itertools as it
import pickle

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


class TestIpAddrHypothesis:
    @given(st.ip_addresses(v=4))
    def test_ipv4(self, py_ipaddr: str) -> None:
        assert isinstance(py_ipaddr, ipaddress.IPv4Address)
        ip = ry.Ipv4Addr(py_ipaddr)
        assert isinstance(ip, ry.Ipv4Addr)
        assert isinstance(ip.to_py(), pyip.IPv4Address)

    @given(st.ip_addresses(v=6))
    def test_ipv6(self, py_ipaddr: str) -> None:
        assert isinstance(py_ipaddr, ipaddress.IPv6Address)
        ip = ry.Ipv6Addr(py_ipaddr)
        assert isinstance(ip, ry.Ipv6Addr)
        assert isinstance(ip.to_py(), pyip.IPv6Address)

    @given(st.ip_addresses())
    def test_ipaddr_repr(
        self, py_ipaddr: ipaddress.IPv4Address | ipaddress.IPv6Address
    ) -> None:
        ry_ip = ry.IpAddr(py_ipaddr)
        repr_str = "ry." + repr(ry_ip)
        assert eval(repr_str) == ry_ip
        assert isinstance(eval(repr_str), ry.IpAddr)

        if ry_ip.is_ipv4:
            ry_ipv4 = ry_ip.to_ipv4()
            repr_str_ipv4 = "ry." + repr(ry_ipv4)
            assert eval(repr_str_ipv4) == ry_ipv4
            assert isinstance(eval(repr_str_ipv4), ry.Ipv4Addr)
        else:
            ry_ipv6 = ry_ip.to_ipv6()
            repr_str_ipv6 = "ry." + repr(ry_ipv6)
            assert eval(repr_str_ipv6) == ry_ipv6
            assert isinstance(eval(repr_str_ipv6), ry.Ipv6Addr)

    @given(st.ip_addresses())
    def test_ipaddr_pickle(
        self, py_ipaddr: ipaddress.IPv4Address | ipaddress.IPv6Address
    ) -> None:
        ry_ip = ry.IpAddr(py_ipaddr)
        pickled = pickle.dumps(ry_ip)
        unpickled = pickle.loads(pickled)
        assert isinstance(unpickled, ry.IpAddr)
        assert ry_ip == unpickled
        if ry_ip.is_ipv4:
            ry_ipv4 = ry_ip.to_ipv4()
            pickled_ipv4 = pickle.dumps(ry_ipv4)
            unpickled_ipv4 = pickle.loads(pickled_ipv4)
            assert isinstance(unpickled_ipv4, ry.Ipv4Addr)
            assert ry_ipv4 == unpickled_ipv4
        else:
            ry_ipv6 = ry_ip.to_ipv6()
            pickled_ipv6 = pickle.dumps(ry_ipv6)
            unpickled_ipv6 = pickle.loads(pickled_ipv6)
            assert isinstance(unpickled_ipv6, ry.Ipv6Addr)
            assert ry_ipv6 == unpickled_ipv6

    @given(st.ip_addresses())
    def test_ipaddr(
        self, py_ipaddr: ipaddress.IPv4Address | ipaddress.IPv6Address
    ) -> None:
        assert isinstance(py_ipaddr, (ipaddress.IPv6Address, ipaddress.IPv4Address))
        ip = ry.IpAddr(py_ipaddr)
        assert isinstance(ip, ry.IpAddr)
        py_ip_from_rs_ip = ip.to_py()
        if isinstance(py_ipaddr, ipaddress.IPv4Address):
            assert ip.version == 4
            assert ip.is_ipv4
            assert not ip.is_ipv6
            assert str(ip) == str(py_ipaddr)
            assert py_ip_from_rs_ip == py_ipaddr
            assert isinstance(py_ip_from_rs_ip, pyip.IPv4Address)
        else:
            assert ip.version == 6
            assert not ip.is_ipv4
            assert ip.is_ipv6
            assert py_ip_from_rs_ip == py_ipaddr
            assert isinstance(py_ip_from_rs_ip, pyip.IPv6Address)


class TestIpv4Addr:
    def test_py_conversion(self) -> None:
        ry_ip = ry.Ipv4Addr(1, 2, 3, 4)
        py_ip = ry_ip.to_py()

        assert isinstance(ry_ip, ry.Ipv4Addr)
        assert isinstance(py_ip, pyip.IPv4Address)

    def test_ipv4_addr(self) -> None:
        assert ry.Ipv4Addr(0, 0, 0, 0) == ry.Ipv4Addr.UNSPECIFIED
        assert ry.Ipv4Addr(1, 2, 3, 4) != ry.Ipv4Addr.UNSPECIFIED
        assert ry.Ipv4Addr(1, 2, 3, 4) == ry.Ipv4Addr(1, 2, 3, 4)
        assert ry.Ipv4Addr(1, 2, 3, 4) != ry.Ipv4Addr(5, 6, 7, 8)

    def test_new_ipv4(self) -> None:
        rust_like = ry.Ipv4Addr(192, 168, 0, 1)
        # from_py = pyip.IPv4Address("192.168.0.1")
        from_str = ry.Ipv4Addr("192.168.0.1")
        from_int = ry.Ipv4Addr(3232235521)
        from_bytes = ry.Ipv4Addr(b"\xc0\xa8\x00\x01")

        arr = (
            rust_like,
            # from_py,
            from_str,
            from_int,
            from_bytes,
        )

        for left, right in it.product(arr, arr):
            assert left == right
            assert left != ry.Ipv4Addr(1, 2, 3, 4)
            assert left != ry.Ipv4Addr.UNSPECIFIED

    def test_properties(self) -> None:
        ipv4 = ry.Ipv4Addr(192, 168, 0, 1)
        assert not ipv4.is_broadcast
        assert not ipv4.is_documentation
        assert not ipv4.is_link_local
        assert not ipv4.is_loopback
        assert not ipv4.is_multicast
        assert ipv4.is_private
        assert not ipv4.is_unspecified

    def test_properties_unstable(self) -> None:
        ipv4 = ry.Ipv4Addr(192, 168, 0, 1)

        with pytest.raises(NotImplementedError):
            ipv4.is_benchmarking
        with pytest.raises(NotImplementedError):
            ipv4.is_global
        with pytest.raises(NotImplementedError):
            ipv4.is_reserved
        with pytest.raises(NotImplementedError):
            ipv4.is_shared
