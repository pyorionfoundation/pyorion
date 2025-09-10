# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Dirs API - Access to system standard directories.

Wrapper around the Rust ``dirs`` crate via the PyOrion event loop.
"""

from typing import Optional

from pyorion.runtime.runtime_handle import event_register


class DirsAPI:
    """Asynchronous API wrapper for system standard directories.

    Provides access to common user and system paths such as home,
    cache, config, documents, downloads, etc.
    """

    def __init__(self) -> None:
        """Initialize a new :class:`DirsAPI` instance."""

    async def home_dir(self) -> Optional[str]:
        """Get the path to the user's home directory.

        :return: Absolute path to the home directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.homeDir")

    async def cache_dir(self) -> Optional[str]:
        """Get the path to the cache directory.

        :return: Absolute path to the cache directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.cacheDir")

    async def config_dir(self) -> Optional[str]:
        """Get the path to the global configuration directory.

        :return: Absolute path to the config directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.configDir")

    async def config_local_dir(self) -> Optional[str]:
        """Get the path to the user-specific configuration directory.

        :return: Absolute path to the local config directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.configLocalDir")

    async def data_dir(self) -> Optional[str]:
        """Get the path to the global data directory.

        :return: Absolute path to the data directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.dataDir")

    async def data_local_dir(self) -> Optional[str]:
        """Get the path to the user-specific data directory.

        :return: Absolute path to the local data directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.dataLocalDir")

    async def desktop_dir(self) -> Optional[str]:
        """Get the path to the desktop directory.

        :return: Absolute path to the desktop directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.desktopDir")

    async def document_dir(self) -> Optional[str]:
        """Get the path to the documents directory.

        :return: Absolute path to the documents directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.documentDir")

    async def download_dir(self) -> Optional[str]:
        """Get the path to the downloads directory.

        :return: Absolute path to the downloads directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.downloadDir")

    async def executable_dir(self) -> Optional[str]:
        """Get the path to the executables directory.

        :return: Absolute path to the executables directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.executableDir")

    async def font_dir(self) -> Optional[str]:
        """Get the path to the system fonts directory.

        :return: Absolute path to the fonts directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.fontDir")

    async def picture_dir(self) -> Optional[str]:
        """Get the path to the pictures directory.

        :return: Absolute path to the pictures directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.pictureDir")

    async def preference_dir(self) -> Optional[str]:
        """Get the path to the preferences directory.

        :return: Absolute path to the preferences directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.preferenceDir")

    async def public_dir(self) -> Optional[str]:
        """Get the path to the public (shared) directory.

        :return: Absolute path to the public directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.publicDir")

    async def runtime_dir(self) -> Optional[str]:
        """Get the path to the runtime directory.

        :return: Absolute path to the runtime directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.runtimeDir")

    async def state_dir(self) -> Optional[str]:
        """Get the path to the state directory.

        :return: Absolute path to the state directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.stateDir")

    async def template_dir(self) -> Optional[str]:
        """Get the path to the templates directory.

        :return: Absolute path to the templates directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.templateDir")

    async def video_dir(self) -> Optional[str]:
        """Get the path to the videos directory.

        :return: Absolute path to the videos directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.videoDir")

    async def audio_dir(self) -> Optional[str]:
        """Get the path to the audio/music directory.

        :return: Absolute path to the audio directory or ``None`` if unavailable.
        :rtype: Optional[str]
        """
        return await event_register("dirs.audioDir")
