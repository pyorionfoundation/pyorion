# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Clipboard API - Manage text and image content in the system clipboard.

This module provides asynchronous access to the system clipboard
for both text and images via the Rust event loop.
"""

import base64

from pyorion.runtime.runtime_handle import event_register
from pyorion.setup.types import ClipboardImage


class ClipboardAPI:
    """API for accessing the system clipboard.

    This class provides methods to set, retrieve, and clear text or image
    content in the system clipboard. All communication is handled through
    the internal event system (:func:`event_register`).
    """

    def __init__(self) -> None:
        """Initialize a new ClipboardAPI instance."""

    async def set_text(self, text: str) -> str:
        """Set the given text into the clipboard.

        :param text: The text to store in the clipboard.
        :type text: str
        :return: A confirmation message from the event system.
        :rtype: str
        """
        return await event_register("clipboard.set_text", args=[text])

    async def get_text(self) -> str:
        """Retrieve the current text from the clipboard.

        :return: The text stored in the clipboard.
        :rtype: str
        """
        return await event_register("clipboard.get_text", result_type=str)

    async def clear(self) -> str:
        """Clear all contents from the clipboard.

        :return: A confirmation message from the event system.
        :rtype: str
        """
        return await event_register("clipboard.clear")

    async def set_image(self, width: int, height: int, raw_bytes: bytes) -> str:
        """Set an image into the clipboard.

        :param width: The width of the image in pixels.
        :type width: int
        :param height: The height of the image in pixels.
        :type height: int
        :param raw_bytes: The raw image data (e.g., RGB or RGBA).
        :type raw_bytes: bytes
        :return: A confirmation message from the event system.
        :rtype: str
        """
        b64 = base64.b64encode(raw_bytes).decode("ascii")
        return await event_register("clipboard.set_image", args=[width, height, b64])

    async def get_image(self) -> ClipboardImage:
        """Retrieve the current image from the clipboard.

        :return: An object containing image data, width, and height.
        :rtype: ClipboardImage
        """
        return await event_register(
            method="clipboard.get_image",
            result_type=ClipboardImage,
        )
