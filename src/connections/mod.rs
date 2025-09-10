// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub mod handler;
pub mod unix_conn;
pub mod utils;
/// Starts the platform-specific connection handler.
///
/// # Purpose
/// This function serves as the entry point for establishing an inter-process
/// communication (IPC) channel. It abstracts away platform differences by
/// delegating to the appropriate backend:
///
/// - On **Windows** → [`windows_conn::platform_main`] is called.
///   - Uses a Named Pipe server with Windows security descriptors.
/// - On **Unix-like systems** → [`unix_conn::platform_main`] is called.
///   - Typically uses a Unix domain socket as the IPC mechanism.
///
/// This allows the rest of the application to work with a unified API,
/// regardless of the underlying operating system.
///
/// # Parameters
/// - `proxy`: A [`FrameEventLoopProxy`] that allows communication with the
///   main event loop for dispatching events or messages.
/// - `pending`: A shared map (`PendingMap`) that tracks pending requests or
///   responses awaiting processing.
/// - `name`: A string identifier used to construct the IPC endpoint
///   (e.g., pipe name on Windows or socket path on Unix).
///
/// # Return Value
/// Returns `Ok(())` if the connection server starts successfully and continues
/// to accept clients. If an error occurs (e.g., endpoint creation fails),
/// a `std::io::Error` is returned.
///
/// # Notes
/// - This function is asynchronous and should be awaited.
/// - The platform-specific implementations define the actual behavior of
///   connection handling and client communication.
/// - Conditional compilation (`#[cfg(windows)]` / `#[cfg(unix)]`) ensures that
///   only the relevant backend is compiled for the target OS.
///
/// [`FrameEventLoopProxy`]: crate::utils::FrameEventLoopProxy
/// [`windows_conn::platform_main`]: crate::windows_conn::platform_main
/// [`unix_conn::platform_main`]: crate::unix_conn::platform_main
pub mod windows_conn;
use pyo3::prelude::*;

#[allow(dead_code)]
pub async fn start_connection(
    proxy: crate::utils::FrameEventLoopProxy,
    pending: super::utils::PendingMap,
    name: String,
) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        return windows_conn::platform_main(proxy, pending, &name).await;
    }

    #[cfg(unix)]
    {
        return unix_conn::platform_main(proxy, pending, &name).await;
    }
}

#[pyo3::pyfunction]
pub fn send_event_over_platform<'py>(
    py: Python<'py>,
    name: String,
    message: String,
) -> PyResult<Bound<'py, PyAny>> {
    #[cfg(windows)]
    let fut = async move {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::windows::named_pipe::ClientOptions;
        use tokio::time::{sleep, Duration};
        use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;

        let pipe_full_name = format!(r"\\.\pipe\{}", name);

        let mut client = loop {
            match ClientOptions::new().open(&pipe_full_name) {
                Ok(c) => break c,
                Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => {}
                Err(e) => return Err(anyhow::Error::from(e)),
            }
            sleep(Duration::from_millis(10)).await;
        };

        // Nachricht mit Länge schicken
        let msg_bytes = message.as_bytes();
        let len = msg_bytes.len() as u32;
        client.write_all(&len.to_le_bytes()).await?;
        client.write_all(msg_bytes).await?;
        client.flush().await?;

        // Antwort lesen
        let mut len_buf = [0u8; 4];
        client.read_exact(&mut len_buf).await?;
        let resp_len = u32::from_le_bytes(len_buf) as usize;

        let mut resp_buf = vec![0u8; resp_len];
        client.read_exact(&mut resp_buf).await?;
        let resp_str = String::from_utf8_lossy(&resp_buf).to_string();

        Ok::<String, anyhow::Error>(resp_str)
    };

    #[cfg(unix)]
    let fut = async move {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::UnixStream;

        let path = format!("/tmp/{}", name);
        let mut stream = UnixStream::connect(&path).await?;

        // Nachricht mit Länge schicken
        let msg_bytes = message.as_bytes();
        let len = msg_bytes.len() as u32;
        stream.write_all(&len.to_le_bytes()).await?;
        stream.write_all(msg_bytes).await?;
        stream.flush().await?;

        // Antwort lesen
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).await?;
        let resp_len = u32::from_le_bytes(len_buf) as usize;

        let mut resp_buf = vec![0u8; resp_len];
        stream.read_exact(&mut resp_buf).await?;
        let resp_str = String::from_utf8_lossy(&resp_buf).to_string();

        Ok::<String, anyhow::Error>(resp_str)
    };

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        match fut.await {
            Ok(resp) => Python::with_gil(|py| utils::json_to_py(py, &resp)),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
        }
    })
}
