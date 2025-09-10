// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn handle_client<S>(
    stream: &mut S,
    proxy: crate::utils::FrameEventLoopProxy,
    pending: crate::utils::PendingMap,
) -> tokio::io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    loop {
        // === 1. Länge lesen ===
        let mut len_buf = [0u8; 4];
        if let Err(_) | Ok(0) = stream.read_exact(&mut len_buf).await {
            return Ok(()); // Verbindung beendet
        }
        let len = u32::from_le_bytes(len_buf) as usize;

        // === 2. Nachricht lesen ===
        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf).await?;
        let request_str = match String::from_utf8(buf) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // === 3. JSON in ApiRequest parsen ===
        let req: crate::api_manager::ApiRequest = match serde_json::from_str(&request_str) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("[platform] JSON parse error: {:?}", e);
                continue;
            }
        };

        // === 4. Future registrieren ===
        let (tx, rx) = tokio::sync::oneshot::channel();
        {
            let mut map = pending.lock().unwrap();
            map.insert(req.0.clone(), tx);
        }

        let _ = proxy.send_event(crate::utils::UserEvent::Request(req.clone()));

        // === 5. Antwort senden ===
        let resp = match rx.await {
            Ok(resp) => resp,
            Err(_) => crate::api_manager::ApiResponse(
                req.0,
                500,
                "Internal server error".to_string(),
                serde_json::json!(null),
            ),
        };

        let response_json = serde_json::to_string(&resp)?;
        let resp_bytes = response_json.as_bytes();
        let resp_len = resp_bytes.len() as u32;

        stream.write_all(&resp_len.to_le_bytes()).await?;
        stream.write_all(resp_bytes).await?;
        stream.flush().await?;
    }
}
