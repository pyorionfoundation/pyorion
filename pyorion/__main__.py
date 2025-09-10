# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""pyorion.__main__

Command-line entry point for the PyOrion framework.
"""

import click

from pyorion.utils import remove_pycash as _remove_pycash


@click.group()
def cli() -> None:
    """PyOrion CLI."""


@cli.command()
def remove_pycash() -> None:
    """Remove all __pycache__ directories."""
    _remove_pycash()


if __name__ == "__main__":
    cli()
