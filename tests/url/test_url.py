"""Tests for ryo3-url library"""

from __future__ import annotations

import ipaddress as pyip
from pathlib import Path

import pytest

import ry


def test_parse_error() -> None:
    """
    use url::{Url, ParseError};

    assert!(Url::parse("ry_http://[:::1]") == Err(ParseError::InvalidIpv6Address))
    """

    with pytest.raises(ValueError):
        ry.URL.parse("ry_http://[:::1]")


def test_url_from_url() -> None:
    """Test that we can create a URL from a URL"""
    url = ry.URL("http://example.com")
    url_from_url = ry.URL(url)
    assert url == url_from_url


def test_url_parse() -> None:
    """Test that we can create a URL from a URL"""
    url = ry.URL("http://example.com")
    url_from_url = ry.URL(url)
    assert url == url_from_url


def test_parse_url_readme() -> None:
    """
    use url::{Url, Host, Position};
    let issue_list_url = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    )?;

    assert!(issue_list_url.scheme() == "https");
    assert!(issue_list_url.username() == "");
    assert!(issue_list_url.password() == None);
    assert!(issue_list_url.host_str() == Some("github.com"));
    assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
    assert!(issue_list_url.port() == None);
    assert!(issue_list_url.path() == "/rust-lang/rust/issues");
    assert!(issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
            Some(vec!["rust-lang", "rust", "issues"]));
    assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
    assert!(&issue_list_url[Position::BeforePath..] == "/rust-lang/rust/issues?labels=E-easy&state=open");
    assert!(issue_list_url.fragment() == None);
    assert!(!issue_list_url.cannot_be_a_base());
    """
    u = ry.URL.parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    )
    assert u.scheme == "https"
    assert u.username == ""
    assert u.password is None
    assert u.host == "github.com"
    assert u.host == "github.com"

    assert u.port is None
    assert u.path == "/rust-lang/rust/issues"
    assert list(u.path_segments) == ["rust-lang", "rust", "issues"]

    assert u.query == "labels=E-easy&state=open"
    assert u.fragment is None
    u_from_str = ry.URL.from_str(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    )
    assert u == u_from_str


def test_inheritance() -> None:
    with pytest.raises(TypeError):

        class MyURL(ry.URL):  # type: ignore[misc]
            ...


def test_str_subclass() -> None:
    class S(str): ...

    assert str(ry.URL(S("http://example.com"))) == "http://example.com/"


def test_absolute_url_without_host() -> None:
    with pytest.raises(ValueError):
        ry.URL("http://:8080/")


def test_url_is_not_str() -> None:
    url = ry.URL("http://example.com")
    assert not isinstance(url, str)


def test_str() -> None:
    url = ry.URL("http://example.com:8888/path/to?a=1&b=2")
    assert str(url) == "http://example.com:8888/path/to?a=1&b=2"


def test_repr() -> None:
    url = ry.URL("http://example.com")
    assert "URL('http://example.com/')" == repr(url)


@pytest.mark.parametrize(
    "url_str, expected",
    [
        ("https://127.0.0.1/", None),
        ("https://[::1]/", None),
        ("https://example.com/", "example.com"),
        ("https://subdomain.example.com/", "subdomain.example.com"),
        ("mailto:rms@example.net", None),
    ],
)
def test_domain(
    url_str: str,
    expected: str | None,
) -> None:
    assert ry.URL(url_str).domain == expected


class TestJoinUrl:
    def test_join_empty(self) -> None:
        u = ry.URL("http://example.com")
        empty_tuple = ()
        joined = u.join(*empty_tuple)
        assert str(joined) == "http://example.com/"

    def test_join(self) -> None:
        u = ry.URL("http://example.com")
        joined = u.join("foo")
        assert str(joined) == "http://example.com/foo"
        joined_multiple = u.join("foo").join("bar")
        assert str(joined_multiple) == "http://example.com/foo/bar"
        joined_varargs = u.join("foo", "bar")
        assert str(joined_varargs) == "http://example.com/foo/bar"

    def test_join_truediv(self) -> None:
        u = ry.URL("http://example.com")
        joined = u / "foo"
        assert str(joined) == "http://example.com/foo"
        joined_multiple = u / "foo" / "bar"
        assert str(joined_multiple) == "http://example.com/bar"
        joined_varargs = u / "foo" / "bar"
        assert str(joined_varargs) == "http://example.com/bar"

    def test_join_truediv_trailing_slash(self) -> None:
        u = ry.URL("http://example.com")
        joined = u / "foo"
        assert str(joined) == "http://example.com/foo"
        joined_multiple = u / "foo/" / "bar"
        assert str(joined_multiple) == "http://example.com/foo/bar"
        joined_varargs = u / "foo/" / "bar"
        assert str(joined_varargs) == "http://example.com/foo/bar"


