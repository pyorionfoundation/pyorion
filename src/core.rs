// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;
use pyo3::Python;
use pyorion_options::window::WindowOptions;
use std::sync::Arc;

use crate::{
    api_manager::{ApiManager, ApiResponse},
    context::AppContext,
    lock,
    utils::{ArcMut, FrameEventLoop, FrameEventLoopProxy, PendingMap, UserEvent},
};

#[allow(dead_code)]
pub struct App {
    api_manager: Arc<std::sync::Mutex<ApiManager>>,
    pub rt: std::sync::Arc<tokio::runtime::Runtime>,
    pub runtime_handel: std::sync::Arc<tokio::runtime::Handle>,
    pub proxy: FrameEventLoopProxy,
    response_map: PendingMap,
    pub ctx: ArcMut<AppContext>,
}

impl App {
    pub fn new(
        event_loop: &mut FrameEventLoop,
        sock_cfg: Option<crate::assets::WebSocketConfig>,
        options: &WindowOptions,
        uds_name: String,
    ) -> Result<std::sync::Arc<App>> {
        let proxy = event_loop.create_proxy();

        let rt = std::sync::Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?,
        );

        let (window_id, window, webview) =
            crate::window::create_frame(&event_loop, options, sock_cfg)?;

        let ctx = AppContext::new()?;

        {
            let mut ctx_lock = lock!(ctx)?;
            ctx_lock.add_window(window_id, Arc::new(window), Arc::new(webview));
        }

        let handle = rt.handle().clone();

        let cloned_proxy = proxy.clone();

        let api_manager = ApiManager::new();
        {
            let mut api_manager = lock!(api_manager)?;
            crate::api::register_api_instances(&mut api_manager);
        }

        let app = Arc::new(Self {
            api_manager: api_manager.clone(),
            rt: rt.clone(),
            runtime_handel: std::sync::Arc::new(handle),
            proxy,
            response_map: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
            ctx: ctx.clone(),
        });

        {
            let mut m = lock!(api_manager).unwrap();
            m.bind_app_context(&app);
        }
        let map = app.clone().response_map.clone();

        rt.spawn(crate::connections::start_connection(
            cloned_proxy.clone(),
            map,
            uds_name.to_string(),
        ));
        Ok(app)
    }

    #[allow(dead_code)]
    pub fn api_manager(&self) -> Result<std::sync::MutexGuard<'_, ApiManager>> {
        lock!(self.api_manager)
    }
    #[allow(dead_code)]
    pub fn app_context(&self) -> Result<std::sync::MutexGuard<'_, AppContext>> {
        lock!(self.ctx)
    }
    #[allow(dead_code)]
    pub fn respond(&self, key: u8, response: ApiResponse) {
        if let Some(sender) = self.response_map.lock().unwrap().remove(&key) {
            let _ = sender.send(response);
        } else {
            eprintln!("No transmitter found for key {}", key);
        }
    }

    pub fn run(
        self: Arc<Self>,
        event_loop: FrameEventLoop,
        _mp_event: pyo3::Py<pyo3::PyAny>,
    ) -> Result<()> {
        let api_manager = self.api_manager.clone();
        let ctx = self.ctx.clone();
        let this = self.clone();

        event_loop.run(move |event, target, control_flow| {
            *control_flow = tao::event_loop::ControlFlow::Wait;

            match event {
                tao::event::Event::WindowEvent { event, .. } => match event {
                    tao::event::WindowEvent::CloseRequested => {
                        let mp_event = Python::with_gil(|py| _mp_event.clone_ref(py));
                        let _ = ctx.lock().unwrap().close_window(mp_event, control_flow);
                    }
                    _ => {}
                },
                tao::event::Event::UserEvent(event) => match event {
                    UserEvent::Request(req) => {
                        let mut manager = api_manager.lock().unwrap();
                        match manager.call(req, target, control_flow) {
                            Ok(res) => this.respond(res.0, res),
                            Err(err) => {
                                eprintln!("API call failed: {:?}", err);
                                // evtl. ein ApiResponse mit Fehler zurückschicken
                            }
                        };
                    }
                    UserEvent::Shutdown => {
                        let mp_event = Python::with_gil(|py| _mp_event.clone_ref(py));
                        let _ = ctx.lock().unwrap().close_window(mp_event, control_flow);
                    }
                },
                _ => {}
            }
        });
    }
}
