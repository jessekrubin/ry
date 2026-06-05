import sys
import typing as t

if sys.version_info >= (3, 13):
    from warnings import deprecated
else:
    from typing_extensions import deprecated

from ry.ryo3 import (
    audio_dir,
    cache_dir,
    config_dir,
    config_local_dir,
    data_dir,
    data_local_dir,
    desktop_dir,
    document_dir,
    download_dir,
    executable_dir,
    font_dir,
    home_dir,
    picture_dir,
    preference_dir,
    public_dir,
    runtime_dir,
    state_dir,
    template_dir,
    video_dir,
)

deprecated(
    "`ry.dirs` is deprecated; use `ry.*_dir` functions instead [removal: 0.0.96]"
)


def _deprecated_dir_fn(
    name: str, dir_fn: t.Callable[[], str | None]
) -> t.Callable[[], str | None]:
    # @
    def _dir_fn() -> str | None:
        return dir_fn()

    return deprecated(
        f"`ry.dirs.{name}` is deprecated; use `ry.{name}_dir` instead [removal: 0.0.96]"  # type: ignore
    )(_dir_fn)


audio = _deprecated_dir_fn("audio", audio_dir)
cache = _deprecated_dir_fn("cache", cache_dir)
config = _deprecated_dir_fn("config", config_dir)
config_local = _deprecated_dir_fn("config_local", config_local_dir)
data = _deprecated_dir_fn("data", data_dir)
data_local = _deprecated_dir_fn("data_local", data_local_dir)
desktop = _deprecated_dir_fn("desktop", desktop_dir)
document = _deprecated_dir_fn("document", document_dir)
download = _deprecated_dir_fn("download", download_dir)
executable = _deprecated_dir_fn("executable", executable_dir)
font = _deprecated_dir_fn("font", font_dir)
home = _deprecated_dir_fn("home", home_dir)
picture = _deprecated_dir_fn("picture", picture_dir)
preference = _deprecated_dir_fn("preference", preference_dir)
public = _deprecated_dir_fn("public", public_dir)
runtime = _deprecated_dir_fn("runtime", runtime_dir)
state = _deprecated_dir_fn("state", state_dir)
template = _deprecated_dir_fn("template", template_dir)
video = _deprecated_dir_fn("video", video_dir)
__all__ = (
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
