from __future__ import annotations

from typing import TYPE_CHECKING, cast

import pytest

if TYPE_CHECKING:
    from ry.ryo3._lz4rip import (
        _Lz4BlockMode as Lz4BlockMode,  # pyright: ignore[reportMissingModuleSource]
    )
    from ry.ryo3._lz4rip import (
        _Lz4BlockSize as Lz4BlockSize,  # pyright: ignore[reportMissingModuleSource]
    )

_LZ4_BLOCK_MODES: tuple[Lz4BlockMode, ...] = (
    "independent",
    "linked",
)

_LZ4_BLOCK_SIZES: tuple[Lz4BlockSize, ...] = (
    "auto",
    "max-64kb",
    "max-256kb",
    "max-1mb",
    "max-4mb",
    0,
    4,
    5,
    6,
    7,
)


@pytest.fixture(params=_LZ4_BLOCK_MODES)
def lz4_block_mode(request: pytest.FixtureRequest) -> Lz4BlockMode:
    return cast("Lz4BlockMode", request.param)


@pytest.fixture
def lz4_block_modes() -> tuple[Lz4BlockMode, ...]:
    return _LZ4_BLOCK_MODES


@pytest.fixture(params=_LZ4_BLOCK_SIZES)
def lz4_block_size(request: pytest.FixtureRequest) -> Lz4BlockSize:
    return cast("Lz4BlockSize", request.param)


@pytest.fixture
def lz4_block_sizes() -> tuple[Lz4BlockSize, ...]:
    return _LZ4_BLOCK_SIZES
