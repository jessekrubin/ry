import re
import warnings

import pytest

import ry


def _deprecation_message(
    old: str, new: str, removal_version: str, *, escape: bool = True
) -> str:
    msg = f"`{old}` is deprecated; use `{new}` instead [removal: {removal_version}]"
    return re.escape(msg) if escape else msg


def test_deprecation_msg() -> None:
    assert (
        _deprecation_message("old_func", "new_func", "v0.1.0", escape=False)
        == "`old_func` is deprecated; use `new_func` instead [removal: v0.1.0]"
    )


class TestDeprecationWarningEg:
    def _deprecated_fn(self) -> None:
        msg = _deprecation_message(
            "ry.deprecated_fn", "ry.new_fn", "v0.1.0", escape=False
        )
        warnings.warn(msg, DeprecationWarning, stacklevel=1)

    def test_deprecated_fn_warning(self) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message("ry.deprecated_fn", "ry.new_fn", "v0.1.0"),
        ):
            self._deprecated_fn()


# =============================================================================
# `Headers.stringify`/`Headers.from_json` (remove in v0.0.113)
# =============================================================================
class TestHeadersJsonDeprecation:
    h = ry.Headers({"accept": "*/*", "set-cookie": ["b=2", "a=1"]})

    def test_stringify(self) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "Headers.stringify", "ry.stringify(headers)", "v0.0.113"
            ),
        ):
            s = self.h.stringify()  # type: ignore[deprecated]  # ty: ignore[deprecated]  # pyrefly: ignore[deprecated]
        assert s == ry.stringify(self.h).decode()

    def test_from_json(self) -> None:
        data = ry.stringify(self.h).decode()
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "Headers.from_json", "Headers(ry.parse_json(data))", "v0.0.113"
            ),
        ):
            h = ry.Headers.from_json(data)  # type: ignore[deprecated]  # ty: ignore[deprecated]  # pyrefly: ignore[deprecated]
        assert h == ry.Headers(ry.parse_json(data)) == self.h  # type: ignore[arg-type]  # ty: ignore[invalid-argument-type]
