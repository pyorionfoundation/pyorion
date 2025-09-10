// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use pyorion_options::window::WindowEffectsConfig;
use tao::window::Window;

#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(windows)]
pub(crate) mod windows;

pub fn set_window_effects(
    window: &Window,
    effects: Option<WindowEffectsConfig>,
) -> crate::Result<()> {
    if let Some(_effects) = effects {
        #[cfg(windows)]
        windows::apply_effects(window, _effects);
        #[cfg(target_os = "macos")]
        macos::apply_effects(window, _effects);
    } else {
        #[cfg(windows)]
        windows::clear_effects(window);
    }
    Ok(())
}
