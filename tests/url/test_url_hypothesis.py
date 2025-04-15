"""Tests for ryo3-url library"""

from __future__ import annotations

from hypothesis import given
from hypothesis.provisional import domains as st_domains
from hypothesis.provisional import urls as st_urls

import ry


@given(st_urls())
def test_parse_url(
    url: str,
) -> None:
    u = ry.URL.parse(url)
    print(url)
    assert u.scheme is not None
    assert u.host is not None
    assert u.path is not None


@given(st_domains())
def test_parse_domain(
    domain: str,
) -> None:
    u = ry.URL.parse(domain)
    print(domain)
    assert u.scheme is not None
    assert u.host is not None
    assert u.path is not None
