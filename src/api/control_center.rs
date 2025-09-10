// Copyright 2025-2030 PyFrame Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
use anyhow::Result;
use notify_rust::Notification;
use pyorion_macros::api;

pub fn control_center_api(_api_manager: &mut ApiManager) {
    _api_manager.register_api("controlcenter.notification", notification);
}

#[api]
fn notification(
    summary: String,
    body: Option<String>,
    app_id: Option<String>,
    appname: Option<String>,
    icon: Option<String>,
    auto_icon: Option<bool>,
    image_path: Option<String>,
    sound_name: Option<String>,
    subtitle: Option<String>,
    timeout: Option<i32>,
    id: Option<u32>,
    action: Option<(String, String)>,
) -> Result<()> {
    let mut binding = Notification::new();
    let notify = binding.summary(&summary);

    if let Some(body) = body {
        notify.body(&body);
    }

    #[cfg(target_os = "windows")]
    if let Some(app_id) = app_id {
        notify.app_id(&app_id);
    }

    #[cfg(not(target_os = "windows"))]
    let _ = app_id;

    if let Some(appname) = appname {
        notify.appname(&appname);
    }

    #[cfg(target_os = "windows")]
    if let Some(path) = image_path {
        notify.image_path(&path);
    }

    #[cfg(not(target_os = "windows"))]
    let _ = image_path;

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    if let Some(sound) = sound_name {
        notify.sound_name(&sound);
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let _ = sound_name;

    if let Some(subtitle) = subtitle {
        notify.subtitle(&subtitle);
    }

    if let Some(id) = &id {
        notify.id(*id);
    }

    if let Some(true) = auto_icon {
        notify.auto_icon();
    }

    if let Some(icon) = icon {
        notify.icon(&icon);
    }

    if let Some(timeout) = timeout {
        notify.timeout(timeout);
    }

    if let Some((identifier, label)) = &action {
        notify.action(identifier, label);
    }

    notify.show()?;
    Ok(())
}
