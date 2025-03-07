"""ry api ~ type annotations"""

from __future__ import annotations

import datetime as pydt
import typing as t

from ry import dirs as dirs  # noqa: RUF100
from ry import http as http  # noqa: RUF100
from ry import xxhash as xxhash  # noqa: RUF100
from ry._types import Buffer as Buffer  # noqa: RUF100
from ry._types import FsPathLike
from ry.http import Headers as Headers  # noqa: RUF100
from ry.http import HttpStatus as HttpStatus  # noqa: RUF100

from ._brotli import brotli as brotli
from ._brotli import brotli_decode as brotli_decode
from ._brotli import brotli_encode as brotli_encode
from ._bytes import Bytes as Bytes
from ._bzip2 import bzip2 as bzip2
from ._bzip2 import bzip2_decode as bzip2_decode
from ._bzip2 import bzip2_encode as bzip2_encode
from ._flate2 import gunzip as gunzip
from ._flate2 import gzip as gzip
from ._flate2 import gzip_decode as gzip_decode
from ._flate2 import gzip_encode as gzip_encode
from ._fnv import FnvHasher as FnvHasher
from ._fnv import fnv1a as fnv1a
from ._fspath import FsPath as FsPath
from ._globset import Glob as Glob
from ._globset import GlobSet as GlobSet
from ._globset import Globster as Globster
from ._globset import glob as glob
from ._globset import globster as globster
from ._heck import camel_case as camel_case
from ._heck import kebab_case as kebab_case
from ._heck import pascal_case as pascal_case
from ._heck import shouty_kebab_case as shouty_kebab_case
from ._heck import shouty_snake_case as shouty_snake_case
from ._heck import snake_case as snake_case
from ._heck import snek_case as snek_case
from ._heck import title_case as title_case
from ._heck import train_case as train_case
from ._jiff import Date as Date
from ._jiff import DateDifference as DateDifference
from ._jiff import DateTime as DateTime
from ._jiff import DateTimeDifference as DateTimeDifference
from ._jiff import DateTimeRound as DateTimeRound
from ._jiff import ISOWeekDate as ISOWeekDate
from ._jiff import Offset as Offset
from ._jiff import SignedDuration as SignedDuration
from ._jiff import Time as Time
from ._jiff import TimeDifference as TimeDifference
from ._jiff import TimeSpan as TimeSpan
from ._jiff import Timestamp as Timestamp
from ._jiff import TimestampDifference as TimestampDifference
from ._jiff import TimestampRound as TimestampRound
from ._jiff import TimeZone as TimeZone
from ._jiff import ZonedDateTime as ZonedDateTime
from ._jiff import ZonedDateTimeDifference as ZonedDateTimeDifference
from ._jiff import ZonedDateTimeRound as ZonedDateTimeRound
from ._jiff import date as date
from ._jiff import datetime as datetime
from ._jiff import offset as offset
from ._jiff import time as time
from ._jiff import timespan as timespan
from ._jiter import JsonParseKwargs as JsonParseKwargs
from ._jiter import JsonPrimitive as JsonPrimitive
from ._jiter import JsonValue as JsonValue
from ._jiter import json_cache_clear as json_cache_clear
from ._jiter import json_cache_usage as json_cache_usage
from ._jiter import parse_json as parse_json
from ._jiter import parse_json_bytes as parse_json_bytes
from ._quick_maths import quick_maths as quick_maths
from ._regex import Regex as Regex
from ._reqwest import HttpClient as HttpClient
from ._reqwest import ReqwestError as ReqwestError
from ._reqwest import Response as Response
from ._reqwest import ResponseStream as ResponseStream
from ._reqwest import fetch as fetch
from ._same_file import is_same_file as is_same_file
from ._shlex import shplit as shplit
from ._size import Size as Size
from ._size import SizeFormatter as SizeFormatter
from ._size import fmt_size as fmt_size
from ._size import parse_size as parse_size
from ._sqlformat import SqlfmtQueryParams as SqlfmtQueryParams
from ._sqlformat import sqlfmt as sqlfmt
from ._sqlformat import sqlfmt_params as sqlfmt_params
from ._std import Duration as Duration
from ._std import FileType as FileType
from ._std import Instant as Instant
from ._std import Metadata as Metadata
from ._std import canonicalize as canonicalize
from ._std import copy as copy
from ._std import create_dir as create_dir
from ._std import create_dir_all as create_dir_all
from ._std import exists as exists
from ._std import instant as instant
from ._std import is_dir as is_dir
from ._std import is_file as is_file
from ._std import is_symlink as is_symlink
from ._std import metadata as metadata
from ._std import read as read
from ._std import read_bytes as read_bytes
from ._std import read_stream as read_stream
from ._std import read_text as read_text
from ._std import remove_dir as remove_dir
from ._std import remove_dir_all as remove_dir_all
from ._std import remove_file as remove_file
from ._std import rename as rename
from ._std import sleep as sleep
from ._std import write as write
from ._std import write_bytes as write_bytes
from ._std import write_text as write_text
from ._tokio import asleep as asleep
from ._tokio import copy_async as copy_async
from ._tokio import create_dir_async as create_dir_async
from ._tokio import metadata_async as metadata_async
from ._tokio import read_async as read_async
from ._tokio import read_dir_async as read_dir_async
from ._tokio import remove_dir_async as remove_dir_async
from ._tokio import remove_file_async as remove_file_async
from ._tokio import rename_async as rename_async
from ._tokio import sleep_async as sleep_async
from ._tokio import write_async as write_async
from ._unindent import unindent as unindent
from ._unindent import unindent_bytes as unindent_bytes
from ._url import URL as URL
from ._walkdir import WalkDirEntry as WalkDirEntry
from ._walkdir import WalkdirGen as WalkdirGen
from ._walkdir import walkdir as walkdir
from ._which import which as which
from ._which import which_all as which_all
from ._which import which_re as which_re
from ._zstd import zstd as zstd
from ._zstd import zstd_decode as zstd_decode
from ._zstd import zstd_encode as zstd_encode
from .errors import FeatureNotEnabledError as FeatureNotEnabledError

# =============================================================================
# CONSTANTS
# =============================================================================
__version__: str
__authors__: str
__build_profile__: str
__build_timestamp__: str
__pkg_name__: str
__description__: str

# =============================================================================
# SH
# =============================================================================
def pwd() -> str: ...
def home() -> str: ...
def cd(path: FsPathLike) -> None: ...
@t.overload
def ls(
    path: FsPathLike | None = None,
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[False] = False,
) -> list[str]:
    """List directory contents - returns list of strings"""

@t.overload
def ls(
    path: FsPathLike | None = None,
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[True] = True,
) -> list[FsPath]:
    """List directory contents - returns list of FsPath objects"""
