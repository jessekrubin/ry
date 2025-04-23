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


def test_uuid_thing() -> None:
    # Test various ways to create UUIDs

    uuids = [
        ry.UUID("{12345678-1234-5678-1234-567812345678}"),
        ry.UUID("12345678123456781234567812345678"),
        ry.UUID("urn:uuid:12345678-1234-5678-1234-567812345678"),
        ry.UUID(bytes=b"\x12\x34\x56\x78" * 4),
        ry.UUID(
            bytes_le=b"\x78\x56\x34\x12\x34\x12\x78\x56\x12\x34\x56\x78\x12\x34\x56\x78"
        ),
        ry.UUID(fields=(0x12345678, 0x1234, 0x5678, 0x12, 0x34, 0x567812345678)),
        ry.UUID(int=0x12345678123456781234567812345678),
    ]
    assert len(uuids) == 7
    assert all(isinstance(u, ry.UUID) for u in uuids)
    assert all(str(u) == "12345678-1234-5678-1234-567812345678" for u in uuids)


def test_fields() -> None:
    # Test the fields property
    u = ry.UUID("12345678-1234-5678-1234-567812345678")
    assert isinstance(u.fields, tuple)
    assert u.fields == (0x12345678, 0x1234, 0x5678, 0x12, 0x34, 0x567812345678)
    assert u.int == 0x12345678123456781234567812345678
    assert u.hex == "12345678123456781234567812345678"
    assert u.urn == "urn:uuid:12345678-1234-5678-1234-567812345678"


def test_equality() -> None:
    rs_u = ry.UUID("12345678-1234-5678-1234-567812345678")
    py_u = rs_u.to_py()
    assert rs_u == py_u
    assert py_u == rs_u
