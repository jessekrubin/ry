from __future__ import annotations

import ipaddress as pyip
import itertools as it
import typing as t

import pytest

import ry


class _IpAddrConstants(t.TypedDict):
    ob: ry.IpAddr | ry.Ipv4Addr | ry.Ipv6Addr
    repr: str
    str: str


# fmt: off
_IP_ADDR_CONSTANTS: list[_IpAddrConstants] = [
    # ipv4 constants
    {"ob": ry.Ipv4Addr.LOCALHOST,    "repr": "Ipv4Addr('127.0.0.1')",     "str": "127.0.0.1"},
    {"ob": ry.Ipv4Addr.UNSPECIFIED,  "repr": "Ipv4Addr('0.0.0.0')",       "str": "0.0.0.0"  },  # noqa: S104
    # ipv6 constants
    {"ob": ry.Ipv6Addr.LOCALHOST,    "repr": "Ipv6Addr('::1')",           "str": "::1"},
    {"ob": ry.Ipv6Addr.UNSPECIFIED,  "repr": "Ipv6Addr('::')",            "str": "::"},
    # ipaddr constants
    {"ob": ry.IpAddr.BROADCAST,      "repr": "IpAddr('255.255.255.255')", "str": "255.255.255.255"},
    {"ob": ry.IpAddr.LOCALHOST_V4,   "repr": "IpAddr('127.0.0.1')",       "str": "127.0.0.1"},
    {"ob": ry.IpAddr.LOCALHOST_V6,   "repr": "IpAddr('::1')",             "str": "::1"},
    {"ob": ry.IpAddr.UNSPECIFIED_V4, "repr": "IpAddr('0.0.0.0')",         "str": "0.0.0.0"},  # noqa: S104
    {"ob": ry.IpAddr.UNSPECIFIED_V6, "repr": "IpAddr('::')",              "str": "::"},
]
# fmt: on


class TestIpConstants:
    def test_constants_ipv4(self) -> None:
        """
        Test that the constants in the ry module are defined.
        """
        assert ry.Ipv4Addr.LOCALHOST == ry.Ipv4Addr(127, 0, 0, 1)
        assert ry.Ipv4Addr.UNSPECIFIED == ry.Ipv4Addr(0, 0, 0, 0)
        assert ry.Ipv4Addr.BROADCAST == ry.Ipv4Addr(255, 255, 255, 255)

    def test_constants_ipv6(self) -> None:
        """
        Test that the constants in the ry module are defined.
        """
        assert ry.Ipv6Addr.LOCALHOST == ry.Ipv6Addr("::1")
        assert ry.Ipv6Addr.UNSPECIFIED == ry.Ipv6Addr("::")

    def test_constants_ipaddr(self) -> None:
        """
        Test that the constants in the ry module are defined.
        """
        assert ry.IpAddr.LOCALHOST_V4 == ry.Ipv4Addr(127, 0, 0, 1).to_ipaddr()
        assert ry.IpAddr.LOCALHOST_V6 == ry.Ipv6Addr("::1").to_ipaddr()

        assert ry.IpAddr.UNSPECIFIED_V4 == ry.Ipv4Addr(0, 0, 0, 0).to_ipaddr()
        assert ry.IpAddr.UNSPECIFIED_V6 == ry.Ipv6Addr("::").to_ipaddr()
        assert ry.IpAddr.BROADCAST == ry.Ipv4Addr.BROADCAST.to_ipaddr()

    @pytest.mark.parametrize("const", _IP_ADDR_CONSTANTS)
    def test_ip_addr_constants_str_repr(self, const: _IpAddrConstants) -> None:
        """
        Test that the IP address constants are defined.
        """
        ob = const["ob"]
        assert repr(ob) == const["repr"]
        assert str(ob) == const["str"]


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
        assert ipv4.is_unicast
        assert ipv4.is_private
        assert not ipv4.is_unspecified


class TestIpNotImplemented:
    def test_properties_unstable_ipv4(self) -> None:
        ip_obj = ry.Ipv4Addr(192, 168, 0, 1)

        with pytest.raises(NotImplementedError):
            _is_global = ip_obj.is_global  # type: ignore[var-annotated]

    def test_properties_unstable_ipv6(self) -> None:
        ip_obj = ry.Ipv4Addr(192, 168, 0, 1).to_ipaddr().to_ipv6()

        with pytest.raises(NotImplementedError):
            _is_global = ip_obj.is_global  # type: ignore[var-annotated]

    def test_properties_unstable_ip(self) -> None:
        ip_obj = ry.Ipv4Addr(192, 168, 0, 1).to_ipaddr()

        with pytest.raises(NotImplementedError):
            _is_global = ip_obj.is_global  # type: ignore[var-annotated]
