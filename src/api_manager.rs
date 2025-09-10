use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::{core::App, utils::FrameWindowTarget};

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct ApiArguments(Value);

impl ApiArguments {
    #[allow(dead_code)]
    pub fn single<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_value::<(T,)>(self.0.clone())?.0)
    }
    #[allow(dead_code)]
    pub fn get<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_value(self.0.clone())?)
    }
    #[allow(dead_code)]
    pub fn optional<T: serde::de::DeserializeOwned>(&self, args_size: usize) -> Result<T> {
        let mut args = serde_json::from_value::<Vec<serde_json::Value>>(self.0.clone())?;
        args.resize(args_size, json!(null));
        let args = json!(args);
        Ok(serde_json::from_value(args)?)
    }
}
#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct ApiRequest(pub u8, pub String, pub ApiArguments);

impl ApiRequest {
    #[allow(dead_code)]
    pub fn err<C: Into<i32>, S: Into<String>>(&self, code: C, msg: S) -> ApiResponse {
        ApiResponse(self.0, code.into(), msg.into(), json!(null))
    }
    #[allow(dead_code)]
    pub fn ok<D: Serialize>(&self, data: D) -> ApiResponse {
        ApiResponse(self.0, 0, "ok".to_string(), json!(data))
    }
    #[allow(dead_code)]
    pub fn args(&self) -> &ApiArguments {
        &self.2
    }
}
pub type Code = i32;
#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub struct ApiResponse(pub u8, pub Code, pub String, pub Value);
#[allow(dead_code)]
pub type ApiInstance = std::pin::Pin<
    Box<
        dyn Fn(
            Arc<App>,
            ApiRequest,
            &FrameWindowTarget,
            &mut tao::event_loop::ControlFlow,
        ) -> Result<ApiResponse>,
    >,
>;
#[allow(dead_code)]
pub struct ApiManager {
    ctx: Option<Weak<App>>,
    api_instance: HashMap<String, ApiInstance>,
}

impl ApiManager {
    #[allow(dead_code)]
    pub fn new() -> crate::utils::ArcMut<Self> {
        let _self = Self {
            ctx: None,
            api_instance: HashMap::new(),
        };
        crate::utils::arc_mut(_self)
    }
    #[allow(dead_code)]
    pub fn bind_app_context(&mut self, ctx: &Arc<App>) {
        self.ctx = Some(Arc::downgrade(ctx));
    }
    #[allow(dead_code)]
    pub fn register_api<S: Into<String>, T: Serialize + 'static>(
        &mut self,
        name: S,
        api_func: fn(
            Arc<App>,
            ApiRequest,
            &FrameWindowTarget,
            &mut tao::event_loop::ControlFlow,
        ) -> Result<T>,
    ) {
        let api_instance: ApiInstance = Box::pin(move |ctx: Arc<App>, request, target, flow| {
            let result = api_func(ctx, request.clone(), target, flow);
            let response = match result {
                Ok(data) => request.ok(data),
                Err(err) => request.err(-1, err.to_string()),
            };

            Ok(response)
        });

        self.api_instance.insert(name.into(), api_instance);
    }
    #[allow(dead_code)]
    pub fn call(
        &mut self,
        req: ApiRequest,
        target: &FrameWindowTarget,
        flow: &mut tao::event_loop::ControlFlow,
    ) -> anyhow::Result<ApiResponse> {
        if let Some(handler) = self.api_instance.get(&req.1) {
            if let Some(ctx) = self.ctx.as_ref().and_then(|w| w.upgrade()) {
                handler(ctx, req.clone(), target, flow)
            } else {
                Err(anyhow::anyhow!("App reference not available"))
            }
        } else {
            Err(anyhow::anyhow!("Unknown method: {}", req.1))
        }
    }
}
