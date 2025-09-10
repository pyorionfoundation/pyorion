use anyhow::{anyhow, Result};
use pyo3::{Py, PyAny};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tao::{
    event_loop::ControlFlow,
    window::{Window, WindowId},
};

use crate::utils::{arc_mut, ArcMut};

#[derive(Clone)]
pub struct AppContext {
    first_id: Option<WindowId>,
    pub window: Arc<Mutex<HashMap<WindowId, (Arc<Window>, Arc<wry::WebView>)>>>,
}

impl AppContext {
    pub fn new() -> Result<ArcMut<Self>> {
        Ok(arc_mut(Self {
            first_id: None,
            window: Arc::new(Mutex::new(HashMap::new())),
        }))
    }

    pub fn _window_id(&self) -> Result<WindowId> {
        let id = self.first_id.clone().ok_or(anyhow!("No window ID set"))?;
        Ok(id)
    }

    pub fn close_window(
        &mut self,
        mp_event: Py<PyAny>,
        flow: &mut tao::event_loop::ControlFlow,
    ) -> Result<()> {
        if let Some(id) = self.first_id.take() {
            let mut guard = self
                .window
                .lock()
                .map_err(|e| anyhow!("Mutex poison error: {}", e))?;

            if let Some((_window, _webview)) = guard.remove(&id) {
                if guard.is_empty() {
                    // last window -> trigger Python event and end loop
                    pyo3::Python::with_gil(|py| {
                        if let Err(e) = mp_event.call_method0(py, "set") {
                            e.print(py);
                        }
                        py.check_signals().unwrap();
                    });
                    *flow = ControlFlow::Exit;
                }
                Ok(())
            } else {
                Err(anyhow!("Window with id {:?} not found", id))
            }
        } else {
            Err(anyhow!("No window ID set"))
        }
    }

    // Method for adding a window and WebViews
    pub fn add_window(&mut self, id: WindowId, window: Arc<Window>, webview: Arc<wry::WebView>) {
        let mut guard = self
            .window
            .lock()
            .map_err(|e| anyhow!("Mutex poison error: {}", e))
            .unwrap();
        guard.insert(id, (window, webview));
        if self.first_id.is_none() {
            self.first_id = Some(id);
        }
    }

    // Returns the first window
    pub fn get_window(&self) -> Result<Arc<Window>> {
        if let Some(id) = self.first_id {
            let guard = self
                .window
                .lock()
                .map_err(|e| anyhow!("Mutex poison error: {}", e))?;
            guard
                .get(&id)
                .map(|(window, _)| Arc::clone(window))
                .ok_or_else(|| anyhow!("Window with id {:?} not found", id))
        } else {
            Err(anyhow!("No window ID set"))
        }
    }

    // Returns the WebView for the first window
    #[allow(dead_code)]
    pub fn get_webview(&self) -> Result<Arc<wry::WebView>> {
        if let Some(id) = self.first_id {
            let guard = self
                .window
                .lock()
                .map_err(|e| anyhow!("Mutex poison error: {}", e))?;
            guard
                .get(&id)
                .map(|(_, webview)| Arc::clone(webview))
                .ok_or_else(|| anyhow!("WebView with id {:?} not found", id))
        } else {
            Err(anyhow!("No WebView ID set"))
        }
    }
}

impl std::fmt::Debug for AppContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let window_guard = self.window.lock();

        match window_guard {
            Ok(guard) => f
                .debug_struct("AppContext")
                .field("first_id", &self.first_id)
                .field("window_count", &guard.len())
                .field("window_ids", &guard.keys().collect::<Vec<_>>())
                .finish(),
            Err(_) => f
                .debug_struct("AppContext")
                .field("first_id", &self.first_id)
                .field("error", &"Mutex is poisoned")
                .finish(),
        }
    }
}
