import typing as t

from ry._types import Buffer

SHA1_BLOCK_SIZE: t.TypeAlias = t.Literal[64]
SHA1_DIGEST_SIZE: t.TypeAlias = t.Literal[20]

SHA224_BLOCK_SIZE: t.TypeAlias = t.Literal[64]
SHA224_DIGEST_SIZE: t.TypeAlias = t.Literal[28]

SHA256_BLOCK_SIZE: t.TypeAlias = t.Literal[64]
SHA256_DIGEST_SIZE: t.TypeAlias = t.Literal[32]

SHA384_BLOCK_SIZE: t.TypeAlias = t.Literal[128]
SHA384_DIGEST_SIZE: t.TypeAlias = t.Literal[48]

SHA3_256_BLOCK_SIZE: t.TypeAlias = t.Literal[136]
SHA3_256_DIGEST_SIZE: t.TypeAlias = t.Literal[32]

SHA3_384_BLOCK_SIZE: t.TypeAlias = t.Literal[104]
SHA3_384_DIGEST_SIZE: t.TypeAlias = t.Literal[48]

SHA3_512_BLOCK_SIZE: t.TypeAlias = t.Literal[72]
SHA3_512_DIGEST_SIZE: t.TypeAlias = t.Literal[64]

SHA512_BLOCK_SIZE: t.TypeAlias = t.Literal[128]
SHA512_DIGEST_SIZE: t.TypeAlias = t.Literal[64]

SHA512_256_BLOCK_SIZE: t.TypeAlias = t.Literal[128]
SHA512_256_DIGEST_SIZE: t.TypeAlias = t.Literal[32]

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

# fmt: off
sha1: type[_Sha[t.Literal["sha1"], SHA1_BLOCK_SIZE, SHA1_DIGEST_SIZE]]
sha224: type[_Sha[t.Literal["sha224"], SHA224_BLOCK_SIZE, SHA224_DIGEST_SIZE]]
sha256: type[_Sha[t.Literal["sha256"], SHA256_BLOCK_SIZE, SHA256_DIGEST_SIZE]]
sha384: type[_Sha[t.Literal["sha384"], SHA384_BLOCK_SIZE, SHA384_DIGEST_SIZE]]
sha3_256: type[_Sha[t.Literal["sha3_256"], SHA3_256_BLOCK_SIZE, SHA3_256_DIGEST_SIZE]]
sha3_384: type[_Sha[t.Literal["sha3_384"], SHA3_384_BLOCK_SIZE, SHA3_384_DIGEST_SIZE]]
sha3_512: type[_Sha[t.Literal["sha3_512"], SHA3_512_BLOCK_SIZE, SHA3_512_DIGEST_SIZE]]
sha512: type[_Sha[t.Literal["sha512"], SHA512_BLOCK_SIZE, SHA512_DIGEST_SIZE]]
sha512_256: type[_Sha[t.Literal["sha512_256"], SHA512_256_BLOCK_SIZE, SHA512_256_DIGEST_SIZE]]
# fmt: on
