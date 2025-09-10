# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Dialog API - File pickers and message dialogs.

Wrapper around the Rust event loop backend to show dialogs,
pick files or directories, and display messages.
"""

from typing import Any, Literal, Optional

from pyorion.runtime.runtime_handle import event_register


MessageLevel = Literal["info", "warning", "error"]


class DialogAPI:
    """Asynchronous API wrapper for dialog operations."""

    def __init__(self) -> None:
        """Initialize a new :class:`DialogAPI` instance."""

    async def show_message(
        self,
        title: str,
        content: Optional[str] = None,
        level: Optional[MessageLevel] = "info",
    ) -> None:
        """Show a message dialog.

        :param title: The title of the dialog window.
        :type title: str
        :param content: The main message text to display.
        :type content: Optional[str]
        :param level: Message level (``"info"``, ``"warning"``, or ``"error"``).
        :type level: Optional[MessageLevel]

        :return: Nothing.
        :rtype: None
        """
        args = [title, content, level]
        return await event_register("dialog.showMessage", args=args)

    async def pick_file(
        self,
        filters: Optional[list[str]] = None,
        start_dir: Optional[str] = None,
    ) -> dict[str, Any]:
        """Open a file picker dialog for a single file.

        :param filters: List of file type filters (e.g. ``["*.txt", "*.jpg"]``).
        :type filters: Optional[list[str]]
        :param start_dir: The starting directory path.
        :type start_dir: Optional[str]

        :return: Information about the selected file.
        :rtype: dict[str, Any]
        """
        args = [filters, start_dir]
        return await event_register("dialog.pickFile", args=args)

    async def pick_files(
        self,
        filters: Optional[list[str]] = None,
        start_dir: Optional[str] = None,
    ) -> dict[str, Any]:
        """Open a file picker dialog for multiple files.

        :param filters: List of file type filters.
        :type filters: Optional[list[str]]
        :param start_dir: The starting directory path.
        :type start_dir: Optional[str]

        :return: Information about the selected files.
        :rtype: dict[str, Any]
        """
        args = [filters, start_dir]
        return await event_register("dialog.pickFiles", args=args)

    async def pick_dir(
        self,
        start_dir: Optional[str] = None,
    ) -> dict[str, Any]:
        """Open a directory picker dialog for a single directory.

        :param start_dir: The starting directory path.
        :type start_dir: Optional[str]

        :return: Information about the selected directory.
        :rtype: dict[str, Any]
        """
        args = [start_dir]
        return await event_register("dialog.pickDir", args=args)

    async def pick_dirs(
        self,
        start_dir: Optional[str] = None,
    ) -> dict[str, Any]:
        """Open a directory picker dialog for multiple directories.

        :param start_dir: The starting directory path.
        :type start_dir: Optional[str]

        :return: Information about the selected directories.
        :rtype: dict[str, Any]
        """
        args = [start_dir]
        return await event_register("dialog.pickDirs", args=args)

    async def save_file(
        self,
        filters: Optional[list[str]] = None,
        start_dir: Optional[str] = None,
    ) -> dict[str, Any]:
        """Open a file save dialog.

        :param filters: List of file type filters.
        :type filters: Optional[list[str]]
        :param start_dir: The starting directory path.
        :type start_dir: Optional[str]

        :return: Information about the saved file.
        :rtype: dict[str, Any]
        """
        args = [filters, start_dir]
        return await event_register("dialog.saveFile", args=args)
