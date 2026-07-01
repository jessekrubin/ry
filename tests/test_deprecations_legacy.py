from __future__ import annotations

import re
import typing as t

import pytest

import ry


def _deprecation_message(
    old: str, new: str, removal_version: str, *, escape: bool = True
) -> str:
    msg = f"`{old}` is deprecated; use `{new}` instead [removal: {removal_version}]"
    return re.escape(msg) if escape else msg


@pytest.mark.skip(reason="removed in v0.0.93")
class TestJiffDeprecationsInTz:
    """Test deprecations for objs that have an `.intz(tz:str)->Self:` method"""

    def test_jiff_intz_deprecation_date(self) -> None:
        msg = re.escape(
            "`Date.intz` is deprecated; use `Date.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.Date.today().intz("UTC")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]

    def test_jiff_intz_deprecation_datetime(self) -> None:
        msg = re.escape(
            "`DateTime.intz` is deprecated; use `DateTime.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.now().datetime().intz("UTC")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]

    def test_jiff_intz_deprecation_timestamp(self) -> None:
        msg = re.escape(
            "`Timestamp.intz` is deprecated; use `Timestamp.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.now().timestamp().intz("UTC")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]

    def test_jiff_intz_deprecation_zoned_datetime(self) -> None:
        msg = re.escape(
            "`ZonedDateTime.intz` is deprecated; use `ZonedDateTime.in_tz` instead [removal: v0.0.93]"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _d = ry.now().intz("UTC")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]


class TestTokioDeprecations:
    @pytest.mark.skip(reason="removed in v0.0.93")
    @pytest.mark.anyio
    async def test_aiopen_raises_deprecation_warning(self) -> None:
        with pytest.deprecated_call(
            match=re.escape(
                "`aiopen` is deprecated; use `aopen` instead [removal: v0.0.93]"
            )
        ):
            _ = ry.aiopen("some_path.txt", "rb")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]


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


@pytest.mark.skip(reason="removed in v0.0.93")
class TestReqwestDeprecations:
    def test_http_client_deprecation_warning(self) -> None:
        with pytest.deprecated_call(
            match=re.escape(
                "`HttpClient` is deprecated; use `Client` instead [removal: v0.0.93]"
            )
        ):
            _c = ry.HttpClient()  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]


@pytest.mark.skip(reason="removed in v0.0.96")
class TestJiffDeprecations:
    def test_jiff_signed_duration_from_isoformat_deprecated(self) -> None:
        msg = _deprecation_message(
            "SignedDuration.from_isoformat", "SignedDuration.fromisoformat", "v0.0.96"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _sd = ry.SignedDuration.from_isoformat("PT48m")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]

    def test_jiff_timespan_from_isoformat_deprecated(self) -> None:
        msg = _deprecation_message(
            "TimeSpan.from_isoformat", "TimeSpan.fromisoformat", "v0.0.96"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _s = ry.TimeSpan.from_isoformat("PT48m")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]

    def test_jiff_span_parse_common_iso_deprecated(self) -> None:
        msg = _deprecation_message(
            "TimeSpan.parse_common_iso", "TimeSpan.fromisoformat", "v0.0.96"
        )
        with pytest.warns(DeprecationWarning, match=msg):
            _s = ry.TimeSpan.parse_common_iso("PT48m")  # type: ignore[attr-defined]  # ty:ignore[unresolved-attribute]