@pytest.mark.parametrize(
    "base_url_expected",
    [
        ("https://example.net/a/b.html", "https://example.net/a/c.png", "c.png"),
        ("https://example.net/a/b/", "https://example.net/a/b/c.png", "c.png"),
        ("https://example.net/a/b/", "https://example.net/a/d/c.png", "../d/c.png"),
        (
            "https://example.net/a/b.html?c=d",
            "https://example.net/a/b.html?e=f",
            "?e=f",
        ),
    ],
)
def test_url_relative(base_url_expected: tuple[str, str, str]) -> None:
    base, url, expected = base_url_expected
    base_url = ry.URL(base)
    url_obj = ry.URL(url)
    relative = base_url.make_relative(url_obj)
    assert relative == expected


class TestUrlReplace:
    def test_replace_scheme(self) -> None:
        u = ry.URL("http://example.com")
        replaced = u.replace(scheme="https")
        assert str(replaced) == "https://example.com/"
        assert str(u.replace_scheme("https")) == "https://example.com/"

    def test_replace_host(self) -> None:
        u = ry.URL("http://example.com")
        replaced = u.replace(host="example.org")
        assert str(replaced) == "http://example.org/"
        assert str(u.replace_host("example.org")) == "http://example.org/"
        assert u.host_str == "example.com"
        assert u.host == "example.com"

    def test_replace_port(self) -> None:
        u = ry.URL("http://example.com")
        replaced = u.replace(port=8080)
        assert str(replaced) == "http://example.com:8080/"
        assert str(u.replace_port(8080)) == "http://example.com:8080/"

    def test_replace_path(self) -> None:
        u = ry.URL("http://example.com/foo")
        replaced = u.replace(path="/bar/baz")
        assert str(replaced) == "http://example.com/bar/baz"
        assert str(u.replace_path("bar/baz")) == "http://example.com/bar/baz"

    def test_replace_query(self) -> None:
        u = ry.URL("http://example.com/foo?a=1&b=2")
        replaced = u.replace(query="c=3&d=4")
        assert str(replaced) == "http://example.com/foo?c=3&d=4"
        assert str(u.replace_query("c=3&d=4")) == "http://example.com/foo?c=3&d=4"

    def test_replace_fragment(self) -> None:
        u = ry.URL("http://example.com/foo#a_section")
        replaced = u.replace(fragment="another_section")
        assert str(replaced) == "http://example.com/foo#another_section"
        assert (
            str(u.replace_fragment("another_section"))
            == "http://example.com/foo#another_section"
        )

    def test_replace_ip_host(self) -> None:
        u = ry.URL("http://example.com")
        replaced = u.replace(ip_host=pyip.ip_interface("2001:db8:85a3::8a2e:370:7334"))
        assert str(replaced) == "http://[2001:db8:85a3::8a2e:370:7334]/"
        assert (
            str(u.replace_ip_host(pyip.ip_interface("2001:db8:85a3::8a2e:370:7334")))
            == "http://[2001:db8:85a3::8a2e:370:7334]/"
        )

    def test_replace_username(self) -> None:
        u = ry.URL("http://example.com")
        replaced = u.replace(username="user")
        expected = "http://user@example.com/"
        assert str(replaced) == expected
        assert str(u.replace_username("user")) == expected

    def test_replace_password(self) -> None:
        u = ry.URL("http://example.com")
        replaced = u.replace(password="pass")  # noqa: S106
        expected = "http://:pass@example.com/"
        assert str(replaced) == expected
        assert str(u.replace_password("pass")) == expected


def test_socket_addrs() -> None:
    url = ry.URL("http://example.com")
    addrs = url.socket_addrs()

    assert isinstance(addrs, list)
    assert all(isinstance(addr, ry.SocketAddr) for addr in addrs)


def test_port_or_known_default() -> None:
    url = ry.URL("foo://example.com")
    assert url.port_or_known_default is None
    url_with_port = ry.URL("foo://example.com:1456")
    assert url_with_port.port_or_known_default == 1456
    https_url = ry.URL("https://example.com")
    assert https_url.port_or_known_default == 443


def test_from_directory_path() -> None:
    pwd = Path(__file__).parent.resolve()
    file_url = ry.URL.from_directory_path(pwd)
    assert str(file_url).startswith("file://")

    url_fspath = file_url.__fspath__().replace("\\", "/")  # noqa: PLC2801
    assert url_fspath == (str(pwd) + "/").replace("\\", "/")
    assert isinstance(url_fspath, str)


def test_from_filepath() -> None:
    this_file = Path(__file__).resolve()
    file_url = ry.URL.from_filepath(this_file)
    assert str(file_url).startswith("file://")

    url_fspath = file_url.__fspath__().replace("\\", "/")  # noqa: PLC2801
    assert url_fspath == str(this_file).replace("\\", "/")
    assert isinstance(url_fspath, str)


@pytest.mark.parametrize(
    "tdata",
    [
        ("http://example.com", True),
        ("https://example.com", True),
        ("ftp://example.com", True),
        ("ws://example.com", True),
        ("wss://example.com", True),
        ("file:///tmp/foo", True),
        ("custom-scheme://example.com", False),
        ("moz:///tmp/foo", False),
    ],
)
def test_is_special(
    tdata: tuple[str, bool],
) -> None:
    url_str, is_special = tdata
    url = ry.URL(url_str)
    assert url.is_special() is is_special
