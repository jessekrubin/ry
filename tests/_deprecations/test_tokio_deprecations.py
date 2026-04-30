from __future__ import annotations

import pytest

import ry


@pytest.mark.anyio
async def test_aiopen_raises_deprecation_warning() -> None:
    with pytest.deprecated_call(match="`aiopen` is deprecated, use `aopen` instead"):
        _ = ry.aiopen("some_path.txt", "rb")  # type: ignore[deprecated]
