// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use pyorion_options::window::WindowOptions;
use tao::window::{Window, WindowId};
use wry::WebView;

use crate::{utils::FrameWindowTarget, window::builder::FrameBuilder};

pub(crate) mod builder;

pub fn create_frame(
    target: &FrameWindowTarget,
    options: &WindowOptions,
    sock_cfg: Option<crate::assets::WebSocketConfig>,
) -> anyhow::Result<(WindowId, Window, WebView)> {
    let window = FrameBuilder::build_window(target, options)?;
    let id = window.id();
    let webview = FrameBuilder::build_webview(&window, &options.webview, sock_cfg)?;
    Ok((id, window, webview))
}
