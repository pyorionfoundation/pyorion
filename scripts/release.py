# ruff: noqa: D101, D102, D103

"""Release orionframe Python package.

Accepts a string as a parameter, e.g. "py:mypackage:v0.1.0", with parts separated by `/`.

- The first part must be "py"
- The second part is the package name
- The third part is the semver version number
"""

import argparse
import asyncio
import sys
from argparse import ArgumentTypeError
from asyncio import create_subprocess_exec
from enum import Enum
from logging import basicConfig, getLogger
from os import getenv
from typing import NamedTuple, NoReturn


logger = getLogger(__name__)


class Kind(Enum):
    PY = "py"


class ReleaseTag(NamedTuple):
    kind: Kind
    package: str
    version: str
    """Version number without leading `v`."""

    @staticmethod
    def parse(release_tag: str):
        # got it from `GITHUB_REF` env var
        release_tag = release_tag.removeprefix("refs/tags/")

        kind, package, version = release_tag.split("/")

        if version[0] != "v":
            raise ArgumentTypeError(
                f"version number should start with 'v', got: {version}"
            )

        return ReleaseTag(Kind(kind), package, version[1:])

    def write_to_github_output(self) -> None:
        github_output = getenv("GITHUB_OUTPUT")
        if github_output is None:
            logger.warning(
                "`$GITHUB_OUTPUT` is not set, skipping setting github output."
            )
            return
        with open(github_output, "w") as f:
            print(f"kind={self.kind.value}", file=f)
            print(f"package={self.package}", file=f)
            print(f"version={self.version}", file=f)


parser = argparse.ArgumentParser(description="Release orionframe Python package.")
parser.add_argument(
    "release_tag",
    type=ReleaseTag.parse,
    help="release string, e.g. '[refs/tags/]py/mypackage/v0.1.0'",
)
parser.add_argument(
    "--no-dry-run",
    action="store_true",
)


_ASSERT_NEVER_REPR_MAX_LENGTH = 100


def _assert_never(arg: NoReturn, /) -> NoReturn:
    value = repr(arg)
    if len(value) > _ASSERT_NEVER_REPR_MAX_LENGTH:
        value = value[:_ASSERT_NEVER_REPR_MAX_LENGTH] + "..."
    raise AssertionError(f"Expected code to be unreachable, but got: {value}")


async def release_py(package: str, no_dry_run: bool) -> int:
    # <https://docs.astral.sh/uv/guides/publish/>
    #
    # NOTE: use `uv build`(without `--wheel` and `--sdist`) to build `wheel` from `sdist`,
    # so that we can notice broken `sdist`. This is important for wheel with extension modules.
    # ref: <https://docs.astral.sh/uv/concepts/projects/build/#using-uv-build>
    args = ["build", "--package", package, "--no-sources", "--color", "always"]
    if no_dry_run:
        raise RuntimeError(
            "python package should only be released by `pypa/gh-action-pypi-publish`"
        )

    proc = await create_subprocess_exec("uv", *args)
    await proc.wait()

    assert proc.returncode is not None
    return proc.returncode


if __name__ == "__main__":
    basicConfig(level="INFO")

    args = parser.parse_args()
    assert isinstance(args.release_tag, ReleaseTag)
    assert isinstance(args.no_dry_run, bool)

    release_tag = args.release_tag
    no_dry_run = args.no_dry_run

    logger.info(f"kind={release_tag.kind.value}")
    logger.info(f"package={release_tag.package}")
    logger.info(f"version={release_tag.version}")
    release_tag.write_to_github_output()

    async def main() -> int:
        if release_tag.kind == Kind.PY:
            return await release_py(release_tag.package, no_dry_run)
        else:
            _assert_never(release_tag.kind)

    sys.exit(asyncio.run(main()))
