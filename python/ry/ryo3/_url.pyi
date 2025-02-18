from __future__ import annotations

class URL:
    def __init__(
        self, url: str | URL, *, params: dict[str, str] | None = None
    ) -> None: ...

    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def parse(cls, url: str) -> URL: ...
    @classmethod
    def parse_with_params(cls, url: str, params: dict[str, str]) -> URL: ...
    @classmethod
    def from_directory_path(cls, path: str) -> URL: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    # =========================================================================
    # OPERATORS/DUNDER
    # =========================================================================
    def __eq__(self, other: object) -> bool: ...
    def __ge__(self, other: URL) -> bool: ...
    def __gt__(self, other: URL) -> bool: ...
    def __hash__(self) -> int: ...
    def __le__(self, other: URL) -> bool: ...
    def __lt__(self, other: URL) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __rtruediv__(self, relative: str) -> URL: ...
    def __truediv__(self, relative: str) -> URL: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def authority(self) -> str: ...
    @property
    def fragment(self) -> str | None: ...
    @property
    def host(self) -> str | None: ...
    @property
    def host_str(self) -> str | None: ...
    @property
    def netloc(self) -> str: ...
    @property
    def password(self) -> str | None: ...
    @property
    def path(self) -> str: ...
    @property
    def path_segments(self) -> tuple[str, ...]: ...
    @property
    def port(self) -> int | None: ...
    @property
    def port_or_known_default(self) -> int | None: ...
    @property
    def query(self) -> str | None: ...
    @property
    def query_pairs(self) -> list[tuple[str, str]]: ...
    @property
    def scheme(self) -> str: ...
    @property
    def username(self) -> str: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def has_authority(self) -> bool: ...
    def has_host(self) -> bool: ...
    def is_special(self) -> bool: ...
    def join(self, *parts: str) -> URL: ...
    def to_filepath(self) -> str: ...
    def set_fragment(self, fragment: str) -> None: ...
    def set_host(self, host: str) -> None: ...
    def set_ip_host(self, host: str) -> None: ...
    def set_password(self, password: str) -> None: ...
    def set_path(self, path: str) -> None: ...
    def set_port(self, port: int) -> None: ...
    def set_query(self, query: str) -> None: ...
    def set_scheme(self, scheme: str) -> None: ...
    def set_username(self, username: str) -> None: ...
    def socket_addrs(self) -> None: ...
