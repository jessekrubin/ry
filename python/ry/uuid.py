"""ry.uuid ~ compat re-exports"""

import typing as t

from ry.ryo3 import (
    UUID,
    uuid1,
    uuid3,
    uuid4,
    uuid5,
    uuid6,
    uuid7,
    uuid8,
)

getnode = UUID.getnode
MAX: t.Final[UUID] = UUID.MAX
NAMESPACE_DNS: t.Final[UUID] = UUID.NAMESPACE_DNS
NAMESPACE_OID: t.Final[UUID] = UUID.NAMESPACE_OID
NAMESPACE_URL: t.Final[UUID] = UUID.NAMESPACE_URL
NAMESPACE_X500: t.Final[UUID] = UUID.NAMESPACE_X500
NIL: t.Final[UUID] = UUID.NIL
RESERVED_FUTURE: t.Final[str] = UUID.RESERVED_FUTURE
RESERVED_MICROSOFT: t.Final[str] = UUID.RESERVED_MICROSOFT
RESERVED_NCS: t.Final[str] = UUID.RESERVED_NCS
RFC_4122: t.Final[str] = UUID.RFC_4122

__all__ = (
    "MAX",
    "NAMESPACE_DNS",
    "NAMESPACE_OID",
    "NAMESPACE_URL",
    "NAMESPACE_X500",
    "NIL",
    "RESERVED_FUTURE",
    "RESERVED_MICROSOFT",
    "RESERVED_NCS",
    "RFC_4122",
    "UUID",
    "getnode",
    "uuid1",
    "uuid3",
    "uuid4",
    "uuid5",
    "uuid6",
    "uuid7",
    "uuid8",
)
