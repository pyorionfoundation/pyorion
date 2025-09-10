// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(deprecated)]
use pyorion_options::window::{WindowEffect, WindowEffectState, WindowEffectsConfig};
use raw_window_handle::HasWindowHandle;
use window_vibrancy::{NSVisualEffectMaterial, NSVisualEffectState};

pub fn apply_effects(window: impl HasWindowHandle, effects: WindowEffectsConfig) {
    let WindowEffectsConfig {
        effects,
        radius,
        state,
        ..
    } = effects;
    let effect = if let Some(effect) = effects.into_iter().find(|e| {
        matches!(e, |WindowEffect::Titlebar| WindowEffect::Selection
            | WindowEffect::Menu
            | WindowEffect::Popover
            | WindowEffect::Sidebar
            | WindowEffect::HeaderView
            | WindowEffect::Sheet
            | WindowEffect::WindowBackground
            | WindowEffect::HudWindow
            | WindowEffect::FullScreenUI
            | WindowEffect::Tooltip
            | WindowEffect::ContentBackground
            | WindowEffect::UnderWindowBackground
            | WindowEffect::UnderPageBackground)
    }) {
        effect
    } else {
        return;
    };

    let _ = window_vibrancy::apply_vibrancy(
        window,
        match effect {
            WindowEffect::Titlebar => NSVisualEffectMaterial::Titlebar,
            WindowEffect::Selection => NSVisualEffectMaterial::Selection,
            WindowEffect::Menu => NSVisualEffectMaterial::Menu,
            WindowEffect::Popover => NSVisualEffectMaterial::Popover,
            WindowEffect::Sidebar => NSVisualEffectMaterial::Sidebar,
            WindowEffect::HeaderView => NSVisualEffectMaterial::HeaderView,
            WindowEffect::Sheet => NSVisualEffectMaterial::Sheet,
            WindowEffect::WindowBackground => NSVisualEffectMaterial::WindowBackground,
            WindowEffect::HudWindow => NSVisualEffectMaterial::HudWindow,
            WindowEffect::FullScreenUI => NSVisualEffectMaterial::FullScreenUI,
            WindowEffect::Tooltip => NSVisualEffectMaterial::Tooltip,
            WindowEffect::ContentBackground => NSVisualEffectMaterial::ContentBackground,
            WindowEffect::UnderWindowBackground => NSVisualEffectMaterial::UnderWindowBackground,
            WindowEffect::UnderPageBackground => NSVisualEffectMaterial::UnderPageBackground,
            _ => unreachable!(),
        },
        state.map(|s| match s {
            WindowEffectState::FollowsWindowActiveState => {
                NSVisualEffectState::FollowsWindowActiveState
            }
            WindowEffectState::Active => NSVisualEffectState::Active,
            WindowEffectState::Inactive => NSVisualEffectState::Inactive,
        }),
        radius,
    );
}
