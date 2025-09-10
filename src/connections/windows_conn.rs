// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[cfg(windows)]
pub async fn platform_main(
    proxy: crate::utils::FrameEventLoopProxy,
    pending: crate::utils::PendingMap,
    pipe_name: &str,
) -> std::io::Result<()> {
    use tokio::net::windows::named_pipe::ServerOptions;
    use windows_sys::Win32::Security::{
        InitializeSecurityDescriptor, SetSecurityDescriptorDacl, SECURITY_ATTRIBUTES,
        SECURITY_DESCRIPTOR,
    };

    const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
    let pipe_full_name = format!(r"\\.\pipe\{}", pipe_name);

    // 👉 The following applies here: Use descriptors and attributes only temporarily
    let mut server = {
        let mut sd: SECURITY_DESCRIPTOR = unsafe { std::mem::zeroed() };

        let ok = unsafe {
            InitializeSecurityDescriptor(&mut sd as *mut _ as *mut _, SECURITY_DESCRIPTOR_REVISION)
        };
        assert!(ok != 0, "InitSecurityDescriptor failed");

        let ok = unsafe {
            SetSecurityDescriptorDacl(&mut sd as *mut _ as *mut _, 1, std::ptr::null_mut(), 0)
        };
        assert!(ok != 0, "SetSecurityDescriptorDacl failed");

        let mut sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: &mut sd as *mut _ as *mut _,
            bInheritHandle: 0,
        };

        unsafe {
            ServerOptions::new()
                .first_pipe_instance(true)
                .create_with_security_attributes_raw(&pipe_full_name, &mut sa as *mut _ as _)
                .unwrap()
        }
    }; //sd + sa are released again here → Future remains Send

    // println!("Secure Named Pipe Server runs on{}", pipe_full_name);

    loop {
        server.connect().await?;
        let mut inner = server;
        server = ServerOptions::new().create(&pipe_full_name)?;

        let proxy = proxy.clone();
        let pending = pending.clone();

        tokio::spawn(async move {
            if let Err(e) =
                crate::connections::handler::handle_client(&mut inner, proxy, pending).await
            {
                eprintln!("[Pipe] Client error: {:?}", e);
            }
        });
    }
}
