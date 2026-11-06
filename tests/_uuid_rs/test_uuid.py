from __future__ import annotations

import itertools as it
import operator
import os
import pickle
import typing as t
import uuid as pyuuid
import weakref

import pytest

import ry.uuid as ryuuid

_S = "12345678-1234-5678-1234-567812345678"
_RY = ryuuid.UUID(_S)
_PY = pyuuid.UUID(_S)

_LO = "00000000-0000-4000-8000-000000000001"
_HI = "ffffffff-ffff-4fff-bfff-ffffffffffff"
_NOT_UUIDS = [None, 1, 1.5, "abc", b"abc", object(), [1, 2]]


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


@pytest.mark.parametrize("proto", range(pickle.HIGHEST_PROTOCOL + 1))
def test_pickle_all_protocols(proto: int) -> None:
    import copy

    u = ryuuid.uuid4()
    loaded = pickle.loads(pickle.dumps(u, protocol=proto))
    assert type(loaded) is ryuuid.UUID
    assert loaded == u
    assert copy.copy(u) == u
    assert copy.deepcopy(u) == u


def test_uuid_weakref() -> None:
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
    init_kwargs_combinations = ({**a, **b} for a, b in it.combinations(dicsts, 2))
    for init_kwargs in init_kwargs_combinations:
        with pytest.raises(TypeError):
            ryuuid.UUID(**init_kwargs)  # type: ignore[call-overload]  # ty:ignore[no-matching-overload]


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


def test_constants() -> None:
    assert ryuuid.NIL == pyuuid.UUID(int=0)
    assert ryuuid.NAMESPACE_DNS == pyuuid.NAMESPACE_DNS
    assert ryuuid.NAMESPACE_URL == pyuuid.NAMESPACE_URL
    assert ryuuid.NAMESPACE_OID == pyuuid.NAMESPACE_OID
    assert ryuuid.NAMESPACE_X500 == pyuuid.NAMESPACE_X500


# =============================================================================
# format
# =============================================================================


class TestUUIDFormat:
    """`UUID.format`, `__format__`"""

    def _format_expected(
        self, uu: ryuuid.UUID, kind: t.Literal["hyphenated", "simple", "urn", "braced"]
    ) -> str:
        assert kind in {"hyphenated", "simple", "urn", "braced"}, (
            f"Unexpected kind: {kind}"
        )
        match kind:
            case "hyphenated":
                return str(uu)
            case "simple":
                return str(uu).replace("-", "")
            case "urn":
                return f"urn:uuid:{uu}"
            case "braced":
                return "{" + str(uu) + "}"
            case _:
                msg = f"Unexpected kind: {kind}"
                raise ValueError(msg)

    @pytest.mark.parametrize(("kind"), ["hyphenated", "simple", "urn", "braced"])
    def test_format_kinds(
        self, kind: t.Literal["hyphenated", "simple", "urn", "braced"]
    ) -> None:
        expected = self._format_expected(_RY, kind)
        assert _RY.format(kind) == expected
        assert _RY.format(kind=kind) == expected
        assert f"{_RY:{kind}}" == expected
        assert format(_RY, kind) == expected

    def test_format_default(self) -> None:
        assert f"{_RY}" == str(_RY)
        assert _RY.format() == str(_RY) == str(_PY)

    def test_url_simple(self) -> None:
        assert _RY.format("simple") == _PY.hex
        assert _RY.format("urn") == _PY.urn

    def test_format_bad_value(self) -> None:
        with pytest.raises(ValueError, match="Invalid uuid format"):
            _RY.format("nope")  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]

    def test_format_invalid_type(self) -> None:
        with pytest.raises(TypeError):
            _RY.format(123)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]

    def test_dunder_format(self) -> None:
        # requires arg
        with pytest.raises(TypeError):
            _RY.__format__()  # type: ignore[call-arg]  # noqa: PLC2801  # ty:ignore[missing-argument]
        with pytest.raises(TypeError):
            _RY.__format__(123)  # type: ignore[arg-type]  # noqa: PLC2801  # ty:ignore[invalid-argument-type]
        with pytest.raises(TypeError):
            _RY.__format__(format_spec="")  # type: ignore[call-arg]  # noqa: PLC2801  # ty:ignore[positional-only-parameter-as-kwarg]


