// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
use anyhow::Result;
use pyorion_macros::api;
use serde_json::json;

pub fn dirs_api(api_manager: &mut ApiManager) {
    api_manager.register_api("dirs.homeDir", home_dir);
    api_manager.register_api("dirs.cacheDir", cache_dir);
    api_manager.register_api("dirs.configDir", config_dir);
    api_manager.register_api("dirs.configLocalDir", config_local_dir);
    api_manager.register_api("dirs.dataDir", data_dir);
    api_manager.register_api("dirs.dataLocalDir", data_local_dir);
    api_manager.register_api("dirs.desktopDir", desktop_dir);
    api_manager.register_api("dirs.documentDir", document_dir);
    api_manager.register_api("dirs.downloadDir", download_dir);
    api_manager.register_api("dirs.executableDir", executable_dir);
    api_manager.register_api("dirs.fontDir", font_dir);
    api_manager.register_api("dirs.pictureDir", picture_dir);
    api_manager.register_api("dirs.preferenceDir", preference_dir);
    api_manager.register_api("dirs.publicDir", public_dir);
    api_manager.register_api("dirs.runtimeDir", runtime_dir);
    api_manager.register_api("dirs.stateDir", state_dir);
    api_manager.register_api("dirs.templateDir", template_dir);
    api_manager.register_api("dirs.videoDir", video_dir);
    api_manager.register_api("dirs.audioDir", audio_dir);
}

#[api]
fn home_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::home_dir()))
}

#[api]
fn cache_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::cache_dir()))
}
#[api]
pub fn file_name(path: String) -> Result<Option<String>> {
    Ok(std::path::Path::new(&path)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned()))
}

#[api]
fn config_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::config_dir()))
}

#[api]
fn config_local_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::config_local_dir()))
}

#[api]
fn data_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::data_dir()))
}

#[api]
fn data_local_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::data_local_dir()))
}

#[api]
fn desktop_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::desktop_dir()))
}

#[api]
fn document_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::document_dir()))
}

#[api]
fn download_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::download_dir()))
}

#[api]
fn executable_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::executable_dir()))
}

#[api]
fn font_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::font_dir()))
}

#[api]
fn picture_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::picture_dir()))
}

#[api]
fn preference_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::preference_dir()))
}

#[api]
fn public_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::public_dir()))
}

#[api]
fn runtime_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::runtime_dir()))
}

#[api]
fn state_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::state_dir()))
}

#[api]
fn template_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::template_dir()))
}

#[api]
fn video_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::video_dir()))
}

#[api]
fn audio_dir() -> Result<serde_json::Value> {
    Ok(json!(dirs::audio_dir()))
}
