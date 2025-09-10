# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Window control API for PyOrion.

Asynchronous wrapper around ``tao::window::Window`` to manage
window state, size, decorations, themes, and visual effects.
Each call is delegated to the Rust event loop backend.
"""

from pyorion.runtime.runtime_handle import event_register
from pyorion.setup.types import (
    RGBA,
    Color,
    CursorIcon,
    Dimensions,
    Monitor,
    MonitorPosition,
    Position,
    ProgressBarState,
    Size,
    Theme,
    WindowEffect,
    WindowEffectsConfig,
    WindowEffectState,
    WindowSizeConstraints,
)


class Window:
    """Asynchronous API wrapper for ``tao::window::Window``.

    Provide methods to query and control window state,
    visibility, size, decorations, and other attributes.
    Each call delegates to the Rust event loop backend.
    """

    def __init__(self):
        """Initialize a new Window instance."""

    async def set_window_effect(
        self,
        effects: list[WindowEffect],
        state: WindowEffectState | None = None,
        radius: float | None = None,
        color: Color | None = None,
    ) -> bool:
        """Apply a set of visual effects to the window.

        Wrapper for ``tao::window::Window::set_window_effect``.

        :param effects: List of visual effects to apply (Blur, Acrylic, Mica, etc.).
        :type effects: list[WindowEffect]
        :param state: Effect state (active, inactive, or follows window state).
        :type state: Optional[WindowEffectState]
        :param radius: Corner radius for effects (macOS only).
        :type radius: Optional[float]
        :param color: RGBA color used for Blur and Acrylic effects.
        :type color: Optional[Color]
        :return: ``True`` if the operation succeeded.
        :rtype: bool
        """
        config = WindowEffectsConfig(
            effects=effects,
            state=state,
            radius=radius,
            color=color,
        )
        return await event_register(
            "window.set_window_effect",
            config.model_dump(by_alias=True, exclude_none=True),
            result_type=bool,
        )

    async def set_visible(self, visible: bool) -> bool:
        """Set the window's visibility.

        Wrapper for ``tao::window::Window::set_visible``.

        :param bool visible: ``True`` to show the window, ``False`` to hide it.
        :return: ``True`` if the operation succeeded.
        :rtype: bool

        Platform-specific:
            - Android: Unsupported
            - iOS: Must be called on the main thread
        """
        return await event_register("window.set_visible", visible, result_type=bool)

    async def set_title(self, title: str) -> bool:
        """Set the title of the window.

        Wrapper for ``tao::window::Window::set_title``.

        :param str title: New window title.
        :return: ``True`` if the operation succeeded.
        :rtype: bool

        Platform-specific:
            - iOS / Android: Unsupported → always returns ``False``
        """
        return await event_register("window.set_title", title, result_type=bool)

    async def get_title(self) -> str:
        """Get the current title of the window.

        Wrapper for ``tao::window::Window::title``.

        :return: The current window title.
        :rtype: str

        Platform-specific:
            - iOS / Android: Unsupported → returns empty string
        """
        return await event_register("window.get_title", {}, result_type=str)

    async def scale_factor(self) -> float:
        """Return the display scale factor of the window.

        Wrapper for ``tao::window::Window::scale_factor``.

        :return: The scale factor value.
        :rtype: float

        Platform-specific:
            - Android: Always returns ``1.0``
            - iOS: Must be called on the main thread
        """
        return await event_register("window.scale_factor", {}, result_type=float)

    async def set_always_on_bottom(self, always: bool) -> bool:
        """Set whether the window is always kept on bottom.

        Wrapper for ``tao::window::Window::set_always_on_bottom``.
        """
        return await event_register(
            "window.set_always_on_bottom", always, result_type=bool
        )

    async def set_always_on_top(self, always: bool) -> bool:
        """Set whether the window is always kept on top.

        Wrapper for ``tao::window::Window::set_always_on_top``.
        """
        return await event_register(
            "window.set_always_on_top", always, result_type=bool
        )

    async def set_background_color(self, color: RGBA | None) -> bool:
        """Set the background color of the window.

        Wrapper for ``tao::window::Window::set_background_color``.
        """
        return await event_register(
            "window.set_background_color",
            None if color is None else color.as_tuple(),
            result_type=bool,
        )

    async def set_closable(self, closable: bool) -> bool:
        """Set whether the window is closable.

        Wrapper for ``tao::window::Window::set_closable``.
        """
        return await event_register("window.set_closable", closable, result_type=bool)

    async def set_content_protection(self, enabled: bool) -> bool:
        """Enable or disable content protection.

        Wrapper for ``tao::window::Window::set_content_protection``.
        """
        return await event_register(
            "window.set_content_protection", enabled, result_type=bool
        )

    async def set_cursor_grab(self, grab: bool) -> bool:
        """Grab the cursor inside the window.

        Wrapper for ``tao::window::Window::set_cursor_grab``.
        """
        return await event_register("window.set_cursor_grab", grab, result_type=bool)

    async def set_cursor_icon(self, cursor: CursorIcon) -> bool:
        """Set the cursor icon.

        Wrapper for ``tao::window::Window::set_cursor_icon``.
        """
        return await event_register("window.set_cursor_icon", cursor, result_type=bool)

    async def set_cursor_position(self, position: Position) -> bool:
        """Set the cursor position in window coordinates.

        Wrapper for ``tao::window::Window::set_cursor_position``.
        """
        return await event_register(
            "window.set_cursor_position",
            position.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_cursor_visible(self, visible: bool) -> bool:
        """Set cursor visibility.

        Wrapper for ``tao::window::Window::set_cursor_visible``.
        """
        return await event_register(
            "window.set_cursor_visible", visible, result_type=bool
        )

    async def set_decorations(self, decorations: bool) -> bool:
        """Set window decorations.

        Wrapper for ``tao::window::Window::set_decorations``.
        """
        return await event_register(
            "window.set_decorations", decorations, result_type=bool
        )

    async def set_focus(self) -> bool:
        """Focus the window.

        Wrapper for ``tao::window::Window::set_focus``.
        """
        return await event_register("window.set_focus", {}, result_type=bool)

    async def set_focusable(self, focusable: bool) -> bool:
        """Set focusable state.

        Wrapper for ``tao::window::Window::set_focusable``.
        """
        return await event_register("window.set_focusable", focusable, result_type=bool)

    async def get_available_monitors(self) -> list[Monitor]:
        """Return the list of available monitors.

        Wrapper for ``tao::window::Window::available_monitors``.
        """
        return await event_register(
            "window.get_available_monitors", {}, result_type=list[Monitor]
        )

    async def set_fullscreen(self, fullscreen: bool) -> bool:
        """Toggle fullscreen mode.

        Wrapper for ``tao::window::Window::set_fullscreen``.
        """
        return await event_register(
            "window.set_fullscreen", fullscreen, result_type=bool
        )

    async def set_ignore_cursor_events(self, ignore: bool) -> bool:
        """Ignore or catch cursor events.

        Wrapper for ``tao::window::Window::set_ignore_cursor_events``.
        """
        return await event_register(
            "window.set_ignore_cursor_events", ignore, result_type=bool
        )

    async def set_ime_position(self, position: Position) -> bool:
        """Set IME candidate box position.

        Wrapper for ``tao::window::Window::set_ime_position``.
        """
        return await event_register(
            "window.set_ime_position",
            position.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_progress_bar(self, progress: ProgressBarState) -> bool:
        """Set the progress bar state.

        Wrapper for ``tao::window::Window::set_progress_bar``.
        """
        return await event_register(
            "window.set_progress_bar",
            progress.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_inner_size(self, size: Size) -> bool:
        """Set inner size.

        Wrapper for ``tao::window::Window::set_inner_size``.
        """
        return await event_register(
            "window.set_inner_size", size.model_dump(by_alias=True), result_type=bool
        )

    async def set_inner_size_constraints(
        self, constraints: WindowSizeConstraints
    ) -> bool:
        """Set inner size constraints.

        Wrapper for ``tao::window::Window::set_inner_size_constraints``.
        """
        return await event_register(
            "window.set_inner_size_constraints",
            constraints.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_max_inner_size(self, size: Size) -> bool:
        """Set max inner size.

        Wrapper for ``tao::window::Window::set_max_inner_size``.
        """
        return await event_register(
            "window.set_max_inner_size",
            size.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_maximizable(self, maximizable: bool) -> bool:
        """Set maximizable flag.

        Wrapper for ``tao::window::Window::set_maximizable``.
        """
        return await event_register(
            "window.set_maximizable", maximizable, result_type=bool
        )

    async def set_minimized(self, minimized: bool) -> bool:
        """Minimize or restore window.

        Wrapper for ``tao::window::Window::set_minimized``.
        """
        return await event_register("window.set_minimized", minimized, result_type=bool)

    async def set_min_inner_size(self, size: Size) -> bool:
        """Set minimum inner size.

        Wrapper for ``tao::window::Window::set_min_inner_size``.
        """
        return await event_register(
            "window.set_min_inner_size",
            size.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_minimizable(self, minimizable: bool) -> bool:
        """Set minimizable flag.

        Wrapper for ``tao::window::Window::set_minimizable``.
        """
        return await event_register(
            "window.set_minimizable", minimizable, result_type=bool
        )

    async def set_outer_position(self, position: Position) -> bool:
        """Set outer position.

        Wrapper for ``tao::window::Window::set_outer_position``.
        """
        return await event_register(
            "window.set_outer_position",
            position.model_dump(by_alias=True),
            result_type=bool,
        )

    async def set_theme(self, theme: Theme) -> bool:
        """Set window theme.

        Wrapper for ``tao::window::Window::set_theme``.
        """
        return await event_register("window.set_theme", theme, result_type=bool)

    async def set_visible_on_all_workspaces(self, visible: bool) -> bool:
        """Set whether the window is visible on all workspaces.

        Wrapper for ``tao::window::Window::set_visible_on_all_workspaces``.
        """
        return await event_register(
            "window.set_visible_on_all_workspaces", visible, result_type=bool
        )

    async def inner_size(self) -> Dimensions:
        """Return inner size.

        Wrapper for ``tao::window::Window::inner_size``.
        """
        return await event_register("window.inner_size", {}, result_type=Dimensions)

    async def outer_size(self) -> Dimensions:
        """Return outer size.

        Wrapper for ``tao::window::Window::outer_size``.
        """
        return await event_register("window.outer_size", {}, result_type=Dimensions)

    async def outer_position(self) -> MonitorPosition:
        """Return outer position.

        Wrapper for ``tao::window::Window::outer_position``.

        :return: Outer position of the window.
        :rtype: MonitorPosition
        """
        return await event_register(
            "window.outer_position", {}, result_type=MonitorPosition
        )
