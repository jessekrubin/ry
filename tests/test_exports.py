import ry
def test_has_build_profile():
    assert hasattr(ry, "__build_profile__")

def test_has_version_lib():
    assert hasattr(ry, "__version__")

def test_doc_is_not_none():
    assert hasattr(ry, "__doc__")
