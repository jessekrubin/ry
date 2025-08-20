# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "pydantic",
#     "rich",
#     "ry",
# ]
# ///
import asyncio
import hashlib
from typing import TYPE_CHECKING, Any

from pydantic.dataclasses import dataclass
from rich.console import Console

import ry

if TYPE_CHECKING:
    from collections.abc import Coroutine


PACKAGE_NAME = "ry"  # Change to your desired package
PYPI_URL = f"https://pypi.org/pypi/{PACKAGE_NAME}/json"
console = Console()


@dataclass(frozen=True)
class RyPackage:
    url: str
    version: str
    md5_digest: str
    size: int


def md5_hash(s: ry.Bytes) -> str:
    return hashlib.md5(s).hexdigest()  # noqa: S324


async def get_all_versions(package_name: str) -> list[str]:
    """Fetch all available versions of a package from PyPI."""
    response = await ry.fetch(f"https://pypi.org/pypi/{package_name}/json")
    if response.status_code != 200:
        err = Exception(f"Failed to fetch package data: {response.status_code}")
        raise err
    data = await response.json()
    return list(data["releases"].keys())


async def pypi_package_stats(package_name: str) -> tuple[int, int]:
    """Get the total size of all packages for a given package name."""
    response = await ry.fetch(f"https://pypi.org/pypi/{package_name}/json")
    if response.status_code != 200:
        err = Exception(f"Failed to fetch package data: {response.status_code}")
        raise err
    data = await response.json()
    total_size = sum(
        sum(pkg["size"] for pkg in data["releases"][version])
        for version in data["releases"]
    )
    total_number = sum(len(data["releases"][version]) for version in data["releases"])
    return total_size, total_number


async def get_wheel_urls(package_name: str, version: str) -> list[RyPackage]:
    """Fetch .whl file URLs for a specific version."""
    url = f"https://pypi.org/pypi/{package_name}/{version}/json"
    response = await ry.fetch(url)
    if response.status_code != 200:
        console.log(f"Failed to fetch version {version}: {response.status_code}")
        return []

    data = await response.json()
    return [
        RyPackage(
            url=file["url"],
            version=version,
            md5_digest=file["md5_digest"],
            size=file["size"],
        )
        for file in data["urls"]
        if (file["filename"].endswith(".whl") or file["filename"].endswith(".tar.gz"))
        and file["filename"]
    ]


async def scrape_all_wheels(package_name: str) -> dict[str, list[RyPackage]]:
    """Scrape all versions and their respective wheels."""
    versions = await get_all_versions(package_name)
    wheels = {}
    for version in versions:
        wheels[version] = await get_wheel_urls(package_name, version)
    return wheels


async def download_file(pkg: RyPackage, outdir: str) -> None:
    """Download a file from a URL to a specified directory."""
    filename = pkg.url.split("/")[-1]
    outpath = f"{outdir}/{filename}"
    if ry.FsPath(outpath).exists():
        console.log(f"{filename} already exists, skipping download.")
        return
    response = await ry.fetch(ry.URL(pkg.url))
    body = await response.bytes()
    await ry.write_async(outpath, body)
    console.log(f"Downloaded {filename}")


async def download_dists(
    wheels: dict[str, list[RyPackage]], *, by_version: bool = True
) -> None:
    """Download the wheel files."""

    outdir = "dist"
    ry.create_dir_all(outdir)
    if by_version:
        for version, urls in wheels.items():
            outdir = f"dist/{version}"
            ry.create_dir_all(outdir)
            await asyncio.gather(*(download_file(pkg, outdir) for pkg in urls))
    else:
        futs: list[Coroutine[Any, Any, None]] = []
        for version, pkgs in wheels.items():
            outdir = f"dist/{version}"
            ry.create_dir_all(outdir)
            futs.extend(download_file(pkg, outdir) for pkg in pkgs)
        await asyncio.gather(*futs)


async def main() -> None:
    wheels_data = await scrape_all_wheels(PACKAGE_NAME)
    total_size_of_all_wheels = sum(
        sum(pkg.size for pkg in pkgs) for pkgs in wheels_data.values()
    )
    # Save to a JSON file
    json_data = ry.stringify(wheels_data, fmt=True, append_newline=True)
    await ry.write_async(
        f"{PACKAGE_NAME}-wheels.json",
        json_data,
    )
    console.log(
        f"Scraped {PACKAGE_NAME}, saved wheel URLs to {PACKAGE_NAME}_wheels.json"
    )
    await download_dists(wheels_data, by_version=False)

    console.log(f"Total size of all wheels: {ry.fmt_size(total_size_of_all_wheels)}")


if __name__ == "__main__":
    from asyncio import run

    run(main())
