from __future__ import annotations

import ipaddress as pyip
import itertools as it

import pytest

import ry


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


class TestIpNotImplemented:
    def test_properties_unstable_ipv4(self) -> None:
        ip_obj = ry.Ipv4Addr(192, 168, 0, 1)

        with pytest.raises(NotImplementedError):
            _is_benchmarking = ip_obj.is_benchmarking  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_global = ip_obj.is_global  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_reserved = ip_obj.is_reserved  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_shared = ip_obj.is_shared  # type: ignore[var-annotated]

    def test_properties_unstable_ipv6(self) -> None:
        ip_obj = ry.Ipv4Addr(192, 168, 0, 1).to_ipaddr().to_ipv6()

        with pytest.raises(NotImplementedError):
            _is_benchmarking = ip_obj.is_benchmarking  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_global = ip_obj.is_global  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_unicast_global = ip_obj.is_unicast_global  # type: ignore[var-annotated]

    def test_properties_unstable_ip(self) -> None:
        ip_obj = ry.Ipv4Addr(192, 168, 0, 1).to_ipaddr()

        with pytest.raises(NotImplementedError):
            _is_benchmarking = ip_obj.is_benchmarking  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_global = ip_obj.is_global  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_reserved = ip_obj.is_reserved  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_shared = ip_obj.is_shared  # type: ignore[var-annotated]
        with pytest.raises(NotImplementedError):
            _is_unicast_global = ip_obj.is_unicast_global  # type: ignore[var-annotated]
