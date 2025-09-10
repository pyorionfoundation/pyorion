// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use pyorion_macros::api;
use pyorion_options::window::WindowEffectsConfig;

use crate::{api::vibrancy::set_window_effects as effect, api_manager::ApiManager};

/*
set_window_effects




*/
/// Modifies the window's visibility.
///
/// If false, this will hide the window. If true, this will show the window.
/// ## Platform-specific
/// Android: Unsupported.
/// - iOS: Can only be called on the main thread.
///
#[api]
fn set_window_effects(effects: WindowEffectsConfig) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        let _ = effect(&window, Some(effects));
        Ok(true)
    } else {
        Ok(false)
    }
}

#[api]
fn set_visible(visible: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_visible(visible);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets the title of the window.
///
/// Wrapper for [`tao::window::Window::set_title`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_title(title: String) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_title(&title);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Gets the current title of the window.
///
/// Wrapper for [`tao::window::Window::title`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns empty string.
#[api]
fn get_title() -> Result<String> {
    let window = app.app_context()?.get_window()?;
    Ok(window.title())
}

/// Returns the scale factor.
///
/// Wrapper for [`tao::window::Window::scale_factor`].
///
/// ## Platform-specific
/// - Android: Always returns `1.0`.
/// - iOS: Must be called on main thread.
#[api]
fn scale_factor() -> Result<f64> {
    let window = app.app_context()?.get_window()?;
    Ok(window.scale_factor())
}

/// Sets whether the window is always kept on bottom.
///
/// Wrapper for [`tao::window::Window::set_always_on_bottom`].
///
/// ## Platform-specific
/// - Windows: No guarantee but will try.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_always_on_bottom(always_on_bottom: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_always_on_bottom(always_on_bottom);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets whether the window is always kept on top.
///
/// Wrapper for [`tao::window::Window::set_always_on_top`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_always_on_top(always_on_top: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_always_on_top(always_on_top);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets the background color of the window.
///
/// Wrapper for [`tao::window::Window::set_background_color`].
///
/// ## Platform-specific
/// - Windows: Alpha ignored.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_background_color(color: Option<wry::RGBA>) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_background_color(color);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets whether the window is closable.
///
/// Wrapper for [`tao::window::Window::set_closable`].
///
/// ## Platform-specific
/// - Linux: May not affect visible windows.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_closable(closable: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_closable(closable);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Prevents window content capture.
///
/// Wrapper for [`tao::window::Window::set_content_protection`].
///
/// ## Platform-specific
/// - iOS / Android / Linux: Unsupported → returns `false`.
#[api]
fn set_content_protection(enabled: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_content_protection(enabled);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Grabs the cursor inside the window.
///
/// Wrapper for [`tao::window::Window::set_cursor_grab`].
///
/// ## Platform-specific
/// - macOS: Locks cursor visually.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_cursor_grab(grab: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        Ok(window.set_cursor_grab(grab).is_ok())
    } else {
        Ok(false)
    }
}

/// Sets the cursor icon.
///
/// Wrapper for [`tao::window::Window::set_cursor_icon`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_cursor_icon(cursor: pyorion_options::window::CursorIcon) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_cursor_icon(cursor.into());
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets the cursor position in window coordinates.
///
/// Wrapper for [`tao::window::Window::set_cursor_position`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_cursor_position(position: pyorion_options::window::Position) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        Ok(window.set_cursor_position(position).is_ok())
    } else {
        Ok(false)
    }
}

/// Sets cursor visibility.
///
/// Wrapper for [`tao::window::Window::set_cursor_visible`].
///
/// ## Platform-specific
/// - Windows: Hidden only inside window.
/// - macOS: Hidden while window focused.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_cursor_visible(visible: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_cursor_visible(visible);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets window decorations.
///
/// Wrapper for [`tao::window::Window::set_decorations`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_decorations(decorations: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_decorations(decorations);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Focuses the window.
///
/// Wrapper for [`tao::window::Window::set_focus`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_focus() -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_focus();
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets focusable state.
///
/// Wrapper for [`tao::window::Window::set_focusable`].
///
/// ## Platform-specific
/// - macOS: Cannot unfocus if already focused.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_focusable(focusable: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_focusable(focusable);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Returns list of available monitors.
///
/// Wrapper for [`tao::window::Window::available_monitors`].
///
/// ## Platform-specific
/// - iOS: Main thread only.
#[api]
fn get_available_monitors() -> Result<Vec<pyorion_options::window::Monitor>> {
    let window = app.app_context()?.get_window()?;
    let d = window
        .available_monitors()
        .map(|m| pyorion_options::window::Monitor {
            name: m.name(),
            scale_factor: m.scale_factor(),
            size: pyorion_options::window::Dimensions {
                width: m.size().width,
                height: m.size().height,
            },
            position: pyorion_options::window::MonitorPosition {
                x: m.position().x,
                y: m.position().y,
            },
            video_modes: m
                .video_modes()
                .map(|v| pyorion_options::window::MonitorVideoMode {
                    size: pyorion_options::window::Dimensions {
                        width: v.size().width,
                        height: v.size().height,
                    },
                    bit_depth: v.bit_depth(),
                    refresh_rate: v.refresh_rate(),
                })
                .collect(),
        })
        .collect();
    Ok(d)
}

