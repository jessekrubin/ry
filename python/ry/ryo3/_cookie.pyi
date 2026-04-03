import typing as t

from ry.protocols import FromStr, _Parse
from ry.ryo3._std import Duration

_SameSiteKw: t.TypeAlias = t.Literal["Lax", "lax", "Strict", "strict", "None", "none"]
"""same-site kwarg allows title-case and lower-case values"""

@t.final
class Cookie(FromStr, _Parse):
    def __init__(
        self,
        name: str,
        value: str,
        *,
        domain: str | None = None,
        expires: int | None = None,
        http_only: bool | None = None,
        max_age: Duration | None = None,
        partitioned: bool | None = None,
        path: str | None = None,
        permanent: bool = False,
        removal: bool = False,
        same_site: _SameSiteKw | None = None,
        secure: bool | None = None,
    ) -> None:
        """Create a new cookie with the given name and value, and optional attributes

        Args:
            name: The name of the cookie
            value: The value of the cookie
            domain: The domain of the cookie (optional)
            expires: The expiration time of the cookie as a UNIX timestamp (optional)
            http_only: Whether the cookie is HTTP-only (optional)
            max_age: The maximum age of the cookie as a Duration (optional)
            partitioned: Whether the cookie is partitioned (optional)
            path: The path of the cookie (optional)
            permanent: Whether the cookie is permanent (optional)
            removal: Whether the cookie is marked for removal (optional)
            same_site: The same-site attribute of the cookie (optional)
            secure: Whether the cookie is secure (optional)

        Examples:
            >>> from ry import Cookie
            >>> c = Cookie("id", "cookie-monster")
            >>> c
            Cookie("id", "cookie-monster")
            >>> str(c)
            'id=cookie-monster'

        """
    @classmethod
    def from_str(cls, s: str) -> t.Self: ...
    @classmethod
    def parse(cls, s: str | bytes) -> t.Self: ...
    @staticmethod
    def parse_encoded(s: str) -> Cookie: ...

    # -------------------------------------------------------------------------
    # METHODS
    # -------------------------------------------------------------------------
    # -- STRING --
    def encoded(self) -> str: ...
    def stripped(self) -> str: ...
    def encoded_stripped(self) -> str: ...
    def stripped_encoded(self) -> str: ...

    # -------------------------------------------------------------------------
    # PROPERTIES
    # -------------------------------------------------------------------------
    @property
    def name(self) -> str: ...
    @property
    def value(self) -> str: ...
    @property
    def value_trimmed(self) -> str: ...
    @property
    def name_value(self) -> tuple[str, str]: ...
    @property
    def name_value_trimmed(self) -> tuple[str, str]: ...
    @property
    def domain(self) -> str | None: ...
    @property
    def expires(self) -> int | None: ...
    @property
    def http_only(self) -> bool | None: ...
    @property
    def max_age(self) -> Duration | None: ...
    @property
    def partitioned(self) -> bool | None: ...
    @property
    def path(self) -> str | None: ...
    @property
    def same_site(self) -> t.Literal["Lax", "Strict", "None"] | None: ...
    @property
    def secure(self) -> bool | None: ...
