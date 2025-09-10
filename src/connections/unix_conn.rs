// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[cfg(unix)]
pub async fn platform_main(
    proxy: crate::utils::FrameEventLoopProxy,
    pending: crate::utils::PendingMap,
    pipe_name: &str,
) -> std::io::Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use tokio::net::UnixListener;

    // Platform Secure Temp Path
    let mut path: PathBuf = std::env::temp_dir();
    path.push(pipe_name);

    // Remove old file if necessary
    let _ = fs::remove_file(&path);

    // Bind listeners
    let listener = UnixListener::bind(&path)?;

    // Secure socket file: only owner can read/write
    let mut perms = fs::metadata(&path)?.permissions();
    perms.set_mode(0o600);
    fs::set_permissions(&path, perms)?;

    // println!("Secure UDS Server runs on{}", path.display());

    loop {
        let (mut stream, _) = listener.accept().await?;
        let proxy = proxy.clone();
        let pending = pending.clone();

        tokio::spawn(async move {
            if let Err(e) =
                crate::connections::handler::handle_client(&mut stream, proxy, pending).await
            {
                eprintln!("[UDS] Client error: {:?}", e);
            }
        });
    }
}
