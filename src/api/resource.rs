// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::api_manager::ApiManager;
use anyhow::Result;
// use base64::engine::general_purpose::STANDARD;
// use base64::Engine;
// use image::ImageReader;
use mime_guess::from_path;
//  use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use pyorion_macros::api;
use serde::Deserialize;
//  use sha2::{Digest, Sha256};
// use std::collections::HashMap;
use std::fs;
// use std::io::Cursor;
// use std::sync::mpsc::channel;
// use std::sync::mpsc::RecvTimeoutError;
// use std::thread;
// use std::time::{Duration, Instant};
use walkdir::WalkDir;

/// Registrierung aller verfügbaren Ressourcen-APIs
pub fn resource_api(_api_manager: &mut ApiManager) {
    // _api_manager.register_api("resource.watch", watch);
    // _api_manager.register_api("resource.exists", exists);
    // _api_manager.register_api("resource.read", read);
    // _api_manager.register_api("resource.extract", extract);
    _api_manager.register_api("resource.metadata", metadata);
    _api_manager.register_api("resource.list", list);
    _api_manager.register_api("resource.list_recursive", list_recursive);
    _api_manager.register_api("resource.delete", delete);
    _api_manager.register_api("resource.copy", copy);
    // _api_manager.register_api("resource.read_bytes", read_bytes);
    // _api_manager.register_api("resource.read_json", read_json);
    // _api_manager.register_api("resource.mime_type", mime_type);
    // _api_manager.register_api("resource.hash", hash);
    // _api_manager.register_api("resource.translate", translate);
    // _api_manager.register_api("resource.bundle", bundle);
    // _api_manager.register_api("resource.thumbnail", thumbnail);
}

/// Unterstützte Kodierungsarten für das Lesen
#[derive(Deserialize)]
enum EncodeType {
    #[serde(rename = "utf8")]
    Utf8,
    #[serde(rename = "base64")]
    Base64,
    #[serde(rename = "hex")]
    Hex,
}

/// 🔤 Übersetzungsschlüssel aus JSON-Dateien extrahieren
/* #[api]
fn translate(lang_path: String, key: String) -> Result<String> {
    let content = app.resource().load(&lang_path)?;
    let map: HashMap<String, String> = serde_json::from_slice(&content)?;
    Ok(map.get(&key).cloned().unwrap_or_else(|| format!("{{missing:{}}}", key)))
} */

/// 📦 Mehrere Dateien gleichzeitig laden und base64-kodieren
/* #[api]
fn bundle(paths: Vec<String>) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for path in paths {
        let data = app.resource().load(&path)?;
        let encoded = STANDARD.encode(data);
        map.insert(path, encoded);
    }
    Ok(map)
} */

/// 🖼️ Ein Bild verkleinern und als Base64-kodiertes PNG zurückgeben
/* #[api]
fn thumbnail(path: String, max_size: u32) -> Result<String> {
    let data = app.resource().load(&path)?;
    let img = ImageReader::new(Cursor::new(data)).with_guessed_format()?.decode()?;

    let thumbnail = img.thumbnail(max_size, max_size);
    let mut out = Vec::new();
    thumbnail.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png)?;

    Ok(STANDARD.encode(out))
} */
/// Prüft, ob eine Ressource existiert
/* #[api]
fn exists(path: String) -> Result<bool> {
    Ok(app.resource().exists(&path))
} */

/// Liest eine Ressource und kodiert den Inhalt
/* #[api]
fn read(path: String, encode: Option<EncodeType>) -> Result<String> {
    let encode = encode.unwrap_or(EncodeType::Utf8);
    let content = app.resource().load(&path)?;
    let content = match encode {
        EncodeType::Utf8 => String::from_utf8(content)?,
        EncodeType::Base64 => STANDARD.encode(content),
        EncodeType::Hex => hex::encode(content),
    };
    Ok(content)
}
 */
/// Extrahiert (kopiert) eine Ressource in ein Zielverzeichnis
/* #[api]
fn extract(from: String, to: String) -> Result<()> {
    let content = app.resource().load(&from)?;
    fs::write(to, content)?;
    Ok(())
}
 */
/// Gibt grundlegende Metadaten der Datei zurück
#[api]
fn metadata(path: String) -> Result<String> {
    let metadata = fs::metadata(&path)?;
    let info = format!(
        "is_file: {}, is_dir: {}, len: {}",
        metadata.is_file(),
        metadata.is_dir(),
        metadata.len()
    );
    Ok(info)
}

/// Listet alle Einträge im angegebenen Verzeichnis (nicht rekursiv)
#[api]
fn list(dir: String) -> Result<Vec<String>> {
    let entries = fs::read_dir(&dir)?
        .map(|entry| entry.map(|e| e.file_name().into_string().unwrap_or_default()))
        .collect::<std::io::Result<Vec<_>>>()?;
    Ok(entries)
}

/// Listet rekursiv alle Dateien im Verzeichnis
#[api]
fn list_recursive(path: String) -> Result<Vec<String>> {
    let entries = WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect();
    Ok(entries)
}

/// Löscht eine Datei oder ein Verzeichnis rekursiv
#[api]
fn delete(path: String) -> Result<()> {
    if fs::metadata(&path)?.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Kopiert eine Datei von A nach B
#[api]
fn copy(from: String, to: String) -> Result<()> {
    fs::copy(from, to)?;
    Ok(())
}

/// Liest eine Datei und gibt den Inhalt hex-kodiert zurück
/* #[api]
fn read_bytes(path: String) -> Result<String> {
    let bytes = app.resource().load(&path)?;
    Ok(hex::encode(bytes))
}
 */
/// Liest eine JSON-Datei und gibt sie formatiert zurück
/* #[api]
fn read_json(path: String) -> Result<String> {
    let content = app.resource().load(&path)?;
    let json: serde_json::Value = serde_json::from_slice(&content)?;
    Ok(serde_json::to_string_pretty(&json)?)
} */

/// Gibt den MIME-Typ einer Datei anhand ihrer Endung zurück
#[api]
fn mime_type(path: String) -> Result<String> {
    let mime = from_path(&path).first_or_octet_stream();
    Ok(mime.essence_str().to_string())
}

/*
/// Berechnet den SHA-256-Hash einer Datei
#[api]
fn hash(path: String) -> Result<String> {
    let content = app.resource().load(&path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
} */
