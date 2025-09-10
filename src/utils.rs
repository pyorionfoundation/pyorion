use crate::api_manager::{ApiRequest, ApiResponse};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::fmt;
use std::path::Path;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tao::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};

#[allow(dead_code)]
pub type FrameEventLoop = EventLoop<UserEvent>;
#[allow(dead_code)]
pub type FrameEventLoopBuilder = EventLoopBuilder<UserEvent>;
#[allow(dead_code)]
pub type FrameEventLoopProxy = EventLoopProxy<UserEvent>;
#[allow(dead_code)]
pub type FrameWindowTarget = EventLoopWindowTarget<UserEvent>;
#[allow(dead_code)]
pub type PendingMap = Arc<Mutex<HashMap<u8, tokio::sync::oneshot::Sender<ApiResponse>>>>;
#[allow(dead_code)]
pub enum UserEvent {
    Request(ApiRequest),
    Shutdown,
}
#[allow(dead_code)]
pub type ArcMut<T> = Arc<Mutex<T>>;
#[allow(dead_code)]
pub fn arc<T>(t: T) -> Arc<T> {
    Arc::new(t)
}
#[allow(dead_code)]
pub fn arc_mut<T>(t: T) -> ArcMut<T> {
    Arc::new(Mutex::new(t))
}
#[allow(dead_code)]
pub struct IdCounter {
    next_id: u8,
}

impl IdCounter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    #[allow(dead_code)]
    pub fn next<T>(&mut self, excludes: &HashMap<u8, T>) -> Result<u8> {
        for _ in 0..u8::MAX {
            let id = self.next_id;
            if excludes.contains_key(&id) {
                self.next_id += 1;
                continue;
            }
            return Ok(id);
        }
        Err(anyhow!("Failed to find a valid id."))
    }
}

#[macro_export]
macro_rules! unsafe_impl_sync_send {
    ($type:ty) => {
        unsafe impl Send for $type {}
        unsafe impl Sync for $type {}
    };
}

#[macro_export]
macro_rules! set_property_some {
    ($builder:ident, $property:ident, &$value:expr) => {
        if let Some(value) = &$value {
            $builder = $builder.$property(value);
        }
    };
    ($builder:ident, $property:ident, $value:expr) => {
        if let Some(value) = $value {
            $builder = $builder.$property(value.clone());
        }
    };
}

#[macro_export]
macro_rules! set_property {
    ($builder:ident, $property:ident, $value:expr) => {
        $builder = $builder.$property($value);
    };
}

#[macro_export]
macro_rules! lock {
    ($value:expr) => {
        $value
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock {}.", stringify!($value)))
    };
}

#[macro_export]
macro_rules! lock_force {
    ($value:expr) => {
        $value.lock().unwrap()
    };
}

#[macro_export]
macro_rules! logical {
    ($window:expr, $method:ident) => {
        $window.$method().to_logical::<f64>($window.scale_factor())
    };

    ($window:expr, $item:expr, $method:ident) => {
        $item.$method().to_logical::<f64>($window.scale_factor())
    };
}

#[macro_export]
macro_rules! logical_try {
    ($window:expr, $method:ident) => {
        $window.$method()?.to_logical::<f64>($window.scale_factor())
    };
}

#[macro_export]
macro_rules! log_if_err {
    ($result:expr) => {
        if let Err(e) = $result {
            println!("[Error]: {}", e);
        }
    };
}

#[macro_export]
macro_rules! log {
    ($result:expr) => {
        println!("[Info]: {}", $result);
    };
}

#[macro_export]
macro_rules! log_err {
    ($result:expr) => {
        println!("[Error]: {}", $result);
    };
}
#[allow(dead_code)]
pub fn merge_values(dest: Value, src: Value) -> Value {
    match (dest, src) {
        (Value::Null, src) => src,
        (dest, Value::Null) => dest,
        (Value::Object(mut dest_map), Value::Object(src_map)) => {
            for (key, src_val) in src_map {
                let dest_val = dest_map.entry(key).or_insert(Value::Null);
                *dest_val = merge_values(dest_val.take(), src_val);
            }
            Value::Object(dest_map)
        }
        (_, src) => src,
    }
}

#[macro_export]
macro_rules! try_or_log_err {
    ($body:block ) => {
        match (move || -> anyhow::Result<()> { $body })() {
            Ok(_) => {}
            Err(e) => {
                crate::log_err!(e);
            }
        }
    };
}
#[allow(dead_code)]
pub fn url_join(left: &str, right: &str) -> String {
    if right.is_empty() {
        left.to_string()
    } else if left.ends_with("/") {
        format!("{}{}", left, right)
    } else {
        format!("{}/{}", left, right)
    }
}
#[allow(dead_code)]
pub fn merge_id(window_id: u8, item_id: u8) -> u16 {
    ((window_id as u16) << 8) | (item_id as u16)
}
#[allow(dead_code)]
pub fn split_id(merged_id: u16) -> (u8, u8) {
    ((merged_id >> 8) as u8, merged_id as u8)
}

