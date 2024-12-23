from __future__ import annotations

import pytest

from ry import dirs

fns = (
    "audio",
    "audio_dir",
    "cache",
    "cache_dir",
    "config",
    "config_dir",
    "config_local",
    "config_local_dir",
    "data",
    "data_dir",
    "data_local",
    "data_local_dir",
    "desktop",
    "desktop_dir",
    "document",
    "document_dir",
    "download",
    "download_dir",
    "executable",
    "executable_dir",
    "font",
    "font_dir",
    "home",
    "home_dir",
    "picture",
    "picture_dir",
    "preference",
    "preference_dir",
    "public",
    "public_dir",
    "runtime",
    "runtime_dir",
    "state",
    "state_dir",
    "template",
    "template_dir",
    "video",
    "video_dir",
)


@pytest.mark.parametrize("fn", fns)
def test_dirs_fn_exists(fn: str) -> None:
    assert hasattr(dirs, fn)
    fn = getattr(dirs, fn)
    assert callable(fn)


@pytest.mark.parametrize("fn", fns)
def test_dirs_fn_is_str_or_none(fn: str) -> None:
    res = getattr(dirs, fn)()
    assert res is None or isinstance(res, str)
