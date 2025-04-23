import uuid as pyuuid

import ry.dev as ry


def test_uuid_strings() -> None:
    u = ry.UUID("12345678-1234-5678-1234-567812345678")
    assert str(u) == "12345678-1234-5678-1234-567812345678"
    assert repr(u) == "UUID('12345678-1234-5678-1234-567812345678')"


def test_uuid4_func() -> None:
    u = ry.uuid4()
    assert isinstance(u, ry.UUID)
    assert len(str(u)) == 36


def test_uuid_to_python() -> None:
    u = ry.UUID("12345678-1234-5678-1234-567812345678")
    assert u.to_py() == pyuuid.UUID("12345678-1234-5678-1234-567812345678")
