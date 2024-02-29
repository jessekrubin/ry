import ry


def test_has_build_profile() -> None:
    assert hasattr(ry, "__build_profile__")


def test_has_version_lib() -> None:
    assert hasattr(ry, "__version__")


def test_doc_is_not_none() -> None:
    assert hasattr(ry, "__doc__")
