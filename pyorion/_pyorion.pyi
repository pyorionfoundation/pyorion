# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

import multiprocessing
from typing import Any

__version__: str
"""The package version as defined in `Cargo.toml`, modified to match python's versioning semantics."""

def create_webframe(
    config: str,
    sock_cfg: str | None,
    uds_name: str,
    close_event: multiprocessing.Event,  # type: ignore
) -> Any: ...
async def send_event_over_platform(
    name: str,
    message: str,
) -> Any: ...
