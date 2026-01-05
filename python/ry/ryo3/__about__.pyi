import typing as t

__version__: str
__authors__: str
__build_profile__: str
__build_timestamp__: str
__pkg_name__: str
__description__: str
__target__: str
__opt_level__: t.Literal["0", "1", "2", "3", "s", "z"]
__allocator__: t.Literal["mimalloc", "system"]
__crypto_provider__: t.Literal["ring", "aws-lc-rs"]
