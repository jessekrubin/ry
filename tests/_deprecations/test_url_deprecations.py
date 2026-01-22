import typing as t

import pytest

import ry


def test_every_url_replace_method_has_with_counterpart(
    subtests: pytest.Subtests,
) -> None:
    url = ry.URL("http://user:pass@localhost:80/path?query#fragment")
    replace_methods = [
        (e, e.replace("replace_", "with_"))
        for e in dir(url)
        if e.startswith("replace_")
    ]
    for replace_method_name, with_method_name in replace_methods:
        with subtests.test(msg=f"Checking {replace_method_name}"):
            assert hasattr(url, with_method_name), (
                f"{replace_method_name} is missing {with_method_name} counterpart"
            )


@pytest.mark.parametrize(
    ("deprecated_method", "new_method", "args", "kwargs"),
    [
        ("replace_path", "with_path", ("/new/path",), {}),
        ("replace_port", "with_port", (), {"port": 8080}),
        ("replace_query", "with_query", (), {"query": "key=value"}),
        ("replace_scheme", "with_scheme", ("https",), {}),
        ("replace_username", "with_username", ("newuser",), {}),
        ("replace_fragment", "with_fragment", (), {"fragment": "section1"}),
        ("replace_ip_host", "with_ip_host", (ry.Ipv4Addr(127, 0, 0, 1),), {}),
    ],
)
def test_url_replace_deprecations(
    deprecated_method: str,
    new_method: str,
    args: tuple[t.Any, ...],
    kwargs: dict[str, str | int],
) -> None:
    url = ry.URL("http://user:pass@localhost:80/path?query#fragment")

    with pytest.deprecated_call(
        match="`replace_\\*` methods are deprecated, use `with_\\*` methods instead"
    ):
        deprecated_result = getattr(url, deprecated_method)(*args, **kwargs)

    # Call the new method
    new_result = getattr(url, new_method)(*args, **kwargs)

    # Verify that both results are the same
    assert deprecated_result == new_result
