// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
mod clipboard;
mod control_center;
mod dialog;
mod dirs;
mod resource;
mod vibrancy;
mod webview;
mod window;

pub fn register_api_instances(api_manager: &mut ApiManager) {
    window::window_api(api_manager);
    webview::webview_api(api_manager);
    dialog::dialog_api(api_manager);
    control_center::control_center_api(api_manager);
    clipboard::clipboard_api(api_manager);
    dirs::dirs_api(api_manager);
    resource::resource_api(api_manager);
}
