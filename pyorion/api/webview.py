# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""WebView API - Control operations for PyOrion WebView instances.

Wrapper around the Rust event loop backend to manage developer tools
and WebView state.
"""

from pyorion.runtime.runtime_handle import event_register


class WebView:
    """High-level asynchronous wrapper for controlling a WebView instance."""

    def __init__(self) -> None:
        """Initialize a new :class:`WebView` handle."""

    async def is_devtools_open(self) -> bool:
        """Check if the WebView developer tools window is currently open.

        :return: ``True`` if DevTools is open, otherwise ``False``.
        :rtype: bool
        """
        return await event_register("webview.isDevtoolsOpen", None, result_type=bool)

    async def open_devtools(self) -> bool:
        """Open the WebView developer tools window.

        :return: ``True`` if the operation succeeded, otherwise ``False``.
        :rtype: bool
        """
        return await event_register("webview.openDevtools", None, result_type=bool)

    async def close_devtools(self) -> bool:
        """Close the WebView developer tools window.

        :return: ``True`` if the operation succeeded, otherwise ``False``.
        :rtype: bool
        """
        return await event_register("webview.closeDevtools", None, result_type=bool)
