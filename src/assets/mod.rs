// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use serialize_to_javascript::{default_template, Template};

#[derive(Deserialize, Template, Debug, Clone)]
#[default_template("pyorion_socket.js")]
pub struct WebSocketConfig {
    url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    protocols: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_reconnect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reconnect_interval: Option<u64>,
}

pub fn websocket_config(cfg: WebSocketConfig) -> anyhow::Result<String> {
    let serialized = serialize_to_javascript::DefaultTemplate::render_default(
        &cfg,
        &serialize_to_javascript::Options::default(),
    )?;
    Ok(serialized.into_string())
}

pub static _COMMAND_SCRIPT: &str = include_str!("./invoke.js");
