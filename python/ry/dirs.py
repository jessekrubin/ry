import sys

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

audio = deprecated(
    "`ry.audio` is deprecated; use `ry.audio_dir` instead [removal: 0.0.96]"
)(audio_dir)
cache = deprecated(
    "`ry.cache` is deprecated; use `ry.cache_dir` instead [removal: 0.0.96]"
)(cache_dir)
config = deprecated(
    "`ry.config` is deprecated; use `ry.config_dir` instead [removal: 0.0.96]"
)(config_dir)
config_local = deprecated(
    "`ry.config_local` is deprecated; use `ry.config_local_dir` instead [removal: 0.0.96]"
)(config_local_dir)
data = deprecated(
    "`ry.data` is deprecated; use `ry.data_dir` instead [removal: 0.0.96]"
)(data_dir)
data_local = deprecated(
    "`ry.data_local` is deprecated; use `ry.data_local_dir` instead [removal: 0.0.96]"
)(data_local_dir)
desktop = deprecated(
    "`ry.desktop` is deprecated; use `ry.desktop_dir` instead [removal: 0.0.96]"
)(desktop_dir)
document = deprecated(
    "`ry.document` is deprecated; use `ry.document_dir` instead [removal: 0.0.96]"
)(document_dir)
download = deprecated(
    "`ry.download` is deprecated; use `ry.download_dir` instead [removal: 0.0.96]"
)(download_dir)
executable = deprecated(
    "`ry.executable` is deprecated; use `ry.executable_dir` instead [removal: 0.0.96]"
)(executable_dir)
font = deprecated(
    "`ry.font` is deprecated; use `ry.font_dir` instead [removal: 0.0.96]"
)(font_dir)
home = deprecated(
    "`ry.home` is deprecated; use `ry.home_dir` instead [removal: 0.0.96]"
)(home_dir)
picture = deprecated(
    "`ry.picture` is deprecated; use `ry.picture_dir` instead [removal: 0.0.96]"
)(picture_dir)
preference = deprecated(
    "`ry.preference` is deprecated; use `ry.preference_dir` instead [removal: 0.0.96]"
)(preference_dir)
public = deprecated(
    "`ry.public` is deprecated; use `ry.public_dir` instead [removal: 0.0.96]"
)(public_dir)
runtime = deprecated(
    "`ry.runtime` is deprecated; use `ry.runtime_dir` instead [removal: 0.0.96]"
)(runtime_dir)
state = deprecated(
    "`ry.state` is deprecated; use `ry.state_dir` instead [removal: 0.0.96]"
)(state_dir)
template = deprecated(
    "`ry.template` is deprecated; use `ry.template_dir` instead [removal: 0.0.96]"
)(template_dir)
video = deprecated(
    "`ry.video` is deprecated; use `ry.video_dir` instead [removal: 0.0.96]"
)(video_dir)

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
