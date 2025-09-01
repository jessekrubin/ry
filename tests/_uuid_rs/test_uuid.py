from __future__ import annotations

import itertools
import pickle
import uuid as pyuuid
import weakref

import pytest

import ry.uuid as ryuuid


def test_uuid_strings() -> None:
    u = ryuuid.UUID("12345678-1234-5678-1234-567812345678")
    assert str(u) == "12345678-1234-5678-1234-567812345678"
    assert repr(u) == "UUID('12345678-1234-5678-1234-567812345678')"


def test_pickle() -> None:
    u = ryuuid.UUID("12345678-1234-5678-1234-567812345678")
    pickled = pickle.dumps(u)
    unpickled = pickle.loads(pickled)
    assert isinstance(unpickled, ryuuid.UUID)
    assert str(unpickled) == "12345678-1234-5678-1234-567812345678"


def test_uuid_weakref() -> None:
    # bpo-35701: check that weak referencing to a UUID object can be created
    strong = ryuuid.uuid4()
    weak = weakref.ref(strong)
    assert isinstance(weak, weakref.ref)


def test_uuid4_func() -> None:
    u = ryuuid.uuid4()
    assert isinstance(u, ryuuid.UUID)
    assert len(str(u)) == 36


def test_uuid_to_python() -> None:
    u = ryuuid.UUID("12345678-1234-5678-1234-567812345678")
    assert u.to_py() == pyuuid.UUID("12345678-1234-5678-1234-567812345678")


def test_init() -> None:
    # Test the UUID constructor
    with pytest.raises(TypeError):
        ryuuid.UUID()


def test_init_multiple_kwargs_invalid() -> None:
    pyu = pyuuid.UUID("12345678-1234-5678-1234-567812345678")
    init_kwargs = {
        "hex": pyu.hex,
        "bytes": pyu.bytes,
        "bytes_le": pyu.bytes_le,
        "fields": pyu.fields,
        "int": pyu.int,
    }
    dicsts = [{k: v} for k, v in init_kwargs.items()]
    init_kwargs_combinations = (
        {**a, **b} for a, b in itertools.combinations(dicsts, 2)
    )
    for init_kwargs in init_kwargs_combinations:
        with pytest.raises(TypeError):
            ryuuid.UUID(**init_kwargs)  # type: ignore[arg-type]


def test_create_uuid() -> None:
    """Test the UUID constructor

    Based on the python uuid docs: https://docs.python.org/3/library/uuid.html#uuid.UUID
    """
    uuids = [
        ryuuid.UUID("{12345678-1234-5678-1234-567812345678}"),
        ryuuid.UUID("12345678123456781234567812345678"),
        ryuuid.UUID("urn:uuid:12345678-1234-5678-1234-567812345678"),
        ryuuid.UUID(bytes=b"\x12\x34\x56\x78" * 4),
        ryuuid.UUID(
            bytes_le=b"\x78\x56\x34\x12\x34\x12\x78\x56\x12\x34\x56\x78\x12\x34\x56\x78"
        ),
        ryuuid.UUID(fields=(0x12345678, 0x1234, 0x5678, 0x12, 0x34, 0x567812345678)),
        ryuuid.UUID(int=0x12345678123456781234567812345678),
    ]
    assert len(uuids) == 7
    assert all(isinstance(u, ryuuid.UUID) for u in uuids)

    strings = [
        (ix, str(u) == "12345678-1234-5678-1234-567812345678", str(u))
        for ix, u in enumerate(uuids)
    ]
    assert all(s[1] for s in strings), f"UUIDs did not match expected string: {strings}"


def test_fields() -> None:
    # Test the fields property
    u = ryuuid.UUID("12345678-1234-5678-1234-567812345678")
    assert isinstance(u.fields, tuple)
    assert u.fields == (0x12345678, 0x1234, 0x5678, 0x12, 0x34, 0x567812345678)
    assert u.int == 0x12345678123456781234567812345678
    assert u.hex == "12345678123456781234567812345678"
    assert u.urn == "urn:uuid:12345678-1234-5678-1234-567812345678"


def test_equality() -> None:
    rs_u = ryuuid.UUID("12345678-1234-5678-1234-567812345678")
    py_u = rs_u.to_py()
    assert rs_u == py_u
    assert py_u == rs_u
