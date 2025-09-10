// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use pyorion_options::window::{WebViewOptions, WindowOptions};
use tao::{
    dpi::{Position, Size},
    window::{Window, WindowBuilder},
};

use crate::utils::{render_protocol, FrameWindowTarget};

#[allow(dead_code)]
pub struct FrameBuilder;

impl FrameBuilder {
    #[allow(dead_code)]
    pub fn build_window(
        target: &FrameWindowTarget,
        options: &WindowOptions,
    ) -> anyhow::Result<Window> {
        let mut builder = WindowBuilder::new();

        if let Some(v) = options.always_on_bottom {
            builder = builder.with_always_on_bottom(v);
        }
        if let Some(v) = options.always_on_top {
            builder = builder.with_always_on_top(v);
        }
        if let Some(v) = options.background_color {
            let [r, g, b, a] = [v.0, v.1, v.2, v.3];
            builder = builder.with_background_color((r, g, b, a));
        }
        if let Some(v) = options.closable {
            builder = builder.with_closable(v);
        }
        if let Some(v) = options.content_protection {
            builder = builder.with_content_protection(v);
        }
        if let Some(v) = options.decorations {
            builder = builder.with_decorations(v);
        }
        if let Some(v) = options.focusable {
            builder = builder.with_focusable(v);
        }
        if let Some(v) = options.focused {
            builder = builder.with_focused(v);
        }
        if let Some(v) = options.fullscreen {
            if v {
                builder = builder.with_fullscreen(Some(tao::window::Fullscreen::Borderless(None)));
            } else {
                builder = builder.with_fullscreen(None);
            }
        }
        if let Some(v) = options.clone().inner_size {
            builder = builder.with_inner_size::<Size>(v.into());
        }
        if let Some(v) = options.clone().max_inner_size {
            builder = builder.with_max_inner_size::<Size>(v.into());
        }
        if let Some(v) = options.maximizable {
            builder = builder.with_maximizable(v);
        }
        if let Some(v) = options.maximized {
            builder = builder.with_maximized(v);
        }
        if let Some(v) = options.clone().min_inner_size {
            builder = builder.with_min_inner_size::<Size>(v.into());
        }
        if let Some(v) = options.minimizable {
            builder = builder.with_minimizable(v);
        }
        if let Some(v) = options.clone().position {
            builder = builder.with_position(Position::from(v));
        }
        if let Some(v) = options.resizable {
            builder = builder.with_resizable(v);
        }
        if let Some(v) = &options.theme {
            builder = builder.with_theme(Some((*v).clone().into()));
        }
        if let Some(v) = &options.title {
            builder = builder.with_title(v);
        }
        if let Some(v) = options.transparent {
            builder = builder.with_transparent(v);
        }
        if let Some(v) = options.visible {
            builder = builder.with_visible(v);
        }
        if let Some(v) = options.visible_on_all_workspaces {
            builder = builder.with_visible_on_all_workspaces(v);
        }
        if let Some(v) = &options.window_icon {
            builder = builder.with_window_icon(Some(v.to_icon()?));
        }

        let window = builder.build(target)?;
        Ok(window)
    }
    #[allow(dead_code)]
    pub fn build_webview(
        window: &tao::window::Window,
        options: &WebViewOptions,
        sock_cfg: Option<crate::assets::WebSocketConfig>,
    ) -> anyhow::Result<wry::WebView> {
        // websocket_config
        let mut builder = wry::WebViewBuilder::new();

        if let Some(conf) = sock_cfg {
            let socket_conf = crate::assets::websocket_config(conf)?;
            builder = builder
                .with_initialization_script(socket_conf)
                .with_initialization_script(crate::assets::_COMMAND_SCRIPT);
        }
        if let Some(label) = &options.label {
            builder = builder.with_id(label.as_str());
        } else {
            builder = builder.with_id("root_webview");
        }
        let binding = &options.render_protocol;
        let mut builder = render_protocol(builder, binding.clone());
        if let Some(v) = options.transparent {
            builder = builder.with_transparent(v);
        }
        if let Some(v) = options.visible {
            builder = builder.with_visible(v);
        }
        if let Some(v) = options.devtools {
            builder = builder.with_devtools(v);
        }
        if let Some(v) = options.incognito {
            builder = builder.with_incognito(v);
        }
        if let Some(v) = &options.user_agent {
            builder = builder.with_user_agent(v);
        }
        if let Some(v) = &options.initialization_script {
            builder = builder.with_initialization_script(v);
        }
        if let Some(v) = options.accept_first_mouse {
            builder = builder.with_accept_first_mouse(v);
        }
        if let Some(v) = options.autoplay {
            builder = builder.with_autoplay(v);
        }
        if let Some(v) = options.focused {
            builder = builder.with_focused(v);
        }
        if let Some(v) = options.clipboard {
            builder = builder.with_clipboard(v);
        }
        if let Some(v) = options.hotkeys_zoom {
            builder = builder.with_hotkeys_zoom(v);
        }
        if let Some(v) = options.background_color {
            let (r, g, b, a) = v;
            builder = builder.with_background_color((r, g, b, a));
        }
        if let Some(bounds) = options.clone().bounds {
            builder = builder.with_bounds(bounds.into());
        }
        if let Some(map) = &options.headers {
            let mut headers = wry::http::HeaderMap::new();
            for (k, v) in map {
                headers.insert(
                    wry::http::HeaderName::from_bytes(k.as_bytes())?,
                    wry::http::HeaderValue::from_str(v)?,
                );
            }
            builder = builder.with_headers(headers);
        }

        // macOS specific
        if let Some(v) = options.back_forward_navigation_gestures {
            builder = builder.with_back_forward_navigation_gestures(v);
        }

        // Windows specific
        if let Some(v) = options.transparent {
            builder = builder.with_transparent(v);
        }

        let webview = builder.build(&window)?;
        Ok(webview)
    }
}
