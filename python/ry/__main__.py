from __future__ import annotations

import os
import sys

from ry import ryo3
from ry.__about__ import (
    __allocator__,
    __authors__,
    __build_profile__,
    __build_timestamp__,
    __crypto_provider__,
    __git_repo__,
    __git_sha__,
    __opt_level__,
    __pkgroot__,
    __target__,
    __title__,
    __version__,
)


def _lib_info() -> dict[str, object]:
    size = os.path.getsize(ryo3.__file__)
    bi = ryo3._build_info()
    return {
        "package": __title__,
        "version": __version__,
        "authors": __authors__,
        "pkgroot": __pkgroot__,
        "git": {
            "sha": __git_sha__,
            "repo": __git_repo__,
        },
        "ryo3": {
            "file": os.path.basename(ryo3.__file__),
            "bytes": size,
            "size": ryo3.fmt_size(size),
            "build": {
                "profile": __build_profile__,
                "opt_level": __opt_level__,
                "timestamp": __build_timestamp__,
                "target": __target__,
                "host": bi["host"],
                "allocator": __allocator__,
                "crypto_provider": __crypto_provider__,
            },
            "toolchain": {
                "rustc": bi["rustc"],
                "llvm": bi["llvm"],
                "linker": bi["linker"],
                "rustflags": bi["rustflags"],
                "cc": bi["cc"],
            },
        },
    }


def main() -> None:
    """Print package metadata"""
    json_out = ryo3.stringify(_lib_info(), fmt=True, pybytes=True, append_newline=True)
    sys.stdout.buffer.write(json_out)


if __name__ == "__main__":
    main()
