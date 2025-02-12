"""Tests for ryo3-url library"""

from __future__ import annotations

import pytest

import ry.dev as ry


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
    # assert u[ry.Position.BeforePath:] == "/rust-lang/rust/issues?labels=E-easy&state=open"
    assert u.fragment is None


def test_inheritance() -> None:
    with pytest.raises(TypeError):

        class MyURL(ry.URL):
            pass


def test_str_subclass() -> None:
    class S(str):
        pass

    assert str(ry.URL(S("http://example.com"))) == "http://example.com/"


#
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


def test_join() -> None:
    u = ry.URL("http://example.com")
    joined = u.join("foo")
    assert str(joined) == "http://example.com/foo"
    joined_multiple = u.join("foo").join("bar")
    assert str(joined_multiple) == "http://example.com/foo/bar"
    joined_varargs = u.join("foo", "bar")
    assert str(joined_varargs) == "http://example.com/foo/bar"


def test_join_truediv() -> None:
    u = ry.URL("http://example.com")
    joined = u / "foo"
    assert str(joined) == "http://example.com/foo"
    joined_multiple = u / "foo" / "bar"
    assert str(joined_multiple) == "http://example.com/bar"
    joined_varargs = u / "foo" / "bar"
    assert str(joined_varargs) == "http://example.com/bar"


def test_join_truediv_trailing_slash() -> None:
    u = ry.URL("http://example.com")
    joined = u / "foo"
    assert str(joined) == "http://example.com/foo"
    joined_multiple = u / "foo/" / "bar"
    assert str(joined_multiple) == "http://example.com/foo/bar"
    joined_varargs = u / "foo/" / "bar"
    assert str(joined_varargs) == "http://example.com/foo/bar"
