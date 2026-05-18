from __future__ import annotations

import re
import typing as t

import pytest

import ry


class TestTokioDeprecations:
    @pytest.mark.skip(reason="removed in v0.0.93")
    @pytest.mark.anyio
    async def test_aiopen_raises_deprecation_warning(self) -> None:
        with pytest.deprecated_call(
            match=re.escape(
                "`aiopen` is deprecated; use `aopen` instead [removal: v0.0.93]"
            )
        ):
            _ = ry.aiopen("some_path.txt", "rb")  # type: ignore[attr-defined]


@pytest.mark.skip(reason="removed in v0.0.93")
class TestUrlReplaceMethodsDeprecations:
    def test_every_url_replace_method_has_with_counterpart(
        self,
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
        self,
        deprecated_method: str,
        new_method: str,
        args: tuple[t.Any, ...],
        kwargs: dict[str, str | int],
    ) -> None:
        url = ry.URL("http://user:pass@localhost:80/path?query#fragment")

        warning_msg = re.escape(
            f"`URL.{deprecated_method}` is deprecated; use `URL.{new_method}` instead [removal: v0.0.93]"
        )
        with pytest.deprecated_call(match=warning_msg):
            deprecated_result = getattr(url, deprecated_method)(*args, **kwargs)

        # Call the new method
        new_result = getattr(url, new_method)(*args, **kwargs)

        # Verify that both results are the same
        assert deprecated_result == new_result
