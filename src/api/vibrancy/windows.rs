// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use pyorion_options::window::{WindowEffect as Effect, WindowEffectsConfig};
use raw_window_handle::HasWindowHandle;
use window_vibrancy;

#[allow(dead_code)]
/// Effekte auf ein Fenster anwenden
pub fn apply_effects(window: impl HasWindowHandle, effects: WindowEffectsConfig) {
    let WindowEffectsConfig { effects, color, .. } = effects;

    // nur den ersten relevanten Effekt anwenden
    let effect = if let Some(effect) = effects.iter().find(|e| {
        matches!(
            e,
            Effect::Mica
                | Effect::MicaDark
                | Effect::MicaLight
                | Effect::Acrylic
                | Effect::Blur
                | Effect::Tabbed
                | Effect::TabbedDark
                | Effect::TabbedLight
        )
    }) {
        effect
    } else {
        return;
    };

    match effect {
        Effect::Blur => {
            let _ = window_vibrancy::apply_blur(window, color.map(Into::into));
        }
        Effect::Acrylic => {
            let _ = window_vibrancy::apply_acrylic(window, color.map(Into::into));
        }
        Effect::Mica => {
            let _ = window_vibrancy::apply_mica(window, None);
        }
        Effect::MicaDark => {
            let _ = window_vibrancy::apply_mica(window, Some(true));
        }
        Effect::MicaLight => {
            let _ = window_vibrancy::apply_mica(window, Some(false));
        }
        Effect::Tabbed => {
            let _ = window_vibrancy::apply_tabbed(window, None);
        }
        Effect::TabbedDark => {
            let _ = window_vibrancy::apply_tabbed(window, Some(true));
        }
        Effect::TabbedLight => {
            let _ = window_vibrancy::apply_tabbed(window, Some(false));
        }
        _ => unreachable!(),
    };
}

/// Alle Effekte zurücksetzen
pub fn clear_effects(window: impl HasWindowHandle) {
    let _ = window_vibrancy::clear_blur(&window);
    let _ = window_vibrancy::clear_acrylic(&window);
    let _ = window_vibrancy::clear_mica(&window);
}
