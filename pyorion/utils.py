# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Utility helpers for PyOrion."""

import base64
import dataclasses
import shutil
import socket
from pathlib import Path
from typing import Any

from pydantic import BaseModel


def find_folder(folder_name: str) -> Path | None:
    """Search recursively for a folder starting from the current working directory.

    :param folder_name: The name of the folder to search for.
    :type folder_name: str
    :return: Path to the folder if found, otherwise ``None``.
    :rtype: Path | None
    """
    start_path = Path.cwd()
    for path in start_path.rglob(folder_name):
        if path.is_dir():
            return path
    return None


def load_html(path: Path | str | None) -> str:
    """Load HTML content from a file or return a fallback snippet.

    :param path: File path to an HTML file. If ``None`` or missing, a fallback
                 snippet will be returned.
    :type path: Path | str | None
    :return: The HTML content.
    :rtype: str
    """
    if path is None:
        return _fallback_html()

    html_src = Path.cwd() / path
    if not html_src.exists():
        return _fallback_html()

    return html_src.read_text(encoding="utf-8")


def _fallback_html() -> str:
    """Return a default HTML snippet as fallback content.

    :return: A minimal HTML snippet.
    :rtype: str
    """
    return r""" ... """


def find_free_ports_and_create_addrs() -> str:
    """Find a free TCP port and return a localhost address.

    :return: Localhost address with a free port (e.g. ``127.0.0.1:8080``).
    :rtype: str
    """

    def find_free_port() -> int:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.bind(("", 0))
            return sock.getsockname()[1]

    port = find_free_port()
    return f"127.0.0.1:{port}"


def debug(fn: str, message: Any) -> None:
    """Print a debug message to stdout.

    :param fn: The function or source identifier.
    :type fn: str
    :param message: The message content to log.
    :type message: Any
    :return: None
    :rtype: None
    """
    print(f"[DEBUG] from {fn} message to send: {message}")


def make_json_safe(obj: Any) -> Any:
    """Convert arbitrary Python objects into JSON-serializable structures.

    :param obj: The object to convert.
    :type obj: Any
    :return: A JSON-safe representation of the object.
    :rtype: Any
    """
    if obj is None:
        return None
    if isinstance(obj, (str, int, float, bool)):
        return obj
    if isinstance(obj, Path):
        return str(obj)
    if isinstance(obj, (bytes, bytearray)):
        return base64.b64encode(obj).decode("ascii")
    if isinstance(obj, BaseModel):
        return obj.model_dump(by_alias=True)
    if dataclasses.is_dataclass(obj) and not isinstance(obj, type):
        return dataclasses.asdict(obj)
    if isinstance(obj, dict):
        return {str(k): make_json_safe(v) for k, v in obj.items()}
    if isinstance(obj, (list, tuple, set)):
        return [make_json_safe(v) for v in obj]
    return str(obj)


def normalize_args(args: Any | None) -> list[Any]:
    """Normalize arguments to a JSON-safe list.

    :param args: The arguments to normalize.
    :type args: Any | None
    :return: List of JSON-safe arguments.
    :rtype: list[Any]
    """
    if args is None:
        return []
    if isinstance(args, list):
        return [make_json_safe(a) for a in args]
    return [make_json_safe(args)]


def remove_pycash(directory: Path | str | None = None) -> None:
    """Remove all ``__pycache__`` directories recursively.

    :param directory: Start directory. Defaults to the current working directory.
    :type directory: Path | str | None
    :return: None
    :rtype: None
    """
    directory = Path.cwd() if directory is None else Path(directory)

    for pycache in directory.rglob("__pycache__"):
        try:
            shutil.rmtree(pycache)
            print(f"Deleted: {pycache}")
        except Exception as e:
            print(f"Error deleting {pycache}: {e}")
