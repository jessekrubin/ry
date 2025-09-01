"""Tests for ryo3-url library"""

from __future__ import annotations

from urllib.parse import urlparse as pyurlparse

from hypothesis import given
from hypothesis.provisional import urls as st_urls

import ry


@given(st_urls())
def test_parse_url(
    url: str,
) -> None:
    u = ry.URL.parse(url)
    assert u.scheme is not None
    assert u.host is not None
    assert u.path is not None
    pyparsed = pyurlparse(url)

    assert u.scheme == pyparsed.scheme
    assert u.host == pyparsed.hostname
    assert u.port == pyparsed.port
    if pyparsed.username:
        assert u.username == pyparsed.username
    else:
        assert u.username == ""

    if pyparsed.password:
        assert u.password == pyparsed.password
    else:
        assert u.password is None
    if pyparsed.query:
        assert u.query == pyparsed.query
    else:
        assert u.query is None

    assert isinstance(u.is_special(), bool)

    if u.has_authority():
        assert u.authority is not None
    assert isinstance(u.has_host(), bool)
    assert u.origin == f"{u.scheme}://{u.host}" + (f":{u.port}" if u.port else "")


@given(st_urls())
def test_url_dunders(
    url: str,
) -> None:
    u = ry.URL.parse(url)
    u_hash = hash(u)
    assert u_hash == hash(ry.URL.parse(url))
    assert len(str(u)) == len(u)
