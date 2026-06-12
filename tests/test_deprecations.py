import re
import warnings

import pytest


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