const MIMETYPE_PLAIN: &str = "text/plain";

#[allow(dead_code)]
pub enum MimeType {
    Css,
    Csv,
    Html,
    Ico,
    Js,
    Json,
    Jsonld,
    Mp4,
    OctetStream,
    Rtf,
    Svg,
    Txt,
    Jpeg,
    Gif,
    Png,
    Webp,
    Tiff,
    Bmp,
    AudioAac,
    AudioMpeg,
    AudioOgg,
    AudioWav,
    AudioWebm,
    AudioXMpegurl,
    Pdf,
    Zip,
    Gzip,
    Tar,
    SevenZip,
    MsWord,
    MsExcel,
    MsPowerPoint,
    Wasm,
    Xml,
    Markdown,
    FontWoff,
    FontWoff2,
    FontTtf,
    FontOtf,
    Php,
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mime = match self {
            MimeType::Css => "text/css",
            MimeType::Csv => "text/csv",
            MimeType::Html => "text/html",
            MimeType::Ico => "image/vnd.microsoft.icon",
            MimeType::Js => "text/javascript",
            MimeType::Json => "application/json",
            MimeType::Jsonld => "application/ld+json",
            MimeType::Mp4 => "video/mp4",
            MimeType::OctetStream => "application/octet-stream",
            MimeType::Rtf => "application/rtf",
            MimeType::Svg => "image/svg+xml",
            MimeType::Txt => MIMETYPE_PLAIN,
            MimeType::Jpeg => "image/jpeg",
            MimeType::Gif => "image/gif",
            MimeType::Png => "image/png",
            MimeType::Webp => "image/webp",
            MimeType::Tiff => "image/tiff",
            MimeType::Bmp => "image/bmp",
            MimeType::AudioAac => "audio/aac",
            MimeType::AudioMpeg => "audio/mpeg",
            MimeType::AudioOgg => "audio/ogg",
            MimeType::AudioWav => "audio/wav",
            MimeType::AudioWebm => "audio/webm",
            MimeType::AudioXMpegurl => "audio/x-mpegurl",
            MimeType::Pdf => "application/pdf",
            MimeType::Zip => "application/zip",
            MimeType::Gzip => "application/gzip",
            MimeType::Tar => "application/x-tar",
            MimeType::SevenZip => "application/x-7z-compressed",
            MimeType::MsWord => "application/msword",
            MimeType::MsExcel => "application/vnd.ms-excel",
            MimeType::MsPowerPoint => "application/vnd.ms-powerpoint",
            MimeType::Wasm => "application/wasm",
            MimeType::Xml => "text/xml",
            MimeType::Markdown => "text/markdown",
            MimeType::FontWoff => "font/woff",
            MimeType::FontWoff2 => "font/woff2",
            MimeType::FontTtf => "font/ttf",
            MimeType::FontOtf => "font/otf",
            MimeType::Php => "application/x-php",
        };
        write!(f, "{mime}")
    }
}

impl MimeType {
    #[allow(dead_code)]
    pub fn parse_from_uri(uri: &str) -> MimeType {
        Self::parse_from_uri_with_fallback(uri, MimeType::Html)
    }

    pub fn parse_from_uri_with_fallback(uri: &str, fallback: MimeType) -> MimeType {
        let suffix = uri.split('.').next_back();
        match suffix {
            Some("bin") => Self::OctetStream,
            Some("css" | "less" | "sass" | "styl") => Self::Css,
            Some("csv") => Self::Csv,
            Some("html") => Self::Html,
            Some("ico") => Self::Ico,
            Some("js") => Self::Js,
            Some("json") => Self::Json,
            Some("jsonld") => Self::Jsonld,
            Some("mjs") => Self::Js,
            Some("mp4") => Self::Mp4,
            Some("rtf") => Self::Rtf,
            Some("svg") => Self::Svg,
            Some("txt") => Self::Txt,
            Some("jpeg" | "jpg") => Self::Jpeg,
            Some("gif") => Self::Gif,
            Some("png") => Self::Png,
            Some("webp") => Self::Webp,
            Some("tiff") => Self::Tiff,
            Some("bmp") => Self::Bmp,
            Some("aac") => Self::AudioAac,
            Some("mp3" | "mpeg") => Self::AudioMpeg,
            Some("ogg") => Self::AudioOgg,
            Some("wav") => Self::AudioWav,
            Some("webm") => Self::AudioWebm,
            Some("m3u8") => Self::AudioXMpegurl,
            Some("pdf") => Self::Pdf,
            Some("zip") => Self::Zip,
            Some("gzip") => Self::Gzip,
            Some("tar") => Self::Tar,
            Some("7z") => Self::SevenZip,
            Some("doc") => Self::MsWord,
            Some("xls") => Self::MsExcel,
            Some("ppt") => Self::MsPowerPoint,
            Some("wasm") => Self::Wasm,
            Some("xml") => Self::Xml,
            Some("md" | "markdown") => Self::Markdown,
            Some("woff") => Self::FontWoff,
            Some("woff2") => Self::FontWoff2,
            Some("ttf") => Self::FontTtf,
            Some("otf") => Self::FontOtf,
            Some("php") => Self::Php,
            _ => fallback, // Handle the `None` case or any other unhandled cases
        }
    }
    #[allow(dead_code)]
    pub fn parse(content: &[u8], uri: &str) -> String {
        Self::parse_with_fallback(content, uri, MimeType::Html)
    }
    #[allow(dead_code)]
    pub fn parse_with_fallback(content: &[u8], uri: &str, fallback: MimeType) -> String {
        let mime = infer::get(content).map(|info| info.mime_type());

        match mime {
            Some(mime) if mime == MIMETYPE_PLAIN => {
                Self::parse_from_uri_with_fallback(uri, fallback).to_string()
            }
            None => Self::parse_from_uri_with_fallback(uri, fallback).to_string(),
            Some(mime) => mime.to_string(),
        }
    }
}

