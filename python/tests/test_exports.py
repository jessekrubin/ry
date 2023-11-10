import ry


def test_has_build_profile():
    assert hasattr(ry, "__build_profile__")

def test_has_version_lib():
    assert hasattr(ry, "__version__")
