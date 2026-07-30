"""ryo3-aws-lc ~ types"""

import typing as t

from ry._types import Buffer

_Sha1Name: t.TypeAlias = t.Literal["sha1"]
_Sha1BlockSize: t.TypeAlias = t.Literal[64]
_Sha1DigestSize: t.TypeAlias = t.Literal[20]

_Sha224Name: t.TypeAlias = t.Literal["sha224"]
_Sha224BlockSize: t.TypeAlias = t.Literal[64]
_Sha224DigestSize: t.TypeAlias = t.Literal[28]

_Sha256Name: t.TypeAlias = t.Literal["sha256"]
_Sha256BlockSize: t.TypeAlias = t.Literal[64]
_Sha256DigestSize: t.TypeAlias = t.Literal[32]

_Sha384Name: t.TypeAlias = t.Literal["sha384"]
_Sha384BlockSize: t.TypeAlias = t.Literal[128]
_Sha384DigestSize: t.TypeAlias = t.Literal[48]

_Sha3_256Name: t.TypeAlias = t.Literal["sha3_256"]
_Sha3_256BlockSize: t.TypeAlias = t.Literal[136]
_Sha3_256DigestSize: t.TypeAlias = t.Literal[32]

_Sha3_384Name: t.TypeAlias = t.Literal["sha3_384"]
_Sha3_384BlockSize: t.TypeAlias = t.Literal[104]
_Sha3_384DigestSize: t.TypeAlias = t.Literal[48]

_Sha3_512Name: t.TypeAlias = t.Literal["sha3_512"]
_Sha3_512BlockSize: t.TypeAlias = t.Literal[72]
_Sha3_512DigestSize: t.TypeAlias = t.Literal[64]

_Sha512Name: t.TypeAlias = t.Literal["sha512"]
_Sha512BlockSize: t.TypeAlias = t.Literal[128]
_Sha512DigestSize: t.TypeAlias = t.Literal[64]

_Sha512_256Name: t.TypeAlias = t.Literal["sha512_256"]
_Sha512_256BlockSize: t.TypeAlias = t.Literal[128]
_Sha512_256DigestSize: t.TypeAlias = t.Literal[32]

_TName = t.TypeVar("_TName", bound=str)
_TBlockSize = t.TypeVar("_TBlockSize", bound=int)
_TDigestSize = t.TypeVar("_TDigestSize", bound=int)

@t.type_check_only
class _Sha(t.Generic[_TName, _TBlockSize, _TDigestSize]):
    name: _TName
    digest_size: _TDigestSize
    block_size: _TBlockSize
    def copy(self) -> t.Self: ...
    def digest(self) -> bytes: ...
    def hexdigest(self) -> str: ...
    def update(self, obj: Buffer, /) -> None: ...
    @staticmethod
    def oneshot(data: Buffer) -> bytes: ...
    @staticmethod
    def oneshot_hex(data: Buffer) -> str: ...

sha1: type[_Sha[_Sha1Name, _Sha1BlockSize, _Sha1DigestSize]]
sha224: type[_Sha[_Sha224Name, _Sha224BlockSize, _Sha224DigestSize]]
sha256: type[_Sha[_Sha256Name, _Sha256BlockSize, _Sha256DigestSize]]
sha384: type[_Sha[_Sha384Name, _Sha384BlockSize, _Sha384DigestSize]]
sha3_256: type[_Sha[_Sha3_256Name, _Sha3_256BlockSize, _Sha3_256DigestSize]]
sha3_384: type[_Sha[_Sha3_384Name, _Sha3_384BlockSize, _Sha3_384DigestSize]]
sha3_512: type[_Sha[_Sha3_512Name, _Sha3_512BlockSize, _Sha3_512DigestSize]]
sha512: type[_Sha[_Sha512Name, _Sha512BlockSize, _Sha512DigestSize]]
sha512_256: type[_Sha[_Sha512_256Name, _Sha512_256BlockSize, _Sha512_256DigestSize]]
