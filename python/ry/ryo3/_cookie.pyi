import typing as t

from ry.protocols import FromStr, _Parse
from ry.ryo3._std import Duration

_SameSiteKw: t.TypeAlias = t.Literal["Lax", "lax", "Strict", "strict", "None", "none"]
"""same-site kwarg allows title-case and lower-case values"""

@t.final
class Cookie(FromStr, _Parse):
    def __new__(
        cls,
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
    ) -> t.Self:
        """Create a new cookie with the given name and value, and optional attributes

        Parameters
        ----------
        name : str
            name of the cookie
        value : str
            value of the cookie
        domain : str | None, optional
            domain of the cookie, by default None
        expires : int | None, optional
            expiration time of the cookie as a UNIX timestamp, by default None
        http_only : bool | None, optional
            whether the cookie is HTTP-only, by default None
        max_age : Duration | None, optional
            maximum age of the cookie as a Duration, by default None
        partitioned : bool | None, optional
            whether the cookie is partitioned, by default None
        path : str | None, optional
            path of the cookie, by default None
        permanent : bool, optional
            whether the cookie is permanent, by default False
        removal : bool, optional
            whether the cookie is marked for removal, by default False
        same_site : _SameSiteKw | None, optional
            same-site attribute of the cookie, by default None
        secure : bool | None, optional
            whether the cookie is secure, by default None

        Examples
        --------
        >>> from ry import Cookie
        >>> c = Cookie("id", "cookie-monster")
        >>> c
        Cookie("id", "cookie-monster")
        >>> str(c)
        'id=cookie-monster'

        """
    @classmethod
    def from_str(cls, s: str, /) -> t.Self: ...
    @classmethod
    def parse(cls, value: str | bytes, /) -> t.Self: ...
    @staticmethod
    def parse_encoded(s: str, /) -> Cookie: ...

    # -------------------------------------------------------------------------
    # METHODS
    # -------------------------------------------------------------------------
    # -- STRING --
    def to_string(self) -> str: ...
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
