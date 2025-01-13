from __future__ import annotations

import ry


def test_check_build_profile() -> None:
    assert ry.__build_profile__ is not None
    assert ry.__build_profile__ == "debug" or ry.__build_profile__ == "release", (
        f"utiles.__build_profile__ is not 'debug'/'release': {ry.__build_profile__}"
    )