#[allow(dead_code)]
fn get_wry_response(
    request: wry::http::Request<Vec<u8>>,
    index_page: Option<String>, // Default index page filename
    root: &str,
) -> Result<wry::http::Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let path = request.uri().path();
    let root = std::path::PathBuf::from(root);
    let file_path: String = if path == "/" {
        match index_page {
            Some(index) => index,
            None => "index.html".to_string(),
        }
    } else {
        path[1..].to_string()
    };

    let content = std::fs::read(std::fs::canonicalize(root.join(&file_path))?)?;

    // Dynamically determine MIME
    let mime_type = MimeType::parse_from_uri(&file_path).to_string();
    // Create and return the HTTP response
    wry::http::Response::builder()
        .header(wry::http::header::CONTENT_TYPE, mime_type)
        .body(content)
        .map_err(Into::into)
}

fn split_root_and_index(input: &str) -> Result<(String, String), String> {
    if input.trim().is_empty() {
        return Err("Path must not be empty".to_string());
    }

    let path = Path::new(input);

    if path.is_dir() {
        // if only directory → index.html as standard
        return Ok((input.to_string(), "index.html".to_string()));
    }

    // Determine root
    let root_path = path
        .parent()
        .map(|p| {
            let s = p.to_string_lossy().to_string();
            if s.is_empty() {
                ".".to_string()
            } else {
                s
            }
        })
        .unwrap_or_else(|| ".".to_string());

    // Determine index_page
    let index_page = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "index.html".to_string());

    Ok((root_path, index_page))
}

pub fn render_protocol<'a>(
    mut builder: wry::WebViewBuilder<'a>,
    root_path: Option<String>,
) -> wry::WebViewBuilder<'a> {
    let main_root = root_path.unwrap_or_else(|| ".".to_string());

    if main_root.starts_with("http://") || main_root.starts_with("https://") {
        // URL
        return builder.with_url(&main_root);
    }

    if main_root.contains("<html>") || main_root.contains("<!DOCTYPE html>") {
        return builder.with_html(&main_root);
    }

    let (main_root_clone, index_page) = match split_root_and_index(&main_root) {
        Ok((root, index)) => (root, index),
        Err(e) => {
            eprintln!("❌ Error in split root and index: {}", e);
            return builder; // abort, build WebView without protocol
        }
    };
    builder = builder.with_asynchronous_custom_protocol(
        "wry".into(),
        move |_webview_id, request, responder| {
            let response = get_wry_response(
                request,
                Some(index_page.clone()), // Standard Index
                &main_root_clone,         // Root directory
            );

            match response {
                Ok(http_response) => responder.respond(http_response),
                Err(e) => responder.respond(
                    wry::http::Response::builder()
                        .header(wry::http::header::CONTENT_TYPE, "text/plain")
                        .status(500)
                        .body(e.to_string().as_bytes().to_vec())
                        .unwrap(),
                ),
            }
        },
    );

    let url = format!("wry://localhost");
    builder.with_url(&url)
}

// #[allow(dead_code)]
// pub fn auto_bounds(
// width: i32,
// height: i32,
// index: usize,
// total: usize,
// ) -> managers::window::options::WebViewBounds {
// Spalten/Zeilen berechnen (Grid)
// let cols = (total as f64).sqrt().ceil() as i32;
// let rows = ((total as f64) / (cols as f64)).ceil() as i32;

// let col = (index as i32) % cols;
// let row = (index as i32) / cols;

// managers::window::options::WebViewBounds {
// position: managers::window::options::Position {
// x: Some((width / cols) * col),
// y: Some((height / rows) * row),
// unit: managers::window::options::UnitType::Logical,
// },
// size: managers::window::options::Size {
// width: Some(width / cols),
// height: Some(height / rows),
// unit: managers::window::options::UnitType::Logical,
// },
// }
// }
