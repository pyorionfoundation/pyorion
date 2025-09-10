// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// remove console window in windows system
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use pyo3::prelude::*;

use crate::utils::FrameEventLoopBuilder;
mod api;
mod api_manager;
mod assets;
mod connections;
mod context;
mod core;
mod utils;
mod window;

#[pyfunction]
fn create_webframe(
    config: String,
    sock_cfg: Option<String>,
    uds_name: String,
    close_event: Py<PyAny>,
) -> Result<()> {
    let options: &pyorion_options::window::WindowOptions = &serde_json::from_str(&config)?;

    let sock_cfg_json: Option<assets::WebSocketConfig> = match sock_cfg {
        Some(s) => Some(serde_json::from_str(&s)?),
        None => None,
    };

    let mut event_loop = FrameEventLoopBuilder::with_user_event().build();
    let app = core::App::new(&mut event_loop, sock_cfg_json, options, uds_name)?;
    app.run(event_loop, close_event)
}

pub fn get_pyorion_version() -> &'static str {
    // Mapping Cargo versioning (e.g., "1.0-alpha1") to Python's PEP 440 format (e.g., "1.0.0a1")
    // This conversion is a simplified compatibility adjustment and covers most common cases.

    static PYFRAME_VERSION: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    PYFRAME_VERSION.get_or_init(|| {
        env!("CARGO_PKG_VERSION")
            .replace("-alpha", "a")
            .replace("-beta", "b")
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn _pyorion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", get_pyorion_version())?;
    m.add_function(wrap_pyfunction!(create_webframe, m)?)?;
    m.add_function(wrap_pyfunction!(
        crate::connections::send_event_over_platform,
        m
    )?)?;
    Ok(())
}
