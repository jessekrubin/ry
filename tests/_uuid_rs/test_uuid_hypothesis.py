from __future__ import annotations

import copy
import pickle
import uuid as pyuuid

from hypothesis import given
from hypothesis import strategies as st

import ry.uuid as ryuuid

uuid_strategy = st.uuids()


def py_ry_equal(
    pyu: pyuuid.UUID,
    ryu: ryuuid.UUID,
) -> None:
    assert isinstance(pyu, pyuuid.UUID)
    assert isinstance(ryu, ryuuid.UUID)

    assert str(pyu) == str(ryu)
    assert repr(pyu) == repr(ryu)
    assert pyu.hex == ryu.hex
    assert pyu.int == ryu.int
    assert pyu.urn == ryu.urn
    assert pyu.bytes == ryu.bytes
    assert pyu.bytes_le == ryu.bytes_le
    assert pyu.fields == ryu.fields
    assert pyu.time_low == ryu.time_low
    assert pyu.time_mid == ryu.time_mid
    assert pyu.time_hi_version == ryu.time_hi_version
    assert pyu.clock_seq_hi_variant == ryu.clock_seq_hi_variant
    assert pyu.clock_seq_low == ryu.clock_seq_low
    assert pyu.node == ryu.node
    assert pyu.time == ryu.time
    assert pyu.clock_seq == ryu.clock_seq
    assert pyu.variant == ryu.variant
    if pyu.version is not None:
        assert pyu.version == ryu.version
    assert ryu == pyu


@given(uuid_strategy)
def test_pyuuid_equiv(py_obj: pyuuid.UUID) -> None:
    ry_obj = ryuuid.UUID(py_obj.hex)
    py_ry_equal(py_obj, ry_obj)


@given(uuid_strategy)
def test_pickle_round_trip(py_obj: pyuuid.UUID) -> None:
    ry_obj = ryuuid.UUID(py_obj.hex)
    pickled = pickle.dumps(ry_obj)
    unpickled = pickle.loads(pickled)
    assert isinstance(unpickled, ryuuid.UUID)
    assert str(unpickled) == str(ry_obj)
    assert ry_obj == unpickled


@given(uuid_strategy)
def test_copy(py_obj: pyuuid.UUID) -> None:
    ry_obj = ryuuid.UUID(py_obj.hex)
    assert isinstance(ry_obj, ryuuid.UUID)
    assert isinstance(copy.copy(ry_obj), ryuuid.UUID)
    assert isinstance(copy.deepcopy(ry_obj), ryuuid.UUID)
    assert copy.copy(ry_obj) == ry_obj
    assert copy.deepcopy(ry_obj) == ry_obj
    assert copy.copy(ry_obj) is not ry_obj
    assert copy.deepcopy(ry_obj) is not ry_obj
