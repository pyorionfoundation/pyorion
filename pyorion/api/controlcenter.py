# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""ControlCenter API - System notifications and related operations.

Wrapper around the Rust event loop backend to send notifications
and interact with the system Control Center.
"""

from typing import Any, Optional

from pyorion.runtime.runtime_handle import event_register


class ControlCenterAPI:
    """Asynchronous API wrapper for system Control Center operations."""

    def __init__(self) -> None:
        """Initialize a new :class:`ControlCenterAPI` instance."""

    async def notification(
        self,
        summary: str,
        body: Optional[str] = None,
        app_id: Optional[str] = None,
        appname: Optional[str] = None,
        icon: Optional[str] = None,
        auto_icon: Optional[bool] = None,
        image_path: Optional[str] = None,
        sound_name: Optional[str] = None,
        subtitle: Optional[str] = None,
        timeout: Optional[int] = None,
        notification_id: Optional[int] = None,
        action: Optional[tuple[str, str]] = None,
    ) -> Optional[dict[str, Any]]:
        """Send a system notification through the Control Center.

        :param summary: The notification summary (title text).
        :type summary: str
        :param body: The body text of the notification.
        :type body: Optional[str]
        :param app_id: Optional application identifier.
        :type app_id: Optional[str]
        :param appname: Application name to display in the notification.
        :type appname: Optional[str]
        :param icon: Path to an icon file.
        :type icon: Optional[str]
        :param auto_icon: Whether to automatically choose an icon.
        :type auto_icon: Optional[bool]
        :param image_path: Path to an image to include in the notification.
        :type image_path: Optional[str]
        :param sound_name: Name of the sound to play with the notification.
        :type sound_name: Optional[str]
        :param subtitle: Subtitle or secondary text.
        :type subtitle: Optional[str]
        :param timeout: Timeout in milliseconds before the notification disappears.
        :type timeout: Optional[int]
        :param notification_id: ID of the notification to replace, if any.
        :type notification_id: Optional[int]
        :param action: Tuple of (action_key, label) for notification action button.
        :type action: Optional[tuple[str, str]]

        :return: Notification metadata as a dictionary, or ``None`` if unsupported.
        :rtype: Optional[dict[str, Any]]
        """
        args = [
            summary,
            body,
            app_id,
            appname,
            icon,
            auto_icon,
            image_path,
            sound_name,
            subtitle,
            timeout,
            notification_id,
            action,
        ]
        return await event_register("controlcenter.notification", args=args)