class TestUUIDRichcmp:
    """`UUID.__richcmp__`"""

    @pytest.mark.parametrize(
        "op",
        [operator.eq, operator.ne, operator.lt, operator.le, operator.gt, operator.ge],
        ids=lambda o: o.__name__,
    )
    @pytest.mark.parametrize(("a", "b"), list(it.product([_LO, _HI], repeat=2)))
    def test_richcmp_stdlib_parity(
        self, op: t.Callable[[object, object], bool], a: str, b: str
    ) -> None:
        expected = op(pyuuid.UUID(a), pyuuid.UUID(b))
        assert op(ryuuid.UUID(a), ryuuid.UUID(b)) is expected  # ry vs ry
        assert op(ryuuid.UUID(a), pyuuid.UUID(b)) is expected  # ry vs stdlib

    @pytest.mark.parametrize("other", _NOT_UUIDS, ids=repr)
    def test_eq_ne_non_uuid(self, other: object) -> None:
        u = ryuuid.UUID(_LO)
        assert (pyuuid.UUID(_LO) == other) is False
        assert (u == other) is False
        assert (u != other) is True
        assert (other == u) is False
        assert (other != u) is True

    @pytest.mark.parametrize("other", _NOT_UUIDS, ids=repr)
    @pytest.mark.parametrize(
        "op",
        [operator.lt, operator.le, operator.gt, operator.ge],
        ids=lambda o: o.__name__,
    )
    def test_ord_nonuuid_err(
        self, op: t.Callable[[object, object], bool], other: object
    ) -> None:
        with pytest.raises(TypeError, match="not supported between instances"):
            op(ryuuid.UUID(_LO), other)

    def test_dunder_eq_not_implemented(self) -> None:
        u = ryuuid.UUID(_LO)
        assert u.__eq__(None) is NotImplemented  # noqa: PLC2801
        assert u.__lt__(1) is NotImplemented  # type: ignore[operator]  # noqa: PLC2801

    def test_mixed_list(self) -> None:
        u = ryuuid.UUID(_LO)
        assert u not in [None, 1, "abc"]
        assert u in [None, 1, pyuuid.UUID(_LO)]
        assert u in [None, ryuuid.UUID(_LO)]


class TestUUIDFork:
    """Fix for forking

    revealed by old flask server + gunicorn i was messing with
    """

    @pytest.mark.parametrize("gen", ["uuid4", "uuid7", "uuid8"])
    def test_fork_child_generator(self, gen: str) -> None:
        fork = getattr(os, "fork", None)
        if fork is None:
            pytest.skip("no fork")
        fn = getattr(ryuuid, gen)
        fn()
        for _ in range(10):
            r, w = os.pipe()
            pid = fork()
            if pid == 0:  # pragma: no cover - child
                try:
                    os.write(w, str(fn()).encode())
                finally:
                    os._exit(0)
            os.close(w)
            parent_next = str(fn())
            os.waitpid(pid, 0)
            child_next = os.read(r, 64).decode()
            os.close(r)
            assert child_next
            assert child_next != parent_next

    def test_reseed_rng_callable(self) -> None:
        assert ryuuid.UUID._reseed_rng() is None


# =============================================================================
# gen fns as class attrs (like `ry.JSON`)
# =============================================================================


class TestNamespaceUUID:
    @pytest.mark.parametrize(
        "name", ["uuid1", "uuid3", "uuid4", "uuid5", "uuid6", "uuid7", "uuid8"]
    )
    def test_gen_fns_on_class(self, name: str) -> None:
        cls_fn = getattr(ryuuid.UUID, name)
        mod_fn = getattr(ryuuid, name)
        assert callable(cls_fn)
        assert cls_fn.__name__ == mod_fn.__name__ == name
        assert getattr(ryuuid.uuid4(), name).__name__ == name

    def test_gen_fns_on_class_work(self) -> None:
        ns = ryuuid.NAMESPACE_DNS
        assert ryuuid.UUID.uuid3(ns, "x") == ryuuid.uuid3(ns, "x")
        assert ryuuid.UUID.uuid5(ns, "x") == ryuuid.uuid5(ns, "x")
        assert ryuuid.UUID.uuid4().version == 4
        assert ryuuid.UUID.uuid6().version == 6
        assert ryuuid.UUID.uuid7().version == 7
        assert ryuuid.UUID.uuid8().version == 8
        assert ryuuid.UUID.uuid8(a=1, b=2, c=3) == ryuuid.uuid8(a=1, b=2, c=3)
        # instance access too
        assert ryuuid.uuid4().uuid4().version == 4
        with pytest.raises(NotImplementedError):
            ryuuid.UUID.uuid1()