/// Toggles fullscreen.
///
/// Wrapper for [`tao::window::Window::set_fullscreen`].
///
/// ## Platform-specific
/// - macOS: Exclusive or Borderless.
/// - iOS: Main thread only.
/// - Windows: Disables screensaver.
/// - Linux: Fullscreen current monitor.
/// - Android: Unsupported → returns `false`.
#[api]
fn set_fullscreen(fullscreen: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        if fullscreen {
            window.set_fullscreen(Some(tao::window::Fullscreen::Borderless(None)));
        } else {
            window.set_fullscreen(None);
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Ignores or catches cursor events.
///
/// Wrapper for [`tao::window::Window::set_ignore_cursor_events`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_ignore_cursor_events(ignore: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        Ok(window.set_ignore_cursor_events(ignore).is_ok())
    } else {
        Ok(false)
    }
}

/// Sets IME candidate box position.
///
/// Wrapper for [`tao::window::Window::set_ime_position`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_ime_position(position: pyorion_options::window::Position) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_ime_position(position);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets progress bar state.
///
/// Wrapper for [`tao::window::Window::set_progress_bar`].
///
/// ## Platform-specific
/// - Linux / macOS: App-wide progress bar.
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_progress_bar(progress: pyorion_options::window::ProgressBarState) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_progress_bar(progress.into());
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets inner size.
///
/// Wrapper for [`tao::window::Window::set_inner_size`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_inner_size(size: pyorion_options::window::Size) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_inner_size(size);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets inner size constraints.
///
/// Wrapper for [`tao::window::Window::set_inner_size_constraints`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_inner_size_constraints(
    constraints: pyorion_options::window::WindowSizeConstraints,
) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_inner_size_constraints(constraints.into());
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets max inner size.
///
/// Wrapper for [`tao::window::Window::set_max_inner_size`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_max_inner_size(max_size: pyorion_options::window::Size) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_max_inner_size(Some(max_size));
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets maximizable flag.
///
/// Wrapper for [`tao::window::Window::set_maximizable`].
///
/// ## Platform-specific
/// - macOS: Disables zoom button.
/// - Linux / iOS / Android: Unsupported → returns `false`.
#[api]
fn set_maximizable(maximizable: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_maximizable(maximizable);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Minimizes or restores window.
///
/// Wrapper for [`tao::window::Window::set_minimized`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_minimized(minimized: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_minimized(minimized);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets minimum inner size.
///
/// Wrapper for [`tao::window::Window::set_min_inner_size`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_min_inner_size(min_size: pyorion_options::window::Size) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_min_inner_size(Some(min_size));
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets minimizable flag.
///
/// Wrapper for [`tao::window::Window::set_minimizable`].
///
/// ## Platform-specific
/// - Linux / iOS / Android: Unsupported → returns `false`.
#[api]
fn set_minimizable(minimizable: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_minimizable(minimizable);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets outer position.
///
/// Wrapper for [`tao::window::Window::set_outer_position`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_outer_position(position: pyorion_options::window::Position) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_outer_position(position);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets window theme.
///
/// Wrapper for [`tao::window::Window::set_theme`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_theme(theme: pyorion_options::window::Theme) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        let main_theme = match theme {
            pyorion_options::window::Theme::Light => tao::window::Theme::Light,
            pyorion_options::window::Theme::Dark => tao::window::Theme::Dark,
        };
        window.set_theme(Some(main_theme));
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Sets whether visible on all workspaces.
///
/// Wrapper for [`tao::window::Window::set_visible_on_all_workspaces`].
///
/// ## Platform-specific
/// - iOS / Android: Unsupported → returns `false`.
#[api]
fn set_visible_on_all_workspaces(visible: bool) -> Result<bool> {
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_visible_on_all_workspaces(visible);
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(target_os = "windows")]
#[api]
fn set_enable(enable: bool) -> Result<bool> {
    use tao::platform::windows::WindowExtWindows;
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_enable(enable);
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(not(target_os = "windows"))]
#[api]
fn set_enable(_enable: bool) -> Result<bool> {
    Ok(false)
}

#[cfg(target_os = "windows")]
#[api]
fn set_rtl(rtl: bool) -> Result<bool> {
    use tao::platform::windows::WindowExtWindows;
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_rtl(rtl);
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(not(target_os = "windows"))]
#[api]
fn set_rtl(_rtl: bool) -> Result<bool> {
    Ok(false)
}

#[cfg(target_os = "windows")]
#[api]
fn set_undecorated_shadow(shadow: bool) -> Result<bool> {
    use tao::platform::windows::WindowExtWindows;
    if let Ok(window) = app.app_context()?.get_window() {
        window.set_undecorated_shadow(shadow);
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(not(target_os = "windows"))]
#[api]
fn set_undecorated_shadow(_shadow: bool) -> Result<bool> {
    Ok(false)
}

/// Returns inner size.
#[api]
fn inner_size() -> Result<tao::dpi::PhysicalSize<u32>> {
    let window = app.app_context()?.get_window()?;
    Ok(window.inner_size())
}

/// Returns outer size.
#[api]
fn outer_size() -> Result<tao::dpi::PhysicalSize<u32>> {
    let window = app.app_context()?.get_window()?;
    Ok(window.outer_size())
}

/// Returns outer position.
#[api]
fn outer_position() -> Result<tao::dpi::PhysicalPosition<i32>> {
    let window = app.app_context()?.get_window()?;
    Ok(window.outer_position()?)
}

pub fn window_api(api_manager: &mut ApiManager) {
    api_manager.register_api("window.set_title", set_title);
    api_manager.register_api("window.get_title", get_title);
    api_manager.register_api("window.scale_factor", scale_factor);
    api_manager.register_api("window.set_always_on_bottom", set_always_on_bottom);
    api_manager.register_api("window.set_always_on_top", set_always_on_top);
    api_manager.register_api("window.set_background_color", set_background_color);
    api_manager.register_api("window.set_closable", set_closable);
    api_manager.register_api("window.set_content_protection", set_content_protection);
    api_manager.register_api("window.set_cursor_grab", set_cursor_grab);
    api_manager.register_api("window.set_cursor_icon", set_cursor_icon);
    api_manager.register_api("window.set_cursor_position", set_cursor_position);
    api_manager.register_api("window.set_cursor_visible", set_cursor_visible);
    api_manager.register_api("window.set_decorations", set_decorations);
    api_manager.register_api("window.set_focus", set_focus);
    api_manager.register_api("window.set_focusable", set_focusable);
    api_manager.register_api("window.get_available_monitors", get_available_monitors);
    api_manager.register_api("window.set_fullscreen", set_fullscreen);
    api_manager.register_api("window.set_ignore_cursor_events", set_ignore_cursor_events);
    api_manager.register_api("window.set_ime_position", set_ime_position);
    api_manager.register_api("window.set_progress_bar", set_progress_bar);
    api_manager.register_api("window.set_inner_size", set_inner_size);
    api_manager.register_api(
        "window.set_inner_size_constraints",
        set_inner_size_constraints,
    );
    api_manager.register_api("window.set_max_inner_size", set_max_inner_size);
    api_manager.register_api("window.set_maximizable", set_maximizable);
    api_manager.register_api("window.set_minimized", set_minimized);
    api_manager.register_api("window.set_min_inner_size", set_min_inner_size);
    api_manager.register_api("window.set_minimizable", set_minimizable);
    api_manager.register_api("window.set_outer_position", set_outer_position);
    api_manager.register_api("window.set_theme", set_theme);
    api_manager.register_api("window.set_visible", set_visible);
    api_manager.register_api(
        "window.set_visible_on_all_workspaces",
        set_visible_on_all_workspaces,
    );
    api_manager.register_api("window.set_enable", set_enable);
    api_manager.register_api("window.set_rtl", set_rtl);
    api_manager.register_api("window.set_undecorated_shadow", set_undecorated_shadow);
    api_manager.register_api("window.inner_size", inner_size);
    api_manager.register_api("window.outer_size", outer_size);
    api_manager.register_api("window.outer_position", outer_position);
    api_manager.register_api("window.set_window_effect", set_window_effects);
}
