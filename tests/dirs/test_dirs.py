from __future__ import annotations

import pytest

import ry

_DIRS_FNS = (
    "audio_dir",
    "cache_dir",
    "config_dir",
    "config_local_dir",
    "data_dir",
    "data_local_dir",
    "desktop_dir",
    "document_dir",
    "download_dir",
    "executable_dir",
    "font_dir",
    "home_dir",
    "picture_dir",
    "preference_dir",
    "public_dir",
    "runtime_dir",
    "state_dir",
    "template_dir",
    "video_dir",
)


@pytest.mark.parametrize("fn", _DIRS_FNS)
def test_dirs_w_suffix_in_ry_root(fn: str) -> None:
    assert hasattr(ry, fn)
    fn = getattr(ry, fn)
    assert callable(fn)


@pytest.mark.parametrize("fn", _DIRS_FNS)
def test_dirs_fn_is_str_or_none(fn: str) -> None:
    res = getattr(ry, fn)()
    assert res is None or isinstance(res, str)
