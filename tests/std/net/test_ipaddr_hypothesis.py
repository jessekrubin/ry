from __future__ import annotations

import ipaddress
import ipaddress as pyip
import pickle

from hypothesis import given
from hypothesis import strategies as st

import ry


class TestIpAddrHypothesis:
    @given(st.ip_addresses(v=4))
    def test_ipv4(self, py_ipaddr: ipaddress.IPv4Address) -> None:
        assert isinstance(py_ipaddr, ipaddress.IPv4Address)
        ip = ry.Ipv4Addr(py_ipaddr)
        assert isinstance(ip, ry.Ipv4Addr)
        assert isinstance(ip.to_py(), pyip.IPv4Address)

    @given(st.ip_addresses(v=6))
    def test_ipv6(self, py_ipaddr: ipaddress.IPv6Address) -> None:
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


class TestIpAddrPickling:
    @given(st.ip_addresses(v=4))
    def test_ipv4addr_pickle(self, py_ipaddr: ipaddress.IPv4Address) -> None:
        ry_ip = ry.Ipv4Addr(py_ipaddr)
        pickled = pickle.dumps(ry_ip)
        unpickled = pickle.loads(pickled)
        assert isinstance(unpickled, ry.Ipv4Addr)
        assert ry_ip == unpickled

    @given(st.ip_addresses(v=6))
    def test_ipv6addr_pickle(self, py_ipaddr: ipaddress.IPv6Address) -> None:
        ry_ip = ry.Ipv6Addr(py_ipaddr)
        pickled = pickle.dumps(ry_ip)
        unpickled = pickle.loads(pickled)
        assert isinstance(unpickled, ry.Ipv6Addr)
        assert ry_ip == unpickled

    @given(st.ip_addresses())
    def test_ipaddr_pickle(
        self, py_ipaddr: ipaddress.IPv4Address | ipaddress.IPv6Address
    ) -> None:
        ry_ip = ry.IpAddr(py_ipaddr)
        pickled = pickle.dumps(ry_ip)
        unpickled = pickle.loads(pickled)
        assert isinstance(unpickled, ry.IpAddr)
        assert ry_ip == unpickled
