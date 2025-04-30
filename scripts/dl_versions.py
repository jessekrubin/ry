# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "ry",
# ]
# ///
import asyncio
import dataclasses
import hashlib
import json
from collections.abc import Coroutine
from typing import Any

import ry

PACKAGE_NAME = "ry"  # Change to your desired package
PYPI_URL = f"https://pypi.org/pypi/{PACKAGE_NAME}/json"


@dataclasses.dataclass
class RyPackage:
    url: str
    version: str
    md5_digest: str


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


async def get_wheel_urls(package_name: str, version: str) -> list[RyPackage]:
    """Fetch .whl file URLs for a specific version."""
    url = f"https://pypi.org/pypi/{package_name}/{version}/json"
    response = await ry.fetch(url)
    if response.status_code != 200:
        print(f"Failed to fetch version {version}: {response.status_code}")
        return []

    data = await response.json()
    return [
        RyPackage(url=file["url"], version=version, md5_digest=file["md5_digest"])
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
        print(f"{filename} already exists, skipping download.")
        return
    response = await ry.fetch(ry.URL(pkg.url))
    body = await response.bytes()
    await ry.write_async(outpath, body)
    print(f"Downloaded {filename}")


async def download_dists(
    wheels: dict[str, list[RyPackage]], by_version: bool = True
) -> None:
    """Download the wheel files."""

    outdir = ry.FsPath("dist") / PACKAGE_NAME
    ry.create_dir_all(outdir)
    if by_version:
        for version, urls in wheels.items():
            outdir_str = str(outdir / f"{version}")
            ry.create_dir_all(outdir_str)
            await asyncio.gather(*(download_file(pkg, outdir_str) for pkg in urls))
    else:
        futs: list[Coroutine[Any, Any, None]] = []
        for version, pkgs in wheels.items():
            outdir_str = str(outdir / f"{version}")
            ry.create_dir_all(outdir_str)
            futs.extend(download_file(pkg, outdir_str) for pkg in pkgs)
        await asyncio.gather(*futs)


async def main() -> None:
    wheels_data = await scrape_all_wheels(PACKAGE_NAME)
    print(ry.json_cache_usage())
    return

    # Save to a JSON file
    with open(f"{PACKAGE_NAME}_wheels.json", "w") as f:
        json.dump(
            {
                version: [dataclasses.asdict(pkg) for pkg in pkgs]
                for version, pkgs in wheels_data.items()
            },
            f,
            indent=4,
        )

    print(f"Scraped {PACKAGE_NAME}, saved wheel URLs to {PACKAGE_NAME}_wheels.json")
    await download_dists(wheels_data, by_version=False)


if __name__ == "__main__":
    try:
        import uvloop

        uvloop.run(main())
    except ImportError:
        from asyncio import run

        run(main())
