// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use pyorion_macros::api;

use crate::api_manager::ApiManager;
use serde::Deserialize;
use serde_json::{json, Value};
use tao::window::Window;

pub fn dialog_api(_api_manager: &mut ApiManager) {
    _api_manager.register_api("dialog.showMessage", show_message);
    _api_manager.register_api("dialog.pickFile", pick_file);
    _api_manager.register_api("dialog.pickFiles", pick_files);
    _api_manager.register_api("dialog.pickDir", pick_dir);
    _api_manager.register_api("dialog.pickDirs", pick_dirs);
    _api_manager.register_api("dialog.saveFile", save_file);
}

#[derive(Deserialize)]
enum MessageLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

#[api]
fn show_message(title: String, content: Option<String>, level: Option<MessageLevel>) -> Result<()> {
    let parent = app.app_context()?.get_window()?;
    let content = content.unwrap_or_default();
    let level = level.unwrap_or(MessageLevel::Info);

    rfd::MessageDialog::new()
        .set_title(&title)
        .set_description(&content)
        .set_parent(&parent)
        .set_level(match level {
            MessageLevel::Info => rfd::MessageLevel::Info,
            MessageLevel::Warning => rfd::MessageLevel::Warning,
            MessageLevel::Error => rfd::MessageLevel::Error,
        })
        .show();

    Ok(())
}

fn _create_dialog(
    parent: &Window,
    filters: Option<Vec<String>>,
    start_dir: Option<String>,
) -> rfd::FileDialog {
    let mut dialog = rfd::FileDialog::new();
    if let Some(extensions) = filters {
        if !extensions.is_empty() {
            let extensions = extensions.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            dialog = dialog.add_filter("pick", &extensions);
        }
    }
    if let Some(dir) = start_dir {
        dialog = dialog.set_directory(dir);
    }
    dialog.set_parent(parent)
}

// In allen relevanten Funktionen (ohne & vor parent):

#[api]
fn pick_file(filters: Option<Vec<String>>, start_dir: Option<String>) -> Result<Value> {
    let parent = app.app_context()?.get_window()?;
    let dialog = _create_dialog(&parent, filters, start_dir);

    match dialog.pick_file() {
        Some(file) => Ok(json!(file)),
        None => Ok(json!(null)),
    }
}

#[api]
fn pick_files(filters: Option<Vec<String>>, start_dir: Option<String>) -> Result<Value> {
    let parent = app.app_context()?.get_window()?;
    let dialog = _create_dialog(&parent, filters, start_dir);

    match dialog.pick_files() {
        Some(files) => Ok(json!(files)),
        None => Ok(json!(null)),
    }
}

#[api]
fn pick_dir(start_dir: Option<String>) -> Result<Value> {
    let parent = app.app_context()?.get_window()?;
    let dialog = _create_dialog(&parent, None, start_dir);

    match dialog.pick_folder() {
        Some(dir) => Ok(json!(dir)),
        None => Ok(json!(null)),
    }
}

#[api]
fn pick_dirs(start_dir: Option<String>) -> Result<Value> {
    let parent = app.app_context()?.get_window()?;
    let dialog = _create_dialog(&parent, None, start_dir);

    match dialog.pick_folders() {
        Some(dirs) => Ok(json!(dirs)),
        None => Ok(json!(null)),
    }
}

#[api]
fn save_file(filters: Option<Vec<String>>, start_dir: Option<String>) -> Result<Value> {
    let parent = app.app_context()?.get_window()?;
    let dialog = _create_dialog(&parent, filters, start_dir);

    match dialog.save_file() {
        Some(file) => Ok(json!(file)),
        None => Ok(json!(null)),
    }
}
