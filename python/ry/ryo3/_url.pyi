"""ryo3-url ~ types"""

import typing as t
from ipaddress import IPv4Address, IPv6Address

from ry._types import FsPathLike
from ry.protocols import FromStr, ToString, _Parse
from ry.ryo3._std import IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr

@t.final
class URL(FromStr, ToString, _Parse):
    def __new__(
        cls, url: str | bytes | URL, *, params: dict[str, str] | None = None
    ) -> t.Self: ...
    # =========================================================================
    # CLASSMETHODS
    # =========================================================================
    @classmethod
    def parse(cls, value: str | bytes, /) -> t.Self: ...
    @classmethod
    def from_str(cls, s: str, /) -> t.Self: ...
    @classmethod
    def parse_with_params(cls, url: str | bytes, params: dict[str, str]) -> t.Self: ...
    @classmethod
    def from_directory_path(cls, path: FsPathLike) -> t.Self: ...
    @classmethod
    def from_filepath(cls, path: FsPathLike) -> t.Self: ...
    # =========================================================================
    # STRING
    # =========================================================================
    def to_string(self) -> str: ...
    def __fspath__(self) -> str: ...

    # =========================================================================
    # PROPERTIES
    # =========================================================================
    @property
    def authority(self) -> str: ...
    @property
    def domain(self) -> str | None: ...
    @property
    def fragment(self) -> str | None: ...
    @property
    def host(self) -> str | Ipv4Addr | Ipv6Addr | None: ...
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
    def port(self) -> int | None:
        """The port number for this URL, if any.

        Note
        ----
        the default port numbers are never reflected by the serialization,
        use the `port_or_known_default` if you want a default port number returned.

        Default port numbers:
            - `http`  | `ws`  => `80`
            - `https` | `wss` => `443`
            - `ftp`           => `21`

        Examples
        --------
        >>> from ry import URL
        >>> assert URL("https://rotatingsandwiches.com:3000").port == 3000
        >>> assert URL("https://rotatingsandwiches.com").port is None
        >>> assert URL("https://rotatingsandwiches.com:443/").port is None
        >>> assert URL("ssh://rotatingsandwiches.com:22").port == 22

        """

    @property
    def port_or_known_default(self) -> int | None:
        """The port number, or the default port number if known.

        Default port numbers:
            - `http`  | `ws`  => `80`
            - `https` | `wss` => `443`
            - `ftp`           => `21`

        Examples
        --------
        >>> from ry import URL
        >>> URL("https://rotatingsandwiches.com:3000").port_or_known_default
        3000
        >>> URL("https://rotatingsandwiches.com").port_or_known_default
        443
        >>> URL("https://rotatingsandwiches.com:443/").port_or_known_default
        443
        >>> URL("ssh://rotatingsandwiches.com:22").port_or_known_default
        22

        """
    @property
    def query(self) -> str | None: ...
    @property
    def query_string(self) -> str: ...
    @property
    def query_pairs(self) -> tuple[tuple[str, str], ...]: ...
    @property
    def scheme(self) -> str: ...
    @property
    def user(self) -> str:
        """alias for yarl compatibility"""
    @property
    def username(self) -> str: ...
    @property
    def origin(self) -> str: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def has_authority(self) -> bool: ...
    def has_host(self) -> bool: ...
    def equiv(self, other: URL | str) -> bool: ...
    def is_special(self) -> bool: ...
    def join(self, *parts: str) -> URL: ...
    def make_relative(self, other: URL) -> t.Self: ...
    def socket_addrs(
        self, default_port_number: int | None = None
    ) -> list[SocketAddr]: ...
    def to_filepath(self) -> str: ...
    def replace(
        self,
        *,
        scheme: str = ...,
        username: str = ...,
        password: str | None = ...,
        host: str
        | IPv4Address
        | IPv6Address
        | Ipv4Addr
        | Ipv6Addr
        | IpAddr
        | None = ...,
        port: int | None = ...,
        path: str = ...,
        query: str | None = ...,
        fragment: str | None = ...,
    ) -> t.Self:
        """Return a new URL with the given parts replaced (like `dataclasses.replace`)

        Omitted kwargs are kept and `None` clears a part. Parts are applied in
        url order (scheme, username, password, host, port, path, query,
        fragment), so combined replacements don't depend on kwarg order.

        Parameters
        ----------
        scheme : str, optional
            url scheme (`http`, `https`, `ws`, `wss`, `ftp`, `file`)
        username : str, optional
            username; `""` removes it (the password is kept).
        password : str | None, optional
            New password; `None` removes it.
        host : str | IPv4Address | IPv6Address | Ipv4Addr | Ipv6Addr | IpAddr | None, optional
            host to set; a str is parsed based on the URL's scheme.
        port : int | None, optional
            port (0-65535); `None` removes it.
        path : str, optional
            path; a leading `/` is added for urls with a host.
        query : str | None, optional
            query string (without the `?`); `None` removes it, `""` leaves
            an empty `?`.
        fragment : str | None, optional
            fragment (without the `#`); `None` removes it, `""` leaves an empty `#`.

        Raises
        ------
        TypeError
            If `None` is given for `scheme`, `username` or `path`, or wrong kw type
        ValueError
            If the url can't take the new value (e.g. a port on a `file:` url,
            removing the host of an `http` url, an invalid host).
        OverflowError
            If `port` is outside 0-65535.

        Examples
        --------
        >>> import ry
        >>> u = ry.URL("http://user:pass@example.com:8080/foo?a=1#frag")
        >>> u.replace(path="/bar", query=None, fragment=None)
        URL('http://user:pass@example.com:8080/bar')
        >>> u.replace(port=None, password=None)
        URL('http://user@example.com/foo?a=1#frag')
        >>> u.replace(host=ry.Ipv4Addr(127, 0, 0, 1))
        URL('http://user:pass@127.0.0.1:8080/foo?a=1#frag')

        """
    def with_fragment(self, fragment: str | None = None) -> t.Self:
        """Return a new URL with the fragment replaced

        Parameters
        ----------
        fragment : str | None, optional
            fragment (without the `#`); `None` (the default) removes it, `""`
            leaves an empty `#`.

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/a#x").with_fragment("y")
        URL('http://example.com/a#y')
        >>> ry.URL("http://example.com/a#x").with_fragment(None)
        URL('http://example.com/a')

        """
    def with_host(
        self,
        host: str
        | IPv4Address
        | IPv6Address
        | Ipv4Addr
        | Ipv6Addr
        | IpAddr
        | None = None,
    ) -> t.Self:
        """Return a new URL with the host replaced

        Parameters
        ----------
        host : str | IPv4Address | IPv6Address | Ipv4Addr | Ipv6Addr | IpAddr | None, optional
            host to set; a str is parsed based on the URL's scheme. `None`
            (the default) removes it.

        Raises
        ------
        ValueError
            If the host is invalid or can't be removed.

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/a").with_host("example.org")
        URL('http://example.org/a')
        >>> ry.URL("http://example.com/a").with_host(ry.Ipv6Addr.parse("::1"))
        URL('http://[::1]/a')
        >>> ry.URL("foo://example.com/a").with_host(None)
        URL('foo:/a')

        """
    def with_ip_host(
        self, address: IPv4Address | IPv6Address | Ipv4Addr | Ipv6Addr | IpAddr
    ) -> t.Self:
        """Return a new URL with an ip address as the host

        Parameters
        ----------
        address : IPv4Address | IPv6Address | Ipv4Addr | Ipv6Addr | IpAddr
            ip address to set as the host (`with_host` & `replace(host=...)`
            accept ip addresses too).

        Raises
        ------
        ValueError
            If the url can't have a host (e.g. `mailto:`).

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/a").with_ip_host(ry.Ipv4Addr(10, 0, 0, 1))
        URL('http://10.0.0.1/a')

        """
    def with_password(self, password: str | None = None) -> t.Self:
        """Return a new URL with the password replaced

        Parameters
        ----------
        password : str | None, optional
            password; `None` (the default) removes it.

        Raises
        ------
        ValueError
            If the url can't have a password (no host, or `file:`).

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://user@example.com/").with_password("pw")
        URL('http://user:pw@example.com/')

        """
    def with_path(self, path: str) -> t.Self:
        """Return a new URL with the path replaced

        Parameters
        ----------
        path : str
            path; a leading `/` is added for urls with a host.

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/a?q=1").with_path("b/c")
        URL('http://example.com/b/c?q=1')

        """
    def with_port(self, port: int | None = None) -> t.Self:
        """Return a new URL with the port replaced

        Parameters
        ----------
        port : int | None, optional
            port (0-65535); `None` (the default) removes it.

        Raises
        ------
        ValueError
            If the url can't have a port (no host, or `file:`).
        OverflowError
            If `port` is outside 0-65535.

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/").with_port(8080)
        URL('http://example.com:8080/')
        >>> ry.URL("http://example.com:8080/").with_port(None)
        URL('http://example.com/')

        """
    def with_query(self, query: str | None = None) -> t.Self:
        """Return a new URL with the query string replaced

        Parameters
        ----------
        query : str | None, optional
            query string (without the `?`); `None` (the default) removes it,
            `""` leaves an empty `?`.

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/a?x=1").with_query("y=2")
        URL('http://example.com/a?y=2')
        >>> ry.URL("http://example.com/a?x=1").with_query(None)
        URL('http://example.com/a')

        """
    def with_scheme(self, scheme: str) -> t.Self:
        """Return a new URL with the scheme replaced

        Parameters
        ----------
        scheme : str
            url scheme (`http`, `https`, `ws`, `wss`, `ftp`, `file`)

        Raises
        ------
        ValueError
            If the scheme is invalid or the switch isn't allowed.

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/").with_scheme("https")
        URL('https://example.com/')

        """
    def with_username(self, username: str) -> t.Self:
        """Return a new URL with the username replaced

        Parameters
        ----------
        username : str
            username; `""` removes it (the password is kept).

        Raises
        ------
        ValueError
            If the url can't have a username (no host, or `file:`).

        Examples
        --------
        >>> import ry
        >>> ry.URL("http://example.com/").with_username("me")
        URL('http://me@example.com/')

        """

    # =========================================================================
    # OPERATORS/DUNDER
    # =========================================================================
    def __len__(self) -> int: ...
    def __truediv__(self, relative: str, /) -> t.Self: ...
    def __rtruediv__(self, relative: str, /) -> t.Self: ...
    def __lt__(self, other: t.Self, /) -> bool: ...
    def __le__(self, other: t.Self, /) -> bool: ...
    def __eq__(self, other: object, /) -> bool: ...
    def __ne__(self, other: object, /) -> bool: ...
    def __gt__(self, other: t.Self, /) -> bool: ...
    def __ge__(self, other: t.Self, /) -> bool: ...
    def __hash__(self) -> int: ...
